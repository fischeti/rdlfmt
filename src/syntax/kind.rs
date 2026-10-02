//! The single flat kind enum used for both tokens and nodes.
//!
//! Everything from `WHITESPACE` to `LEX_ERROR` is a token; everything from
//! `SOURCE_FILE` on is a node. Tokens follow the lexer rules in `SystemRDL.g4`,
//! plus the Clause 16 preprocessor forms that grammar leaves out. Nodes do not
//! mirror the parser rules one to one: pure-alternation rules are flattened
//! away, and braced blocks (`ENUM_BODY`, `STRUCT_BODY`, `UDP_BODY`) get nodes
//! of their own, because that is where indentation is decided.

use logos::{Lexer, Logos};

/// Kinds of tokens and nodes in a SystemRDL syntax tree.
///
/// Keywords are lexed like every other token, so keyword-ness is purely
/// lexical, as in `SystemRDL.g4`. Longest match keeps `regfile` and `r_field`
/// names, and `\reg` an escaped identifier. Where the grammar accepts a keyword
/// as a name, the parser uses [`SyntaxKind::is_ident_like`].
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    //--------------------------------------------------------------------
    // Trivia
    //--------------------------------------------------------------------
    // Ordinary tokens, not a hidden channel: the tree keeps every byte.
    #[regex(r"[ \t\r\n]+")]
    WHITESPACE,
    // `allow_greedy`, because running to the end of the line is the point.
    #[regex(r"//[^\r\n]*", allow_greedy = true)]
    LINE_COMMENT,
    // Written this way, not as `([^*]|\*[^/])*`, so that `/***/` matches.
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/")]
    BLOCK_COMMENT,

    /// A text-substitution or file-inclusion directive: `` `define ``,
    /// `` `include ``, `` `line ``, `` `undef `` (Clause 16, Table 32).
    ///
    /// One token running to the end of the logical line, continuations
    /// included: a macro body is substitution text, not code to reformat.
    #[regex(r"`(define|include|line|undef)", directive_line, priority = 20)]
    DIRECTIVE,

    /// A conditional-compilation directive: `` `if ``, `` `ifdef ``,
    /// `` `ifndef ``, `` `elsif ``, `` `else ``, `` `endif ``.
    ///
    /// Separate from [`SyntaxKind::DIRECTIVE`] because it is written flush left.
    /// It also runs to the end of its line, so the `FOO` of `` `ifdef FOO `` is
    /// not parsed as code.
    #[regex(r"`(ifdef|ifndef|elsif|endif|else|if)", directive_line, priority = 20)]
    COND_DIRECTIVE,

    //--------------------------------------------------------------------
    // Literals and identifiers
    //--------------------------------------------------------------------
    #[regex(r"[0-9][0-9_]*")]
    INT_NUMBER,
    #[regex(r"0[xX][0-9a-fA-F][0-9a-fA-F_]*")]
    HEX_NUMBER,
    // Verilog-style sized literal, `8'hA5`. Longest match tells it from a
    // cast width: `32'(` lexes as INT_NUMBER, TICK, L_PAREN.
    #[regex(r"[0-9]+'([bB][01][01_]*|[dD][0-9][0-9_]*|[hH][0-9a-fA-F][0-9a-fA-F_]*)")]
    VLOG_NUMBER,
    // Matches the grammar exactly: the only escapes are `\"` and `\\`.
    #[regex(r#""([^"\\]|\\["\\])*""#)]
    STRING_LITERAL,
    #[regex(r"\\?[a-zA-Z_][a-zA-Z0-9_]*")]
    IDENT,
    /// A text macro reference: `` `WIDTH ``, `` `MAX ``.
    ///
    /// An atom that may stand for a value or a name; see
    /// [`SyntaxKind::is_ident_like`]. Longest match keeps `` `defineFOO `` a
    /// reference rather than a directive.
    #[regex(r"`[a-zA-Z_][a-zA-Z0-9_]*")]
    MACRO_REF,

    //--------------------------------------------------------------------
    // Keywords
    //--------------------------------------------------------------------
    // A keyword beats `IDENT` as a longer literal match, except `r` and `w`,
    // which tie and need an explicit priority.
    //
    // Variant order matters: `is_keyword` tests `BOOLEAN_KW..=WITHIN_KW`.
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
    /// Whitespace, comments and preprocessor directives, conditionals included:
    /// the tokens the parser never decides on. See [`crate::syntax::parser`]
    /// for why ignoring a conditional is safe.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
        ) || self.is_directive()
    }

    pub fn is_comment(self) -> bool {
        matches!(self, SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT)
    }

    /// A preprocessor directive of either kind; each owns its line.
    pub fn is_directive(self) -> bool {
        matches!(self, SyntaxKind::DIRECTIVE | SyntaxKind::COND_DIRECTIVE)
    }

    pub fn is_keyword(self) -> bool {
        SyntaxKind::BOOLEAN_KW <= self && self <= SyntaxKind::WITHIN_KW
    }

    /// True for tokens that may stand in for a name: identifiers, keywords
    /// (several grammar rules accept one where a name is expected), and macro
    /// references, which may expand to anything.
    pub fn is_ident_like(self) -> bool {
        matches!(self, SyntaxKind::IDENT | SyntaxKind::MACRO_REF) || self.is_keyword()
    }

    /// Recovers a kind from its raw discriminant.
    ///
    /// # Panics
    /// If `raw` is not a valid discriminant.
    pub fn from_raw(raw: u16) -> SyntaxKind {
        assert!(raw < SyntaxKind::__LAST as u16, "invalid SyntaxKind: {raw}");
        // SAFETY: `SyntaxKind` is `#[repr(u16)]` with no explicit
        // discriminants, so its variants occupy 0..__LAST, and the assert
        // above puts `raw` in that range.
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw) }
    }

    pub fn to_raw(self) -> u16 {
        self as u16
    }
}

/// Extends a [`SyntaxKind::DIRECTIVE`] match to the end of its logical line.
///
/// A backslash immediately before a newline continues the directive; anywhere
/// else it is ordinary text.
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
            // Safe byte by byte: the loop only stops on ASCII or at the end,
            // so `i` is a char boundary when handed to `bump`.
            _ => i += 1,
        }
    }

    lex.bump(i);
}
