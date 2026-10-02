//! Per-node formatting rules, reached through the [`format_node`] dispatch.
//!
//! Expressions never break, so at most nodes the only question is whether the
//! parts are separate words ([`spaced`]) or one word ([`tight`]).
//! [`braced_body`] and [`param_list`] are the only rules that lay out lines;
//! the rest are variations on `spaced` that mark alignment cells or space a
//! delimited list.
//!
//! A rule walks its children with [`each`], which routes trivia to
//! [`Formatter::trivia`] in source order, so a comment deep inside a child is
//! still written before the child's first token.
//!
//! A rule separates its children from each other and never says what comes
//! before its first one: that belongs to the parent, the only one that knows
//! (`sw` takes a space in `default sw = rw` but none in `a.b->sw`). Requests are
//! minimums, so a rule asks for the least its construct needs.

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

        // Everything that reads as one word: a reference and its subscripts
        // (`a.b[0].c`), and an arrow, which binds as tightly as the dot
        // (`a.b->sw`).
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

        // Comma-separated lists inside an expression, which never break. A
        // macro call is one of them: `` `MAX(a, b) `` stands for a value.
        CONCATENATE | REPLICATE | ARRAY_LITERAL | STRUCT_LITERAL | MACRO_CALL => {
            flat_list(f, node)
        }

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

/// Top-level items, one per line, keeping the blank lines between them.
fn source_file(f: &mut Formatter, node: &SyntaxNode) {
    let mut region = false;
    // A bare token here is a stray `;`, which gets a line of its own.
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

/// `{ ... }`: the opening brace on the owning statement's line, one item per
/// line indented one level, and the closing brace on a line of its own.
///
/// Except that a body with nothing to break for stays on one line (see
/// [`is_flat`]), and `sw` and `hw` may share a line (see [`shares_line_with`]).
fn braced_body(f: &mut Formatter, node: &SyntaxNode) {
    // Whatever the owning statement asked for, `{` may not abut it.
    f.request(Sep::Space);

    if is_flat(node) {
        // Whitespace inside the braces is dropped: all it could say is where
        // a line breaks, and `is_flat` has ruled that out.
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
            // Pinned, so a blank line in front of `}` is dropped.
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

/// Children separated by single spaces: `default regwidth = 32`, `a + b`.
///
/// Except that `;` and `,` attach to what precedes them (`};`), and so does a
/// subscript (`STATUS[7:0]`).
fn spaced(f: &mut Formatter, node: &SyntaxNode) {
    spaced_by(f, node, element);
}

/// [`spaced`], with `emit` writing each child, which is where a rule that
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

/// `external my_reg r @ 0x0;`, with the type, name, reset and address clauses
/// as cells.
fn explicit_component_inst(f: &mut Formatter, node: &SyntaxNode) {
    let instances = node
        .descendants()
        .filter(|child| child.kind() == SyntaxKind::COMPONENT_INST)
        .count();
    let parameterized = node
        .descendants()
        .any(|child| child.kind() == SyntaxKind::PARAM_INST);
    // A parameter list or a second instance has cells of its own, so such a
    // statement is not aligned.
    if instances != 1 || parameterized {
        spaced(f, node);
        return;
    }

    // The first token is the type: anything before it, like `external` or
    // `alias ctrl`, is a node.
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

/// The single instance of an aligned instantiation.
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

/// Children with nothing between them: `a.b[0].c`, `[7:0]`, `-a`.
fn tight(f: &mut Formatter, node: &SyntaxNode) {
    each(f, node, |f, _, child| element(f, child));
}

/// `this inside { 1, 2, [3:4] };`: spaced up to the brace, then a value list.
///
/// Not [`flat_list`], because the brace here is spaced from the keyword before
/// it, while in `'{1, 2}` it is not.
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

/// `#(...)`, a parameter definition or instantiation: one element stays on the
/// line, more than one go one per line, as the style guide asks.
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

/// Whether `node` holds something that would break a flat rendering anyway: a
/// line comment, a multi-line block comment, or a directive.
fn forces_break(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens()
        .filter_map(NodeOrToken::into_token)
        .any(|tok| {
            tok.kind() == SyntaxKind::LINE_COMMENT
                || tok.kind().is_directive()
                || (tok.kind().is_comment() && tok.text().contains('\n'))
        })
}

/// `#(.W(8))`, `{ a, b }`, `'{ a, b }`: a space after each comma, and braces
/// (but not parentheses) padded from their contents unless the list is empty.
fn flat_list(f: &mut Formatter, node: &SyntaxNode) {
    let padded = node.children().next().is_some();
    each(f, node, |f, prev, child| match child {
        NodeOrToken::Token(tok) => {
            if padded && tok.kind() == SyntaxKind::R_BRACE {
                f.request(Sep::Space);
            }
            f.token(&tok);
            // Requested after writing, so it also holds a comment off the
            // brace.
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

/// `abool: true`: the colon attaches to the name.
fn struct_kv(f: &mut Formatter, node: &SyntaxNode) {
    each(f, node, |f, _, child| {
        let colon = child.kind() == SyntaxKind::COLON;
        element(f, child);
        if colon {
            f.request(Sep::Space);
        }
    });
}

/// The body layout applied to parentheses, except that blank lines between
/// elements are dropped (see [`Formatter::allow_blank_lines`]).
fn broken_list(f: &mut Formatter, node: &SyntaxNode) {
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

/// Whether `item` is reproduced verbatim. The last marker in its leading
/// trivia updates `region`, which tracks `off` and `on` across a body.
fn is_suppressed(region: &mut bool, item: &SyntaxNode) -> bool {
    let marker = leading_trivia(item)
        .filter(|tok| tok.kind().is_comment())
        .filter_map(|tok| Suppression::parse(tok.text()))
        .last();
    match marker {
        Some(Suppression::Off) => *region = true,
        Some(Suppression::On) => *region = false,
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

/// Whether a body stays on one line: it holds no items, and either no comments
/// or only block comments with no line break among them. An empty body
/// collapses to `{}`; `{ /* later */ }` keeps its shape.
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

/// Whether `item` stays on the line of `prev`: the style guide lets `sw` and
/// `hw` share a line. Only an existing shared line is kept; none is created.
fn shares_line_with(item: &SyntaxNode, prev: Option<&SyntaxNode>) -> bool {
    prev.is_some_and(|prev| {
        is_sw_or_hw(prev)
            && is_sw_or_hw(item)
            && !leading_trivia(item).any(|tok| tok.text().contains('\n'))
    })
}

/// A `sw = ...` or `hw = ...` assignment, but not `default sw = ...`.
fn is_sw_or_hw(node: &SyntaxNode) -> bool {
    node.kind() == SyntaxKind::LOCAL_PROPERTY_ASSIGNMENT
        && tokens(node)
            .find(|tok| !tok.kind().is_trivia())
            .is_some_and(|tok| matches!(tok.kind(), SyntaxKind::SW_KW | SyntaxKind::HW_KW))
}
