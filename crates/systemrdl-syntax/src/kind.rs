//! The single flat kind enum used for both tokens and nodes.
//!
//! rowan has no type hierarchy: a tree is built from one `#[repr(u16)]` enum
//! where some variants are leaves carrying text (tokens) and the rest are
//! interior nodes carrying children. Everything from `WHITESPACE` down to
//! `LEX_ERROR` is a token; everything from `SOURCE_FILE` on is a node.
//!
//! The token half mirrors the lexer rules in `SystemRDL.g4`; the node half is
//! *not* a 1:1 mirror of the parser rules. Rules that are pure alternation with
//! no formatting decision attached (`literal`, `number`, `string_literal`,
//! `udp_attr`, `struct_type`) are flattened away -- they would only add tree
//! depth for the formatter to walk through. Conversely a few nodes exist here
//! that the grammar inlines (`ENUM_BODY`, `STRUCT_BODY`, `UDP_BODY`), because a
//! braced block is exactly where an indentation decision lives.

use logos::Logos;

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
    /// Whitespace and comments -- the tokens the parser passes through
    /// verbatim and never makes a decision on.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
        )
    }

    pub fn is_comment(self) -> bool {
        matches!(self, SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT)
    }

    pub fn is_keyword(self) -> bool {
        SyntaxKind::BOOLEAN_KW <= self && self <= SyntaxKind::WITHIN_KW
    }

    /// True for tokens that may stand in for a name.
    ///
    /// Several grammar rules accept a keyword where an identifier is expected
    /// (`basic_data_type`, `normal_prop_assign`, ...), which is unavoidable
    /// given how many short words SystemRDL reserves.
    pub fn is_ident_like(self) -> bool {
        self == SyntaxKind::IDENT || self.is_keyword()
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
