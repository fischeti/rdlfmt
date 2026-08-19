//! The properties that must hold for every input, forever.
//!
//! These are the tests that make it safe to add rules. A formatter fails in one
//! of three ways, and each has an invariant here:
//!
//! * it changes what the code *means* -- caught by [`significant_tokens`],
//! * it loses a comment, or buries code behind one -- caught by [`comments`],
//! * it cannot agree with itself on an answer -- caught by idempotence.
//!
//! Snapshot tests say what the output looks like; these say what it may never
//! do. Both are needed, but only these survive a style change unedited.

use rdlfmt::format;
use rdlfmt::syntax::{SyntaxKind, lex};

/// Every token that carries meaning, as `(kind, text)`.
///
/// Comparing text as well as kind is the point: it catches a rule that
/// rewrites `0xA5` to `0xa5`, or drops the sign off a literal, neither of
/// which a kind-only comparison would notice.
fn significant_tokens(src: &str) -> Vec<(SyntaxKind, String)> {
    lex(src)
        .iter()
        .filter(|(kind, _)| !kind.is_trivia())
        .map(|(kind, text)| (kind, text.to_owned()))
        .collect()
}

/// Every comment, in order. Text is trimmed at the end because a line comment
/// runs to the newline and may pick up trailing spaces the formatter drops.
fn comments(src: &str) -> Vec<String> {
    lex(src)
        .iter()
        .filter(|(kind, _)| kind.is_comment())
        .map(|(_, text)| text.trim_end().to_owned())
        .collect()
}

/// Every preprocessor directive, in order.
///
/// Checked separately from [`significant_tokens`] for the same reason comments
/// are: a directive is trivia, so that filter drops it -- and a dropped
/// `` `include `` is not a lost comment but lost *code*, arriving from another
/// file. Kind is compared along with text, because turning a `` `endif `` into a
/// `` `define `` would be no better. Trimmed at the end for the same reason as a
/// comment: a directive runs to its newline and may pick up spaces the
/// formatter drops.
fn directives(src: &str) -> Vec<(SyntaxKind, String)> {
    lex(src)
        .iter()
        .filter(|(kind, _)| kind.is_directive())
        .map(|(kind, text)| (kind, text.trim_end().to_owned()))
        .collect()
}

/// Asserts every invariant, and returns the formatted output for the caller to
/// make further claims about.
#[track_caller]
fn check(src: &str) -> String {
    let out = format(src).expect("test input should parse cleanly");

    assert_eq!(
        significant_tokens(src),
        significant_tokens(&out),
        "formatting changed the token stream\n--- input ---\n{src}\n--- output ---\n{out}"
    );
    assert_eq!(
        comments(src),
        comments(&out),
        "formatting changed the comments\n--- input ---\n{src}\n--- output ---\n{out}"
    );
    assert_eq!(
        directives(src),
        directives(&out),
        "formatting changed the directives\n--- input ---\n{src}\n--- output ---\n{out}"
    );

    let again = format(&out).expect("formatted output should parse cleanly");
    assert_eq!(
        out, again,
        "formatting is not idempotent\n--- once ---\n{out}\n--- twice ---\n{again}"
    );

    out
}

#[test]
fn sample_file() {
    check(include_str!("../samples/sample.rdl"));
}

/// The corpus file is written in the formatter's own output style and exercises
/// every construct that has a rule. Formatting it must therefore change nothing.
///
/// This is the snapshot test: unlike the invariants above, it fails on any
/// deliberate change to the style, which is the point -- the diff is the review.
/// Regenerate it by formatting the file and committing the result, but only
/// after reading what moved.
#[test]
fn the_output_style_is_a_fixed_point() {
    let canonical = include_str!("../samples/kitchen-sink.rdl");
    assert_eq!(check(canonical), canonical);
}

#[test]
fn empty_input() {
    assert_eq!(format("").unwrap(), "");
    assert_eq!(format("\n\n  \n").unwrap(), "");
}

#[test]
fn only_comments() {
    let out = check("// just a comment\n");
    assert_eq!(out, "// just a comment\n");
}

#[test]
fn refuses_broken_input() {
    let err = format("reg {").unwrap_err();
    assert!(!err.errors().is_empty());
}

//--------------------------------------------------------------------------
// Trivia handling. These exercise the separator model directly, since it is
// the part that is finished rather than the rules, which are still verbatim.
//--------------------------------------------------------------------------

#[test]
fn blank_lines_between_items_are_capped() {
    let out = check("addrmap a {};\n\n\n\n\naddrmap b {};\n");
    assert_eq!(out, "addrmap a {};\n\naddrmap b {};\n");
}

#[test]
fn missing_blank_line_is_not_invented() {
    let out = check("addrmap a {};\naddrmap b {};\n");
    assert_eq!(out, "addrmap a {};\naddrmap b {};\n");
}

