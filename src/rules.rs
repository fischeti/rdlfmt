//! Per-node formatting rules.
//!
//! One function per group of node kinds, reached through the [`format_node`]
//! dispatch. Every kind has a rule except [`SyntaxKind::ERROR`], which falls
//! through to [`Formatter::verbatim`] -- see its docs for why that arm stays.
//!
//! Four rules cover the language. [`spaced`] and [`tight`] between them handle
//! almost everything, because with expressions never breaking, the question at
//! most nodes is only whether their parts are separate words or one word.
//! [`braced_body`] and [`param_list`] are the two that lay anything out.
//!
//! # The shape every rule has
//!
//! A rule walks `children_with_tokens()` and does three things with what it
//! finds: hands trivia to [`Formatter::trivia`], requests separation before
//! each significant child, and recurses. Trivia is handled *in place* rather
//! than hoisted out, which is what lets a comment buried at the front of a
//! deeply nested child still be emitted before the child's first token --
//! recursion reaches it in source order without anyone having to look for it.
//!
//! # Who decides what
//!
//! A rule separates its own children from *each other* and never says what
//! comes before its first one. That belongs to the parent, which is the only
//! one that knows: `sw` needs a space in front of it in `default sw = rw`, and
//! none in `a.b->sw`, and [`normal_prop_assign`](SyntaxKind::NORMAL_PROP_ASSIGN)
//! cannot tell which it is in. Getting this backwards -- having each rule pad
//! its own left edge -- is what forces formatters into trimming passes.
//!
//! Requests are minimums that combine by [`Ord::max`], so a rule states the
//! least separation its construct needs and never has to consider what the
//! surrounding one asked for. `reg my_reg` needs a space between the two; the
//! statement being the first in a body, and so wanting a newline in front of
//! `reg`, is not that rule's problem.

use crate::formatter::{Formatter, Sep};
use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

pub(crate) fn format_node(f: &mut Formatter, node: &SyntaxNode) {
    use SyntaxKind::*;
    match node.kind() {
        SOURCE_FILE => source_file(f, node),

        COMPONENT_BODY | UDP_BODY | ENUM_BODY | ENUM_ENTRY_BODY | STRUCT_BODY | CONSTRAINT_BODY => {
            braced_body(f, node)
        }

        // Everything whose parts read as a sentence: keywords, names, types,
        // operators and their operands, separated by single spaces.
        COMPONENT_DEF
        | COMPONENT_NAMED_DEF
        | COMPONENT_ANON_DEF
        | UDP_DEF
        | ENUM_DEF
        | ENUM_ENTRY
        | STRUCT_DEF
        | CONSTRAINT_DEF
        | CONSTRAINT_NAMED_DEF
        | CONSTRAINT_ANON_DEF
        | CONSTRAINT_INSTS
        | LOCAL_PROPERTY_ASSIGNMENT
        | NORMAL_PROP_ASSIGN
        | ENCODE_PROP_ASSIGN
        | PROP_MOD_ASSIGN
        | PROP_KEYWORD
        | PROP_MOD
        | ENUM_PROP_ASSIGN
        | EXPLICIT_COMPONENT_INST
        | COMPONENT_INSTS
        | COMPONENT_INST
        | COMPONENT_INST_ALIAS
        | COMPONENT_TYPE
        | COMPONENT_INST_TYPE
        | FIELD_INST_RESET
        | INST_ADDR_FIXED
        | INST_ADDR_STRIDE
        | INST_ADDR_ALIGN
        | DATA_TYPE
        | BASIC_DATA_TYPE
        | STRUCT_ELEM
        | UDP_TYPE
        | UDP_USAGE
        | UDP_DEFAULT
        | UDP_CONSTRAINT
        | UDP_DATA_TYPE
        | UDP_COMP_TYPE
        // Expressions never break, so an operator is just another part of the
        // sentence: `a + b`, `a ? b : c`, `longint unsigned WIDTH = 32`.
        | BINARY_EXPR
        | TERNARY_EXPR
        | PARAM_DEF_ELEM
        // Constraints: `this > 0`, `this inside myEnum`, `a = 1`.
        | CONSTR_RELATIONAL
        | CONSTR_PROP_ASSIGN
        | CONSTR_INSIDE_ENUM => spaced(f, node),

        // Everything that reads as one word. A reference and its subscripts are
        // a single name (`a.b[0].c`), and an arrow binds as tightly as the dot
        // does (`a.b->sw`) -- the property assignment hanging off it spaces
        // itself from the inside, which is why the arrow needs no rule of its
        // own.
        INSTANCE_REF
        | INSTANCE_REF_ELEMENT
        | PROP_REF
        | DYNAMIC_PROPERTY_ASSIGNMENT
        | ARRAY_SUFFIX
        | RANGE_SUFFIX
        | ARRAY_TYPE_SUFFIX
        // A prefix operator, a bracketing, or a cast binds to its operand:
        // `-a`, `(a + b)`, `bit'(x)`, `32'(x)`, `.WIDTH(8)`, `A::B`, `a:1`.
        | UNARY_EXPR
        | PAREN_EXPR
        | LITERAL
        | ENUM_LITERAL
        | CAST_TYPE
        | CAST_WIDTH
        | PARAM_ASSIGNMENT
        // `this`, and a single `inside` value: `4` or the range `[3:4]`.
        | CONSTR_LHS
        | CONSTR_INSIDE_VALUE => tight(f, node),

        STRUCT_KV => struct_kv(f, node),
        CONSTR_INSIDE_VALUES => inside_values(f, node),

        // Comma-separated lists that are part of an expression, and so never
        // break however many elements they hold. A macro call belongs here
        // rather than with `PARAM_INST`: it is an atom in an expression, and
        // breaking `` `MAX(a, b) `` across lines would read as a construct of
        // its own when it stands for a single value.
        CONCATENATE | REPLICATE | ARRAY_LITERAL | STRUCT_LITERAL | MACRO_CALL => {
            flat_list(f, node)
        }

        // The only construct in the language whose layout is in question.
        PARAM_DEF | PARAM_INST => param_list(f, node),

        _ => f.verbatim(node),
    }
}

