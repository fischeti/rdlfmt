//! Lexer tests.
//!
//! The first two are the ones that matter: the token stream must cover the
//! input completely and contiguously. Everything downstream depends on it, so
//! these run over the real sample plus a pile of adversarial snippets.

use systemrdl_syntax::{SyntaxKind, lex};

const SAMPLE: &str = include_str!("../../../samples/sample.rdl");

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
    // Broken on purpose.
    "$ ` §",
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

#[test]
fn unrecognised_bytes_become_a_single_error_run() {
    use SyntaxKind::*;
    // A stretch of garbage is one token, not one per byte.
    assert_eq!(
        kinds("reg $$$ field"),
        [(REG_KW, "reg"), (LEX_ERROR, "$$$"), (FIELD_KW, "field")]
    );
}