#[test]
fn items_are_split_onto_their_own_lines() {
    let out = check("addrmap a {}; addrmap b {};\n");
    assert_eq!(out, "addrmap a {};\naddrmap b {};\n");
}

#[test]
fn leading_comment_starts_the_file() {
    let out = check("// header\naddrmap a {};\n");
    assert_eq!(out, "// header\naddrmap a {};\n");
}

#[test]
fn blank_line_before_a_comment_is_kept() {
    let out = check("addrmap a {};\n\n// about b\naddrmap b {};\n");
    assert_eq!(out, "addrmap a {};\n\n// about b\naddrmap b {};\n");
}

#[test]
fn trailing_comment_stays_on_its_line() {
    let out = check("addrmap a {}; // about a\naddrmap b {};\n");
    assert_eq!(out, "addrmap a {}; // about a\naddrmap b {};\n");
}

#[test]
fn trailing_comment_does_not_swallow_what_follows() {
    // The failure this guards against is `addrmap b` ending up behind the
    // `//`, which would delete it. Also covered by the token-stream invariant,
    // but worth failing here with a legible message.
    let out = check("addrmap a {}; // about a\naddrmap b {};\n");
    assert!(out.ends_with("// about a\naddrmap b {};\n"), "got:\n{out}");
}

#[test]
fn block_comment_on_its_own_line_stays_there() {
    // Regression: `*/` used to abut the following token, because a comment
    // knew what preceded it but nothing answered the same question on its
    // trailing side.
    let out = check("/* about a */\naddrmap a {};\n");
    assert_eq!(out, "/* about a */\naddrmap a {};\n");
}

#[test]
fn inline_block_comment_stays_inline() {
    let out = check("/* about a */ addrmap a {};\n");
    assert_eq!(out, "/* about a */ addrmap a {};\n");
}

#[test]
fn block_comment_never_abuts_its_neighbour() {
    let out = check("/* about a */addrmap a {};\n");
    assert_eq!(out, "/* about a */ addrmap a {};\n");
}

#[test]
fn block_comment_between_statements_does_not_merge_them() {
    // The mirror of the line-comment pin: a block comment can be followed on
    // its line, so it must never drag the next statement up beside it. It stays
    // on the first line here because the *parser* attached it there -- a
    // same-line comment is a trailing comment whatever its spelling -- but the
    // second statement still gets its own line, which is the point.
    let out = check("addrmap a {}; /* x */ addrmap b {};\n");
    assert_eq!(out, "addrmap a {}; /* x */\naddrmap b {};\n");
}

#[test]
fn block_comment_inside_a_body_leads_what_it_precedes() {
    // Unlike the line-comment case, this one is not pinned, so the body's
    // one-item-per-line rule wins and the comment travels with the item it
    // introduces rather than being stranded on the brace line.
    let out = check("addrmap a { /* x */ name = \"y\";};\n");
    assert_eq!(out, "addrmap a {\n    /* x */ name = \"y\";\n};\n");
}

#[test]
fn multiline_block_comment_is_followed_by_a_line_break() {
    let out = check("/* one\n   two */\naddrmap a {};\n");
    assert_eq!(out, "/* one\n   two */\naddrmap a {};\n");
}

#[test]
fn file_ends_with_exactly_one_newline() {
    assert_eq!(check("addrmap a {};").as_str(), "addrmap a {};\n");
    assert_eq!(check("addrmap a {};\n\n\n").as_str(), "addrmap a {};\n");
}

#[test]
fn no_trailing_whitespace_on_any_line() {
    let out = check("addrmap a {};   \n\n   \naddrmap b {};\n");
    for line in out.lines() {
        assert_eq!(line, line.trim_end(), "trailing whitespace in:\n{out}");
    }
}

//--------------------------------------------------------------------------
// Braced bodies.
//--------------------------------------------------------------------------

#[test]
fn body_contents_are_indented() {
    let out = check("addrmap a {reg r {} x;};\n");
    assert_eq!(out, "addrmap a {\n    reg r {} x;\n};\n");
}

#[test]
fn nesting_compounds_indentation() {
    let out = check("addrmap a {reg {field {sw = rw;} f[0:0];} r @ 0x0;};\n");
    assert_eq!(
        out,
        "addrmap a {\n    reg {\n        field {\n            sw = rw;\n        } f[0:0];\n    } r @ 0x0;\n};\n"
    );
}

#[test]
fn empty_body_stays_on_one_line() {
    assert_eq!(check("addrmap a {};\n"), "addrmap a {};\n");
    assert_eq!(check("addrmap a {   };\n"), "addrmap a {};\n");
    assert_eq!(check("addrmap a {\n\n};\n"), "addrmap a {};\n");
}

#[test]
fn body_holding_only_a_comment_still_breaks() {
    // Collapsing to `{}` would have to put the comment somewhere it was not
    // written, so a comment counts as content.
    let out = check("addrmap a {\n    // later\n};\n");
    assert_eq!(out, "addrmap a {\n    // later\n};\n");
}