/// Top-level items, one per line, with blank lines between them preserved.
///
/// The `Sep::Newline` request before each item is what makes the author's blank
/// lines count for anything: one arrives later, as the item's own leading
/// trivia, and widens the break this asked for.
fn source_file(f: &mut Formatter, node: &SyntaxNode) {
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            // The grammar wraps every top-level construct in a node, so a bare
            // token here is stray input the parser could not place. Give it a
            // line of its own rather than letting it run into a neighbour.
            NodeOrToken::Token(tok) => {
                f.request(Sep::Newline);
                f.token(&tok);
            }
            NodeOrToken::Node(item) => {
                f.request(Sep::Newline);
                format_node(f, &item);
            }
        }
    }
}

/// `{ ... }` -- the one layout in the language that is never in question.
///
/// The style guide asks for the opening brace on the line of the statement that
/// owns it, the contents indented one level, and the closing brace alone on its
/// line. There is no width to measure and no alternative to weigh, which is the
/// whole reason this formatter needs no document IR.
///
/// Two exceptions, both from the style guide: an empty body has nothing to
/// indent, and `sw`/`hw` may share a line. See [`shares_line_with`].
fn braced_body(f: &mut Formatter, node: &SyntaxNode) {
    // A floor rather than a decision: whatever the owning statement wanted, `{`
    // may not abut the name in front of it.
    f.request(Sep::Space);

    if is_empty(node) {
        // Whitespace between the braces is dropped rather than routed through
        // `trivia`: with nothing between `{` and `}` to separate, there is
        // nothing for it to say.
        for child in node.children_with_tokens() {
            if let NodeOrToken::Token(tok) = child
                && !tok.kind().is_trivia()
            {
                f.token(&tok);
            }
        }
        return;
    }

    let mut prev: Option<SyntaxNode> = None;
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::L_BRACE => {
                f.token(&tok);
                f.indent();
                f.settle_width();
            }
            NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::R_BRACE => {
                f.dedent();
                // Pinned, not requested: a blank line in front of `}` is an
                // editing artefact rather than a grouping to preserve.
                f.pin(Sep::Newline);
                f.token(&tok);
            }
            NodeOrToken::Token(tok) => {
                f.request(Sep::Newline);
                f.token(&tok);
            }
            NodeOrToken::Node(item) => {
                f.request(if shares_line_with(&item, prev.as_ref()) {
                    Sep::Space
                } else {
                    Sep::Newline
                });
                format_node(f, &item);
                prev = Some(item);
            }
        }
    }
}

/// Children separated by single spaces, terminators attached.
///
/// The default for anything built out of keywords, names and operators, which
/// is most of the language: `reg my_reg #(...)`, `default regwidth = 32`,
/// `longint unsigned WIDTH`, `alias foo`, `@ 0x10`. The style guide asks for a
/// space on both sides of every assignment and expression operator, and this is
/// what provides it.
///
/// Two things are tight instead. A `;` or `,` attaches to what precedes it,
/// closing brace included, so a component definition ends `};`. And a subscript
/// is part of the name it follows, so `STATUS[7:0]` and `data[4]` do not come
/// apart.
fn spaced(f: &mut Formatter, node: &SyntaxNode) {
    let mut first = true;
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) => {
                if !first && !is_terminator(&tok) {
                    f.request(Sep::Space);
                }
                f.token(&tok);
                first = false;
            }
            NodeOrToken::Node(child) => {
                if !first && !is_suffix(child.kind()) {
                    f.request(Sep::Space);
                }
                format_node(f, &child);
                first = false;
            }
        }
    }
}

