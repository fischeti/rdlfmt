//! Parser tests.
//!
//! Three things are being checked: that the tree still round-trips (the
//! property everything else rests on), that trivia lands where the convention
//! says it should, and that the tree *shape* is right -- mostly operator
//! precedence, where a formatter would otherwise silently reparenthesise.

use rdlfmt::syntax::{SyntaxElement, SyntaxNode, parse};

const SAMPLE: &str = include_str!("../samples/sample.rdl");

/// Well-formed SystemRDL covering constructs the sample does not reach.
const VALID: &[&str] = &[
    "addrmap top {};",
    "reg r {};",
    "field {} f;",
    "reg my_reg #(longint unsigned W = 32, boolean S = true) {};",
    "my_reg inst;",
    "my_reg a, b, c;",
    "my_reg arr[4];",
    "field {} f[7:0];",
    "my_reg r @ 0x0;",
    "my_reg r[4] @ 0x10 += 0x4;",
    "my_reg r @ 0x0 += 0x4 %= 0x8;",
    "external my_reg r;",
    "internal reg r {} x;",
    "alias other my_reg bar;",
    "my_reg #(.W(8), .S(false)) inst;",
    "regwidth = 32;",
    "default regwidth = 32;",
    "donttest;",
    "sw = rw; hw = r;",
    "encode = my_enum;",
    "posedge sig;",
    "a->regwidth = 32;",
    "a.b[0].c->name = \"x\";",
    "enum e { A = 0; B = 1; };",
    "enum e { A = 0 { name = \"a\"; desc = \"d\"; }; };",
    "struct s { longint unsigned x; string name; };",
    "abstract struct base { bit flag; };",
    "struct derived : base { accesstype a; };",
    "struct s { reg r; longint unsigned arr[]; };",
    "property p { type = string; component = field | reg; default = \"x\"; };",
    "property p { type = ref; component = all; constraint = componentwidth; };",
    "property p { type = number[]; component = constraint; };",
    "constraint c { this inside {1, 2, [3:4]}; };",
    "constraint c { x == 1; };",
    "constraint c { this inside my_enum; };",
    "constraint { x < 4; } c1, c2;",
    "x = a::b;",
    "x = my_struct'{a: 1, b: 2};",
    "x = '{1, 2, 3};",
    "x = '{};",
    "x = {a, b, c};",
    "x = {4{a, b}};",
    "x = bit'(y);",
    "x = 32'(y);",
    "x = (w)'(y);",
    "x = 8'hA5;",
    "x = a->sw;",
    "x = a.b->name;",
    "x = -1 + 2 * 3 ** 4;",
    "x = a ? b : c;",
    "x = !a && (b || c);",
    "x = &a | ~^b;",
    "x = a[3][2];",
    // Directives are trivia, so they parse anywhere at all.
    "`include \"other.rdl\"",
    "`ifdef A\naddrmap top {};\n`endif",
    "addrmap top {\n`ifndef SKIP\n    my_reg r;\n`endif\n};",
    "`ifdef A\nmy_reg a;\n`elsif B\nmy_reg b;\n`else\nmy_reg c;\n`endif",
    "`define W 32\naddrmap top {};",
    "addrmap top {\n`include \"regs.rdl\"\n};",
    "reg r #(\n`define W 32\nlongint unsigned W = 32) {};",
    // A macro reference may stand for a value...
    "x = `W;",
    "x = `W - 1;",
    "field {} f[`W-1:0];",
    "x = `MAX(1, 2);",
    "x = `NOW() + 1;",
    // ...or for a name.
    "`MY_REG_T inst;",
    "my_reg `INST_NAME;",
    "x = `MY_ENUM::IDLE;",
];

/// Input that must produce errors but must still yield a round-tripping tree.
const BROKEN: &[&str] = &[
    "reg",
    "reg {",
    "reg r { field",
    "x = ;",
    "x = 1 +;",
    "}}}",
    "$$$",
    "x = (1;",
];

/// Input the reference grammar rejects that this parser accepts anyway.
///
/// A formatter is not a validator, and refusing to format a file over a stray
/// semicolon or an empty block is worse than formatting it. These are accepted
/// silently and preserved verbatim; catching them is the compiler's job.
const LENIENT: &[&str] = &[
    // `root: (root_elem ';')*` has no empty statement.
    "reg r {} ;;;",
    // `enum_def` requires at least one entry.
    "enum e { };",
    // `udp_def` requires at least one attribute.
    "property p { };",
];