#[test]
fn comment_written_beside_the_opening_brace_stays_beside_it() {
    // The general trivia rule decides this, and decides it correctly: the
    // comment did not follow a newline, so it annotates the brace it trails.
    // Moving it down a line would make it read as annotating the first item.
    let out = check("addrmap a { // later\n};\n");
    assert_eq!(out, "addrmap a { // later\n};\n");

    let out = check("addrmap a { // about the body\n    name = \"x\";\n};\n");
    assert_eq!(
        out,
        "addrmap a { // about the body\n    name = \"x\";\n};\n"
    );
}

#[test]
fn blank_line_after_opening_brace_is_dropped() {
    let out = check("addrmap a {\n\n    name = \"x\";\n};\n");
    assert_eq!(out, "addrmap a {\n    name = \"x\";\n};\n");
}

#[test]
fn blank_line_before_closing_brace_is_dropped() {
    let out = check("addrmap a {\n    name = \"x\";\n\n};\n");
    assert_eq!(out, "addrmap a {\n    name = \"x\";\n};\n");
}

#[test]
fn blank_line_before_an_opening_brace_is_dropped() {
    // Regression: whitespace used to request a blank line outright, which beat
    // the space the body asks for in front of `{` and left the brace stranded
    // on a line of its own.
    let out = check("addrmap a\n\n{\n    name = \"x\";\n};\n");
    assert_eq!(out, "addrmap a {\n    name = \"x\";\n};\n");
}

#[test]
fn a_blank_line_does_not_break_what_never_breaks() {
    // The same regression everywhere else an author can leave one. A blank line
    // widens a break the rules decided on; it cannot introduce one.
    let out = check("addrmap a {\n    name =\n\n(\"x\");\n};\n");
    assert_eq!(out, "addrmap a {\n    name = (\"x\");\n};\n");

    let out = check("reg r #\n\n(longint unsigned W = 8)\n\n{\n    regwidth = W;\n};\n");
    assert_eq!(
        out,
        "reg r #(longint unsigned W = 8) {\n    regwidth = W;\n};\n"
    );
}

#[test]
fn blank_lines_inside_a_body_are_preserved() {
    let out = check("addrmap a {\n    name = \"x\";\n\n    desc = \"y\";\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    name = \"x\";\n\n    desc = \"y\";\n};\n"
    );
}

#[test]
fn blank_lines_inside_a_broken_param_list_are_dropped() {
    // Parameters are parts of one construct, not statements, so there is no
    // grouping in here for a blank line to record.
    let out = check(
        "reg r #(\n    longint unsigned W = 8,\n\n    longint unsigned Y = 1\n) {\n    regwidth = W;\n};\n",
    );
    assert_eq!(
        out,
        "reg r #(\n    longint unsigned W = 8,\n    longint unsigned Y = 1\n) {\n    regwidth = W;\n};\n"
    );
}

#[test]
fn a_param_list_does_not_disable_blank_lines_for_what_follows() {
    // The setting is restored on the way out, so the body after the list still
    // keeps the author's grouping.
    let out = check(
        "reg r #(\n    longint unsigned W = 8,\n\n    longint unsigned Y = 1\n) {\n    regwidth = W;\n\n    field {} f[W];\n};\n",
    );
    assert_eq!(
        out,
        "reg r #(\n    longint unsigned W = 8,\n    longint unsigned Y = 1\n) {\n    regwidth = W;\n\n    field {} f[W];\n};\n"
    );
}

#[test]
fn comment_at_the_top_of_a_body_is_indented_with_it() {
    let out = check("addrmap a {\n// about x\nname = \"x\";\n};\n");
    assert_eq!(out, "addrmap a {\n    // about x\n    name = \"x\";\n};\n");
}

#[test]
fn no_space_before_the_terminating_semicolon() {
    let out = check("addrmap a {\n    name = \"x\";\n} ;\n");
    assert!(out.ends_with("};\n"), "got:\n{out}");
}

#[test]
fn enum_entry_body_is_a_body_like_any_other() {
    let out = check("enum e {A = 0;B = 1 {desc = \"b\";};};\n");
    assert_eq!(
        out,
        "enum e {\n    A = 0;\n    B = 1 {\n        desc = \"b\";\n    };\n};\n"
    );
}

#[test]
fn udp_and_struct_bodies_break_too() {
    let out = check("property p {component = field;type = string;};\n");
    assert_eq!(
        out,
        "property p {\n    component = field;\n    type = string;\n};\n"
    );

    let out = check("struct s {longint unsigned a;};\n");
    assert_eq!(out, "struct s {\n    longint unsigned a;\n};\n");
}

#[test]
fn statement_parts_are_spaced() {
    let out = check("reg  my_reg   #(longint unsigned W = 32)  {\n    regwidth = W;\n};\n");
    assert!(
        out.starts_with("reg my_reg #(longint unsigned W = 32) {\n"),
        "got:\n{out}"
    );
}

