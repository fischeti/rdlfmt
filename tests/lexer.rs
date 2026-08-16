//! Lexer tests.
//!
//! The first two are the ones that matter: the token stream must cover the
//! input completely and contiguously. Everything downstream depends on it, so
//! these run over the real sample plus a pile of adversarial snippets.

use rdlfmt::syntax::{SyntaxKind, lex};

const SAMPLE: &str = include_str!("../samples/sample.rdl");

/// Every snippet here gets round-tripped, including the deliberately broken
/// ones -- garbage input must survive the lexer intact, not be dropped.
const SNIPPETS: &[&str] = &[
    "",
    " ",
    "\n\n\n",
    "reg",
    "\\reg",
    "regwidth",
    "r w rw hw sw na wr rw1 w1",
    "8'hA5",
    "32'(x)",
    "0x1F 0X1f 0 007 1_000",
    "8'b1010 4'd12 16'hDEAD_BEEF",
    "\"a\\\"b\" \"\" \"\\\\\"",
    "/**/",
    "/***/",
    "/* a * b */",
    "/* multi\nline */",
    "// trailing",
    "// no newline at eof",
    "a = b; a == b; a += b; a %= b; a ** b; a * b",
    "~& ~| ~^ ^~ && || << >> <= >= != -> :: @ ?",
    "x[7:0]",
    "field {} STATUS[7:0] = 8'hA5;",
    "`include \"other.rdl\"",
    "`define W 32",
    "`define W 32\n",
    "`define BODY a + \\\n    b\nreg r {};",
    "`undef W",
    "`line 5 \"f.rdl\" 0",
    "`ifdef A\n`else\n`endif",
    "f[`W-1:0]",
    "`MAX(1, 2)",
    // Broken on purpose.
    "$ ` §",
    "`",
    "`define",
    "`define trailing backslash \\",
    "\"unterminated",
    "/* unterminated",
    "reg $$ field",
];

fn concat(src: &str) -> String {
    lex(src).iter().map(|(_, text)| text).collect()
}

#[test]
fn round_trips_the_sample() {
    assert_eq!(concat(SAMPLE), SAMPLE);
}

#[test]
fn round_trips_snippets() {
    for src in SNIPPETS {
        assert_eq!(&concat(src), src, "round-trip failed for {src:?}");
    }
}

#[test]
fn ranges_are_contiguous_and_complete() {
    for src in SNIPPETS.iter().chain(std::iter::once(&SAMPLE)) {
        let tokens = lex(src);
        let mut offset = 0;
        for i in 0..tokens.len() {
            let range = tokens.range(i);
            let start = usize::from(range.start());
            assert_eq!(start, offset, "gap before token {i} in {src:?}");
            offset = usize::from(range.end());
        }
        assert_eq!(offset, src.len(), "tokens do not reach end of {src:?}");
    }
}

/// Helper: (kind, text) pairs with trivia dropped, for readable assertions.
fn kinds(src: &str) -> Vec<(SyntaxKind, &str)> {
    lex(src)
        .iter()
        .filter(|(kind, _)| !kind.is_trivia())
        .collect()
}

/// Helper: every token, trivia included. Needed where the token under test
/// *is* trivia, which [`kinds`] filters out.
fn all(src: &str) -> Vec<(SyntaxKind, &str)> {
    lex(src).iter().collect()
}

#[test]
fn keywords_beat_the_identifier_rule_but_only_when_unescaped() {
    use SyntaxKind::*;
    assert_eq!(kinds("reg"), [(REG_KW, "reg")]);
    // A leading backslash is what lets a keyword be reused as a name. This
    // falls out of longest-match: `\reg` is four bytes to `reg`'s three.
    assert_eq!(kinds("\\reg"), [(IDENT, "\\reg")]);
    // Same mechanism keeps a keyword prefix from splitting an identifier.
    assert_eq!(kinds("regwidth"), [(IDENT, "regwidth")]);
    assert_eq!(kinds("r_field"), [(IDENT, "r_field")]);
    // `r` and `w` are the only keywords that do not win on length, so they
    // carry an explicit priority. Everything else here already outweighs
    // IDENT -- this pins the boundary in case a shorter keyword is ever added.
    assert_eq!(
        kinds("r w rw hw sw"),
        [
            (R_KW, "r"),
            (W_KW, "w"),
            (RW_KW, "rw"),
            (HW_KW, "hw"),
            (SW_KW, "sw")
        ]
    );
    // The escape applies to them too.
    assert_eq!(kinds("\\r \\w"), [(IDENT, "\\r"), (IDENT, "\\w")]);
}

#[test]
fn apostrophe_has_three_meanings() {
    use SyntaxKind::*;
    // Sized literal: one token.
    assert_eq!(kinds("8'hA5"), [(VLOG_NUMBER, "8'hA5")]);
    // Width cast: falls back to three tokens, because `'(` cannot start a
    // sized literal. This is the DFA backtracking to its last accepting state.
    assert_eq!(
        kinds("32'(x)"),
        [
            (INT_NUMBER, "32"),
            (TICK, "'"),
            (L_PAREN, "("),
            (IDENT, "x"),
            (R_PAREN, ")")
        ]
    );
    // Array literal.
    assert_eq!(kinds("'{}"), [(TICK, "'"), (L_BRACE, "{"), (R_BRACE, "}")]);
}