/// Children with nothing between them.
///
/// For constructs that are one lexical unit despite having structure:
/// `a.b[0].c`, `[7:0]`, `->sw`. Nothing here requests separation, so the tokens
/// land exactly as adjacent as they were written -- but trivia still routes
/// normally, so a comment wedged into a reference is not silently lost.
fn tight(f: &mut Formatter, node: &SyntaxNode) {
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) => f.token(&tok),
            NodeOrToken::Node(child) => format_node(f, &child),
        }
    }
}

/// `this inside {1, 2, [3:4]};`
///
/// The one node that needs both shapes at once. Up to the brace it reads as a
/// sentence, so `this` and `inside` are spaced; from the brace on it is a value
/// list like a concatenation, so the delimiters attach to their contents.
///
/// Not folded into [`flat_list`], which cannot help here: the space belongs to
/// the *keyword* before the brace, and the same brace is tight in `'{1, 2}` and
/// `T'{p:1}`. Deciding that from a shared rule would mean tracking the previous
/// token's kind everywhere to serve one construct.
fn inside_values(f: &mut Formatter, node: &SyntaxNode) {
    let mut first = true;
    let mut braced = false;
    let mut after_comma = false;

    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) => {
                match tok.kind() {
                    SyntaxKind::L_BRACE => {
                        f.request(Sep::Space);
                        f.token(&tok);
                        // Pad the contents, as every other brace list does.
                        f.request(Sep::Space);
                        braced = true;
                    }
                    SyntaxKind::R_BRACE => {
                        f.request(Sep::Space);
                        f.token(&tok);
                        braced = false;
                    }
                    _ => {
                        if !first && !braced && !is_terminator(&tok) {
                            f.request(Sep::Space);
                        }
                        f.token(&tok);
                    }
                }
                after_comma = tok.kind() == SyntaxKind::COMMA;
                first = false;
            }
            NodeOrToken::Node(child) => {
                if after_comma || (!first && !braced) {
                    f.request(Sep::Space);
                }
                format_node(f, &child);
                after_comma = false;
                first = false;
            }
        }
    }
}

/// The two shapes a delimited list can take.
///
/// This enum is the entire layout question in this formatter, and
/// [`param_list_layout`] is the only place it is answered. Both are structural:
/// nothing here measures a rendered width, which is why the rules can write
/// directly into the output instead of building a document to be measured
/// later. If a construct ever does need width, this is the one function to
/// change -- it can render flat into a scratch buffer and count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    Flat,
    Broken,
}

/// One element stays on the line; more than one goes one-per-line.
///
/// The style guide asks for parameter lists to follow the same convention as
/// braces, and this is the count that decides when to apply it.
fn param_list_layout(node: &SyntaxNode) -> Layout {
    let elements = node
        .children()
        .filter(|child| {
            matches!(
                child.kind(),
                SyntaxKind::PARAM_DEF_ELEM | SyntaxKind::PARAM_ASSIGNMENT
            )
        })
        .count();

    if elements > 1 || forces_break(node) {
        Layout::Broken
    } else {
        Layout::Flat
    }
}

/// Whether something inside `node` makes a flat rendering impossible.
///
/// A line comment runs to the end of its line and a multi-line block comment
/// brings its own newlines, so either one lands a break in the middle of what
/// was meant to be a single line. A preprocessor directive is the same case
/// twice over: it must both begin and end a line. Breaking deliberately is
/// better than emitting a line the formatter did not plan.
fn forces_break(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens().any(|child| match child {
        NodeOrToken::Node(_) => false,
        NodeOrToken::Token(tok) => {
            tok.kind() == SyntaxKind::LINE_COMMENT
                || tok.kind().is_directive()
                || (tok.kind().is_comment() && tok.text().contains('\n'))
        }
    })
}

/// `#(...)` -- a parameter definition or instantiation.
fn param_list(f: &mut Formatter, node: &SyntaxNode) {
    match param_list_layout(node) {
        Layout::Flat => flat_list(f, node),
        Layout::Broken => broken_list(f, node),
    }
}

/// `#(A = 1, B = 2)`, `{ a, b }`, `'{ a, b }` -- one space after each comma,
/// and braces padded from their contents.
///
/// The padding is keyed to the brace rather than to the list, which is what
/// keeps a flat parameter list tight: `#(.W(8))` and `#(longint unsigned W =
/// 32)` are parenthesised, so the arms below never fire for them.
///
/// An empty list is never padded -- `{}` and `'{}` have nothing to hold apart.
fn flat_list(f: &mut Formatter, node: &SyntaxNode) {
    let padded = node.children().next().is_some();
    let mut after_comma = false;

    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) => {
                if padded && tok.kind() == SyntaxKind::R_BRACE {
                    f.request(Sep::Space);
                }
                f.token(&tok);
                // Requested *after* writing, which the next thing written
                // materialises -- the separator model needs no notion of
                // "space after" beyond leaving one pending.
                if padded && tok.kind() == SyntaxKind::L_BRACE {
                    f.request(Sep::Space);
                }
                after_comma = tok.kind() == SyntaxKind::COMMA;
            }
            NodeOrToken::Node(element) => {
                if after_comma {
                    f.request(Sep::Space);
                }
                format_node(f, &element);
                after_comma = false;
            }
        }
    }
}

