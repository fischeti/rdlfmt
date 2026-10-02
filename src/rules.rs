//! Per-node formatting rules.
//!
//! One function per group of node kinds, reached through the [`format_node`]
//! dispatch. Every kind has a rule except [`SyntaxKind::ERROR`], which falls
//! through to [`Formatter::verbatim`] -- see its docs for why that arm stays.
//!
//! [`spaced`] and [`tight`] between them handle almost everything, because with
//! expressions never breaking, the question at most nodes is only whether their
//! parts are separate words or one word. [`braced_body`] and [`param_list`] are
//! the two that lay anything out, and the rest are variations on `spaced` that
//! mark alignment cells or space a delimited list.
//!
//! # The shape every rule has
//!
//! A rule walks the node's children through [`each`], which hands trivia to
//! [`Formatter::trivia`] and everything else to the rule, which requests
//! separation before it and recurses. Trivia is handled *in place* rather than
//! hoisted out, which is what lets a comment buried at the front of a deeply
//! nested child still be emitted before the child's first token -- recursion
//! reaches it in source order without anyone having to look for it.
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

use crate::formatter::{AlignPoint, Formatter, RowFamily, Sep, leading_trivia, tokens};
use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode};
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
        // Constraints: `this > 0`, `this inside myEnum`, `a = 1`.
        | CONSTR_RELATIONAL
        | CONSTR_PROP_ASSIGN
        | CONSTR_INSIDE_ENUM
        // An instantiation that `explicit_component_inst` does not align: a
        // parameterized one, or one declaring several instances.
        | COMPONENT_INSTS
        | COMPONENT_INST => spaced(f, node),

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
        EXPLICIT_COMPONENT_INST => explicit_component_inst(f, node),
        PARAM_DEF_ELEM => param_def_elem(f, node),
        ENUM_ENTRY => enum_entry(f, node),

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

/// Walks the children of `node`: trivia goes to [`Formatter::trivia`], and
/// everything else to `on`, along with the kind of the significant child
/// before it.
fn each(
    f: &mut Formatter,
    node: &SyntaxNode,
    mut on: impl FnMut(&mut Formatter, Option<SyntaxKind>, SyntaxElement),
) {
    let mut prev = None;
    for child in node.children_with_tokens() {
        if let NodeOrToken::Token(tok) = &child
            && tok.kind().is_trivia()
        {
            f.trivia(tok);
            continue;
        }
        let kind = child.kind();
        on(f, prev, child);
        prev = Some(kind);
    }
}

/// Writes a token, or formats a node by its own rule.
fn element(f: &mut Formatter, child: SyntaxElement) {
    match child {
        NodeOrToken::Token(tok) => f.token(&tok),
        NodeOrToken::Node(node) => format_node(f, &node),
    }
}

/// Top-level items, one per line, with blank lines between them preserved.
///
/// The `Sep::Newline` request before each item is what makes the author's blank
/// lines count for anything: one arrives later, as the item's own leading
/// trivia, and widens the break this asked for.
fn source_file(f: &mut Formatter, node: &SyntaxNode) {
    let mut region = false;
    // The grammar wraps every top-level construct in a node, so a bare token
    // here is stray input the parser could not place. It gets a line of its own
    // rather than running into a neighbour.
    each(f, node, |f, _, child| {
        f.request(Sep::Newline);
        match child {
            NodeOrToken::Node(item) => {
                let verbatim = is_suppressed(&mut region, &item);
                statement(f, &item, verbatim);
            }
            stray => element(f, stray),
        }
    });
}