#[test]
fn a_whole_file_reindents() {
    let out = check("addrmap top{\nname=\"x\";\n  reg{field{sw=rw;}f[0:0];}r1@0x0;\n}\n;\n");
    assert_eq!(
        out,
        concat!(
            "addrmap top {\n",
            "    name = \"x\";\n",
            "    reg {\n",
            "        field {\n",
            "            sw = rw;\n",
            "        } f[0:0];\n",
            "    } r1 @ 0x0;\n",
            "};\n",
        )
    );
}

//--------------------------------------------------------------------------
// Spacing within statements.
//--------------------------------------------------------------------------

#[test]
fn operators_are_surrounded_by_spaces() {
    let out = check("addrmap a {\n    default   regwidth=32;\n};\n");
    assert_eq!(out, "addrmap a {\n    default regwidth = 32;\n};\n");
}

#[test]
fn instantiation_operators_are_spaced() {
    let out = check("addrmap a {\n    my_reg data[4]@0x10+=0x4%=0x8;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    my_reg data[4] @ 0x10 += 0x4 %= 0x8;\n};\n"
    );
}

#[test]
fn subscripts_stay_attached_to_their_name() {
    let out = check("reg r {\n    field {} f1 [ 7 : 0 ];\n};\n");
    assert_eq!(out, "reg r {\n    field {} f1[7:0];\n};\n");

    let out = check("addrmap a {\n    my_reg r [ 4 ] @ 0x0;\n};\n");
    assert_eq!(out, "addrmap a {\n    my_reg r[4] @ 0x0;\n};\n");
}

#[test]
fn field_reset_is_spaced() {
    let out = check("reg r {\n    field {} STATUS[7:0]=8'hA5;\n};\n");
    assert_eq!(out, "reg r {\n    field {} STATUS[7:0] = 8'hA5;\n};\n");
}

#[test]
fn instance_references_read_as_one_word() {
    let out = check("addrmap a {\n    b . c [ 0 ] . d -> sw = rw;\n};\n");
    assert_eq!(out, "addrmap a {\n    b.c[0].d->sw = rw;\n};\n");
}

#[test]
fn instance_lists_are_comma_separated() {
    let out = check("addrmap a {\n    my_reg  x ,y , z;\n};\n");
    assert_eq!(out, "addrmap a {\n    my_reg x, y, z;\n};\n");
}

#[test]
fn data_types_keep_their_two_words() {
    let out = check("struct s {\n    longint   unsigned   a [ ];\n};\n");
    assert_eq!(out, "struct s {\n    longint unsigned a[];\n};\n");
}

#[test]
fn udp_attributes_are_spaced() {
    let out = check("property p {\n    component=field|reg;\n    type=string;\n};\n");
    assert_eq!(
        out,
        "property p {\n    component = field | reg;\n    type = string;\n};\n"
    );
}

#[test]
fn external_and_alias_instantiations_are_spaced() {
    let out =
        check("addrmap a {\n    external  my_reg  r@0x0;\n    alias  r  my_reg  s@0x4;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    external my_reg r @ 0x0;\n    alias r  my_reg s @ 0x4;\n};\n"
    );
}

//--------------------------------------------------------------------------
// Expressions. These never break, so every rule here is about spacing.
//--------------------------------------------------------------------------

/// Wraps `expr` in the smallest thing that parses, and returns how it was laid
/// out. Keeps the expression tests down to the expression.
#[track_caller]
fn expr(src: &str) -> String {
    let out = check(&format!("addrmap a {{\n    x = {src};\n}};\n"));
    out.lines()
        .nth(1)
        .expect("a body line")
        .trim()
        .trim_start_matches("x = ")
        .trim_end_matches(';')
        .to_owned()
}

#[test]
fn binary_operators_are_spaced() {
    assert_eq!(expr("W*2+(X-1)"), "W * 2 + (X - 1)");
    assert_eq!(expr("a>>2|b&c"), "a >> 2 | b & c");
}

#[test]
fn unary_operators_bind_to_their_operand() {
    assert_eq!(expr("-a"), "-a");
    assert_eq!(expr("~ a"), "~a");
    assert_eq!(expr("!  a"), "!a");
}

#[test]
fn ternary_is_spaced() {
    assert_eq!(expr("b?c:d"), "b ? c : d");
}

#[test]
fn casts_bind_tightly() {
    assert_eq!(expr("bit ' ( x )"), "bit'(x)");
    assert_eq!(expr("32 ' ( y )"), "32'(y)");
}

#[test]
fn expression_lists_stay_on_one_line() {
    // Concatenations and literals are expressions, and expressions never
    // break -- the element count rule is for parameter lists only.
    assert_eq!(expr("{a,b,c}"), "{ a, b, c }");
    assert_eq!(expr("'{1,2,3}"), "'{ 1, 2, 3 }");
    assert_eq!(expr("T'{p:1,q:2}"), "T'{ p: 1, q: 2 }");
    // Replication is brace-delimited too, so it pads the same way.
    assert_eq!(expr("{2{a}}"), "{ 2{ a } }");
}

