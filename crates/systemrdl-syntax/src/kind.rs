//! The single flat kind enum used for both tokens and nodes.
//!
//! rowan has no type hierarchy: a tree is built from one `#[repr(u16)]` enum
//! where some variants are leaves carrying text (tokens) and the rest are
//! interior nodes carrying children. Everything from `WHITESPACE` down to
//! `LEX_ERROR` is a token; everything from `SOURCE_FILE` on is a node.
//!
//! The token half mirrors the lexer rules in `SystemRDL.g4`, plus the Clause 16
//! preprocessor forms that grammar does not cover -- it describes the language
//! *after* preprocessing, which is not the language a formatter is handed.
//!
//! The node half is *not* a 1:1 mirror of the parser rules. Rules that are pure
//! alternation with no formatting decision attached (`literal`, `number`,
//! `string_literal`, `udp_attr`, `struct_type`) are flattened away -- they would
//! only add tree depth for the formatter to walk through. Conversely a few nodes
//! exist here that the grammar inlines (`ENUM_BODY`, `STRUCT_BODY`, `UDP_BODY`),
//! because a braced block is exactly where an indentation decision lives.

use logos::{Lexer, Logos};

/// Kinds of tokens and nodes in a SystemRDL syntax tree.
///
/// Keywords are recognised by `logos` directly, like every other token. Two
/// details make that work:
///
/// * Longest-match keeps keywords from swallowing names that merely start with
///   one (`regfile`, `r_field`), and keeps escaped identifiers intact: `\reg`
///   is four bytes to `reg`'s three, so it lexes as [`SyntaxKind::IDENT`] and
///   the backslash does its job of letting a keyword be reused as a name.
/// * `r` and `w` are the only keywords that do *not* beat the identifier rule
///   on length, so they carry an explicit `priority`. Without it logos fails
///   the build rather than picking a winner -- see the note on the keyword
///   block below.
///
/// Note this makes keyword-ness purely lexical, matching `SystemRDL.g4`, where
/// the keyword rules precede `ID`. Rules that accept a keyword where a name is
/// expected do so by naming them explicitly (`prop_keyword`, `basic_data_type`),
/// which on this side is [`SyntaxKind::is_ident_like`].
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    //--------------------------------------------------------------------
    // Trivia
    //--------------------------------------------------------------------
    // Not skipped. A formatter needs every byte of the input in the tree, so
    // whitespace and comments are ordinary tokens here rather than the hidden
    // channel they live on in the ANTLR grammar.
    #[regex(r"[ \t\r\n]+")]
    WHITESPACE,
    // `allow_greedy` because run-to-end-of-line is the intended behaviour;
    // logos flags `[^\r\n]*` as a dot-equivalent repetition by default.
    #[regex(r"//[^\r\n]*", allow_greedy = true)]
    LINE_COMMENT,
    // The classic non-nesting block comment pattern. Written this way (rather
    // than the tempting `([^*]|\*[^/])*`) so that `/***/` matches.
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/")]
    BLOCK_COMMENT,

    /// A text-substitution or file-inclusion directive: `` `define ``,
    /// `` `include ``, `` `line ``, `` `undef `` (Clause 16, Table 32).
    ///
    /// Matched as *one* token running to the end of the logical line,
    /// backslash-continuations included, because the payload is not SystemRDL.
    /// A macro body is arbitrary substitution text; lexing into it would let
    /// the formatter reshape something that is not code, and `` `define A 1+2 ``
    /// is not an expression the way `1 + 2` is.
    #[regex(r"`(define|include|line|undef)", directive_line, priority = 20)]
    DIRECTIVE,

    /// A conditional-compilation directive: `` `if ``, `` `ifdef ``,
    /// `` `ifndef ``, `` `elsif ``, `` `else ``, `` `endif ``.
    ///
    /// A separate kind from [`SyntaxKind::DIRECTIVE`] because it is the one
    /// that can move a brace between branches -- nothing downstream acts on the
    /// distinction today, but a `` `endif `` should not be filed under the same
    /// name as a `` `define ``, and this is where a future region analysis would
    /// start.
    ///
    /// Runs to the end of the line for the same reason the others do, though
    /// the reason is sharper here: the `FOO` of `` `ifdef FOO `` is a macro
    /// name, and left outside the token the parser would read it as the start
    /// of an instantiation and report an error on the statement below.
    #[regex(r"`(ifdef|ifndef|elsif|endif|else|if)", directive_line, priority = 20)]
    COND_DIRECTIVE,

    //--------------------------------------------------------------------
    // Literals and identifiers
    //--------------------------------------------------------------------
    #[regex(r"[0-9][0-9_]*")]
    INT_NUMBER,
    #[regex(r"0[xX][0-9a-fA-F][0-9a-fA-F_]*")]
    HEX_NUMBER,
    // Verilog-style sized literal, e.g. `8'hA5`. Note this competes with
    // INT_NUMBER + TICK: `8'hA5` lexes as one VLOG_NUMBER because it is the
    // longer match, while `32'(` falls back to INT_NUMBER, TICK, L_PAREN.
    #[regex(r"[0-9]+'([bB][01][01_]*|[dD][0-9][0-9_]*|[hH][0-9a-fA-F][0-9a-fA-F_]*)")]
    VLOG_NUMBER,
    // Matches the grammar exactly: the only escapes are `\"` and `\\`.
    #[regex(r#""([^"\\]|\\["\\])*""#)]
    STRING_LITERAL,
    #[regex(r"\\?[a-zA-Z_][a-zA-Z0-9_]*")]
    IDENT,
    /// A text macro reference: `` `WIDTH ``, `` `MAX ``.
    ///
    /// What it expands to is unknowable without the definitions, so it is
    /// treated as an atom that may stand either for a value or for a name --
    /// see [`SyntaxKind::is_ident_like`]. Longest-match keeps it from
    /// swallowing the directives above, and keeps `` `defineFOO `` a macro
    /// reference rather than a malformed `` `define ``.
    #[regex(r"`[a-zA-Z_][a-zA-Z0-9_]*")]
    MACRO_REF,

    //--------------------------------------------------------------------
    // Keywords
    //--------------------------------------------------------------------
    // Every keyword out-prioritises `IDENT` by being a longer literal match,
    // except the single-letter `r` and `w`: those tie with the identifier rule
    // at logos' default priority 2 and need the explicit bump below.
    //
    // Variant order is load-bearing -- `is_keyword` tests the range
    // `BOOLEAN_KW..=WITHIN_KW`.
    #[token("boolean")]
    BOOLEAN_KW,
    #[token("bit")]
    BIT_KW,
    #[token("longint")]
    LONGINT_KW,
    #[token("unsigned")]
    UNSIGNED_KW,
    #[token("string")]
    STRING_KW,
    #[token("accesstype")]
    ACCESSTYPE_KW,
    #[token("addressingtype")]
    ADDRESSINGTYPE_KW,
    #[token("onreadtype")]
    ONREADTYPE_KW,
    #[token("onwritetype")]
    ONWRITETYPE_KW,

    #[token("alias")]
    ALIAS_KW,
    #[token("external")]
    EXTERNAL_KW,
    #[token("internal")]
    INTERNAL_KW,

    #[token("addrmap")]
    ADDRMAP_KW,
    #[token("regfile")]
    REGFILE_KW,
    #[token("reg")]
    REG_KW,
    #[token("field")]
    FIELD_KW,
    #[token("mem")]
    MEM_KW,
    #[token("signal")]
    SIGNAL_KW,

    #[token("true")]
    TRUE_KW,
    #[token("false")]
    FALSE_KW,

    #[token("na")]
    NA_KW,
    #[token("rw")]
    RW_KW,
    #[token("wr")]
    WR_KW,
    #[token("r", priority = 3)]
    R_KW,
    #[token("w", priority = 3)]
    W_KW,
    #[token("rw1")]
    RW1_KW,
    #[token("w1")]
    W1_KW,
    #[token("rclr")]
    RCLR_KW,
    #[token("rset")]
    RSET_KW,
    #[token("ruser")]
    RUSER_KW,
    #[token("woset")]
    WOSET_KW,
    #[token("woclr")]
    WOCLR_KW,
    #[token("wot")]
    WOT_KW,
    #[token("wzs")]
    WZS_KW,
    #[token("wzc")]
    WZC_KW,
    #[token("wzt")]
    WZT_KW,
    #[token("wclr")]
    WCLR_KW,
    #[token("wset")]
    WSET_KW,
    #[token("wuser")]
    WUSER_KW,

    #[token("compact")]
    COMPACT_KW,
    #[token("regalign")]
    REGALIGN_KW,
    #[token("fullalign")]
    FULLALIGN_KW,
    #[token("hw")]
    HW_KW,
    #[token("sw")]
    SW_KW,

    #[token("posedge")]
    POSEDGE_KW,
    #[token("negedge")]
    NEGEDGE_KW,
    #[token("bothedge")]
    BOTHEDGE_KW,
    #[token("level")]
    LEVEL_KW,
    #[token("nonsticky")]
    NONSTICKY_KW,

    #[token("abstract")]
    ABSTRACT_KW,
    #[token("all")]
    ALL_KW,
    #[token("component")]
    COMPONENT_KW,
    #[token("componentwidth")]
    COMPONENTWIDTH_KW,
    #[token("constraint")]
    CONSTRAINT_KW,
    #[token("default")]
    DEFAULT_KW,
    #[token("enum")]
    ENUM_KW,
    #[token("encode")]
    ENCODE_KW,
    #[token("inside")]
    INSIDE_KW,
    #[token("number")]
    NUMBER_KW,
    #[token("property")]
    PROPERTY_KW,
    #[token("ref")]
    REF_KW,
    #[token("struct")]
    STRUCT_KW,
    #[token("this")]
    THIS_KW,
    #[token("type")]
    TYPE_KW,

    // Reserved by Annex D. Not used by any parser rule, but recognised so that
    // using one as an identifier is a clean error rather than a confusing one.
    #[token("alternate")]
    ALTERNATE_KW,
    #[token("byte")]
    BYTE_KW,
    #[token("int")]
    INT_KW,
    #[token("precedencetype")]
    PRECEDENCETYPE_KW,
    #[token("real")]
    REAL_KW,
    #[token("shortint")]
    SHORTINT_KW,
    #[token("shortreal")]
    SHORTREAL_KW,
    #[token("signed")]
    SIGNED_KW,
    #[token("with")]
    WITH_KW,
    #[token("within")]
    WITHIN_KW,

    //--------------------------------------------------------------------
    // Operators
    //--------------------------------------------------------------------
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("!")]
    BNOT,
    #[token("~")]
    NOT,
    #[token("&&")]
    BAND,
    #[token("~&")]
    NAND,
    #[token("&")]
    AND,
    #[token("|")]
    OR,
    #[token("||")]
    BOR,
    #[token("~|")]
    NOR,
    #[token("^")]
    XOR,
    // Two spellings, one kind. Which one the author wrote is preserved in the
    // token's text, so the formatter can leave it alone.
    #[token("~^")]
    #[token("^~")]
    XNOR,
    #[token("<<")]
    LSHIFT,
    #[token(">>")]
    RSHIFT,
    #[token("*")]
    MULT,
    #[token("**")]
    EXP,
    #[token("/")]
    DIV,
    #[token("%")]
    MOD,
    #[token("==")]
    EQ,
    #[token("=")]
    ASSIGN,
    #[token("!=")]
    NEQ,
    #[token("<=")]
    LEQ,
    #[token("<")]
    LT,
    #[token(">=")]
    GEQ,
    #[token(">")]
    GT,
    #[token("@")]
    AT,
    #[token("+=")]
    INC,
    #[token("%=")]
    ALIGN,

    //--------------------------------------------------------------------
    // Punctuation
    //--------------------------------------------------------------------
    #[token(";")]
    SEMICOLON,
    #[token("{")]
    L_BRACE,
    #[token("}")]
    R_BRACE,
    #[token("(")]
    L_PAREN,
    #[token(")")]
    R_PAREN,
    #[token("[")]
    L_BRACK,
    #[token("]")]
    R_BRACK,
    #[token("#")]
    HASH,
    #[token(",")]
    COMMA,
    #[token(".")]
    DOT,
    #[token(":")]
    COLON,
    #[token("::")]
    DOUBLE_COLON,
    #[token("->")]
    ARROW,
    #[token("?")]
    QUESTION,
    #[token("'")]
    TICK,

    /// Bytes the lexer could not match. Kept in the tree so that even garbage
    /// input round-trips.
    LEX_ERROR,

    /// Sentinel returned when the parser looks past the last token. Never
    /// produced by the lexer and never present in a tree.
    EOF,

    //--------------------------------------------------------------------
    // Nodes
    //--------------------------------------------------------------------
    SOURCE_FILE,

    COMPONENT_DEF,
    COMPONENT_NAMED_DEF,
    COMPONENT_ANON_DEF,
    COMPONENT_BODY,
    COMPONENT_TYPE,
    COMPONENT_INST_TYPE,
    COMPONENT_INSTS,
    COMPONENT_INST,
    COMPONENT_INST_ALIAS,
    EXPLICIT_COMPONENT_INST,
    FIELD_INST_RESET,
    INST_ADDR_FIXED,
    INST_ADDR_STRIDE,
    INST_ADDR_ALIGN,

    PARAM_DEF,
    PARAM_DEF_ELEM,
    PARAM_INST,
    PARAM_ASSIGNMENT,

    LOCAL_PROPERTY_ASSIGNMENT,
    DYNAMIC_PROPERTY_ASSIGNMENT,
    NORMAL_PROP_ASSIGN,
    ENCODE_PROP_ASSIGN,
    PROP_MOD_ASSIGN,
    PROP_KEYWORD,
    PROP_MOD,

    UDP_DEF,
    UDP_BODY,
    UDP_TYPE,
    UDP_DATA_TYPE,
    UDP_USAGE,
    UDP_COMP_TYPE,
    UDP_DEFAULT,
    UDP_CONSTRAINT,

    ENUM_DEF,
    ENUM_BODY,
    ENUM_ENTRY,
    ENUM_ENTRY_BODY,
    ENUM_PROP_ASSIGN,

    STRUCT_DEF,
    STRUCT_BODY,
    STRUCT_ELEM,

    CONSTRAINT_DEF,
    CONSTRAINT_NAMED_DEF,
    CONSTRAINT_ANON_DEF,
    CONSTRAINT_BODY,
    CONSTRAINT_INSTS,
    CONSTR_RELATIONAL,
    CONSTR_PROP_ASSIGN,
    CONSTR_INSIDE_VALUES,
    CONSTR_INSIDE_ENUM,
    CONSTR_LHS,
    CONSTR_INSIDE_VALUE,

    UNARY_EXPR,
    BINARY_EXPR,
    TERNARY_EXPR,
    PAREN_EXPR,
    CONCATENATE,
    REPLICATE,
    CAST_TYPE,
    CAST_WIDTH,

    MACRO_CALL,

    LITERAL,
    ARRAY_LITERAL,
    STRUCT_LITERAL,
    STRUCT_KV,
    ENUM_LITERAL,

    INSTANCE_REF,
    INSTANCE_REF_ELEMENT,
    PROP_REF,

    DATA_TYPE,
    BASIC_DATA_TYPE,
    RANGE_SUFFIX,
    ARRAY_SUFFIX,
    ARRAY_TYPE_SUFFIX,

    /// Wraps input the parser could not make sense of.
    ERROR,

    /// Sentinel; must stay last. Only used to bound-check [`SyntaxKind::from_raw`].
    #[doc(hidden)]
    __LAST,
}