/// `{ ... }` -- the one layout in the language that is never in question.
///
/// The style guide asks for the opening brace on the line of the statement that
/// owns it, the contents indented one level, and the closing brace alone on its
/// line. There is no width to measure and no alternative to weigh; the
/// alignment IR records only padding boundaries after this layout is settled.
///
/// Two exceptions, both from the style guide: an empty body has nothing to
/// indent, and `sw`/`hw` may share a line. See [`shares_line_with`].
fn braced_body(f: &mut Formatter, node: &SyntaxNode) {
    // A floor rather than a decision: whatever the owning statement wanted, `{`
    // may not abut the name in front of it.
    f.request(Sep::Space);

    if is_flat(node) {
        // Whitespace between the braces is dropped rather than routed through
        // `trivia`: all it could say is where a line breaks, and `is_flat`
        // has already ruled that out wherever there is anything to separate.
        let mut inside = false;
        let mut padded = false;
        for tok in node
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
        {
            match tok.kind() {
                // What comes before the brace is the owning statement's.
                _ if !inside && tok.kind().is_trivia() => f.trivia(&tok),
                SyntaxKind::L_BRACE => {
                    f.token(&tok);
                    inside = true;
                }
                SyntaxKind::WHITESPACE => {}
                SyntaxKind::BLOCK_COMMENT => {
                    f.trivia(&tok);
                    padded = true;
                }
                SyntaxKind::R_BRACE if padded => {
                    f.request(Sep::Space);
                    f.token(&tok);
                }
                _ => f.token(&tok),
            }
        }
        return;
    }

    let mut prev: Option<SyntaxNode> = None;
    let mut region = false;
    each(f, node, |f, _, child| match child {
        NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::L_BRACE => {
            f.token(&tok);
            f.indent();
            f.settle_width();
            f.open_alignment_scope();
        }
        NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::R_BRACE => {
            f.close_alignment_scope();
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
            let verbatim = is_suppressed(&mut region, &item);
            f.request(if shares_line_with(&item, prev.as_ref()) {
                Sep::Space
            } else {
                Sep::Newline
            });
            f.begin_row(if verbatim {
                RowFamily::Other
            } else {
                row_family(item.kind())
            });
            statement(f, &item, verbatim);
            f.end_row();
            prev = Some(item);
        }
    });
}

/// Formats a statement, or reproduces it as written if a marker suppressed it.
fn statement(f: &mut Formatter, item: &SyntaxNode, verbatim: bool) {
    if verbatim {
        f.verbatim(item);
    } else {
        format_node(f, item);
    }
}