#[test]
fn an_empty_list_is_not_padded() {
    assert_eq!(expr("{}"), "{}");
    assert_eq!(expr("'{}"), "'{}");
}

#[test]
fn parameter_lists_are_not_padded() {
    // The padding belongs to the brace, not to the idea of a list: a flat
    // parameter list is parenthesised and stays tight.
    let out = check("reg r #( longint unsigned W = 8 ) {};\n");
    assert_eq!(out, "reg r #(longint unsigned W = 8) {};\n");

    let out = check("addrmap a {\n    my_reg #( .W ( 8 ) ) y @ 0x0;\n};\n");
    assert_eq!(out, "addrmap a {\n    my_reg #(.W(8)) y @ 0x0;\n};\n");
}

#[test]
fn enum_literals_bind_tightly() {
    assert_eq!(expr("A :: B"), "A::B");
}

#[test]
fn a_line_comment_in_an_expression_still_cannot_swallow_code() {
    // There is no broken layout for a concatenation, so the newline a line
    // comment forces comes from the trivia handler rather than from a rule.
    // The result is unindented, but it is never wrong.
    check("addrmap a {\n    x = {p, // note\n    q};\n};\n");
}

//--------------------------------------------------------------------------
// Parameter lists -- the only place a layout decision exists.
//--------------------------------------------------------------------------

#[test]
fn one_parameter_stays_on_the_line() {
    let out = check("reg r #( longint unsigned W = 32 ) {};\n");
    assert_eq!(out, "reg r #(longint unsigned W = 32) {};\n");
}

#[test]
fn more_than_one_parameter_goes_one_per_line() {
    let out = check("reg r #(longint unsigned W = 32, boolean S = true) {};\n");
    assert_eq!(
        out,
        "reg r #(\n    longint unsigned W = 32,\n    boolean          S = true\n) {};\n"
    );
}

#[test]
fn one_parameter_instantiation_stays_on_the_line() {
    let out = check("addrmap a {\n    my_reg #( .W ( 8 ) ) y @ 0x4;\n};\n");
    assert_eq!(out, "addrmap a {\n    my_reg #(.W(8)) y @ 0x4;\n};\n");
}

#[test]
fn more_than_one_parameter_instantiation_goes_one_per_line() {
    let out = check("addrmap a {\n    my_reg #(.W(8),.V(1)) x @ 0x0;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    my_reg #(\n        .W(8),\n        .V(1)\n    ) x @ 0x0;\n};\n"
    );
}

#[test]
fn a_comment_forces_a_single_parameter_to_break() {
    // One element would be flat, but a line comment cannot be flattened: the
    // `)` would end up behind the `//`.
    let out = check("reg r #( // the width\n    longint unsigned W = 32) {};\n");
    assert_eq!(
        out,
        "reg r #( // the width\n    longint unsigned W = 32\n) {};\n"
    );
}

#[test]
fn a_multiline_block_comment_forces_a_break_too() {
    let out = check("reg r #(/* one\n   two */ longint unsigned W = 32) {};\n");
    assert!(out.starts_with("reg r #(\n"), "got:\n{out}");
}

//--------------------------------------------------------------------------
// Constraints.
//--------------------------------------------------------------------------

#[test]
fn relational_constraints_are_spaced() {
    let out = check("constraint  c  {  this  >  0 ;  } ;\n");
    assert_eq!(out, "constraint c {\n    this > 0;\n};\n");
}

#[test]
fn constraint_property_assignments_are_spaced() {
    let out = check("constraint c {a=1;b=2;};\n");
    assert_eq!(out, "constraint c {\n    a = 1;\n    b = 2;\n};\n");
}

#[test]
fn inside_value_lists_are_a_brace_list_like_any_other() {
    // Padded like a concatenation, and never broken: it is a list of values,
    // not a body.
    let out = check("constraint c {this inside {1,2,[3:4]};};\n");
    assert_eq!(
        out,
        "constraint c {\n    this inside { 1, 2, [3:4] };\n};\n"
    );
}

#[test]
fn inside_an_enum_is_spaced() {
    let out = check("constraint c {this  inside  myEnum;};\n");
    assert_eq!(out, "constraint c {\n    this inside myEnum;\n};\n");
}

#[test]
fn anonymous_constraints_carry_an_instance_list() {
    let out = check("addrmap a {\n constraint {this<8;} c1,c2;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    constraint {\n        this < 8;\n    } c1, c2;\n};\n"
    );
}

//--------------------------------------------------------------------------
// The style guide's `sw`/`hw` exception.
//--------------------------------------------------------------------------

#[test]
fn sw_and_hw_may_share_a_line() {
    let out = check("reg r {\n    field {sw = rw; hw = r;} f[0:0];\n};\n");
    assert_eq!(
        out,
        "reg r {\n    field {\n        sw = rw; hw = r;\n    } f[0:0];\n};\n"
    );
}