fn round_trips(src: &str) -> bool {
    parse(src).syntax().to_string() == src
}

#[test]
fn valid_input_round_trips_and_parses_cleanly() {
    for src in VALID {
        let parsed = parse(src);
        assert_eq!(
            parsed.syntax().to_string(),
            *src,
            "round-trip failed for {src:?}"
        );
        assert!(
            parsed.errors().is_empty(),
            "unexpected errors for {src:?}: {:?}",
            parsed.errors()
        );
    }
}

#[test]
fn sample_parses_cleanly() {
    let parsed = parse(SAMPLE);
    assert!(
        parsed.errors().is_empty(),
        "errors in sample.rdl: {:?}",
        parsed.errors()
    );
    assert_eq!(parsed.syntax().to_string(), SAMPLE);
}

#[test]
fn deliberately_lenient_input_is_accepted_and_preserved() {
    for src in LENIENT {
        let parsed = parse(src);
        assert_eq!(
            parsed.syntax().to_string(),
            *src,
            "round-trip failed for {src:?}"
        );
        assert!(
            parsed.errors().is_empty(),
            "{src:?} should be accepted leniently, got {:?}",
            parsed.errors()
        );
    }
}

#[test]
fn broken_input_still_round_trips() {
    for src in BROKEN {
        let parsed = parse(src);
        assert_eq!(
            parsed.syntax().to_string(),
            *src,
            "round-trip failed for {src:?}"
        );
        assert!(
            !parsed.errors().is_empty(),
            "expected errors for {src:?} but got none"
        );
    }
}

/// A conditional whose branches are whole statements parses like any other
/// file, because the directives are invisible to the parser.
///
/// Note both branches end up in the tree at once. That is not an approximation
/// of what the preprocessor would do -- it is the only honest reading, since
/// which branch survives depends on definitions this crate never sees, and a
/// formatter has to preserve the ones that do not.
#[test]
fn conditionals_around_whole_statements_parse() {
    let src = "addrmap top {\n`ifdef FOO\n    my_reg r1;\n`else\n    my_reg r2;\n`endif\n};";
    let parsed = parse(src);

    assert_eq!(parsed.syntax().to_string(), src, "round-trip failed");
    assert!(
        parsed.errors().is_empty(),
        "unexpected errors: {:?}",
        parsed.errors()
    );
}

/// A conditional whose branches split a construct does *not* parse, and needs
/// no special case to be caught: with the directives invisible the source reads
/// as `addrmap top { regfile top {`, whose braces never balance.
#[test]
fn a_conditional_that_splits_a_construct_is_an_error() {
    let src = "`ifdef A\naddrmap top {\n`else\nregfile top {\n`endif\n    my_reg r;\n};";
    let parsed = parse(src);

    assert_eq!(parsed.syntax().to_string(), src, "round-trip failed");
    assert!(
        !parsed.errors().is_empty(),
        "expected errors for a straddling conditional but got none"
    );
}

/// A formatter must never be handed a tree it cannot reproduce, so this is the
/// single most important invariant in the crate.
#[test]
fn round_trips_arbitrary_prefixes_of_the_sample() {
    // Truncating at every byte boundary is a cheap way to exercise the error
    // recovery paths against realistic-but-incomplete input.
    for end in 0..SAMPLE.len() {
        if !SAMPLE.is_char_boundary(end) {
            continue;
        }
        let src = &SAMPLE[..end];
        assert!(
            round_trips(src),
            "round-trip failed for prefix of len {end}"
        );
    }
}

//--------------------------------------------------------------------------
// Trivia convention
//--------------------------------------------------------------------------

/// Finds the first node of the given kind, depth-first.
fn find(node: &SyntaxNode, kind: &str) -> Option<SyntaxNode> {
    if format!("{:?}", node.kind()) == kind {
        return Some(node.clone());
    }
    node.children().find_map(|c| find(&c, kind))
}