fn row_family(kind: SyntaxKind) -> RowFamily {
    match kind {
        SyntaxKind::EXPLICIT_COMPONENT_INST => RowFamily::Instantiation,
        SyntaxKind::ENUM_ENTRY => RowFamily::EnumEntry,
        _ => RowFamily::Other,
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
    spaced_by(f, node, element);
}

/// [`spaced`], with `emit` writing each child -- which is where a rule that
/// marks alignment cells does so.
fn spaced_by(
    f: &mut Formatter,
    node: &SyntaxNode,
    mut emit: impl FnMut(&mut Formatter, SyntaxElement),
) {
    each(f, node, |f, prev, child| {
        let attached = match &child {
            NodeOrToken::Token(tok) => is_terminator(tok.kind()),
            NodeOrToken::Node(node) => is_suffix(node.kind()),
        };
        if prev.is_some() && !attached {
            f.request(Sep::Space);
        }
        emit(f, child);
    });
}

/// An explicit component instantiation, divided into the semantic cells which
/// are meaningful across neighbouring statements.
fn explicit_component_inst(f: &mut Formatter, node: &SyntaxNode) {
    let instances = node
        .descendants()
        .filter(|child| child.kind() == SyntaxKind::COMPONENT_INST)
        .count();
    let parameterized = node
        .descendants()
        .any(|child| child.kind() == SyntaxKind::PARAM_INST);
    // A parameter list introduces another cell structure in the middle of the
    // statement. Treat the whole statement as a boundary rather than making a
    // neighbouring simple instantiation line up across it.
    if instances != 1 || parameterized {
        spaced(f, node);
        return;
    }

    // The first token is the type being instantiated: anything before it, like
    // `external` or `alias ctrl`, is a node.
    let mut typed = false;
    spaced_by(f, node, |f, child| match child {
        NodeOrToken::Node(insts) if insts.kind() == SyntaxKind::COMPONENT_INSTS => {
            spaced_by(f, &insts, component_inst_aligned);
        }
        child => {
            if !typed
                && child
                    .as_token()
                    .is_some_and(|tok| !is_terminator(tok.kind()))
            {
                f.align_before(AlignPoint::InstType);
                typed = true;
            }
            element(f, child);
        }
    });
}

/// The one instance of an aligned instantiation: its name, then each of the
/// reset and address clauses, as cells of their own.
fn component_inst_aligned(f: &mut Formatter, child: SyntaxElement) {
    let NodeOrToken::Node(inst) = child else {
        return element(f, child);
    };
    if inst.kind() != SyntaxKind::COMPONENT_INST {
        return format_node(f, &inst);
    }
    f.align_before(AlignPoint::InstName);
    spaced_by(f, &inst, |f, part| {
        let point = match part.kind() {
            SyntaxKind::FIELD_INST_RESET => Some(AlignPoint::InstReset),
            SyntaxKind::INST_ADDR_FIXED => Some(AlignPoint::InstAddress),
            SyntaxKind::INST_ADDR_STRIDE => Some(AlignPoint::InstStride),
            SyntaxKind::INST_ADDR_ALIGN => Some(AlignPoint::InstAlign),
            _ => None,
        };
        if let Some(point) = point {
            f.align_before(point);
        }
        element(f, part);
    });
}

/// `longint unsigned W = 32`, with the name and the default as cells.
fn param_def_elem(f: &mut Formatter, node: &SyntaxNode) {
    let mut typed = false;
    spaced_by(f, node, |f, child| {
        match child.kind() {
            SyntaxKind::DATA_TYPE => typed = true,
            SyntaxKind::ASSIGN => f.align_before(AlignPoint::ParamDefault),
            kind if typed && kind.is_ident_like() => {
                f.align_before(AlignPoint::ParamName);
                typed = false;
            }
            _ => {}
        }
        element(f, child);
    });
}

/// `IDLE = 0;`, with the value as a cell.
fn enum_entry(f: &mut Formatter, node: &SyntaxNode) {
    spaced_by(f, node, |f, child| {
        if child.kind() == SyntaxKind::ASSIGN {
            f.align_before(AlignPoint::EnumValue);
        }
        element(f, child);
    });
}

/// Children with nothing between them.
///
/// For constructs that are one lexical unit despite having structure:
/// `a.b[0].c`, `[7:0]`, `->sw`. Nothing here requests separation, so the tokens
/// land exactly as adjacent as they were written -- but trivia still routes
/// normally, so a comment wedged into a reference is not silently lost.
fn tight(f: &mut Formatter, node: &SyntaxNode) {
    each(f, node, |f, _, child| element(f, child));
}

/// `this inside {1, 2, [3:4]};`
///
/// The one node that needs both shapes at once. Up to the brace it reads as a
/// sentence, so `this` and `inside` are spaced; from the brace on it is a value
/// list like a concatenation, so the delimiters attach to their contents.
///
/// Not folded into [`flat_list`], which cannot help here: the space belongs to
/// the *keyword* before the brace, and the same brace is tight in `'{1, 2}` and
/// `T'{p:1}`.
fn inside_values(f: &mut Formatter, node: &SyntaxNode) {
    let mut braced = false;
    each(f, node, |f, prev, child| match child {
        NodeOrToken::Token(tok) => match tok.kind() {
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
            kind => {
                if prev.is_some() && !braced && !is_terminator(kind) {
                    f.request(Sep::Space);
                }
                f.token(&tok);
            }
        },
        NodeOrToken::Node(value) => {
            if prev == Some(SyntaxKind::COMMA) || (prev.is_some() && !braced) {
                f.request(Sep::Space);
            }
            format_node(f, &value);
        }
    });
}

/// `#(...)` -- a parameter definition or instantiation.
///
/// One element stays on the line; more than one goes one-per-line. The style
/// guide asks for parameter lists to follow the same convention as braces, and
/// this is the count that decides when to apply it. Nothing here measures a
/// rendered width.
fn param_list(f: &mut Formatter, node: &SyntaxNode) {
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
        broken_list(f, node);
    } else {
        flat_list(f, node);
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
    node.descendants_with_tokens()
        .filter_map(NodeOrToken::into_token)
        .any(|tok| {
            tok.kind() == SyntaxKind::LINE_COMMENT
                || tok.kind().is_directive()
                || (tok.kind().is_comment() && tok.text().contains('\n'))
        })
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
    each(f, node, |f, prev, child| match child {
        NodeOrToken::Token(tok) => {
            if padded && tok.kind() == SyntaxKind::R_BRACE {
                f.request(Sep::Space);
            }
            f.token(&tok);
            // Requested *after* writing, so that it also holds a comment off
            // the brace.
            if padded && tok.kind() == SyntaxKind::L_BRACE {
                f.request(Sep::Space);
            }
        }
        NodeOrToken::Node(element) => {
            if prev == Some(SyntaxKind::COMMA) {
                f.request(Sep::Space);
            }
            format_node(f, &element);
        }
    });
}

/// `abool: true` -- the colon attaches to the member name, the value is spaced
/// off it.
fn struct_kv(f: &mut Formatter, node: &SyntaxNode) {
    each(f, node, |f, _, child| {
        let colon = child.kind() == SyntaxKind::COLON;
        element(f, child);
        if colon {
            f.request(Sep::Space);
        }
    });
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

    each(f, node, |f, _, child| match child {
        NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::L_PAREN => {
            f.token(&tok);
            f.indent();
            f.settle_width();
            f.open_alignment_scope();
        }
        NodeOrToken::Token(tok) if tok.kind() == SyntaxKind::R_PAREN => {
            f.close_alignment_scope();
            f.dedent();
            f.pin(Sep::Newline);
            f.token(&tok);
        }
        // `#` and each `,` attach to what precedes them.
        NodeOrToken::Token(tok) => f.token(&tok),
        NodeOrToken::Node(element) => {
            f.request(Sep::Newline);
            f.begin_row(if element.kind() == SyntaxKind::PARAM_DEF_ELEM {
                RowFamily::ParameterDefinition
            } else {
                RowFamily::Other
            });
            format_node(f, &element);
            f.end_row();
        }
    });

    f.allow_blank_lines(outer);
}

/// What a `rdlfmt:` marker comment asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suppression {
    /// Reproduce statements verbatim from here to the end of the body.
    Off,
    /// Resume formatting.
    On,
    /// Reproduce just the statement that follows.
    Skip,
}