impl SyntaxKind {
    /// Whitespace, comments and preprocessor directives -- the tokens the
    /// parser passes through verbatim and never makes a decision on.
    ///
    /// The conditionals are in here too, which is worth a word. It is not that
    /// they are harmless -- a `` `ifdef `` really can open a brace its `` `else ``
    /// closes -- but that a formatter never has to know. See the module docs in
    /// [`crate::parser`] for why ignoring them is safe rather than merely cheap.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
        ) || self.is_directive()
    }

    pub fn is_comment(self) -> bool {
        matches!(self, SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT)
    }

    /// A preprocessor directive of either kind -- the tokens that own a whole
    /// line of the file.
    pub fn is_directive(self) -> bool {
        matches!(self, SyntaxKind::DIRECTIVE | SyntaxKind::COND_DIRECTIVE)
    }

    pub fn is_keyword(self) -> bool {
        SyntaxKind::BOOLEAN_KW <= self && self <= SyntaxKind::WITHIN_KW
    }

    /// True for tokens that may stand in for a name.
    ///
    /// Several grammar rules accept a keyword where an identifier is expected
    /// (`basic_data_type`, `normal_prop_assign`, ...), which is unavoidable
    /// given how many short words SystemRDL reserves.
    ///
    /// A macro reference qualifies too, and for a stronger reason: it may
    /// expand to anything, so `` `MY_REG_T inst; `` is as plausible as
    /// `` field f[`WIDTH-1:0]; ``. Admitting it here is what lets one rule --
    /// [`expect_name`](crate::parser) -- cover every position a macro can name
    /// something in, instead of each of them growing a case for it.
    pub fn is_ident_like(self) -> bool {
        matches!(self, SyntaxKind::IDENT | SyntaxKind::MACRO_REF) || self.is_keyword()
    }

    /// Recovers a kind from its raw discriminant.
    ///
    /// # Panics
    /// If `raw` is not a valid discriminant.
    pub fn from_raw(raw: u16) -> SyntaxKind {
        assert!(raw < SyntaxKind::__LAST as u16, "invalid SyntaxKind: {raw}");
        // SAFETY: `SyntaxKind` is `#[repr(u16)]` and declares no explicit
        // discriminants, so its variants occupy 0..__LAST contiguously. The
        // assert above establishes that `raw` is in that range. This is the
        // standard rowan idiom for the `Language::kind_from_raw` round trip.
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw) }
    }

    pub fn to_raw(self) -> u16 {
        self as u16
    }
}

/// Extends a [`SyntaxKind::DIRECTIVE`] match to the end of its logical line.
///
/// A directive ends at a newline, except that a backslash immediately before
/// one continues it -- which is how a `` `define `` spells a multi-line macro
/// body. A backslash anywhere else is ordinary text (SystemRDL uses it to
/// escape identifiers), so it is stepped over rather than treated as an escape.
fn directive_line(lex: &mut Lexer<SyntaxKind>) {
    let rest = lex.remainder().as_bytes();
    let mut i = 0;

    while i < rest.len() {
        match rest[i] {
            b'\n' | b'\r' => break,
            b'\\' => {
                let mut j = i + 1;
                if rest.get(j) == Some(&b'\r') {
                    j += 1;
                }
                if rest.get(j) == Some(&b'\n') {
                    i = j + 1;
                } else {
                    i += 1;
                }
            }
            // Bumping through a multi-byte character one byte at a time is
            // safe: every byte of one is >= 0x80 and so matches this arm, and
            // the loop only *stops* on ASCII or at the end -- so `i` is always
            // a char boundary by the time it is handed to `bump`.
            _ => i += 1,
        }
    }

    lex.bump(i);
}