/// `abool: true` -- the colon attaches to the member name, the value is spaced
/// off it.
fn struct_kv(f: &mut Formatter, node: &SyntaxNode) {
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) => {
                f.token(&tok);
                if tok.kind() == SyntaxKind::COLON {
                    f.request(Sep::Space);
                }
            }
            NodeOrToken::Node(value) => format_node(f, &value),
        }
    }
}

/// The braced-body layout applied to parentheses: `(` ends the line, elements
/// are indented one per line, `)` gets a line of its own.
///
/// Unlike a body, this drops any blank line the author left between elements.
/// Parameters are parts of one construct rather than statements, so there is no
/// grouping in here to preserve -- see [`Formatter::allow_blank_lines`].
fn broken_list(f: &mut Formatter, node: &SyntaxNode) {
    // Saved and restored rather than set back to `true`: what holds outside a
    // parameter list is the caller's business, not this rule's.
    let outer = f.allow_blank_lines(false);

    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Token(tok) if tok.kind().is_trivia() => f.trivia(&tok),
            NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::L_PAREN => {
                f.token(&tok);
                f.indent();
                f.settle_width();
            }
            NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::R_PAREN => {
                f.dedent();
                f.pin(Sep::Newline);
                f.token(&tok);
            }
            // `#` and each `,` attach to what precedes them.
            NodeOrToken::Token(tok) => f.token(&tok),
            NodeOrToken::Node(element) => {
                f.request(Sep::Newline);
                format_node(f, &element);
            }
        }
    }

    f.allow_blank_lines(outer);
}

fn is_terminator(tok: &SyntaxToken) -> bool {
    matches!(tok.kind(), SyntaxKind::SEMICOLON | SyntaxKind::COMMA)
}

fn is_suffix(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::ARRAY_SUFFIX | SyntaxKind::RANGE_SUFFIX | SyntaxKind::ARRAY_TYPE_SUFFIX
    )
}

/// Whether a braced node holds nothing worth breaking for.
///
/// Comments count as content. `addrmap a { /* later */ };` keeps its shape,
/// because collapsing it would put a comment somewhere it was not written.
fn is_empty(node: &SyntaxNode) -> bool {
    node.children_with_tokens().all(|child| match child {
        NodeOrToken::Node(_) => false,
        NodeOrToken::Token(tok) => matches!(
            tok.kind(),
            SyntaxKind::WHITESPACE | SyntaxKind::L_BRACE | SyntaxKind::R_BRACE
        ),
    })
}

/// The style guide's one exception to a statement per line: `sw` and `hw` may
/// share, "as they're nearly always used together".
///
/// Preserved rather than imposed. Authors who write them apart keep them apart,
/// and nothing is ever joined that was not already joined -- which is also what
/// makes the rule idempotent, since the output it produces is an input it
/// recognises.
fn shares_line_with(item: &SyntaxNode, prev: Option<&SyntaxNode>) -> bool {
    prev.is_some_and(|prev| is_sw_or_hw(prev) && is_sw_or_hw(item) && !preceded_by_newline(item))
}

/// A `sw = ...` or `hw = ...` assignment, and not `default sw = ...`, which
/// leads with a keyword and reads as a statement of its own.
fn is_sw_or_hw(node: &SyntaxNode) -> bool {
    node.kind() == SyntaxKind::LOCAL_PROPERTY_ASSIGNMENT
        && first_significant(node)
            .is_some_and(|tok| matches!(tok.kind(), SyntaxKind::SW_KW | SyntaxKind::HW_KW))
}

fn first_significant(node: &SyntaxNode) -> Option<SyntaxToken> {
    tokens_of(node).find(|tok| !tok.kind().is_trivia())
}

/// Whether the author put a line break in front of `node`.
fn preceded_by_newline(node: &SyntaxNode) -> bool {
    tokens_of(node)
        .take_while(|tok| tok.kind().is_trivia())
        .any(|tok| tok.text().contains('\n'))
}

fn tokens_of(node: &SyntaxNode) -> impl Iterator<Item = SyntaxToken> {
    let end = node.text_range().end();
    std::iter::successors(node.first_token(), |tok: &SyntaxToken| tok.next_token())
        .take_while(move |tok| tok.text_range().end() <= end)
}