#[test]
fn sw_and_hw_written_apart_stay_apart() {
    // Preserved, not imposed: nothing is ever joined that was not already
    // joined, so the rule cannot churn a file that spells them out.
    let out = check("reg r {\n    field {\n        sw = rw;\n        hw = r;\n    } f[0:0];\n};\n");
    assert_eq!(
        out,
        "reg r {\n    field {\n        sw = rw;\n        hw = r;\n    } f[0:0];\n};\n"
    );
}

#[test]
fn only_sw_and_hw_may_share() {
    let out = check("reg r {\n    field {sw = rw; onwrite = woclr;} f[0:0];\n};\n");
    assert_eq!(
        out,
        "reg r {\n    field {\n        sw = rw;\n        onwrite = woclr;\n    } f[0:0];\n};\n"
    );
}

#[test]
fn a_defaulted_sw_is_a_statement_of_its_own() {
    let out = check("reg r {\n    field {default sw = rw; hw = r;} f[0:0];\n};\n");
    assert_eq!(
        out,
        "reg r {\n    field {\n        default sw = rw;\n        hw = r;\n    } f[0:0];\n};\n"
    );
}

#[test]
fn hand_wrapped_statement_is_pulled_back_onto_one_line() {
    // This used to be reachable only as verbatim text, and so kept whatever
    // indentation it was written with. Now that instantiations have a rule,
    // the wrap is a spacing decision like any other and gets normalised away.
    let out = check("addrmap a {\n    my_reg r\n        @ 0x0;\n};\n");
    assert_eq!(out, "addrmap a {\n    my_reg r @ 0x0;\n};\n");
}

//--------------------------------------------------------------------------
// Preprocessor directives. The brace-neutral ones are trivia, so what is
// being checked here is placement: a directive owns its line, keeps its
// payload byte for byte, and is indented with the code around it.
//--------------------------------------------------------------------------

#[test]
fn a_directive_keeps_its_payload_exactly() {
    // The spacing inside is the author's, not the formatter's: a macro body is
    // substitution text, and normalising it would be rewriting a string.
    let out = check("`define   W    32\n");
    assert_eq!(out, "`define   W    32\n");
}

#[test]
fn a_directive_is_indented_with_the_code_around_it() {
    let out = check("addrmap top {\n`include \"regs.rdl\"\n    my_reg r;\n};\n");
    assert_eq!(
        out,
        "addrmap top {\n    `include \"regs.rdl\"\n    my_reg r;\n};\n"
    );
}

#[test]
fn a_directive_takes_a_line_of_its_own() {
    // Nothing may share a line with a directive on either side: what follows
    // one would otherwise be swallowed into a macro body.
    let out = check("my_reg a; `undef W\nmy_reg b;\n");
    assert_eq!(out, "my_reg a;\n`undef W\nmy_reg b;\n");
}

#[test]
fn blank_lines_around_a_directive_are_preserved() {
    let out = check("`include \"a.rdl\"\n\naddrmap top {};\n");
    assert_eq!(out, "`include \"a.rdl\"\n\naddrmap top {};\n");
}

#[test]
fn trailing_whitespace_in_a_directive_is_dropped() {
    let out = check("`define W 32   \naddrmap top {};\n");
    assert_eq!(out, "`define W 32\naddrmap top {};\n");
}

#[test]
fn a_continued_directive_keeps_its_own_line_breaks() {
    // The continuation is inside the macro body, so its layout is not the
    // formatter's to decide -- the whole directive is one token.
    let out = check("addrmap top {\n`define B a + \\\n        b\n};\n");
    assert_eq!(out, "addrmap top {\n    `define B a + \\\n        b\n};\n");
}

#[test]
fn a_directive_before_a_closing_brace_stays_inside_the_body() {
    let out = check("addrmap top {\n    my_reg r;\n`undef W\n};\n");
    assert_eq!(out, "addrmap top {\n    my_reg r;\n    `undef W\n};\n");
}

#[test]
fn a_directive_forces_a_parameter_list_to_break() {
    // A single parameter would otherwise stay on the line, which a directive
    // makes impossible: it must both begin and end one.
    let out = check("reg r #(\n`include \"p.rdl\"\nlongint unsigned W = 32) {};\n");
    assert_eq!(
        out,
        "reg r #(\n    `include \"p.rdl\"\n    longint unsigned W = 32\n) {};\n"
    );
}

#[test]
fn a_macro_reference_is_an_atom_in_an_expression() {
    let out = check("reg r {\n    field {} f[`W-1:0];\n};\n");
    assert_eq!(out, "reg r {\n    field {} f[`W - 1:0];\n};\n");
}

#[test]
fn a_macro_call_never_breaks() {
    let out = check("addrmap top {\n    a = `MAX(1,2);\n};\n");
    assert_eq!(out, "addrmap top {\n    a = `MAX(1, 2);\n};\n");
}

