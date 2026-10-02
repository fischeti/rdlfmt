//! Hand-written recursive-descent parser producing a lossless rowan tree.
//!
//! # Trivia convention
//!
//! Whitespace and comments are never skipped, only placed. As in
//! rust-analyzer:
//!
//! * **Leading trivia belongs to the item that follows it**, so a blank line
//!   before a register is part of that register.
//! * **A comment on the same line as the preceding token stays with it**, so in
//!   `sw = rw; // writable` the comment is inside the assignment it annotates.
//!
//! The first follows from flushing trivia lazily, when the next real token is
//! consumed; the second from `Parser::finish_stmt`.
//!
//! # Preprocessor directives
//!
//! A formatter is handed unpreprocessed files, so Clause 16 directives must
//! survive the parse. Every one of them, conditionals included, is trivia: one
//! token covering its whole line, invisible to the parser.
//!
//! For a conditional that looks reckless, since its branches may trade a brace:
//!
//! ```text
//! `ifdef A
//! addrmap top {
//! `else
//! regfile top {
//! `endif
//! ```
//!
//! It is safe because preprocessing depends only on the token sequence and on
//! each directive owning its line. [`crate::format`] verifies the first and the
//! formatter guarantees the second, so the preprocessed result is unchanged for
//! every set of macro definitions. The case above reads as `addrmap top {
//! regfile top {`, whose braces do not balance, so it is refused as a parse
//! error.
//!
//! A macro *reference* is not trivia but an atom that may stand for a value or
//! a name; see [`SyntaxKind::is_ident_like`].
//!
//! # Divergences from `SystemRDL.g4`
//!
//! * Statement terminators are inside the statement node rather than siblings
//!   of it, which keeps a trailing comment with its statement.
//! * `component_def`'s four alternatives are parsed as one superset. This is
//!   not a validator.
//! * Pure-alternation rules with no formatting decision (`literal`, `number`,
//!   `udp_attr`, `struct_type`) are flattened away.

mod grammar;

use crate::syntax::kind::SyntaxKind;
use crate::syntax::lexer::{Lexed, lex};
use crate::syntax::tree::SyntaxNode;
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};
use std::ops::Range;

/// A syntax error, reported without stopping the parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    /// Byte offsets into the source.
    pub range: Range<usize>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{}: {}",
            self.range.start, self.range.end, self.message
        )
    }
}

/// The result of parsing: always a complete tree, plus any errors found.
/// Unparsable stretches are wrapped in [`SyntaxKind::ERROR`] nodes.
#[derive(Debug, Clone)]
pub struct Parsed {
    green: GreenNode,
    errors: Vec<ParseError>,
}

impl Parsed {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn ok(&self) -> Option<SyntaxNode> {
        self.errors.is_empty().then(|| self.syntax())
    }
}

/// Parses SystemRDL source into a lossless syntax tree.
pub fn parse(src: &str) -> Parsed {
    let mut p = Parser::new(src);
    grammar::source_file(&mut p);
    p.finish()
}

/// How deeply constructs may nest before the parser refuses the input.
///
/// Parsing and formatting both recurse once per level, so deeper input would
/// overflow the stack. The bound keeps the deepest admitted input formattable
/// on a 2 MiB stack, the default for a spawned thread, in a debug build.
const MAX_DEPTH: usize = 256;