#[test]
fn multi_character_operators_win_over_their_prefixes() {
    use SyntaxKind::*;
    assert_eq!(
        kinds("= == + += * ** % %= < <= << > >= >> ! != ~ ~& ~| ~^ ^ ^~ & && | || - ->  : ::"),
        [
            (ASSIGN, "="),
            (EQ, "=="),
            (PLUS, "+"),
            (INC, "+="),
            (MULT, "*"),
            (EXP, "**"),
            (MOD, "%"),
            (ALIGN, "%="),
            (LT, "<"),
            (LEQ, "<="),
            (LSHIFT, "<<"),
            (GT, ">"),
            (GEQ, ">="),
            (RSHIFT, ">>"),
            (BNOT, "!"),
            (NEQ, "!="),
            (NOT, "~"),
            (NAND, "~&"),
            (NOR, "~|"),
            (XNOR, "~^"),
            (XOR, "^"),
            (XNOR, "^~"),
            (AND, "&"),
            (BAND, "&&"),
            (OR, "|"),
            (BOR, "||"),
            (MINUS, "-"),
            (ARROW, "->"),
            (COLON, ":"),
            (DOUBLE_COLON, "::"),
        ]
    );
}

#[test]
fn comments_are_tokens_not_holes() {
    use SyntaxKind::*;
    // The whole point: these are in the stream, not on a hidden channel.
    let src = "reg // hi\n/* there */ x";
    let toks = lex(src);
    let comments: Vec<_> = toks.iter().filter(|(kind, _)| kind.is_comment()).collect();
    assert_eq!(
        comments,
        [(LINE_COMMENT, "// hi"), (BLOCK_COMMENT, "/* there */")]
    );
    // A line comment stops at the newline; the newline itself is whitespace.
    assert!(
        toks.iter()
            .any(|(kind, text)| kind == WHITESPACE && text == "\n")
    );
}

#[test]
fn block_comment_edge_cases() {
    use SyntaxKind::*;
    // Not via `kinds()`: comments are trivia, which that helper filters out.
    for src in ["/**/", "/***/", "/****/", "/* * */", "/** a **/"] {
        let toks: Vec<_> = lex(src).iter().collect();
        assert_eq!(toks, [(BLOCK_COMMENT, src)], "failed on {src:?}");
    }
}

/// Every directive is one token covering its whole logical line -- the payload
/// is substitution text, not SystemRDL, and must not be lexed as if it were.
#[test]
fn directives_are_one_opaque_token_per_line() {
    use SyntaxKind::*;
    for src in [
        "`define W 32",
        "`include \"other.rdl\"",
        "`undef W",
        "`line 5 \"f.rdl\" 0",
        // The payload is never lexed, so nothing in it can be mistaken for code.
        "`define X reg r {};",
    ] {
        assert_eq!(all(src), [(DIRECTIVE, src)], "failed on {src:?}");
    }
    // A conditional takes its operand with it. Left outside the token, the
    // `FOO` here would read as the start of an instantiation.
    for src in ["`ifdef FOO", "`ifndef FOO", "`elsif FOO", "`else", "`endif"] {
        assert_eq!(all(src), [(COND_DIRECTIVE, src)], "failed on {src:?}");
    }

    // The line ends at the newline, which stays behind as whitespace.
    assert_eq!(
        all("`define W 32\nreg"),
        [
            (DIRECTIVE, "`define W 32"),
            (WHITESPACE, "\n"),
            (REG_KW, "reg")
        ]
    );
    // A backslash before the newline continues the directive; one anywhere
    // else is ordinary text, and does not.
    assert_eq!(
        all("`define B a + \\\n  b\nreg"),
        [
            (DIRECTIVE, "`define B a + \\\n  b"),
            (WHITESPACE, "\n"),
            (REG_KW, "reg")
        ]
    );
    assert_eq!(
        all("`define E \\esc\nreg"),
        [
            (DIRECTIVE, "`define E \\esc"),
            (WHITESPACE, "\n"),
            (REG_KW, "reg")
        ]
    );
}

#[test]
fn backtick_forms_are_told_apart_by_longest_match() {
    use SyntaxKind::*;
    // A directive name is only a directive when the identifier ends there.
    assert_eq!(all("`define"), [(DIRECTIVE, "`define")]);
    assert_eq!(all("`defineFOO"), [(MACRO_REF, "`defineFOO")]);
    assert_eq!(all("`else"), [(COND_DIRECTIVE, "`else")]);
    assert_eq!(all("`elsewhere"), [(MACRO_REF, "`elsewhere")]);
    // `ifdef` beats the `if` it starts with, on length.
    assert_eq!(all("`ifdef"), [(COND_DIRECTIVE, "`ifdef")]);
    // A bare backtick names nothing and stays an error.
    assert_eq!(all("`"), [(LEX_ERROR, "`")]);
}

#[test]
fn macro_references_are_atoms() {
    use SyntaxKind::*;
    assert_eq!(
        kinds("f[`W-1:0]"),
        [
            (IDENT, "f"),
            (L_BRACK, "["),
            (MACRO_REF, "`W"),
            (MINUS, "-"),
            (INT_NUMBER, "1"),
            (COLON, ":"),
            (INT_NUMBER, "0"),
            (R_BRACK, "]")
        ]
    );
}

#[test]
fn unrecognised_bytes_become_a_single_error_run() {
    use SyntaxKind::*;
    // A stretch of garbage is one token, not one per byte.
    assert_eq!(
        kinds("reg $$$ field"),
        [(REG_KW, "reg"), (LEX_ERROR, "$$$"), (FIELD_KW, "field")]
    );
}