#[test]
fn a_macro_reference_may_stand_for_a_name() {
    let out = check("addrmap top {\n    `MY_REG_T   r1;\n};\n");
    assert_eq!(out, "addrmap top {\n    `MY_REG_T r1;\n};\n");
}

#[test]
fn a_conditional_around_whole_statements_is_laid_out_like_any_directive() {
    let out = check("addrmap top {\n`ifdef FOO\nmy_reg r1;\n`else\nmy_reg r2;\n`endif\n};\n");
    assert_eq!(
        out,
        "addrmap top {\n    `ifdef FOO\n    my_reg r1;\n    `else\n    my_reg r2;\n    `endif\n};\n"
    );
}

#[test]
fn a_conditional_operand_is_part_of_the_directive_not_an_instantiation() {
    // `FOO` left outside the token would be read as the start of a statement
    // and take `my_reg` with it.
    let out = check("`ifdef FOO\nmy_reg r;\n`endif\n");
    assert_eq!(out, "`ifdef FOO\nmy_reg r;\n`endif\n");
}

/// The one thing a conditional is refused for, and it needs no rule of its own:
/// a branch that opens a brace another branch closes leaves the file
/// unbalanced, which is an ordinary parse error.
#[test]
fn refuses_a_conditional_that_splits_a_construct() {
    let err = format("`ifdef A\naddrmap top {\n`else\nregfile top {\n`endif\n};\n").unwrap_err();
    assert!(!err.errors().is_empty());
}

//--------------------------------------------------------------------------
// Line endings
//--------------------------------------------------------------------------
//
// A formatter that imposed its own would rewrite every line of a CRLF file,
// turning a whitespace tidy-up into a whole-file diff for anyone on Windows.
// These pin the input's ending to the output's. Note that `check` cannot catch
// a regression here on its own: line endings live inside the whitespace it
// filters out, which is why `format` checks them separately.

/// Counts CR bytes, which is enough to tell the two styles apart.
fn crs(src: &str) -> usize {
    src.bytes().filter(|&b| b == b'\r').count()
}

#[test]
fn crlf_input_stays_crlf() {
    let out = check("addrmap top {\r\n    reg {} r;\r\n};\r\n");
    assert_eq!(crs(&out), 3, "expected CRLF throughout, got {out:?}");
}

#[test]
fn lf_input_stays_lf() {
    let out = check("addrmap top {\n    reg {} r;\n};\n");
    assert_eq!(crs(&out), 0, "expected LF throughout, got {out:?}");
}

/// The formatting still happens; only the endings are borrowed from the input.
#[test]
fn a_crlf_file_is_still_reformatted() {
    let out = check("addrmap top{\r\n  reg{}r;\r\n};\r\n");
    assert_eq!(out, "addrmap top {\r\n    reg {} r;\r\n};\r\n");
}

/// Blank-line handling counts `\n`, which CRLF also contains -- so the rule
/// that caps runs of blank lines must not care which style it is looking at.
#[test]
fn blank_lines_collapse_under_crlf_too() {
    let out = check("addrmap top {\r\n    reg {} a;\r\n\r\n\r\n\r\n    reg {} b;\r\n};\r\n");
    assert_eq!(
        out,
        "addrmap top {\r\n    reg {} a;\r\n\r\n    reg {} b;\r\n};\r\n"
    );
}

/// A file with no line break at all has no ending to preserve; LF is the
/// default rather than an inference from nothing.
#[test]
fn a_single_line_file_gets_lf() {
    let out = check("addrmap top {};");
    assert_eq!(out, "addrmap top {};\n");
}

//--------------------------------------------------------------------------
// Alignment. Padding rather than layout: every test here would still pass
// with the same tokens on the same lines if the columns were switched off,
// which is what makes it safe to argue about them separately.
//--------------------------------------------------------------------------

#[test]
fn instantiations_in_a_run_line_up() {
    let out = check("addrmap a {\n    r ctrl @ 0x0;\n    r status @ 0x4;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl   @ 0x0;\n    r status @ 0x4;\n};\n"
    );
}

/// The modifier is a column even where it is absent, so an instantiation with
/// no `external` lines its *type* up with the rest of the run rather than
/// starting where the modifiers do.
#[test]
fn a_missing_modifier_leaves_an_empty_cell() {
    let out = check("addrmap a {\n    external x b @ 0x0;\n    y c @ 0x4;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    external x b @ 0x0;\n             y c @ 0x4;\n};\n"
    );
}

/// A blank line is the author saying these are two groups, and grouping outranks
/// alignment: what the author has said about a file is not the formatter's to
/// overrule. See `Formatter::allow_blank_lines` for the same principle applied
/// to the other axis.
#[test]
fn a_blank_line_ends_a_run() {
    let out = check("addrmap a {\n    r ctrl @ 0x0;\n\n    r status @ 0x4;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl @ 0x0;\n\n    r status @ 0x4;\n};\n"
    );
}

