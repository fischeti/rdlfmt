//! Hand-written recursive-descent parser producing a lossless rowan tree.
//!
//! # Trivia convention
//!
//! Whitespace and comments are never skipped, only *placed*. The rule, which
//! matches rust-analyzer's, is:
//!
//! * **Leading trivia belongs to the item that follows it.** A run of
//!   whitespace and comments ends up inside the node that starts after it,
//!   which is what makes "blank line before this register" a property of the
//!   register rather than of its neighbour.
//! * **A comment on the same line as the preceding token stays with it.** So
//!   in `sw = rw; // software may write`, the comment lands inside the property
//!   assignment it annotates rather than leading the next one.
//!
//! Mechanically this falls out of two things: trivia is flushed lazily (only
//! when a real token is emitted, so opening a node first captures it), and
//! [`Parser::eat_trailing_comment`] is called just before closing a statement.
//!
//! # Preprocessor directives
//!
//! An unpreprocessed file is the only kind a formatter is ever handed, so
//! Clause 16 directives have to survive the parse. Every one of them is trivia,
//! conditionals included, and each is a single token covering its whole line --
//! so the parser is not merely tolerant of `` `ifdef `` but blind to it.
//!
//! For `` `define `` and friends that is obviously fine. For a conditional it
//! looks reckless, since the branches of one may hand a brace back and forth:
//!
//! ```text
//! `ifdef A
//! addrmap top {
//! `else
//! regfile top {
//! `endif
//! ```
//!
//! What makes it safe is that a formatter has no obligation to *understand* a
//! conditional, only to avoid breaking it -- and preprocessing depends on
//! nothing but the sequence of tokens and the rule that a directive owns its
//! line. `systemrdl_fmt::format` verifies the first (its output lexes to the
//! same token stream as its input) and the formatter guarantees the second, so
//! whatever it does with the whitespace in between, the preprocessed result is
//! unchanged for every set of macro definitions at once.
//!
//! The pathological case above is still not formatted -- with the directives
//! invisible it reads as `addrmap top { regfile top {`, whose braces do not
//! balance, and the ordinary error path refuses it. That is the whole
//! mechanism: no region analysis, no branch enumeration, and no way for a
//! conditional to produce plausible-looking but wrong output.
//!
//! A macro *reference* is not trivia: it is an atom that may stand for a value
//! or for a name, which is why [`SyntaxKind::is_ident_like`] admits it.
//!
//! # Divergences from `SystemRDL.g4`
//!
//! The tree shape is tuned for formatting rather than mirroring the reference
//! grammar exactly:
//!
//! * Statement terminators are pulled *inside* the statement node. The grammar
//!   has `root: (root_elem ';')*`, which would leave every `;` as a sibling of
//!   the thing it terminates -- awkward when deciding where a trailing comment
//!   goes.
//! * `component_def`'s four alternatives are parsed as one linear superset,
//!   since a formatter has no reason to reject an anonymous definition that is
//!   missing its instantiation list. Over-acceptance is deliberate: this is not
//!   a validator, and refusing to format questionable input is worse than
//!   formatting it.
//! * Pure-alternation rules that carry no formatting decision (`literal`,
//!   `number`, `udp_attr`, `struct_type`) are flattened away.

mod grammar;

use crate::kind::SyntaxKind;
use crate::lexer::{Lexed, lex};
use crate::syntax::SyntaxNode;
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
///
/// A tree is produced even for badly broken input -- unparseable stretches are
/// wrapped in [`SyntaxKind::ERROR`] nodes rather than discarded. Check
/// [`Parsed::errors`] before formatting; rewriting a file the parser did not
/// fully understand is how a formatter corrupts code.
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

pub(crate) struct Parser<'a> {
    tokens: Lexed<'a>,
    /// Index into `tokens`, counting trivia.
    pos: usize,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Parser {
            tokens: lex(src),
            pos: 0,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
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
    ///
    /// [`ParseError`] stays on `usize` -- it is the crate's public diagnostic
    /// type, and `&src[err.range]` should just work for callers.
    fn current_range(&self) -> Range<usize> {
        // `Lexed::range` collapses a past-the-end index to an empty range at
        // the end of input, which is where an "unexpected EOF" belongs.
        let range = self
            .tokens
            .range(self.nth_index(0).unwrap_or(self.tokens.len()));
        usize::from(range.start())..usize::from(range.end())
    }

    //----------------------------------------------------------------------
    // Tree building
    //----------------------------------------------------------------------

    /// Copies token `i` into the tree verbatim.
    ///
    /// `Lexed::text` borrows from the source rather than from `self.tokens`,
    /// so this needs no dance to keep `self.builder` mutably reachable.
    fn push(&mut self, i: usize) {
        self.builder
            .token(self.tokens.kind(i).into(), self.tokens.text(i));
    }

    /// Emits pending trivia. Called lazily, so that a node opened beforehand
    /// captures the trivia as its own leading content.
    pub(crate) fn flush_trivia(&mut self) {
        // Past-the-end reads as EOF, which is not trivia, so this also
        // terminates at the end of input.
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

    /// Pulls a same-line trailing comment into the node being closed.
    ///
    /// Only same-line, single-line comments qualify: a block comment spanning
    /// newlines reads as introducing what comes next, not annotating what came
    /// before. Note a `LINE_COMMENT` token stops before its newline, so the
    /// newline correctly stays behind as leading trivia for the next item.
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