#[test]
fn same_line_trailing_comment_stays_with_its_statement() {
    let src = "reg r {\n    sw = rw; // software may write\n    hw = r;\n};";
    let tree = parse(src).syntax();
    let assign = find(&tree, "LOCAL_PROPERTY_ASSIGNMENT").expect("no assignment node");
    assert!(
        assign.to_string().contains("// software may write"),
        "trailing comment escaped its statement: {:?}",
        assign.to_string()
    );
}

#[test]
fn own_line_comment_leads_the_following_statement() {
    let src = "reg r {\n    sw = rw;\n    // about hw\n    hw = r;\n};";
    let tree = parse(src).syntax();
    let assigns: Vec<_> = find(&tree, "COMPONENT_BODY")
        .unwrap()
        .children()
        .filter(|n| format!("{:?}", n.kind()) == "LOCAL_PROPERTY_ASSIGNMENT")
        .collect();
    assert_eq!(assigns.len(), 2);
    assert!(
        !assigns[0].to_string().contains("// about hw"),
        "comment on its own line was pulled onto the previous statement"
    );
    assert!(
        assigns[1].to_string().contains("// about hw"),
        "comment on its own line did not lead the next statement"
    );
}

#[test]
fn blank_lines_are_preserved_in_the_tree() {
    let src = "reg a {};\n\n\nreg b {};";
    let tree = parse(src).syntax();
    assert_eq!(tree.to_string(), src);
    // The blank run leads the second definition, so it is recoverable as a
    // property of `reg b` rather than lost between siblings.
    let defs: Vec<_> = tree.children().collect();
    assert_eq!(defs.len(), 2);
    assert!(defs[1].to_string().starts_with("\n\n\n"));
}

//--------------------------------------------------------------------------
// Tree shape
//--------------------------------------------------------------------------

/// Renders a subtree as an S-expression, dropping trivia, for shape assertions.
fn sexp(node: &SyntaxNode) -> String {
    let mut out = format!("({:?}", node.kind());
    for child in node.children_with_tokens() {
        match child {
            SyntaxElement::Node(n) => {
                out.push(' ');
                out.push_str(&sexp(&n));
            }
            SyntaxElement::Token(t) if !t.kind().is_trivia() => {
                out.push(' ');
                out.push_str(t.text());
            }
            SyntaxElement::Token(_) => {}
        }
    }
    out.push(')');
    out
}

/// Parses `x = <expr>;` and renders just the expression.
fn expr_sexp(expr: &str) -> String {
    let src = format!("x = {expr};");
    let parsed = parse(&src);
    assert!(
        parsed.errors().is_empty(),
        "errors parsing {expr:?}: {:?}",
        parsed.errors()
    );
    let assign = find(&parsed.syntax(), "NORMAL_PROP_ASSIGN").expect("no assignment");
    let node = assign.children().last().expect("no expression");
    sexp(&node)
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    assert_eq!(
        expr_sexp("1 + 2 * 3"),
        "(BINARY_EXPR (LITERAL 1) + (BINARY_EXPR (LITERAL 2) * (LITERAL 3)))"
    );
    assert_eq!(
        expr_sexp("1 * 2 + 3"),
        "(BINARY_EXPR (BINARY_EXPR (LITERAL 1) * (LITERAL 2)) + (LITERAL 3))"
    );
}

#[test]
fn binary_operators_are_left_associative() {
    assert_eq!(
        expr_sexp("1 - 2 - 3"),
        "(BINARY_EXPR (BINARY_EXPR (LITERAL 1) - (LITERAL 2)) - (LITERAL 3))"
    );
}

#[test]
fn ternary_is_right_associative_and_binds_loosest() {
    assert_eq!(
        expr_sexp("1 ? 2 : 3 ? 4 : 5"),
        "(TERNARY_EXPR (LITERAL 1) ? (LITERAL 2) : \
         (TERNARY_EXPR (LITERAL 3) ? (LITERAL 4) : (LITERAL 5)))"
    );
    assert_eq!(
        expr_sexp("1 || 2 ? 3 : 4"),
        "(TERNARY_EXPR (BINARY_EXPR (LITERAL 1) || (LITERAL 2)) ? (LITERAL 3) : (LITERAL 4))"
    );
}