/// A row that runs out of cells ends the column. An instantiation with no
/// address closes no cell around its name, so however long that name is it
/// cannot stretch the address column of the rows above it.
#[test]
fn a_row_with_fewer_cells_ends_the_column() {
    let out = check("addrmap a {\n    r ctrl @ 0x0;\n    r s @ 0x4;\n    r loooooong;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl @ 0x0;\n    r s    @ 0x4;\n    r loooooong;\n};\n"
    );
}

/// Nothing to line up with, so nothing is padded -- which is what keeps a lone
/// statement looking the same as it did before there were columns at all.
#[test]
fn a_statement_on_its_own_is_not_padded() {
    let out = check("addrmap a {\n    external x b @ 0x0;\n};\n");
    assert_eq!(out, "addrmap a {\n    external x b @ 0x0;\n};\n");
}

/// Rows align only at equal depth: a nested body has its own widths, and
/// padding it out to its parent's would align things that are not in a list
/// together.
#[test]
fn indentation_separates_runs() {
    let out = check(
        "addrmap a {\n    r ctrl @ 0x0;\n    addrmap b {\n        r c @ 0x0;\n        r dddd @ 0x4;\n    } inner @ 0x100;\n};\n",
    );
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl @ 0x0;\n    addrmap b {\n        r c    @ 0x0;\n        r dddd @ 0x4;\n    } inner @ 0x100;\n};\n"
    );
}

/// A statement broken over several lines is not a row, so it neither aligns nor
/// widens the rows around it.
#[test]
fn a_multiline_statement_is_left_alone() {
    let out = check(
        "addrmap a {\n    r ctrl @ 0x0;\n    r #(.W(1), .X(2)) wide @ 0x4;\n    r s @ 0x8;\n};\n",
    );
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl @ 0x0;\n    r #(\n        .W(1),\n        .X(2)\n    ) wide @ 0x4;\n    r s @ 0x8;\n};\n"
    );
}

/// A broken parameter list is a run of rows like any other. A flat one is a
/// single row, and `parameter_lists_are_not_padded` covers that it stays one.
#[test]
fn broken_parameter_lists_line_up() {
    let out = check("reg r #(longint unsigned W = 32, boolean S = true) {};\n");
    assert_eq!(
        out,
        "reg r #(\n    longint unsigned W = 32,\n    boolean          S = true\n) {};\n"
    );
}

#[test]
fn enum_entries_line_up() {
    let out = check("enum e {\n    IDLE = 0;\n    RUNNING = 1;\n};\n");
    assert_eq!(out, "enum e {\n    IDLE    = 0;\n    RUNNING = 1;\n};\n");
}

/// An entry with a body breaks its line, so it is not part of the run -- and the
/// run does not resume across it, because a column needs adjacent rows.
#[test]
fn an_enum_entry_with_a_body_leaves_the_run() {
    let out =
        check("enum e {\n    IDLE = 0;\n    BUSY = 1 { name = \"b\"; };\n    DONE = 2;\n};\n");
    assert_eq!(
        out,
        "enum e {\n    IDLE = 0;\n    BUSY = 1 {\n        name = \"b\";\n    };\n    DONE = 2;\n};\n"
    );
}

/// The column alignment is most often wanted for, and the one the rules can only
/// reach through the trivia path.
#[test]
fn trailing_comments_line_up() {
    let out = check("addrmap a {\n    r ctrl @ 0x0; // one\n    r s @ 0x1000; // two\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl @ 0x0;    // one\n    r s    @ 0x1000; // two\n};\n"
    );
}

/// A comment on the line *before* a statement is not in that statement's row,
/// even though the parser hands it over as leading trivia. Treating it as one
/// would decline every annotated instantiation in a file.
#[test]
fn a_leading_comment_does_not_end_a_run() {
    let out = check("addrmap a {\n    // why\n    r ctrl @ 0x0;\n    r status @ 0x4;\n};\n");
    assert_eq!(
        out,
        "addrmap a {\n    // why\n    r ctrl   @ 0x0;\n    r status @ 0x4;\n};\n"
    );
}

#[test]
fn a_comment_only_line_inside_a_run_is_transparent() {
    let out = check(
        "addrmap a {\n    r ctrl @ 0x0;\n    // both belong together\n    r status @ 0x4;\n};\n",
    );
    assert_eq!(
        out,
        "addrmap a {\n    r ctrl   @ 0x0;\n    // both belong together\n    r status @ 0x4;\n};\n"
    );
}

/// Padding is whitespace, so it must survive the same round trip as the rest:
/// a second format has to compute the same columns from output that already has
/// them, which is only true because a cell's width is measured from the cell
/// before it and never from the column it landed in.
#[test]
fn alignment_is_a_fixed_point() {
    let once = check(
        "addrmap a {\n    external x bbbb @ 0x0;\n    y c @ 0x4; // hm\n    y dd @ 0x8;\n};\n",
    );
    let twice = check(&once);
    assert_eq!(once, twice);
}