pub(crate) struct Parser<'a> {
    tokens: Lexed<'a>,
    /// Index into `tokens`, counting trivia.
    pos: usize,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<ParseError>,
    /// Levels of nesting entered and not yet left. See [`Parser::nested`].
    depth: usize,
    /// Whether `depth` has exceeded [`MAX_DEPTH`]. If so, the rest of the input
    /// has been consumed and further errors are suppressed.
    too_deep: bool,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Parser {
            tokens: lex(src),
            pos: 0,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
            depth: 0,
            too_deep: false,
        }
    }

    fn finish(self) -> Parsed {
        // Trailing trivia must already have been flushed *inside* the root node
        // by `grammar::source_file`; flushing here would emit it as a second
        // root-level child, which rowan rejects.
        debug_assert_eq!(self.pos, self.tokens.len(), "tokens left unconsumed");
        Parsed {
            green: self.builder.finish(),
            errors: self.errors,
        }
    }

    //----------------------------------------------------------------------
    // Lookahead. All of it skips trivia -- the parser never makes a decision
    // based on whitespace.
    //----------------------------------------------------------------------

    /// Index of the `n`th significant token ahead; `n == 0` is the current one.
    ///
    /// `None` means there is no such token, which the callers below report as
    /// [`SyntaxKind::EOF`].
    fn nth_index(&self, n: usize) -> Option<usize> {
        (self.pos..self.tokens.len())
            .filter(|&i| !self.tokens.kind(i).is_trivia())
            .nth(n)
    }

    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        self.nth_index(n)
            .map_or(SyntaxKind::EOF, |i| self.tokens.kind(i))
    }

    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    pub(crate) fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.contains(&self.current())
    }

    pub(crate) fn at_end(&self) -> bool {
        self.at(SyntaxKind::EOF)
    }

    /// Byte range of the current significant token, for error reporting.
    fn current_range(&self) -> Range<usize> {
        // Past the end, this is an empty range at the end of input.
        let range = self
            .tokens
            .range(self.nth_index(0).unwrap_or(self.tokens.len()));
        usize::from(range.start())..usize::from(range.end())
    }

    //----------------------------------------------------------------------
    // Tree building
    //----------------------------------------------------------------------

    /// Copies token `i` into the tree.
    fn push(&mut self, i: usize) {
        self.builder
            .token(self.tokens.kind(i).into(), self.tokens.text(i));
    }

    /// Emits pending trivia. Called lazily, so that a node opened beforehand
    /// takes the trivia as its leading content.
    pub(crate) fn flush_trivia(&mut self) {
        while self.tokens.kind(self.pos).is_trivia() {
            self.push(self.pos);
            self.pos += 1;
        }
    }

    /// Consumes the current significant token into the tree.
    pub(crate) fn bump(&mut self) {
        self.flush_trivia();
        if self.pos < self.tokens.len() {
            self.push(self.pos);
            self.pos += 1;
        }
    }

    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Consumes `kind`, or records an error and consumes nothing.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {kind:?}, found {:?}", self.current()));
        false
    }

    pub(crate) fn error(&mut self, message: impl Into<String>) {
        if self.too_deep {
            return;
        }
        let range = self.current_range();
        self.errors.push(ParseError {
            message: message.into(),
            range,
        });
    }

    /// Records an error and consumes the offending token inside an ERROR node,
    /// so the parser always makes progress.
    pub(crate) fn error_and_bump(&mut self, message: impl Into<String>) {
        self.start_node(SyntaxKind::ERROR);
        self.error(message);
        if !self.at_end() {
            self.bump();
        }
        self.finish_node();
    }

    /// Runs `f` one level of nesting deeper, or gives up on the input if that is
    /// too deep.
    ///
    /// Giving up records one error and consumes the rest of the input into the
    /// tree, which stays lossless, and leaves every rule on the stack at the
    /// end of input, so the parse unwinds without recursing further.
    pub(crate) fn nested(&mut self, f: impl FnOnce(&mut Self)) {
        if self.enter() {
            f(self);
        }
        self.leave();
    }

    /// Enters one level of nesting, returning whether that was allowed. Each
    /// call must be matched by a [`Parser::leave`], allowed or not.
    pub(crate) fn enter(&mut self) -> bool {
        self.depth += 1;
        if self.depth <= MAX_DEPTH {
            return true;
        }
        if !self.too_deep {
            self.error(format!("nested more than {MAX_DEPTH} levels deep"));
            self.too_deep = true;
            while self.pos < self.tokens.len() {
                self.push(self.pos);
                self.pos += 1;
            }
        }
        false
    }

    pub(crate) fn leave(&mut self) {
        self.depth -= 1;
    }

    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    pub(crate) fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub(crate) fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub(crate) fn start_node_at(&mut self, cp: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(cp, kind.into());
    }

    /// Pulls a comment on the same line into the node being closed. A block
    /// comment spanning lines does not qualify: it introduces what follows.
    pub(crate) fn eat_trailing_comment(&mut self) {
        let mut i = self.pos;
        while self.tokens.kind(i) == SyntaxKind::WHITESPACE && !self.tokens.text(i).contains('\n') {
            i += 1;
        }
        if !(self.tokens.kind(i).is_comment() && !self.tokens.text(i).contains('\n')) {
            return;
        }
        for j in self.pos..=i {
            self.push(j);
        }
        self.pos = i + 1;
    }

    /// Closes a statement node, taking any trailing comment with it.
    pub(crate) fn finish_stmt(&mut self) {
        self.eat_trailing_comment();
        self.finish_node();
    }
}