#[test]
fn full_precedence_ladder() {
    // Loosest to tightest, one operator per level: the tree should nest
    // strictly rightwards.
    assert_eq!(
        expr_sexp("1 || 2 && 3 | 4 ^ 5 & 6 == 7 < 8 << 9 + 10 * 11 ** 12"),
        "(BINARY_EXPR (LITERAL 1) || (BINARY_EXPR (LITERAL 2) && (BINARY_EXPR (LITERAL 3) | \
         (BINARY_EXPR (LITERAL 4) ^ (BINARY_EXPR (LITERAL 5) & (BINARY_EXPR (LITERAL 6) == \
         (BINARY_EXPR (LITERAL 7) < (BINARY_EXPR (LITERAL 8) << (BINARY_EXPR (LITERAL 9) + \
         (BINARY_EXPR (LITERAL 10) * (BINARY_EXPR (LITERAL 11) ** (LITERAL 12))))))))))))"
    );
}

#[test]
fn unary_applies_to_the_primary_only() {
    // Per the reference grammar a unary operator takes an `expr_primary`, so
    // this is `(-1) ** 2`, not `-(1 ** 2)`.
    assert_eq!(
        expr_sexp("-1 ** 2"),
        "(BINARY_EXPR (UNARY_EXPR - (LITERAL 1)) ** (LITERAL 2))"
    );
}

#[test]
fn parentheses_survive_as_nodes() {
    // A formatter needs the author's parentheses preserved, not normalised.
    assert_eq!(
        expr_sexp("(1 + 2) * 3"),
        "(BINARY_EXPR (PAREN_EXPR ( (BINARY_EXPR (LITERAL 1) + (LITERAL 2)) )) * (LITERAL 3))"
    );
}

#[test]
fn casts_are_distinguished_from_sized_literals() {
    assert_eq!(expr_sexp("8'hA5"), "(LITERAL 8'hA5)");
    assert_eq!(
        expr_sexp("32'(y)"),
        "(CAST_WIDTH (LITERAL 32) ' ( (INSTANCE_REF (INSTANCE_REF_ELEMENT y)) ))"
    );
    assert_eq!(
        expr_sexp("bit'(y)"),
        "(CAST_TYPE bit ' ( (INSTANCE_REF (INSTANCE_REF_ELEMENT y)) ))"
    );
}

#[test]
fn concatenation_and_replication_are_distinguished() {
    assert_eq!(
        expr_sexp("{1, 2}"),
        "(CONCATENATE { (LITERAL 1) , (LITERAL 2) })"
    );
    assert_eq!(
        expr_sexp("{4{1, 2}}"),
        "(REPLICATE { (LITERAL 4) (CONCATENATE { (LITERAL 1) , (LITERAL 2) }) })"
    );
}

#[test]
fn range_suffix_is_distinguished_from_array_suffix() {
    let range = parse("field {} f[7:0];").syntax();
    assert!(find(&range, "RANGE_SUFFIX").is_some(), "expected a range");
    let array = parse("my_reg f[4];").syntax();
    assert!(find(&array, "ARRAY_SUFFIX").is_some(), "expected an array");
    assert!(find(&array, "RANGE_SUFFIX").is_none());

    // A ternary's colon inside a subscript must not read as a range separator.
    let ternary = parse("my_reg f[a ? 1 : 2];").syntax();
    assert!(
        find(&ternary, "ARRAY_SUFFIX").is_some(),
        "ternary colon misread as a range separator"
    );
    assert!(find(&ternary, "RANGE_SUFFIX").is_none());
}

#[test]
fn name_started_items_are_disambiguated() {
    // Same leading token, three different constructs.
    assert!(
        find(
            &parse("regwidth = 32;").syntax(),
            "LOCAL_PROPERTY_ASSIGNMENT"
        )
        .is_some()
    );
    assert!(find(&parse("donttest;").syntax(), "LOCAL_PROPERTY_ASSIGNMENT").is_some());
    assert!(find(&parse("my_reg inst;").syntax(), "EXPLICIT_COMPONENT_INST").is_some());
    assert!(
        find(
            &parse("a.b->name = \"x\";").syntax(),
            "DYNAMIC_PROPERTY_ASSIGNMENT"
        )
        .is_some()
    );
}

#[test]
fn escaped_identifier_is_a_name_not_a_keyword() {
    let tree = parse("my_reg \\reg;").syntax();
    assert!(find(&tree, "EXPLICIT_COMPONENT_INST").is_some());
    assert_eq!(tree.to_string(), "my_reg \\reg;");
}