impl Suppression {
    /// Parses one comment's text as a marker.
    fn parse(comment: &str) -> Option<Suppression> {
        match comment
            .strip_prefix("//")?
            .trim()
            .strip_prefix("rdlfmt:")?
            .trim()
        {
            "off" => Some(Suppression::Off),
            "on" => Some(Suppression::On),
            "skip" => Some(Suppression::Skip),
            _ => None,
        }
    }
}

/// Whether `item` is reproduced verbatim, applying the last marker in its
/// leading trivia to `region` -- the suppression state of the statement
/// sequence it belongs to.
fn is_suppressed(region: &mut bool, item: &SyntaxNode) -> bool {
    let marker = leading_trivia(item)
        .filter(|tok| tok.kind().is_comment())
        .filter_map(|tok| Suppression::parse(tok.text()))
        .last();
    match marker {
        Some(Suppression::Off) => *region = true,
        Some(Suppression::On) => *region = false,
        // Governs one statement without disturbing the region around it, so a
        // `skip` inside an `off` block is merely redundant.
        Some(Suppression::Skip) => return true,
        None => {}
    }
    *region
}

fn is_terminator(kind: SyntaxKind) -> bool {
    matches!(kind, SyntaxKind::SEMICOLON | SyntaxKind::COMMA)
}

fn is_suffix(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::ARRAY_SUFFIX | SyntaxKind::RANGE_SUFFIX | SyntaxKind::ARRAY_TYPE_SUFFIX
    )
}

/// Whether a braced node holds nothing worth breaking for: no items, and no
/// comment that needs a line break.
///
/// An empty body collapses to `{}` however many lines it spanned. One holding
/// only block comments keeps its shape: `addrmap a { /* later */ };` stays on
/// one line, but if the author broke a line in there, it stays broken -- moving
/// the comment would put it somewhere it was not written.
fn is_flat(node: &SyntaxNode) -> bool {
    let mut comments = false;
    let mut newline = false;
    let inside = node
        .children_with_tokens()
        .skip_while(|child| child.kind() != SyntaxKind::L_BRACE);
    for child in inside {
        match child.kind() {
            SyntaxKind::L_BRACE | SyntaxKind::R_BRACE => {}
            SyntaxKind::WHITESPACE => {
                newline |= child.to_string().contains('\n');
            }
            SyntaxKind::BLOCK_COMMENT => {
                comments = true;
                newline |= child.to_string().contains('\n');
            }
            _ => return false,
        }
    }
    !(comments && newline)
}

/// The style guide's one exception to a statement per line: `sw` and `hw` may
/// share, "as they're nearly always used together".
///
/// Preserved rather than imposed. Authors who write them apart keep them apart,
/// and nothing is ever joined that was not already joined -- which is also what
/// makes the rule idempotent, since the output it produces is an input it
/// recognises.
fn shares_line_with(item: &SyntaxNode, prev: Option<&SyntaxNode>) -> bool {
    prev.is_some_and(|prev| {
        is_sw_or_hw(prev)
            && is_sw_or_hw(item)
            && !leading_trivia(item).any(|tok| tok.text().contains('\n'))
    })
}

/// A `sw = ...` or `hw = ...` assignment, and not `default sw = ...`, which
/// leads with a keyword and reads as a statement of its own.
fn is_sw_or_hw(node: &SyntaxNode) -> bool {
    node.kind() == SyntaxKind::LOCAL_PROPERTY_ASSIGNMENT
        && tokens(node)
            .find(|tok| !tok.kind().is_trivia())
            .is_some_and(|tok| matches!(tok.kind(), SyntaxKind::SW_KW | SyntaxKind::HW_KW))
}
