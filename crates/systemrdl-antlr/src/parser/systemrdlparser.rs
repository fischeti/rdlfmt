// Generated from SystemRDL.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::systemrdllistener::*;
use super::systemrdlvisitor::*;
use antlr4rust::PredictionContextCache;
use antlr4rust::TokenSource;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr4rust::errors::*;
use antlr4rust::int_stream::EOF;
use antlr4rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, cast, cast_mut};
use antlr4rust::recognizer::{Actions, Recognizer};
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::token::{OwningToken, TOKEN_EOF, Token};
use antlr4rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::tree::*;
use antlr4rust::vocabulary::{Vocabulary, VocabularyImpl};

use antlr4rust::lazy_static;
use antlr4rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const SystemRDL_T__0: i32 = 1;
pub const SystemRDL_T__1: i32 = 2;
pub const SystemRDL_T__2: i32 = 3;
pub const SystemRDL_T__3: i32 = 4;
pub const SystemRDL_T__4: i32 = 5;
pub const SystemRDL_T__5: i32 = 6;
pub const SystemRDL_T__6: i32 = 7;
pub const SystemRDL_T__7: i32 = 8;
pub const SystemRDL_T__8: i32 = 9;
pub const SystemRDL_T__9: i32 = 10;
pub const SystemRDL_T__10: i32 = 11;
pub const SystemRDL_T__11: i32 = 12;
pub const SystemRDL_T__12: i32 = 13;
pub const SystemRDL_T__13: i32 = 14;
pub const SystemRDL_T__14: i32 = 15;
pub const SystemRDL_SL_COMMENT: i32 = 16;
pub const SystemRDL_ML_COMMENT: i32 = 17;
pub const SystemRDL_BOOLEAN_kw: i32 = 18;
pub const SystemRDL_BIT_kw: i32 = 19;
pub const SystemRDL_LONGINT_kw: i32 = 20;
pub const SystemRDL_UNSIGNED_kw: i32 = 21;
pub const SystemRDL_STRING_kw: i32 = 22;
pub const SystemRDL_ACCESSTYPE_kw: i32 = 23;
pub const SystemRDL_ADDRESSINGTYPE_kw: i32 = 24;
pub const SystemRDL_ONREADTYPE_kw: i32 = 25;
pub const SystemRDL_ONWRITETYPE_kw: i32 = 26;
pub const SystemRDL_ALIAS_kw: i32 = 27;
pub const SystemRDL_EXTERNAL_kw: i32 = 28;
pub const SystemRDL_INTERNAL_kw: i32 = 29;
pub const SystemRDL_ADDRMAP_kw: i32 = 30;
pub const SystemRDL_REGFILE_kw: i32 = 31;
pub const SystemRDL_REG_kw: i32 = 32;
pub const SystemRDL_FIELD_kw: i32 = 33;
pub const SystemRDL_MEM_kw: i32 = 34;
pub const SystemRDL_SIGNAL_kw: i32 = 35;
pub const SystemRDL_TRUE_kw: i32 = 36;
pub const SystemRDL_FALSE_kw: i32 = 37;
pub const SystemRDL_NA_kw: i32 = 38;
pub const SystemRDL_RW_kw: i32 = 39;
pub const SystemRDL_WR_kw: i32 = 40;
pub const SystemRDL_R_kw: i32 = 41;
pub const SystemRDL_W_kw: i32 = 42;
pub const SystemRDL_RW1_kw: i32 = 43;
pub const SystemRDL_W1_kw: i32 = 44;
pub const SystemRDL_RCLR_kw: i32 = 45;
pub const SystemRDL_RSET_kw: i32 = 46;
pub const SystemRDL_RUSER_kw: i32 = 47;
pub const SystemRDL_WOSET_kw: i32 = 48;
pub const SystemRDL_WOCLR_kw: i32 = 49;
pub const SystemRDL_WOT_kw: i32 = 50;
pub const SystemRDL_WZS_kw: i32 = 51;
pub const SystemRDL_WZC_kw: i32 = 52;
pub const SystemRDL_WZT_kw: i32 = 53;
pub const SystemRDL_WCLR_kw: i32 = 54;
pub const SystemRDL_WSET_kw: i32 = 55;
pub const SystemRDL_WUSER_kw: i32 = 56;
pub const SystemRDL_COMPACT_kw: i32 = 57;
pub const SystemRDL_REGALIGN_kw: i32 = 58;
pub const SystemRDL_FULLALIGN_kw: i32 = 59;
pub const SystemRDL_HW_kw: i32 = 60;
pub const SystemRDL_SW_kw: i32 = 61;
pub const SystemRDL_POSEDGE_kw: i32 = 62;
pub const SystemRDL_NEGEDGE_kw: i32 = 63;
pub const SystemRDL_BOTHEDGE_kw: i32 = 64;
pub const SystemRDL_LEVEL_kw: i32 = 65;
pub const SystemRDL_NONSTICKY_kw: i32 = 66;
pub const SystemRDL_ABSTRACT_kw: i32 = 67;
pub const SystemRDL_ALL_kw: i32 = 68;
pub const SystemRDL_COMPONENT_kw: i32 = 69;
pub const SystemRDL_COMPONENTWIDTH_kw: i32 = 70;
pub const SystemRDL_CONSTRAINT_kw: i32 = 71;
pub const SystemRDL_DEFAULT_kw: i32 = 72;
pub const SystemRDL_ENUM_kw: i32 = 73;
pub const SystemRDL_ENCODE_kw: i32 = 74;
pub const SystemRDL_INSIDE_kw: i32 = 75;
pub const SystemRDL_NUMBER_kw: i32 = 76;
pub const SystemRDL_PROPERTY_kw: i32 = 77;
pub const SystemRDL_REF_kw: i32 = 78;
pub const SystemRDL_STRUCT_kw: i32 = 79;
pub const SystemRDL_THIS_kw: i32 = 80;
pub const SystemRDL_TYPE_kw: i32 = 81;
pub const SystemRDL_ALTERNATE_kw: i32 = 82;
pub const SystemRDL_BYTE_kw: i32 = 83;
pub const SystemRDL_INT_kw: i32 = 84;
pub const SystemRDL_PRECEDENCETYPE_kw: i32 = 85;
pub const SystemRDL_REAL_kw: i32 = 86;
pub const SystemRDL_SHORTINT_kw: i32 = 87;
pub const SystemRDL_SHORTREAL_kw: i32 = 88;
pub const SystemRDL_SIGNED_kw: i32 = 89;
pub const SystemRDL_WITH_kw: i32 = 90;
pub const SystemRDL_WITHIN_kw: i32 = 91;
pub const SystemRDL_INT: i32 = 92;
pub const SystemRDL_HEX_INT: i32 = 93;
pub const SystemRDL_VLOG_INT: i32 = 94;
pub const SystemRDL_STRING: i32 = 95;
pub const SystemRDL_PLUS: i32 = 96;
pub const SystemRDL_MINUS: i32 = 97;
pub const SystemRDL_BNOT: i32 = 98;
pub const SystemRDL_NOT: i32 = 99;
pub const SystemRDL_BAND: i32 = 100;
pub const SystemRDL_NAND: i32 = 101;
pub const SystemRDL_AND: i32 = 102;
pub const SystemRDL_OR: i32 = 103;
pub const SystemRDL_BOR: i32 = 104;
pub const SystemRDL_NOR: i32 = 105;
pub const SystemRDL_XOR: i32 = 106;
pub const SystemRDL_XNOR: i32 = 107;
pub const SystemRDL_LSHIFT: i32 = 108;
pub const SystemRDL_RSHIFT: i32 = 109;
pub const SystemRDL_MULT: i32 = 110;
pub const SystemRDL_EXP: i32 = 111;
pub const SystemRDL_DIV: i32 = 112;
pub const SystemRDL_MOD: i32 = 113;
pub const SystemRDL_EQ: i32 = 114;
pub const SystemRDL_ASSIGN: i32 = 115;
pub const SystemRDL_NEQ: i32 = 116;
pub const SystemRDL_LEQ: i32 = 117;
pub const SystemRDL_LT: i32 = 118;
pub const SystemRDL_GEQ: i32 = 119;
pub const SystemRDL_GT: i32 = 120;
pub const SystemRDL_AT: i32 = 121;
pub const SystemRDL_INC: i32 = 122;
pub const SystemRDL_ALIGN: i32 = 123;
pub const SystemRDL_WS: i32 = 124;
pub const SystemRDL_ID: i32 = 125;
pub const SystemRDL_EOF: i32 = EOF;
pub const RULE_root: usize = 0;
pub const RULE_eval_expr_root: usize = 1;
pub const RULE_root_elem: usize = 2;
pub const RULE_component_def: usize = 3;
pub const RULE_explicit_component_inst: usize = 4;
pub const RULE_component_inst_alias: usize = 5;
pub const RULE_component_named_def: usize = 6;
pub const RULE_component_anon_def: usize = 7;
pub const RULE_component_body: usize = 8;
pub const RULE_component_body_elem: usize = 9;
pub const RULE_component_insts: usize = 10;
pub const RULE_component_inst: usize = 11;
pub const RULE_field_inst_reset: usize = 12;
pub const RULE_inst_addr_fixed: usize = 13;
pub const RULE_inst_addr_stride: usize = 14;
pub const RULE_inst_addr_align: usize = 15;
pub const RULE_component_inst_type: usize = 16;
pub const RULE_component_type: usize = 17;
pub const RULE_component_type_primary: usize = 18;
pub const RULE_param_def: usize = 19;
pub const RULE_param_def_elem: usize = 20;
pub const RULE_param_inst: usize = 21;
pub const RULE_param_assignment: usize = 22;
pub const RULE_expr: usize = 23;
pub const RULE_expr_primary: usize = 24;
pub const RULE_concatenate: usize = 25;
pub const RULE_replicate: usize = 26;
pub const RULE_paren_expr: usize = 27;
pub const RULE_cast: usize = 28;
pub const RULE_cast_width_expr: usize = 29;
pub const RULE_range_suffix: usize = 30;
pub const RULE_array_suffix: usize = 31;
pub const RULE_array_type_suffix: usize = 32;
pub const RULE_data_type: usize = 33;
pub const RULE_basic_data_type: usize = 34;
pub const RULE_literal: usize = 35;
pub const RULE_number: usize = 36;
pub const RULE_string_literal: usize = 37;
pub const RULE_boolean_literal: usize = 38;
pub const RULE_array_literal: usize = 39;
pub const RULE_struct_literal: usize = 40;
pub const RULE_struct_kv: usize = 41;
pub const RULE_enum_literal: usize = 42;
pub const RULE_accesstype_literal: usize = 43;
pub const RULE_onreadtype_literal: usize = 44;
pub const RULE_onwritetype_literal: usize = 45;
pub const RULE_addressingtype_literal: usize = 46;
pub const RULE_precedencetype_literal: usize = 47;
pub const RULE_instance_ref: usize = 48;
pub const RULE_instance_ref_element: usize = 49;
pub const RULE_prop_ref: usize = 50;
pub const RULE_local_property_assignment: usize = 51;
pub const RULE_dynamic_property_assignment: usize = 52;
pub const RULE_normal_prop_assign: usize = 53;
pub const RULE_encode_prop_assign: usize = 54;
pub const RULE_prop_mod_assign: usize = 55;
pub const RULE_prop_assignment_rhs: usize = 56;
pub const RULE_prop_keyword: usize = 57;
pub const RULE_prop_mod: usize = 58;
pub const RULE_udp_def: usize = 59;
pub const RULE_udp_attr: usize = 60;
pub const RULE_udp_type: usize = 61;
pub const RULE_udp_data_type: usize = 62;
pub const RULE_udp_usage: usize = 63;
pub const RULE_udp_comp_type: usize = 64;
pub const RULE_udp_default: usize = 65;
pub const RULE_udp_constraint: usize = 66;
pub const RULE_enum_def: usize = 67;
pub const RULE_enum_entry: usize = 68;
pub const RULE_enum_prop_assign: usize = 69;
pub const RULE_struct_def: usize = 70;
pub const RULE_struct_elem: usize = 71;
pub const RULE_struct_type: usize = 72;
pub const RULE_constraint_def: usize = 73;
pub const RULE_constraint_named_def: usize = 74;
pub const RULE_constraint_anon_def: usize = 75;
pub const RULE_constraint_body: usize = 76;
pub const RULE_constraint_body_elem: usize = 77;
pub const RULE_constraint_insts: usize = 78;
pub const RULE_constr_relational: usize = 79;
pub const RULE_constr_prop_assign: usize = 80;
pub const RULE_constr_inside_values: usize = 81;
pub const RULE_constr_inside_enum: usize = 82;
pub const RULE_constr_lhs: usize = 83;
pub const RULE_constr_inside_value: usize = 84;
pub const ruleNames: [&'static str; 85] = [
    "root",
    "eval_expr_root",
    "root_elem",
    "component_def",
    "explicit_component_inst",
    "component_inst_alias",
    "component_named_def",
    "component_anon_def",
    "component_body",
    "component_body_elem",
    "component_insts",
    "component_inst",
    "field_inst_reset",
    "inst_addr_fixed",
    "inst_addr_stride",
    "inst_addr_align",
    "component_inst_type",
    "component_type",
    "component_type_primary",
    "param_def",
    "param_def_elem",
    "param_inst",
    "param_assignment",
    "expr",
    "expr_primary",
    "concatenate",
    "replicate",
    "paren_expr",
    "cast",
    "cast_width_expr",
    "range_suffix",
    "array_suffix",
    "array_type_suffix",
    "data_type",
    "basic_data_type",
    "literal",
    "number",
    "string_literal",
    "boolean_literal",
    "array_literal",
    "struct_literal",
    "struct_kv",
    "enum_literal",
    "accesstype_literal",
    "onreadtype_literal",
    "onwritetype_literal",
    "addressingtype_literal",
    "precedencetype_literal",
    "instance_ref",
    "instance_ref_element",
    "prop_ref",
    "local_property_assignment",
    "dynamic_property_assignment",
    "normal_prop_assign",
    "encode_prop_assign",
    "prop_mod_assign",
    "prop_assignment_rhs",
    "prop_keyword",
    "prop_mod",
    "udp_def",
    "udp_attr",
    "udp_type",
    "udp_data_type",
    "udp_usage",
    "udp_comp_type",
    "udp_default",
    "udp_constraint",
    "enum_def",
    "enum_entry",
    "enum_prop_assign",
    "struct_def",
    "struct_elem",
    "struct_type",
    "constraint_def",
    "constraint_named_def",
    "constraint_anon_def",
    "constraint_body",
    "constraint_body_elem",
    "constraint_insts",
    "constr_relational",
    "constr_prop_assign",
    "constr_inside_values",
    "constr_inside_enum",
    "constr_lhs",
    "constr_inside_value",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 124] = [
    None,
    Some("';'"),
    Some("'{'"),
    Some("'}'"),
    Some("','"),
    Some("'#'"),
    Some("'('"),
    Some("')'"),
    Some("'.'"),
    Some("'?'"),
    Some("':'"),
    Some("'''"),
    Some("'['"),
    Some("']'"),
    Some("'::'"),
    Some("'->'"),
    None,
    None,
    Some("'boolean'"),
    Some("'bit'"),
    Some("'longint'"),
    Some("'unsigned'"),
    Some("'string'"),
    Some("'accesstype'"),
    Some("'addressingtype'"),
    Some("'onreadtype'"),
    Some("'onwritetype'"),
    Some("'alias'"),
    Some("'external'"),
    Some("'internal'"),
    Some("'addrmap'"),
    Some("'regfile'"),
    Some("'reg'"),
    Some("'field'"),
    Some("'mem'"),
    Some("'signal'"),
    Some("'true'"),
    Some("'false'"),
    Some("'na'"),
    Some("'rw'"),
    Some("'wr'"),
    Some("'r'"),
    Some("'w'"),
    Some("'rw1'"),
    Some("'w1'"),
    Some("'rclr'"),
    Some("'rset'"),
    Some("'ruser'"),
    Some("'woset'"),
    Some("'woclr'"),
    Some("'wot'"),
    Some("'wzs'"),
    Some("'wzc'"),
    Some("'wzt'"),
    Some("'wclr'"),
    Some("'wset'"),
    Some("'wuser'"),
    Some("'compact'"),
    Some("'regalign'"),
    Some("'fullalign'"),
    Some("'hw'"),
    Some("'sw'"),
    Some("'posedge'"),
    Some("'negedge'"),
    Some("'bothedge'"),
    Some("'level'"),
    Some("'nonsticky'"),
    Some("'abstract'"),
    Some("'all'"),
    Some("'component'"),
    Some("'componentwidth'"),
    Some("'constraint'"),
    Some("'default'"),
    Some("'enum'"),
    Some("'encode'"),
    Some("'inside'"),
    Some("'number'"),
    Some("'property'"),
    Some("'ref'"),
    Some("'struct'"),
    Some("'this'"),
    Some("'type'"),
    Some("'alternate'"),
    Some("'byte'"),
    Some("'int'"),
    Some("'precedencetype'"),
    Some("'real'"),
    Some("'shortint'"),
    Some("'shortreal'"),
    Some("'signed'"),
    Some("'with'"),
    Some("'within'"),
    None,
    None,
    None,
    None,
    Some("'+'"),
    Some("'-'"),
    Some("'!'"),
    Some("'~'"),
    Some("'&&'"),
    Some("'~&'"),
    Some("'&'"),
    Some("'|'"),
    Some("'||'"),
    Some("'~|'"),
    Some("'^'"),
    None,
    Some("'<<'"),
    Some("'>>'"),
    Some("'*'"),
    Some("'**'"),
    Some("'/'"),
    Some("'%'"),
    Some("'=='"),
    Some("'='"),
    Some("'!='"),
    Some("'<='"),
    Some("'<'"),
    Some("'>='"),
    Some("'>'"),
    Some("'@'"),
    Some("'+='"),
    Some("'%='"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 126] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("SL_COMMENT"),
    Some("ML_COMMENT"),
    Some("BOOLEAN_kw"),
    Some("BIT_kw"),
    Some("LONGINT_kw"),
    Some("UNSIGNED_kw"),
    Some("STRING_kw"),
    Some("ACCESSTYPE_kw"),
    Some("ADDRESSINGTYPE_kw"),
    Some("ONREADTYPE_kw"),
    Some("ONWRITETYPE_kw"),
    Some("ALIAS_kw"),
    Some("EXTERNAL_kw"),
    Some("INTERNAL_kw"),
    Some("ADDRMAP_kw"),
    Some("REGFILE_kw"),
    Some("REG_kw"),
    Some("FIELD_kw"),
    Some("MEM_kw"),
    Some("SIGNAL_kw"),
    Some("TRUE_kw"),
    Some("FALSE_kw"),
    Some("NA_kw"),
    Some("RW_kw"),
    Some("WR_kw"),
    Some("R_kw"),
    Some("W_kw"),
    Some("RW1_kw"),
    Some("W1_kw"),
    Some("RCLR_kw"),
    Some("RSET_kw"),
    Some("RUSER_kw"),
    Some("WOSET_kw"),
    Some("WOCLR_kw"),
    Some("WOT_kw"),
    Some("WZS_kw"),
    Some("WZC_kw"),
    Some("WZT_kw"),
    Some("WCLR_kw"),
    Some("WSET_kw"),
    Some("WUSER_kw"),
    Some("COMPACT_kw"),
    Some("REGALIGN_kw"),
    Some("FULLALIGN_kw"),
    Some("HW_kw"),
    Some("SW_kw"),
    Some("POSEDGE_kw"),
    Some("NEGEDGE_kw"),
    Some("BOTHEDGE_kw"),
    Some("LEVEL_kw"),
    Some("NONSTICKY_kw"),
    Some("ABSTRACT_kw"),
    Some("ALL_kw"),
    Some("COMPONENT_kw"),
    Some("COMPONENTWIDTH_kw"),
    Some("CONSTRAINT_kw"),
    Some("DEFAULT_kw"),
    Some("ENUM_kw"),
    Some("ENCODE_kw"),
    Some("INSIDE_kw"),
    Some("NUMBER_kw"),
    Some("PROPERTY_kw"),
    Some("REF_kw"),
    Some("STRUCT_kw"),
    Some("THIS_kw"),
    Some("TYPE_kw"),
    Some("ALTERNATE_kw"),
    Some("BYTE_kw"),
    Some("INT_kw"),
    Some("PRECEDENCETYPE_kw"),
    Some("REAL_kw"),
    Some("SHORTINT_kw"),
    Some("SHORTREAL_kw"),
    Some("SIGNED_kw"),
    Some("WITH_kw"),
    Some("WITHIN_kw"),
    Some("INT"),
    Some("HEX_INT"),
    Some("VLOG_INT"),
    Some("STRING"),
    Some("PLUS"),
    Some("MINUS"),
    Some("BNOT"),
    Some("NOT"),
    Some("BAND"),
    Some("NAND"),
    Some("AND"),
    Some("OR"),
    Some("BOR"),
    Some("NOR"),
    Some("XOR"),
    Some("XNOR"),
    Some("LSHIFT"),
    Some("RSHIFT"),
    Some("MULT"),
    Some("EXP"),
    Some("DIV"),
    Some("MOD"),
    Some("EQ"),
    Some("ASSIGN"),
    Some("NEQ"),
    Some("LEQ"),
    Some("LT"),
    Some("GEQ"),
    Some("GT"),
    Some("AT"),
    Some("INC"),
    Some("ALIGN"),
    Some("WS"),
    Some("ID"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    SystemRDLParserExt<'input>,
    I,
    SystemRDLParserContextType,
    dyn SystemRDLListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SystemRDLTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, SystemRDLParserContextType, dyn SystemRDLListener<'input> + 'a>;

/// Parser for SystemRDL grammar
pub struct SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>>>,
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn set_error_strategy(
        &mut self,
        strategy: Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>>>,
    ) {
        self.err_handler = strategy
    }

    pub fn with_strategy(
        input: I,
        strategy: Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>>>,
    ) -> Self {
        antlr4rust::recognizer::check_version("0", "5");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                SystemRDLParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SystemRDLParser
pub trait SystemRDLParserContext<'input>:
    for<'x> Listenable<dyn SystemRDLListener<'input> + 'x>
    + for<'x> Visitable<dyn SystemRDLVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = SystemRDLParserContextType>
{
}

antlr4rust::coerce_from! { 'input : SystemRDLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn SystemRDLParserContext<'input> + 'input
where
    T: SystemRDLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn SystemRDLVisitor<'input> + 'x))
    }
}

impl<'input> SystemRDLParserContext<'input> for TerminalNode<'input, SystemRDLParserContextType> {}
impl<'input> SystemRDLParserContext<'input> for ErrorNode<'input, SystemRDLParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn SystemRDLParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn SystemRDLListener<'input> + 'input }

pub struct SystemRDLParserContextType;
antlr4rust::tid! {SystemRDLParserContextType}

impl<'input> ParserNodeType<'input> for SystemRDLParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn SystemRDLParserContext<'input> + 'input;
}

impl<'input, I> Deref for SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SystemRDLParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserExt<'input> {}
antlr4rust::tid! { SystemRDLParserExt<'a> }

impl<'input> TokenAware<'input> for SystemRDLParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for SystemRDLParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for SystemRDLParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "SystemRDL.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn SystemRDLParserContext<'input> + 'input)>,
        rule_index: i32,
        pred_index: i32,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            23 => SystemRDLParser::<'input, I>::expr_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn expr_sempred(
        _localctx: Option<&ExprContext<'input>>,
        pred_index: i32,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 13),
            1 => recog.precpred(None, 12),
            2 => recog.precpred(None, 11),
            3 => recog.precpred(None, 10),
            4 => recog.precpred(None, 9),
            5 => recog.precpred(None, 8),
            6 => recog.precpred(None, 7),
            7 => recog.precpred(None, 6),
            8 => recog.precpred(None, 5),
            9 => recog.precpred(None, 4),
            10 => recog.precpred(None, 3),
            11 => recog.precpred(None, 2),
            _ => true,
        }
    }
}
//------------------- root ----------------
pub type RootContextAll<'input> = RootContext<'input>;

pub type RootContext<'input> = BaseParserRuleContext<'input, RootContextExt<'input>>;

#[derive(Clone)]
pub struct RootContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for RootContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for RootContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_root(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_root(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for RootContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_root(self);
    }
}

impl<'input> CustomRuleContext<'input> for RootContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_root
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_root }
}
antlr4rust::tid! {RootContextExt<'a>}

impl<'input> RootContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<RootContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            RootContextExt { ph: PhantomData },
        ))
    }
}

pub trait RootContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<RootContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_EOF, 0)
    }
    fn root_elem_all(&self) -> Vec<Rc<Root_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn root_elem(&self, i: usize) -> Option<Rc<Root_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> RootContextAttrs<'input> for RootContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn root(&mut self) -> Result<Rc<RootContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = RootContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_root);
        let mut _localctx: Rc<RootContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(175);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 27) & !0x3f) == 0 && ((1usize << (_la - 27)) & 7078399) != 0)
                    || (((_la - 60) & !0x3f) == 0 && ((1usize << (_la - 60)) & 686335) != 0)
                    || _la == SystemRDL_ID
                {
                    {
                        {
                            /*InvokeRule root_elem*/
                            recog.base.set_state(170);
                            recog.root_elem()?;

                            recog.base.set_state(171);
                            recog
                                .base
                                .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(177);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(178);
                recog
                    .base
                    .match_token(SystemRDL_EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- eval_expr_root ----------------
pub type Eval_expr_rootContextAll<'input> = Eval_expr_rootContext<'input>;

pub type Eval_expr_rootContext<'input> =
    BaseParserRuleContext<'input, Eval_expr_rootContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_expr_rootContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Eval_expr_rootContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Eval_expr_rootContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_eval_expr_root(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_eval_expr_root(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Eval_expr_rootContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_eval_expr_root(self);
    }
}

impl<'input> CustomRuleContext<'input> for Eval_expr_rootContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_eval_expr_root
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_eval_expr_root }
}
antlr4rust::tid! {Eval_expr_rootContextExt<'a>}

impl<'input> Eval_expr_rootContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Eval_expr_rootContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Eval_expr_rootContextExt { ph: PhantomData },
        ))
    }
}

pub trait Eval_expr_rootContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Eval_expr_rootContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_EOF, 0)
    }
}

impl<'input> Eval_expr_rootContextAttrs<'input> for Eval_expr_rootContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn eval_expr_root(&mut self) -> Result<Rc<Eval_expr_rootContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Eval_expr_rootContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 2, RULE_eval_expr_root);
        let mut _localctx: Rc<Eval_expr_rootContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule expr*/
                recog.base.set_state(180);
                recog.expr_rec(0)?;

                recog.base.set_state(181);
                recog
                    .base
                    .match_token(SystemRDL_EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- root_elem ----------------
pub type Root_elemContextAll<'input> = Root_elemContext<'input>;

pub type Root_elemContext<'input> = BaseParserRuleContext<'input, Root_elemContextExt<'input>>;

#[derive(Clone)]
pub struct Root_elemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Root_elemContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Root_elemContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_root_elem(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_root_elem(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Root_elemContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_root_elem(self);
    }
}

impl<'input> CustomRuleContext<'input> for Root_elemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_root_elem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_root_elem }
}
antlr4rust::tid! {Root_elemContextExt<'a>}

impl<'input> Root_elemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Root_elemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Root_elemContextExt { ph: PhantomData },
        ))
    }
}

pub trait Root_elemContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Root_elemContextExt<'input>>
{
    fn component_def(&self) -> Option<Rc<Component_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn enum_def(&self) -> Option<Rc<Enum_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn udp_def(&self) -> Option<Rc<Udp_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn struct_def(&self) -> Option<Rc<Struct_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constraint_def(&self) -> Option<Rc<Constraint_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn explicit_component_inst(&self) -> Option<Rc<Explicit_component_instContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn local_property_assignment(&self) -> Option<Rc<Local_property_assignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn dynamic_property_assignment(
        &self,
    ) -> Option<Rc<Dynamic_property_assignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Root_elemContextAttrs<'input> for Root_elemContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn root_elem(&mut self) -> Result<Rc<Root_elemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Root_elemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_root_elem);
        let mut _localctx: Rc<Root_elemContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(191);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(1, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule component_def*/
                        recog.base.set_state(183);
                        recog.component_def()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule enum_def*/
                        recog.base.set_state(184);
                        recog.enum_def()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule udp_def*/
                        recog.base.set_state(185);
                        recog.udp_def()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule struct_def*/
                        recog.base.set_state(186);
                        recog.struct_def()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5)?;
                    recog.base.enter_outer_alt(None, 5)?;
                    {
                        /*InvokeRule constraint_def*/
                        recog.base.set_state(187);
                        recog.constraint_def()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6)?;
                    recog.base.enter_outer_alt(None, 6)?;
                    {
                        /*InvokeRule explicit_component_inst*/
                        recog.base.set_state(188);
                        recog.explicit_component_inst()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7)?;
                    recog.base.enter_outer_alt(None, 7)?;
                    {
                        /*InvokeRule local_property_assignment*/
                        recog.base.set_state(189);
                        recog.local_property_assignment()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8)?;
                    recog.base.enter_outer_alt(None, 8)?;
                    {
                        /*InvokeRule dynamic_property_assignment*/
                        recog.base.set_state(190);
                        recog.dynamic_property_assignment()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_def ----------------
pub type Component_defContextAll<'input> = Component_defContext<'input>;

pub type Component_defContext<'input> =
    BaseParserRuleContext<'input, Component_defContextExt<'input>>;

#[derive(Clone)]
pub struct Component_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Component_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Component_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_def }
}
antlr4rust::tid! {Component_defContextExt<'a>}

impl<'input> Component_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_defContextExt<'input>>
{
    fn component_named_def(&self) -> Option<Rc<Component_named_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_inst_type(&self) -> Option<Rc<Component_inst_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_insts(&self) -> Option<Rc<Component_instsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_anon_def(&self) -> Option<Rc<Component_anon_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Component_defContextAttrs<'input> for Component_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_def(&mut self) -> Result<Rc<Component_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_component_def);
        let mut _localctx: Rc<Component_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(217);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(5, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule component_named_def*/
                        recog.base.set_state(193);
                        recog.component_named_def()?;

                        recog.base.set_state(200);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            SystemRDL_EXTERNAL_kw | SystemRDL_INTERNAL_kw => {
                                {
                                    {
                                        /*InvokeRule component_inst_type*/
                                        recog.base.set_state(194);
                                        recog.component_inst_type()?;

                                        /*InvokeRule component_insts*/
                                        recog.base.set_state(195);
                                        recog.component_insts()?;
                                    }
                                }
                            }

                            SystemRDL_T__0 | SystemRDL_T__4 | SystemRDL_ID => {
                                {
                                    recog.base.set_state(198);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SystemRDL_T__4 || _la == SystemRDL_ID {
                                        {
                                            /*InvokeRule component_insts*/
                                            recog.base.set_state(197);
                                            recog.component_insts()?;
                                        }
                                    }
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule component_anon_def*/
                        recog.base.set_state(202);
                        recog.component_anon_def()?;

                        recog.base.set_state(207);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            SystemRDL_EXTERNAL_kw | SystemRDL_INTERNAL_kw => {
                                {
                                    {
                                        /*InvokeRule component_inst_type*/
                                        recog.base.set_state(203);
                                        recog.component_inst_type()?;

                                        /*InvokeRule component_insts*/
                                        recog.base.set_state(204);
                                        recog.component_insts()?;
                                    }
                                }
                            }

                            SystemRDL_T__4 | SystemRDL_ID => {
                                {
                                    /*InvokeRule component_insts*/
                                    recog.base.set_state(206);
                                    recog.component_insts()?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule component_inst_type*/
                        recog.base.set_state(209);
                        recog.component_inst_type()?;

                        /*InvokeRule component_named_def*/
                        recog.base.set_state(210);
                        recog.component_named_def()?;

                        /*InvokeRule component_insts*/
                        recog.base.set_state(211);
                        recog.component_insts()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule component_inst_type*/
                        recog.base.set_state(213);
                        recog.component_inst_type()?;

                        /*InvokeRule component_anon_def*/
                        recog.base.set_state(214);
                        recog.component_anon_def()?;

                        /*InvokeRule component_insts*/
                        recog.base.set_state(215);
                        recog.component_insts()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- explicit_component_inst ----------------
pub type Explicit_component_instContextAll<'input> = Explicit_component_instContext<'input>;

pub type Explicit_component_instContext<'input> =
    BaseParserRuleContext<'input, Explicit_component_instContextExt<'input>>;

#[derive(Clone)]
pub struct Explicit_component_instContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Explicit_component_instContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Explicit_component_instContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_explicit_component_inst(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_explicit_component_inst(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Explicit_component_instContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_explicit_component_inst(self);
    }
}

impl<'input> CustomRuleContext<'input> for Explicit_component_instContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_explicit_component_inst
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_explicit_component_inst }
}
antlr4rust::tid! {Explicit_component_instContextExt<'a>}

impl<'input> Explicit_component_instContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Explicit_component_instContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Explicit_component_instContextExt { ph: PhantomData },
        ))
    }
}

pub trait Explicit_component_instContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Explicit_component_instContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn component_insts(&self) -> Option<Rc<Component_instsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_inst_type(&self) -> Option<Rc<Component_inst_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_inst_alias(&self) -> Option<Rc<Component_inst_aliasContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Explicit_component_instContextAttrs<'input>
    for Explicit_component_instContext<'input>
{
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn explicit_component_inst(
        &mut self,
    ) -> Result<Rc<Explicit_component_instContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Explicit_component_instContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_explicit_component_inst);
        let mut _localctx: Rc<Explicit_component_instContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(220);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_EXTERNAL_kw || _la == SystemRDL_INTERNAL_kw {
                    {
                        /*InvokeRule component_inst_type*/
                        recog.base.set_state(219);
                        recog.component_inst_type()?;
                    }
                }

                recog.base.set_state(223);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ALIAS_kw {
                    {
                        /*InvokeRule component_inst_alias*/
                        recog.base.set_state(222);
                        recog.component_inst_alias()?;
                    }
                }

                recog.base.set_state(225);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                /*InvokeRule component_insts*/
                recog.base.set_state(226);
                recog.component_insts()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_inst_alias ----------------
pub type Component_inst_aliasContextAll<'input> = Component_inst_aliasContext<'input>;

pub type Component_inst_aliasContext<'input> =
    BaseParserRuleContext<'input, Component_inst_aliasContextExt<'input>>;

#[derive(Clone)]
pub struct Component_inst_aliasContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_inst_aliasContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Component_inst_aliasContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_inst_alias(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_inst_alias(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Component_inst_aliasContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_inst_alias(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_inst_aliasContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_inst_alias
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_inst_alias }
}
antlr4rust::tid! {Component_inst_aliasContextExt<'a>}

impl<'input> Component_inst_aliasContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_inst_aliasContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_inst_aliasContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_inst_aliasContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_inst_aliasContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ALIAS_kw
    /// Returns `None` if there is no child corresponding to token ALIAS_kw
    fn ALIAS_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ALIAS_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
}

impl<'input> Component_inst_aliasContextAttrs<'input> for Component_inst_aliasContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_inst_alias(
        &mut self,
    ) -> Result<Rc<Component_inst_aliasContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_inst_aliasContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_component_inst_alias);
        let mut _localctx: Rc<Component_inst_aliasContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(228);
                recog
                    .base
                    .match_token(SystemRDL_ALIAS_kw, &mut recog.err_handler)?;

                recog.base.set_state(229);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_named_def ----------------
pub type Component_named_defContextAll<'input> = Component_named_defContext<'input>;

pub type Component_named_defContext<'input> =
    BaseParserRuleContext<'input, Component_named_defContextExt<'input>>;

#[derive(Clone)]
pub struct Component_named_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_named_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Component_named_defContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_named_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_named_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Component_named_defContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_named_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_named_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_named_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_named_def }
}
antlr4rust::tid! {Component_named_defContextExt<'a>}

impl<'input> Component_named_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_named_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_named_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_named_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_named_defContextExt<'input>>
{
    fn component_type(&self) -> Option<Rc<Component_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn component_body(&self) -> Option<Rc<Component_bodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn param_def(&self) -> Option<Rc<Param_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Component_named_defContextAttrs<'input> for Component_named_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_named_def(
        &mut self,
    ) -> Result<Rc<Component_named_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_named_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_component_named_def);
        let mut _localctx: Rc<Component_named_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule component_type*/
                recog.base.set_state(231);
                recog.component_type()?;

                recog.base.set_state(232);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(234);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__4 {
                    {
                        /*InvokeRule param_def*/
                        recog.base.set_state(233);
                        recog.param_def()?;
                    }
                }

                /*InvokeRule component_body*/
                recog.base.set_state(236);
                recog.component_body()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_anon_def ----------------
pub type Component_anon_defContextAll<'input> = Component_anon_defContext<'input>;

pub type Component_anon_defContext<'input> =
    BaseParserRuleContext<'input, Component_anon_defContextExt<'input>>;

#[derive(Clone)]
pub struct Component_anon_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_anon_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Component_anon_defContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_anon_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_anon_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Component_anon_defContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_anon_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_anon_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_anon_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_anon_def }
}
antlr4rust::tid! {Component_anon_defContextExt<'a>}

impl<'input> Component_anon_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_anon_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_anon_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_anon_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_anon_defContextExt<'input>>
{
    fn component_type(&self) -> Option<Rc<Component_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_body(&self) -> Option<Rc<Component_bodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Component_anon_defContextAttrs<'input> for Component_anon_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_anon_def(
        &mut self,
    ) -> Result<Rc<Component_anon_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_anon_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_component_anon_def);
        let mut _localctx: Rc<Component_anon_defContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule component_type*/
                recog.base.set_state(238);
                recog.component_type()?;

                /*InvokeRule component_body*/
                recog.base.set_state(239);
                recog.component_body()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_body ----------------
pub type Component_bodyContextAll<'input> = Component_bodyContext<'input>;

pub type Component_bodyContext<'input> =
    BaseParserRuleContext<'input, Component_bodyContextExt<'input>>;

#[derive(Clone)]
pub struct Component_bodyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_bodyContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Component_bodyContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_body(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_body(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Component_bodyContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_body(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_bodyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_body
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_body }
}
antlr4rust::tid! {Component_bodyContextExt<'a>}

impl<'input> Component_bodyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_bodyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_bodyContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_bodyContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_bodyContextExt<'input>>
{
    fn component_body_elem_all(&self) -> Vec<Rc<Component_body_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn component_body_elem(&self, i: usize) -> Option<Rc<Component_body_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Component_bodyContextAttrs<'input> for Component_bodyContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_body(&mut self) -> Result<Rc<Component_bodyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_bodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 16, RULE_component_body);
        let mut _localctx: Rc<Component_bodyContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(241);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                recog.base.set_state(247);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 27) & !0x3f) == 0 && ((1usize << (_la - 27)) & 7078399) != 0)
                    || (((_la - 60) & !0x3f) == 0 && ((1usize << (_la - 60)) & 555263) != 0)
                    || _la == SystemRDL_ID
                {
                    {
                        {
                            /*InvokeRule component_body_elem*/
                            recog.base.set_state(242);
                            recog.component_body_elem()?;

                            recog.base.set_state(243);
                            recog
                                .base
                                .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(249);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(250);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_body_elem ----------------
pub type Component_body_elemContextAll<'input> = Component_body_elemContext<'input>;

pub type Component_body_elemContext<'input> =
    BaseParserRuleContext<'input, Component_body_elemContextExt<'input>>;

#[derive(Clone)]
pub struct Component_body_elemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_body_elemContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Component_body_elemContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_body_elem(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_body_elem(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Component_body_elemContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_body_elem(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_body_elemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_body_elem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_body_elem }
}
antlr4rust::tid! {Component_body_elemContextExt<'a>}

impl<'input> Component_body_elemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_body_elemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_body_elemContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_body_elemContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_body_elemContextExt<'input>>
{
    fn component_def(&self) -> Option<Rc<Component_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn enum_def(&self) -> Option<Rc<Enum_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn struct_def(&self) -> Option<Rc<Struct_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constraint_def(&self) -> Option<Rc<Constraint_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn explicit_component_inst(&self) -> Option<Rc<Explicit_component_instContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn local_property_assignment(&self) -> Option<Rc<Local_property_assignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn dynamic_property_assignment(
        &self,
    ) -> Option<Rc<Dynamic_property_assignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Component_body_elemContextAttrs<'input> for Component_body_elemContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_body_elem(
        &mut self,
    ) -> Result<Rc<Component_body_elemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_body_elemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_component_body_elem);
        let mut _localctx: Rc<Component_body_elemContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(259);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(10, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule component_def*/
                        recog.base.set_state(252);
                        recog.component_def()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule enum_def*/
                        recog.base.set_state(253);
                        recog.enum_def()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule struct_def*/
                        recog.base.set_state(254);
                        recog.struct_def()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule constraint_def*/
                        recog.base.set_state(255);
                        recog.constraint_def()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5)?;
                    recog.base.enter_outer_alt(None, 5)?;
                    {
                        /*InvokeRule explicit_component_inst*/
                        recog.base.set_state(256);
                        recog.explicit_component_inst()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6)?;
                    recog.base.enter_outer_alt(None, 6)?;
                    {
                        /*InvokeRule local_property_assignment*/
                        recog.base.set_state(257);
                        recog.local_property_assignment()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7)?;
                    recog.base.enter_outer_alt(None, 7)?;
                    {
                        /*InvokeRule dynamic_property_assignment*/
                        recog.base.set_state(258);
                        recog.dynamic_property_assignment()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_insts ----------------
pub type Component_instsContextAll<'input> = Component_instsContext<'input>;

pub type Component_instsContext<'input> =
    BaseParserRuleContext<'input, Component_instsContextExt<'input>>;

#[derive(Clone)]
pub struct Component_instsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_instsContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Component_instsContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_insts(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_insts(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Component_instsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_insts(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_instsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_insts
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_insts }
}
antlr4rust::tid! {Component_instsContextExt<'a>}

impl<'input> Component_instsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_instsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_instsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_instsContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_instsContextExt<'input>>
{
    fn component_inst_all(&self) -> Vec<Rc<Component_instContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn component_inst(&self, i: usize) -> Option<Rc<Component_instContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn param_inst(&self) -> Option<Rc<Param_instContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Component_instsContextAttrs<'input> for Component_instsContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_insts(&mut self) -> Result<Rc<Component_instsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_instsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_component_insts);
        let mut _localctx: Rc<Component_instsContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(262);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__4 {
                    {
                        /*InvokeRule param_inst*/
                        recog.base.set_state(261);
                        recog.param_inst()?;
                    }
                }

                /*InvokeRule component_inst*/
                recog.base.set_state(264);
                recog.component_inst()?;

                recog.base.set_state(269);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_T__3 {
                    {
                        {
                            recog.base.set_state(265);
                            recog
                                .base
                                .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                            /*InvokeRule component_inst*/
                            recog.base.set_state(266);
                            recog.component_inst()?;
                        }
                    }
                    recog.base.set_state(271);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_inst ----------------
pub type Component_instContextAll<'input> = Component_instContext<'input>;

pub type Component_instContext<'input> =
    BaseParserRuleContext<'input, Component_instContextExt<'input>>;

#[derive(Clone)]
pub struct Component_instContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_instContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Component_instContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_inst(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_inst(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Component_instContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_inst(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_instContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_inst
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_inst }
}
antlr4rust::tid! {Component_instContextExt<'a>}

impl<'input> Component_instContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_instContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_instContextExt { ph: PhantomData },
        ))
    }
}

pub trait Component_instContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_instContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn range_suffix(&self) -> Option<Rc<Range_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn field_inst_reset(&self) -> Option<Rc<Field_inst_resetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn inst_addr_fixed(&self) -> Option<Rc<Inst_addr_fixedContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn inst_addr_stride(&self) -> Option<Rc<Inst_addr_strideContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn inst_addr_align(&self) -> Option<Rc<Inst_addr_alignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn array_suffix_all(&self) -> Vec<Rc<Array_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn array_suffix(&self, i: usize) -> Option<Rc<Array_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Component_instContextAttrs<'input> for Component_instContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_inst(&mut self) -> Result<Rc<Component_instContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_instContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_component_inst);
        let mut _localctx: Rc<Component_instContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(272);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(279);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(14, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(274);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            loop {
                                {
                                    {
                                        /*InvokeRule array_suffix*/
                                        recog.base.set_state(273);
                                        recog.array_suffix()?;
                                    }
                                }
                                recog.base.set_state(276);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if !(_la == SystemRDL_T__11) {
                                    break;
                                }
                            }
                        }
                    }

                    x if x == 2 => {
                        {
                            /*InvokeRule range_suffix*/
                            recog.base.set_state(278);
                            recog.range_suffix()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(282);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ASSIGN {
                    {
                        /*InvokeRule field_inst_reset*/
                        recog.base.set_state(281);
                        recog.field_inst_reset()?;
                    }
                }

                recog.base.set_state(285);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_AT {
                    {
                        /*InvokeRule inst_addr_fixed*/
                        recog.base.set_state(284);
                        recog.inst_addr_fixed()?;
                    }
                }

                recog.base.set_state(288);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_INC {
                    {
                        /*InvokeRule inst_addr_stride*/
                        recog.base.set_state(287);
                        recog.inst_addr_stride()?;
                    }
                }

                recog.base.set_state(291);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ALIGN {
                    {
                        /*InvokeRule inst_addr_align*/
                        recog.base.set_state(290);
                        recog.inst_addr_align()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- field_inst_reset ----------------
pub type Field_inst_resetContextAll<'input> = Field_inst_resetContext<'input>;

pub type Field_inst_resetContext<'input> =
    BaseParserRuleContext<'input, Field_inst_resetContextExt<'input>>;

#[derive(Clone)]
pub struct Field_inst_resetContextExt<'input> {
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Field_inst_resetContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Field_inst_resetContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_field_inst_reset(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_field_inst_reset(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Field_inst_resetContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_field_inst_reset(self);
    }
}

impl<'input> CustomRuleContext<'input> for Field_inst_resetContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_field_inst_reset
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_field_inst_reset }
}
antlr4rust::tid! {Field_inst_resetContextExt<'a>}

impl<'input> Field_inst_resetContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Field_inst_resetContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Field_inst_resetContextExt {
                op: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Field_inst_resetContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Field_inst_resetContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
}

impl<'input> Field_inst_resetContextAttrs<'input> for Field_inst_resetContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn field_inst_reset(
        &mut self,
    ) -> Result<Rc<Field_inst_resetContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Field_inst_resetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 24, RULE_field_inst_reset);
        let mut _localctx: Rc<Field_inst_resetContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(293);
                let tmp = recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;
                cast_mut::<_, Field_inst_resetContext>(&mut _localctx).op = Some(tmp.clone());

                /*InvokeRule expr*/
                recog.base.set_state(294);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- inst_addr_fixed ----------------
pub type Inst_addr_fixedContextAll<'input> = Inst_addr_fixedContext<'input>;

pub type Inst_addr_fixedContext<'input> =
    BaseParserRuleContext<'input, Inst_addr_fixedContextExt<'input>>;

#[derive(Clone)]
pub struct Inst_addr_fixedContextExt<'input> {
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Inst_addr_fixedContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Inst_addr_fixedContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_inst_addr_fixed(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_inst_addr_fixed(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Inst_addr_fixedContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_inst_addr_fixed(self);
    }
}

impl<'input> CustomRuleContext<'input> for Inst_addr_fixedContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_inst_addr_fixed
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_inst_addr_fixed }
}
antlr4rust::tid! {Inst_addr_fixedContextExt<'a>}

impl<'input> Inst_addr_fixedContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Inst_addr_fixedContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Inst_addr_fixedContextExt {
                op: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Inst_addr_fixedContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Inst_addr_fixedContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_AT, 0)
    }
}

impl<'input> Inst_addr_fixedContextAttrs<'input> for Inst_addr_fixedContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn inst_addr_fixed(&mut self) -> Result<Rc<Inst_addr_fixedContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Inst_addr_fixedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_inst_addr_fixed);
        let mut _localctx: Rc<Inst_addr_fixedContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(296);
                let tmp = recog
                    .base
                    .match_token(SystemRDL_AT, &mut recog.err_handler)?;
                cast_mut::<_, Inst_addr_fixedContext>(&mut _localctx).op = Some(tmp.clone());

                /*InvokeRule expr*/
                recog.base.set_state(297);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- inst_addr_stride ----------------
pub type Inst_addr_strideContextAll<'input> = Inst_addr_strideContext<'input>;

pub type Inst_addr_strideContext<'input> =
    BaseParserRuleContext<'input, Inst_addr_strideContextExt<'input>>;

#[derive(Clone)]
pub struct Inst_addr_strideContextExt<'input> {
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Inst_addr_strideContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Inst_addr_strideContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_inst_addr_stride(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_inst_addr_stride(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Inst_addr_strideContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_inst_addr_stride(self);
    }
}

impl<'input> CustomRuleContext<'input> for Inst_addr_strideContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_inst_addr_stride
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_inst_addr_stride }
}
antlr4rust::tid! {Inst_addr_strideContextExt<'a>}

impl<'input> Inst_addr_strideContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Inst_addr_strideContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Inst_addr_strideContextExt {
                op: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Inst_addr_strideContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Inst_addr_strideContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INC
    /// Returns `None` if there is no child corresponding to token INC
    fn INC(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_INC, 0)
    }
}

impl<'input> Inst_addr_strideContextAttrs<'input> for Inst_addr_strideContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn inst_addr_stride(
        &mut self,
    ) -> Result<Rc<Inst_addr_strideContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Inst_addr_strideContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_inst_addr_stride);
        let mut _localctx: Rc<Inst_addr_strideContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(299);
                let tmp = recog
                    .base
                    .match_token(SystemRDL_INC, &mut recog.err_handler)?;
                cast_mut::<_, Inst_addr_strideContext>(&mut _localctx).op = Some(tmp.clone());

                /*InvokeRule expr*/
                recog.base.set_state(300);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- inst_addr_align ----------------
pub type Inst_addr_alignContextAll<'input> = Inst_addr_alignContext<'input>;

pub type Inst_addr_alignContext<'input> =
    BaseParserRuleContext<'input, Inst_addr_alignContextExt<'input>>;

#[derive(Clone)]
pub struct Inst_addr_alignContextExt<'input> {
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Inst_addr_alignContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Inst_addr_alignContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_inst_addr_align(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_inst_addr_align(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Inst_addr_alignContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_inst_addr_align(self);
    }
}

impl<'input> CustomRuleContext<'input> for Inst_addr_alignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_inst_addr_align
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_inst_addr_align }
}
antlr4rust::tid! {Inst_addr_alignContextExt<'a>}

impl<'input> Inst_addr_alignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Inst_addr_alignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Inst_addr_alignContextExt {
                op: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Inst_addr_alignContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Inst_addr_alignContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ALIGN
    /// Returns `None` if there is no child corresponding to token ALIGN
    fn ALIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ALIGN, 0)
    }
}

impl<'input> Inst_addr_alignContextAttrs<'input> for Inst_addr_alignContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn inst_addr_align(&mut self) -> Result<Rc<Inst_addr_alignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Inst_addr_alignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 30, RULE_inst_addr_align);
        let mut _localctx: Rc<Inst_addr_alignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(302);
                let tmp = recog
                    .base
                    .match_token(SystemRDL_ALIGN, &mut recog.err_handler)?;
                cast_mut::<_, Inst_addr_alignContext>(&mut _localctx).op = Some(tmp.clone());

                /*InvokeRule expr*/
                recog.base.set_state(303);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_inst_type ----------------
pub type Component_inst_typeContextAll<'input> = Component_inst_typeContext<'input>;

pub type Component_inst_typeContext<'input> =
    BaseParserRuleContext<'input, Component_inst_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Component_inst_typeContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_inst_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Component_inst_typeContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_inst_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_inst_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Component_inst_typeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_inst_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_inst_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_inst_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_inst_type }
}
antlr4rust::tid! {Component_inst_typeContextExt<'a>}

impl<'input> Component_inst_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_inst_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_inst_typeContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Component_inst_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_inst_typeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EXTERNAL_kw
    /// Returns `None` if there is no child corresponding to token EXTERNAL_kw
    fn EXTERNAL_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_EXTERNAL_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INTERNAL_kw
    /// Returns `None` if there is no child corresponding to token INTERNAL_kw
    fn INTERNAL_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_INTERNAL_kw, 0)
    }
}

impl<'input> Component_inst_typeContextAttrs<'input> for Component_inst_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_inst_type(
        &mut self,
    ) -> Result<Rc<Component_inst_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_inst_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 32, RULE_component_inst_type);
        let mut _localctx: Rc<Component_inst_typeContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(305);
                cast_mut::<_, Component_inst_typeContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(_la == SystemRDL_EXTERNAL_kw || _la == SystemRDL_INTERNAL_kw) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Component_inst_typeContext>(&mut _localctx).kw =
                        Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_type ----------------
pub type Component_typeContextAll<'input> = Component_typeContext<'input>;

pub type Component_typeContext<'input> =
    BaseParserRuleContext<'input, Component_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Component_typeContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Component_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Component_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_type }
}
antlr4rust::tid! {Component_typeContextExt<'a>}

impl<'input> Component_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_typeContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Component_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_typeContextExt<'input>>
{
    fn component_type_primary(&self) -> Option<Rc<Component_type_primaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SIGNAL_kw
    /// Returns `None` if there is no child corresponding to token SIGNAL_kw
    fn SIGNAL_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_SIGNAL_kw, 0)
    }
}

impl<'input> Component_typeContextAttrs<'input> for Component_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_type(&mut self) -> Result<Rc<Component_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_component_type);
        let mut _localctx: Rc<Component_typeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(309);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_ADDRMAP_kw | SystemRDL_REGFILE_kw | SystemRDL_REG_kw
                | SystemRDL_FIELD_kw | SystemRDL_MEM_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule component_type_primary*/
                        recog.base.set_state(307);
                        recog.component_type_primary()?;
                    }
                }

                SystemRDL_SIGNAL_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(308);
                        let tmp = recog
                            .base
                            .match_token(SystemRDL_SIGNAL_kw, &mut recog.err_handler)?;
                        cast_mut::<_, Component_typeContext>(&mut _localctx).kw = Some(tmp.clone());
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- component_type_primary ----------------
pub type Component_type_primaryContextAll<'input> = Component_type_primaryContext<'input>;

pub type Component_type_primaryContext<'input> =
    BaseParserRuleContext<'input, Component_type_primaryContextExt<'input>>;

#[derive(Clone)]
pub struct Component_type_primaryContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Component_type_primaryContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Component_type_primaryContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_component_type_primary(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_component_type_primary(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Component_type_primaryContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_component_type_primary(self);
    }
}

impl<'input> CustomRuleContext<'input> for Component_type_primaryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_component_type_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_component_type_primary }
}
antlr4rust::tid! {Component_type_primaryContextExt<'a>}

impl<'input> Component_type_primaryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Component_type_primaryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Component_type_primaryContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Component_type_primaryContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Component_type_primaryContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ADDRMAP_kw
    /// Returns `None` if there is no child corresponding to token ADDRMAP_kw
    fn ADDRMAP_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ADDRMAP_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REGFILE_kw
    /// Returns `None` if there is no child corresponding to token REGFILE_kw
    fn REGFILE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_REGFILE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REG_kw
    /// Returns `None` if there is no child corresponding to token REG_kw
    fn REG_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_REG_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FIELD_kw
    /// Returns `None` if there is no child corresponding to token FIELD_kw
    fn FIELD_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_FIELD_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MEM_kw
    /// Returns `None` if there is no child corresponding to token MEM_kw
    fn MEM_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_MEM_kw, 0)
    }
}

impl<'input> Component_type_primaryContextAttrs<'input> for Component_type_primaryContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn component_type_primary(
        &mut self,
    ) -> Result<Rc<Component_type_primaryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Component_type_primaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_component_type_primary);
        let mut _localctx: Rc<Component_type_primaryContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(311);
                cast_mut::<_, Component_type_primaryContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 30) & !0x3f) == 0 && ((1usize << (_la - 30)) & 31) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Component_type_primaryContext>(&mut _localctx).kw =
                        Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- param_def ----------------
pub type Param_defContextAll<'input> = Param_defContext<'input>;

pub type Param_defContext<'input> = BaseParserRuleContext<'input, Param_defContextExt<'input>>;

#[derive(Clone)]
pub struct Param_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Param_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Param_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_param_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_param_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Param_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_param_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Param_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_param_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_param_def }
}
antlr4rust::tid! {Param_defContextExt<'a>}

impl<'input> Param_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Param_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Param_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Param_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Param_defContextExt<'input>>
{
    fn param_def_elem_all(&self) -> Vec<Rc<Param_def_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn param_def_elem(&self, i: usize) -> Option<Rc<Param_def_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Param_defContextAttrs<'input> for Param_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn param_def(&mut self) -> Result<Rc<Param_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Param_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_param_def);
        let mut _localctx: Rc<Param_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(313);
                recog
                    .base
                    .match_token(SystemRDL_T__4, &mut recog.err_handler)?;

                recog.base.set_state(314);
                recog
                    .base
                    .match_token(SystemRDL_T__5, &mut recog.err_handler)?;

                /*InvokeRule param_def_elem*/
                recog.base.set_state(315);
                recog.param_def_elem()?;

                recog.base.set_state(320);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_T__3 {
                    {
                        {
                            recog.base.set_state(316);
                            recog
                                .base
                                .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                            /*InvokeRule param_def_elem*/
                            recog.base.set_state(317);
                            recog.param_def_elem()?;
                        }
                    }
                    recog.base.set_state(322);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(323);
                recog
                    .base
                    .match_token(SystemRDL_T__6, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- param_def_elem ----------------
pub type Param_def_elemContextAll<'input> = Param_def_elemContext<'input>;

pub type Param_def_elemContext<'input> =
    BaseParserRuleContext<'input, Param_def_elemContextExt<'input>>;

#[derive(Clone)]
pub struct Param_def_elemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Param_def_elemContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Param_def_elemContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_param_def_elem(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_param_def_elem(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Param_def_elemContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_param_def_elem(self);
    }
}

impl<'input> CustomRuleContext<'input> for Param_def_elemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_param_def_elem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_param_def_elem }
}
antlr4rust::tid! {Param_def_elemContextExt<'a>}

impl<'input> Param_def_elemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Param_def_elemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Param_def_elemContextExt { ph: PhantomData },
        ))
    }
}

pub trait Param_def_elemContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Param_def_elemContextExt<'input>>
{
    fn data_type(&self) -> Option<Rc<Data_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn array_type_suffix(&self) -> Option<Rc<Array_type_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Param_def_elemContextAttrs<'input> for Param_def_elemContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn param_def_elem(&mut self) -> Result<Rc<Param_def_elemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Param_def_elemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_param_def_elem);
        let mut _localctx: Rc<Param_def_elemContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule data_type*/
                recog.base.set_state(325);
                recog.data_type()?;

                recog.base.set_state(326);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(328);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__11 {
                    {
                        /*InvokeRule array_type_suffix*/
                        recog.base.set_state(327);
                        recog.array_type_suffix()?;
                    }
                }

                recog.base.set_state(332);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ASSIGN {
                    {
                        recog.base.set_state(330);
                        recog
                            .base
                            .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(331);
                        recog.expr_rec(0)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- param_inst ----------------
pub type Param_instContextAll<'input> = Param_instContext<'input>;

pub type Param_instContext<'input> = BaseParserRuleContext<'input, Param_instContextExt<'input>>;

#[derive(Clone)]
pub struct Param_instContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Param_instContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Param_instContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_param_inst(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_param_inst(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Param_instContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_param_inst(self);
    }
}

impl<'input> CustomRuleContext<'input> for Param_instContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_param_inst
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_param_inst }
}
antlr4rust::tid! {Param_instContextExt<'a>}

impl<'input> Param_instContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Param_instContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Param_instContextExt { ph: PhantomData },
        ))
    }
}

pub trait Param_instContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Param_instContextExt<'input>>
{
    fn param_assignment_all(&self) -> Vec<Rc<Param_assignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn param_assignment(&self, i: usize) -> Option<Rc<Param_assignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Param_instContextAttrs<'input> for Param_instContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn param_inst(&mut self) -> Result<Rc<Param_instContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Param_instContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_param_inst);
        let mut _localctx: Rc<Param_instContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(334);
                recog
                    .base
                    .match_token(SystemRDL_T__4, &mut recog.err_handler)?;

                recog.base.set_state(335);
                recog
                    .base
                    .match_token(SystemRDL_T__5, &mut recog.err_handler)?;

                /*InvokeRule param_assignment*/
                recog.base.set_state(336);
                recog.param_assignment()?;

                recog.base.set_state(341);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_T__3 {
                    {
                        {
                            recog.base.set_state(337);
                            recog
                                .base
                                .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                            /*InvokeRule param_assignment*/
                            recog.base.set_state(338);
                            recog.param_assignment()?;
                        }
                    }
                    recog.base.set_state(343);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(344);
                recog
                    .base
                    .match_token(SystemRDL_T__6, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- param_assignment ----------------
pub type Param_assignmentContextAll<'input> = Param_assignmentContext<'input>;

pub type Param_assignmentContext<'input> =
    BaseParserRuleContext<'input, Param_assignmentContextExt<'input>>;

#[derive(Clone)]
pub struct Param_assignmentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Param_assignmentContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Param_assignmentContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_param_assignment(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_param_assignment(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Param_assignmentContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_param_assignment(self);
    }
}

impl<'input> CustomRuleContext<'input> for Param_assignmentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_param_assignment
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_param_assignment }
}
antlr4rust::tid! {Param_assignmentContextExt<'a>}

impl<'input> Param_assignmentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Param_assignmentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Param_assignmentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Param_assignmentContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Param_assignmentContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Param_assignmentContextAttrs<'input> for Param_assignmentContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn param_assignment(
        &mut self,
    ) -> Result<Rc<Param_assignmentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Param_assignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_param_assignment);
        let mut _localctx: Rc<Param_assignmentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(346);
                recog
                    .base
                    .match_token(SystemRDL_T__7, &mut recog.err_handler)?;

                recog.base.set_state(347);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(348);
                recog
                    .base
                    .match_token(SystemRDL_T__5, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(349);
                recog.expr_rec(0)?;

                recog.base.set_state(350);
                recog
                    .base
                    .match_token(SystemRDL_T__6, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input> {
    BinaryExprContext(BinaryExprContext<'input>),
    UnaryExprContext(UnaryExprContext<'input>),
    NOPContext(NOPContext<'input>),
    TernaryExprContext(TernaryExprContext<'input>),
    Error(ExprContext<'input>),
}
antlr4rust::tid! {ExprContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ExprContextAll<'input> {}

impl<'input> SystemRDLParserContext<'input> for ExprContextAll<'input> {}

impl<'input> Deref for ExprContextAll<'input> {
    type Target = dyn ExprContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ExprContextAll::*;
        match self {
            BinaryExprContext(inner) => inner,
            UnaryExprContext(inner) => inner,
            NOPContext(inner) => inner,
            TernaryExprContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for ExprContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for ExprContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        self.deref().exit(listener)
    }
}

pub type ExprContext<'input> = BaseParserRuleContext<'input, ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for ExprContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for ExprContext<'input> {}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for ExprContext<'input> {}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr4rust::tid! {ExprContextExt<'a>}

impl<'input> ExprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ExprContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ExprContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<ExprContextExt<'input>>
{
}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input> {}

pub type BinaryExprContext<'input> = BaseParserRuleContext<'input, BinaryExprContextExt<'input>>;

pub trait BinaryExprContextAttrs<'input>: SystemRDLParserContext<'input> {
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EXP
    /// Returns `None` if there is no child corresponding to token EXP
    fn EXP(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_EXP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MULT
    /// Returns `None` if there is no child corresponding to token MULT
    fn MULT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_MULT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_DIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MOD
    /// Returns `None` if there is no child corresponding to token MOD
    fn MOD(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_MOD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_MINUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LSHIFT
    /// Returns `None` if there is no child corresponding to token LSHIFT
    fn LSHIFT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LSHIFT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RSHIFT
    /// Returns `None` if there is no child corresponding to token RSHIFT
    fn RSHIFT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RSHIFT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEQ
    /// Returns `None` if there is no child corresponding to token LEQ
    fn LEQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LEQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_GT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GEQ
    /// Returns `None` if there is no child corresponding to token GEQ
    fn GEQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_GEQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NEQ
    /// Returns `None` if there is no child corresponding to token NEQ
    fn NEQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NEQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AND
    /// Returns `None` if there is no child corresponding to token AND
    fn AND(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_AND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XOR
    /// Returns `None` if there is no child corresponding to token XOR
    fn XOR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_XOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XNOR
    /// Returns `None` if there is no child corresponding to token XNOR
    fn XNOR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_XNOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OR
    /// Returns `None` if there is no child corresponding to token OR
    fn OR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BAND
    /// Returns `None` if there is no child corresponding to token BAND
    fn BAND(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BAND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BOR
    /// Returns `None` if there is no child corresponding to token BOR
    fn BOR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BOR, 0)
    }
}

impl<'input> BinaryExprContextAttrs<'input> for BinaryExprContext<'input> {}

pub struct BinaryExprContextExt<'input> {
    base: ExprContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {BinaryExprContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for BinaryExprContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for BinaryExprContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_BinaryExpr(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_BinaryExpr(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for BinaryExprContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_BinaryExpr(self);
    }
}

impl<'input> CustomRuleContext<'input> for BinaryExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for BinaryExprContext<'input> {
    fn borrow(&self) -> &ExprContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for BinaryExprContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExprContextAttrs<'input> for BinaryExprContext<'input> {}

impl<'input> BinaryExprContextExt<'input> {
    fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::BinaryExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BinaryExprContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type UnaryExprContext<'input> = BaseParserRuleContext<'input, UnaryExprContextExt<'input>>;

pub trait UnaryExprContextAttrs<'input>: SystemRDLParserContext<'input> {
    fn expr_primary(&self) -> Option<Rc<Expr_primaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_MINUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BNOT
    /// Returns `None` if there is no child corresponding to token BNOT
    fn BNOT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BNOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AND
    /// Returns `None` if there is no child corresponding to token AND
    fn AND(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_AND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NAND
    /// Returns `None` if there is no child corresponding to token NAND
    fn NAND(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NAND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OR
    /// Returns `None` if there is no child corresponding to token OR
    fn OR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOR
    /// Returns `None` if there is no child corresponding to token NOR
    fn NOR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XOR
    /// Returns `None` if there is no child corresponding to token XOR
    fn XOR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_XOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XNOR
    /// Returns `None` if there is no child corresponding to token XNOR
    fn XNOR(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_XNOR, 0)
    }
}

impl<'input> UnaryExprContextAttrs<'input> for UnaryExprContext<'input> {}

pub struct UnaryExprContextExt<'input> {
    base: ExprContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {UnaryExprContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for UnaryExprContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for UnaryExprContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_UnaryExpr(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_UnaryExpr(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for UnaryExprContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_UnaryExpr(self);
    }
}

impl<'input> CustomRuleContext<'input> for UnaryExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for UnaryExprContext<'input> {
    fn borrow(&self) -> &ExprContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for UnaryExprContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExprContextAttrs<'input> for UnaryExprContext<'input> {}

impl<'input> UnaryExprContextExt<'input> {
    fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::UnaryExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                UnaryExprContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NOPContext<'input> = BaseParserRuleContext<'input, NOPContextExt<'input>>;

pub trait NOPContextAttrs<'input>: SystemRDLParserContext<'input> {
    fn expr_primary(&self) -> Option<Rc<Expr_primaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> NOPContextAttrs<'input> for NOPContext<'input> {}

pub struct NOPContextExt<'input> {
    base: ExprContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {NOPContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for NOPContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for NOPContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_NOP(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_NOP(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for NOPContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_NOP(self);
    }
}

impl<'input> CustomRuleContext<'input> for NOPContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for NOPContext<'input> {
    fn borrow(&self) -> &ExprContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for NOPContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExprContextAttrs<'input> for NOPContext<'input> {}

impl<'input> NOPContextExt<'input> {
    fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::NOPContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NOPContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type TernaryExprContext<'input> = BaseParserRuleContext<'input, TernaryExprContextExt<'input>>;

pub trait TernaryExprContextAttrs<'input>: SystemRDLParserContext<'input> {
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> TernaryExprContextAttrs<'input> for TernaryExprContext<'input> {}

pub struct TernaryExprContextExt<'input> {
    base: ExprContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {TernaryExprContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for TernaryExprContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for TernaryExprContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_TernaryExpr(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_TernaryExpr(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for TernaryExprContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_TernaryExpr(self);
    }
}

impl<'input> CustomRuleContext<'input> for TernaryExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for TernaryExprContext<'input> {
    fn borrow(&self) -> &ExprContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for TernaryExprContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExprContextAttrs<'input> for TernaryExprContext<'input> {}

impl<'input> TernaryExprContextExt<'input> {
    fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::TernaryExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                TernaryExprContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn expr(&mut self) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        self.expr_rec(0)
    }

    fn expr_rec(&mut self, _p: i32) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 46, RULE_expr, _p);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 46;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: i32;
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(356);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    SystemRDL_PLUS | SystemRDL_MINUS | SystemRDL_BNOT | SystemRDL_NOT
                    | SystemRDL_NAND | SystemRDL_AND | SystemRDL_OR | SystemRDL_NOR
                    | SystemRDL_XOR | SystemRDL_XNOR => {
                        {
                            let mut tmp = UnaryExprContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            recog.base.set_state(353);
                            if let ExprContextAll::UnaryExprContext(ctx) =
                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                            {
                                ctx.op = recog.base.input.lt(1).cloned();
                            } else {
                                unreachable!("cant cast");
                            }
                            _la = recog.base.input.la(1);
                            if {
                                !(((_la - 96) & !0x3f) == 0 && ((1usize << (_la - 96)) & 3823) != 0)
                            } {
                                let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                                if let ExprContextAll::UnaryExprContext(ctx) =
                                    cast_mut::<_, ExprContextAll>(&mut _localctx)
                                {
                                    ctx.op = Some(tmp.clone());
                                } else {
                                    unreachable!("cant cast");
                                }
                            } else {
                                if recog.base.input.la(1) == TOKEN_EOF {
                                    recog.base.matched_eof = true
                                };
                                recog.err_handler.report_match(&mut recog.base);
                                recog.base.consume(&mut recog.err_handler);
                            }
                            /*InvokeRule expr_primary*/
                            recog.base.set_state(354);
                            recog.expr_primary()?;
                        }
                    }

                    SystemRDL_T__1
                    | SystemRDL_T__5
                    | SystemRDL_T__10
                    | SystemRDL_BOOLEAN_kw
                    | SystemRDL_BIT_kw
                    | SystemRDL_LONGINT_kw
                    | SystemRDL_TRUE_kw
                    | SystemRDL_FALSE_kw
                    | SystemRDL_NA_kw
                    | SystemRDL_RW_kw
                    | SystemRDL_WR_kw
                    | SystemRDL_R_kw
                    | SystemRDL_W_kw
                    | SystemRDL_RW1_kw
                    | SystemRDL_W1_kw
                    | SystemRDL_RCLR_kw
                    | SystemRDL_RSET_kw
                    | SystemRDL_RUSER_kw
                    | SystemRDL_WOSET_kw
                    | SystemRDL_WOCLR_kw
                    | SystemRDL_WOT_kw
                    | SystemRDL_WZS_kw
                    | SystemRDL_WZC_kw
                    | SystemRDL_WZT_kw
                    | SystemRDL_WCLR_kw
                    | SystemRDL_WSET_kw
                    | SystemRDL_WUSER_kw
                    | SystemRDL_COMPACT_kw
                    | SystemRDL_REGALIGN_kw
                    | SystemRDL_FULLALIGN_kw
                    | SystemRDL_HW_kw
                    | SystemRDL_SW_kw
                    | SystemRDL_INT
                    | SystemRDL_HEX_INT
                    | SystemRDL_VLOG_INT
                    | SystemRDL_STRING
                    | SystemRDL_ID => {
                        {
                            let mut tmp = NOPContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            /*InvokeRule expr_primary*/
                            recog.base.set_state(355);
                            recog.expr_primary()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(399);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(26, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event()?;
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(397);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(25, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(358);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 13)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 13)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(359);
                                        let tmp = recog
                                            .base
                                            .match_token(SystemRDL_EXP, &mut recog.err_handler)?;
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expr*/
                                        recog.base.set_state(360);
                                        recog.expr_rec(14)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(361);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 12)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 12)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(362);
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 110) & !0x3f) == 0
                                                && ((1usize << (_la - 110)) & 13) != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExprContextAll::BinaryExprContext(ctx) =
                                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(363);
                                        recog.expr_rec(13)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(364);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 11)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 11)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(365);
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if { !(_la == SystemRDL_PLUS || _la == SystemRDL_MINUS) } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExprContextAll::BinaryExprContext(ctx) =
                                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(366);
                                        recog.expr_rec(12)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(367);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 10)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 10)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(368);
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if { !(_la == SystemRDL_LSHIFT || _la == SystemRDL_RSHIFT) }
                                        {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExprContextAll::BinaryExprContext(ctx) =
                                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(369);
                                        recog.expr_rec(11)?;
                                    }
                                }
                                5 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(370);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 9)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 9)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(371);
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 117) & !0x3f) == 0
                                                && ((1usize << (_la - 117)) & 15) != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExprContextAll::BinaryExprContext(ctx) =
                                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(372);
                                        recog.expr_rec(10)?;
                                    }
                                }
                                6 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(373);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 8)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 8)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(374);
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if { !(_la == SystemRDL_EQ || _la == SystemRDL_NEQ) } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExprContextAll::BinaryExprContext(ctx) =
                                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(375);
                                        recog.expr_rec(9)?;
                                    }
                                }
                                7 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(376);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 7)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 7)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(377);
                                        let tmp = recog
                                            .base
                                            .match_token(SystemRDL_AND, &mut recog.err_handler)?;
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expr*/
                                        recog.base.set_state(378);
                                        recog.expr_rec(8)?;
                                    }
                                }
                                8 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(379);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 6)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 6)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(380);
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if { !(_la == SystemRDL_XOR || _la == SystemRDL_XNOR) } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExprContextAll::BinaryExprContext(ctx) =
                                                cast_mut::<_, ExprContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(381);
                                        recog.expr_rec(7)?;
                                    }
                                }
                                9 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(382);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 5)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 5)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(383);
                                        let tmp = recog
                                            .base
                                            .match_token(SystemRDL_OR, &mut recog.err_handler)?;
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expr*/
                                        recog.base.set_state(384);
                                        recog.expr_rec(6)?;
                                    }
                                }
                                10 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(385);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 4)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(386);
                                        let tmp = recog
                                            .base
                                            .match_token(SystemRDL_BAND, &mut recog.err_handler)?;
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expr*/
                                        recog.base.set_state(387);
                                        recog.expr_rec(5)?;
                                    }
                                }
                                11 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            BinaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(388);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 3)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(389);
                                        let tmp = recog
                                            .base
                                            .match_token(SystemRDL_BOR, &mut recog.err_handler)?;
                                        if let ExprContextAll::BinaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expr*/
                                        recog.base.set_state(390);
                                        recog.expr_rec(4)?;
                                    }
                                }
                                12 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            TernaryExprContextExt::new(&**ExprContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        )?;
                                        _localctx = tmp;
                                        recog.base.set_state(391);
                                        if !({
                                            let _localctx = Some(_localctx.clone());
                                            recog.precpred(None, 2)
                                        }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(392);
                                        let tmp = recog
                                            .base
                                            .match_token(SystemRDL_T__8, &mut recog.err_handler)?;
                                        if let ExprContextAll::TernaryExprContext(ctx) =
                                            cast_mut::<_, ExprContextAll>(&mut _localctx)
                                        {
                                            ctx.op = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expr*/
                                        recog.base.set_state(393);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(394);
                                        recog
                                            .base
                                            .match_token(SystemRDL_T__9, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(395);
                                        recog.expr_rec(2)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(401);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(26, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx)?;

        Ok(_localctx)
    }
}
//------------------- expr_primary ----------------
pub type Expr_primaryContextAll<'input> = Expr_primaryContext<'input>;

pub type Expr_primaryContext<'input> =
    BaseParserRuleContext<'input, Expr_primaryContextExt<'input>>;

#[derive(Clone)]
pub struct Expr_primaryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Expr_primaryContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Expr_primaryContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_expr_primary(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_expr_primary(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Expr_primaryContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_expr_primary(self);
    }
}

impl<'input> CustomRuleContext<'input> for Expr_primaryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr_primary }
}
antlr4rust::tid! {Expr_primaryContextExt<'a>}

impl<'input> Expr_primaryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Expr_primaryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_primaryContextExt { ph: PhantomData },
        ))
    }
}

pub trait Expr_primaryContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Expr_primaryContextExt<'input>>
{
    fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn concatenate(&self) -> Option<Rc<ConcatenateContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn replicate(&self) -> Option<Rc<ReplicateContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn paren_expr(&self) -> Option<Rc<Paren_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cast(&self) -> Option<Rc<CastContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn prop_ref(&self) -> Option<Rc<Prop_refContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn instance_ref(&self) -> Option<Rc<Instance_refContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn struct_literal(&self) -> Option<Rc<Struct_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn array_literal(&self) -> Option<Rc<Array_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Expr_primaryContextAttrs<'input> for Expr_primaryContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn expr_primary(&mut self) -> Result<Rc<Expr_primaryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Expr_primaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 48, RULE_expr_primary);
        let mut _localctx: Rc<Expr_primaryContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(411);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(27, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule literal*/
                        recog.base.set_state(402);
                        recog.literal()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule concatenate*/
                        recog.base.set_state(403);
                        recog.concatenate()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule replicate*/
                        recog.base.set_state(404);
                        recog.replicate()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule paren_expr*/
                        recog.base.set_state(405);
                        recog.paren_expr()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5)?;
                    recog.base.enter_outer_alt(None, 5)?;
                    {
                        /*InvokeRule cast*/
                        recog.base.set_state(406);
                        recog.cast()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6)?;
                    recog.base.enter_outer_alt(None, 6)?;
                    {
                        /*InvokeRule prop_ref*/
                        recog.base.set_state(407);
                        recog.prop_ref()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7)?;
                    recog.base.enter_outer_alt(None, 7)?;
                    {
                        /*InvokeRule instance_ref*/
                        recog.base.set_state(408);
                        recog.instance_ref()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8)?;
                    recog.base.enter_outer_alt(None, 8)?;
                    {
                        /*InvokeRule struct_literal*/
                        recog.base.set_state(409);
                        recog.struct_literal()?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9)?;
                    recog.base.enter_outer_alt(None, 9)?;
                    {
                        /*InvokeRule array_literal*/
                        recog.base.set_state(410);
                        recog.array_literal()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- concatenate ----------------
pub type ConcatenateContextAll<'input> = ConcatenateContext<'input>;

pub type ConcatenateContext<'input> = BaseParserRuleContext<'input, ConcatenateContextExt<'input>>;

#[derive(Clone)]
pub struct ConcatenateContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for ConcatenateContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for ConcatenateContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_concatenate(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_concatenate(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for ConcatenateContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_concatenate(self);
    }
}

impl<'input> CustomRuleContext<'input> for ConcatenateContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_concatenate
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_concatenate }
}
antlr4rust::tid! {ConcatenateContextExt<'a>}

impl<'input> ConcatenateContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<ConcatenateContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ConcatenateContextExt { ph: PhantomData },
        ))
    }
}

pub trait ConcatenateContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<ConcatenateContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ConcatenateContextAttrs<'input> for ConcatenateContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn concatenate(&mut self) -> Result<Rc<ConcatenateContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ConcatenateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 50, RULE_concatenate);
        let mut _localctx: Rc<ConcatenateContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(413);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(414);
                recog.expr_rec(0)?;

                recog.base.set_state(419);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_T__3 {
                    {
                        {
                            recog.base.set_state(415);
                            recog
                                .base
                                .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(416);
                            recog.expr_rec(0)?;
                        }
                    }
                    recog.base.set_state(421);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(422);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- replicate ----------------
pub type ReplicateContextAll<'input> = ReplicateContext<'input>;

pub type ReplicateContext<'input> = BaseParserRuleContext<'input, ReplicateContextExt<'input>>;

#[derive(Clone)]
pub struct ReplicateContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for ReplicateContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for ReplicateContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_replicate(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_replicate(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for ReplicateContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_replicate(self);
    }
}

impl<'input> CustomRuleContext<'input> for ReplicateContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_replicate
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_replicate }
}
antlr4rust::tid! {ReplicateContextExt<'a>}

impl<'input> ReplicateContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<ReplicateContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ReplicateContextExt { ph: PhantomData },
        ))
    }
}

pub trait ReplicateContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<ReplicateContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn concatenate(&self) -> Option<Rc<ConcatenateContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ReplicateContextAttrs<'input> for ReplicateContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn replicate(&mut self) -> Result<Rc<ReplicateContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ReplicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_replicate);
        let mut _localctx: Rc<ReplicateContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(424);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(425);
                recog.expr_rec(0)?;

                /*InvokeRule concatenate*/
                recog.base.set_state(426);
                recog.concatenate()?;

                recog.base.set_state(427);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- paren_expr ----------------
pub type Paren_exprContextAll<'input> = Paren_exprContext<'input>;

pub type Paren_exprContext<'input> = BaseParserRuleContext<'input, Paren_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Paren_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Paren_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Paren_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_paren_expr(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_paren_expr(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Paren_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_paren_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Paren_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_paren_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_paren_expr }
}
antlr4rust::tid! {Paren_exprContextExt<'a>}

impl<'input> Paren_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Paren_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Paren_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Paren_exprContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Paren_exprContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Paren_exprContextAttrs<'input> for Paren_exprContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn paren_expr(&mut self) -> Result<Rc<Paren_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Paren_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 54, RULE_paren_expr);
        let mut _localctx: Rc<Paren_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(429);
                recog
                    .base
                    .match_token(SystemRDL_T__5, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(430);
                recog.expr_rec(0)?;

                recog.base.set_state(431);
                recog
                    .base
                    .match_token(SystemRDL_T__6, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- cast ----------------
#[derive(Debug)]
pub enum CastContextAll<'input> {
    CastWidthContext(CastWidthContext<'input>),
    CastTypeContext(CastTypeContext<'input>),
    Error(CastContext<'input>),
}
antlr4rust::tid! {CastContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for CastContextAll<'input> {}

impl<'input> SystemRDLParserContext<'input> for CastContextAll<'input> {}

impl<'input> Deref for CastContextAll<'input> {
    type Target = dyn CastContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use CastContextAll::*;
        match self {
            CastWidthContext(inner) => inner,
            CastTypeContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for CastContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for CastContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        self.deref().exit(listener)
    }
}

pub type CastContext<'input> = BaseParserRuleContext<'input, CastContextExt<'input>>;

#[derive(Clone)]
pub struct CastContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for CastContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for CastContext<'input> {}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for CastContext<'input> {}

impl<'input> CustomRuleContext<'input> for CastContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cast
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cast }
}
antlr4rust::tid! {CastContextExt<'a>}

impl<'input> CastContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<CastContextAll<'input>> {
        Rc::new(CastContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                CastContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait CastContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<CastContextExt<'input>>
{
}

impl<'input> CastContextAttrs<'input> for CastContext<'input> {}

pub type CastWidthContext<'input> = BaseParserRuleContext<'input, CastWidthContextExt<'input>>;

pub trait CastWidthContextAttrs<'input>: SystemRDLParserContext<'input> {
    fn cast_width_expr(&self) -> Option<Rc<Cast_width_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CastWidthContextAttrs<'input> for CastWidthContext<'input> {}

pub struct CastWidthContextExt<'input> {
    base: CastContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {CastWidthContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for CastWidthContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for CastWidthContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_CastWidth(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_CastWidth(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for CastWidthContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_CastWidth(self);
    }
}

impl<'input> CustomRuleContext<'input> for CastWidthContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cast
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cast }
}

impl<'input> Borrow<CastContextExt<'input>> for CastWidthContext<'input> {
    fn borrow(&self) -> &CastContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<CastContextExt<'input>> for CastWidthContext<'input> {
    fn borrow_mut(&mut self) -> &mut CastContextExt<'input> {
        &mut self.base
    }
}

impl<'input> CastContextAttrs<'input> for CastWidthContext<'input> {}

impl<'input> CastWidthContextExt<'input> {
    fn new(ctx: &dyn CastContextAttrs<'input>) -> Rc<CastContextAll<'input>> {
        Rc::new(CastContextAll::CastWidthContext(
            BaseParserRuleContext::copy_from(
                ctx,
                CastWidthContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type CastTypeContext<'input> = BaseParserRuleContext<'input, CastTypeContextExt<'input>>;

pub trait CastTypeContextAttrs<'input>: SystemRDLParserContext<'input> {
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BOOLEAN_kw
    /// Returns `None` if there is no child corresponding to token BOOLEAN_kw
    fn BOOLEAN_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BOOLEAN_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BIT_kw
    /// Returns `None` if there is no child corresponding to token BIT_kw
    fn BIT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BIT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LONGINT_kw
    /// Returns `None` if there is no child corresponding to token LONGINT_kw
    fn LONGINT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LONGINT_kw, 0)
    }
}

impl<'input> CastTypeContextAttrs<'input> for CastTypeContext<'input> {}

pub struct CastTypeContextExt<'input> {
    base: CastContextExt<'input>,
    pub typ: Option<TokenType<'input>>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {CastTypeContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for CastTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for CastTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_CastType(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_CastType(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for CastTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_CastType(self);
    }
}

impl<'input> CustomRuleContext<'input> for CastTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cast
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cast }
}

impl<'input> Borrow<CastContextExt<'input>> for CastTypeContext<'input> {
    fn borrow(&self) -> &CastContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<CastContextExt<'input>> for CastTypeContext<'input> {
    fn borrow_mut(&mut self) -> &mut CastContextExt<'input> {
        &mut self.base
    }
}

impl<'input> CastContextAttrs<'input> for CastTypeContext<'input> {}

impl<'input> CastTypeContextExt<'input> {
    fn new(ctx: &dyn CastContextAttrs<'input>) -> Rc<CastContextAll<'input>> {
        Rc::new(CastContextAll::CastTypeContext(
            BaseParserRuleContext::copy_from(
                ctx,
                CastTypeContextExt {
                    typ: None,
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn cast(&mut self) -> Result<Rc<CastContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = CastContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_cast);
        let mut _localctx: Rc<CastContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(445);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_BOOLEAN_kw | SystemRDL_BIT_kw | SystemRDL_LONGINT_kw => {
                    let tmp = CastTypeContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
                    _localctx = tmp;
                    {
                        recog.base.set_state(433);
                        if let CastContextAll::CastTypeContext(ctx) =
                            cast_mut::<_, CastContextAll>(&mut _localctx)
                        {
                            ctx.typ = recog.base.input.lt(1).cloned();
                        } else {
                            unreachable!("cant cast");
                        }
                        _la = recog.base.input.la(1);
                        if { !(((_la) & !0x3f) == 0 && ((1usize << _la) & 1835008) != 0) } {
                            let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                            if let CastContextAll::CastTypeContext(ctx) =
                                cast_mut::<_, CastContextAll>(&mut _localctx)
                            {
                                ctx.typ = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        recog.base.set_state(434);
                        let tmp = recog
                            .base
                            .match_token(SystemRDL_T__10, &mut recog.err_handler)?;
                        if let CastContextAll::CastTypeContext(ctx) =
                            cast_mut::<_, CastContextAll>(&mut _localctx)
                        {
                            ctx.op = Some(tmp.clone());
                        } else {
                            unreachable!("cant cast");
                        }

                        recog.base.set_state(435);
                        recog
                            .base
                            .match_token(SystemRDL_T__5, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(436);
                        recog.expr_rec(0)?;

                        recog.base.set_state(437);
                        recog
                            .base
                            .match_token(SystemRDL_T__6, &mut recog.err_handler)?;
                    }
                }

                SystemRDL_T__5
                | SystemRDL_TRUE_kw
                | SystemRDL_FALSE_kw
                | SystemRDL_NA_kw
                | SystemRDL_RW_kw
                | SystemRDL_WR_kw
                | SystemRDL_R_kw
                | SystemRDL_W_kw
                | SystemRDL_RW1_kw
                | SystemRDL_W1_kw
                | SystemRDL_RCLR_kw
                | SystemRDL_RSET_kw
                | SystemRDL_RUSER_kw
                | SystemRDL_WOSET_kw
                | SystemRDL_WOCLR_kw
                | SystemRDL_WOT_kw
                | SystemRDL_WZS_kw
                | SystemRDL_WZC_kw
                | SystemRDL_WZT_kw
                | SystemRDL_WCLR_kw
                | SystemRDL_WSET_kw
                | SystemRDL_WUSER_kw
                | SystemRDL_COMPACT_kw
                | SystemRDL_REGALIGN_kw
                | SystemRDL_FULLALIGN_kw
                | SystemRDL_HW_kw
                | SystemRDL_SW_kw
                | SystemRDL_INT
                | SystemRDL_HEX_INT
                | SystemRDL_VLOG_INT
                | SystemRDL_STRING
                | SystemRDL_ID => {
                    let tmp = CastWidthContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
                    _localctx = tmp;
                    {
                        /*InvokeRule cast_width_expr*/
                        recog.base.set_state(439);
                        recog.cast_width_expr()?;

                        recog.base.set_state(440);
                        let tmp = recog
                            .base
                            .match_token(SystemRDL_T__10, &mut recog.err_handler)?;
                        if let CastContextAll::CastWidthContext(ctx) =
                            cast_mut::<_, CastContextAll>(&mut _localctx)
                        {
                            ctx.op = Some(tmp.clone());
                        } else {
                            unreachable!("cant cast");
                        }

                        recog.base.set_state(441);
                        recog
                            .base
                            .match_token(SystemRDL_T__5, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(442);
                        recog.expr_rec(0)?;

                        recog.base.set_state(443);
                        recog
                            .base
                            .match_token(SystemRDL_T__6, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- cast_width_expr ----------------
pub type Cast_width_exprContextAll<'input> = Cast_width_exprContext<'input>;

pub type Cast_width_exprContext<'input> =
    BaseParserRuleContext<'input, Cast_width_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Cast_width_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Cast_width_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Cast_width_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_cast_width_expr(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_cast_width_expr(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Cast_width_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_cast_width_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cast_width_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cast_width_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cast_width_expr }
}
antlr4rust::tid! {Cast_width_exprContextExt<'a>}

impl<'input> Cast_width_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Cast_width_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cast_width_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cast_width_exprContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Cast_width_exprContextExt<'input>>
{
    fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn paren_expr(&self) -> Option<Rc<Paren_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Cast_width_exprContextAttrs<'input> for Cast_width_exprContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn cast_width_expr(&mut self) -> Result<Rc<Cast_width_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cast_width_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 58, RULE_cast_width_expr);
        let mut _localctx: Rc<Cast_width_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(449);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_TRUE_kw
                | SystemRDL_FALSE_kw
                | SystemRDL_NA_kw
                | SystemRDL_RW_kw
                | SystemRDL_WR_kw
                | SystemRDL_R_kw
                | SystemRDL_W_kw
                | SystemRDL_RW1_kw
                | SystemRDL_W1_kw
                | SystemRDL_RCLR_kw
                | SystemRDL_RSET_kw
                | SystemRDL_RUSER_kw
                | SystemRDL_WOSET_kw
                | SystemRDL_WOCLR_kw
                | SystemRDL_WOT_kw
                | SystemRDL_WZS_kw
                | SystemRDL_WZC_kw
                | SystemRDL_WZT_kw
                | SystemRDL_WCLR_kw
                | SystemRDL_WSET_kw
                | SystemRDL_WUSER_kw
                | SystemRDL_COMPACT_kw
                | SystemRDL_REGALIGN_kw
                | SystemRDL_FULLALIGN_kw
                | SystemRDL_HW_kw
                | SystemRDL_SW_kw
                | SystemRDL_INT
                | SystemRDL_HEX_INT
                | SystemRDL_VLOG_INT
                | SystemRDL_STRING
                | SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule literal*/
                        recog.base.set_state(447);
                        recog.literal()?;
                    }
                }

                SystemRDL_T__5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule paren_expr*/
                        recog.base.set_state(448);
                        recog.paren_expr()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- range_suffix ----------------
pub type Range_suffixContextAll<'input> = Range_suffixContext<'input>;

pub type Range_suffixContext<'input> =
    BaseParserRuleContext<'input, Range_suffixContextExt<'input>>;

#[derive(Clone)]
pub struct Range_suffixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Range_suffixContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Range_suffixContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_range_suffix(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_range_suffix(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Range_suffixContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_range_suffix(self);
    }
}

impl<'input> CustomRuleContext<'input> for Range_suffixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_range_suffix
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_range_suffix }
}
antlr4rust::tid! {Range_suffixContextExt<'a>}

impl<'input> Range_suffixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Range_suffixContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Range_suffixContextExt { ph: PhantomData },
        ))
    }
}

pub trait Range_suffixContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Range_suffixContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Range_suffixContextAttrs<'input> for Range_suffixContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn range_suffix(&mut self) -> Result<Rc<Range_suffixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Range_suffixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 60, RULE_range_suffix);
        let mut _localctx: Rc<Range_suffixContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(451);
                recog
                    .base
                    .match_token(SystemRDL_T__11, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(452);
                recog.expr_rec(0)?;

                recog.base.set_state(453);
                recog
                    .base
                    .match_token(SystemRDL_T__9, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(454);
                recog.expr_rec(0)?;

                recog.base.set_state(455);
                recog
                    .base
                    .match_token(SystemRDL_T__12, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- array_suffix ----------------
pub type Array_suffixContextAll<'input> = Array_suffixContext<'input>;

pub type Array_suffixContext<'input> =
    BaseParserRuleContext<'input, Array_suffixContextExt<'input>>;

#[derive(Clone)]
pub struct Array_suffixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Array_suffixContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Array_suffixContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_array_suffix(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_array_suffix(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Array_suffixContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_array_suffix(self);
    }
}

impl<'input> CustomRuleContext<'input> for Array_suffixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_array_suffix
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_array_suffix }
}
antlr4rust::tid! {Array_suffixContextExt<'a>}

impl<'input> Array_suffixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Array_suffixContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Array_suffixContextExt { ph: PhantomData },
        ))
    }
}

pub trait Array_suffixContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Array_suffixContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Array_suffixContextAttrs<'input> for Array_suffixContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn array_suffix(&mut self) -> Result<Rc<Array_suffixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Array_suffixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 62, RULE_array_suffix);
        let mut _localctx: Rc<Array_suffixContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(457);
                recog
                    .base
                    .match_token(SystemRDL_T__11, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(458);
                recog.expr_rec(0)?;

                recog.base.set_state(459);
                recog
                    .base
                    .match_token(SystemRDL_T__12, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- array_type_suffix ----------------
pub type Array_type_suffixContextAll<'input> = Array_type_suffixContext<'input>;

pub type Array_type_suffixContext<'input> =
    BaseParserRuleContext<'input, Array_type_suffixContextExt<'input>>;

#[derive(Clone)]
pub struct Array_type_suffixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Array_type_suffixContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Array_type_suffixContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_array_type_suffix(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_array_type_suffix(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Array_type_suffixContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_array_type_suffix(self);
    }
}

impl<'input> CustomRuleContext<'input> for Array_type_suffixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_array_type_suffix
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_array_type_suffix }
}
antlr4rust::tid! {Array_type_suffixContextExt<'a>}

impl<'input> Array_type_suffixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Array_type_suffixContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Array_type_suffixContextExt { ph: PhantomData },
        ))
    }
}

pub trait Array_type_suffixContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Array_type_suffixContextExt<'input>>
{
}

impl<'input> Array_type_suffixContextAttrs<'input> for Array_type_suffixContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn array_type_suffix(
        &mut self,
    ) -> Result<Rc<Array_type_suffixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Array_type_suffixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_array_type_suffix);
        let mut _localctx: Rc<Array_type_suffixContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(461);
                recog
                    .base
                    .match_token(SystemRDL_T__11, &mut recog.err_handler)?;

                recog.base.set_state(462);
                recog
                    .base
                    .match_token(SystemRDL_T__12, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- data_type ----------------
pub type Data_typeContextAll<'input> = Data_typeContext<'input>;

pub type Data_typeContext<'input> = BaseParserRuleContext<'input, Data_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Data_typeContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Data_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Data_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_data_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_data_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Data_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_data_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Data_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_data_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_data_type }
}
antlr4rust::tid! {Data_typeContextExt<'a>}

impl<'input> Data_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Data_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Data_typeContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Data_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Data_typeContextExt<'input>>
{
    fn basic_data_type(&self) -> Option<Rc<Basic_data_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ACCESSTYPE_kw
    /// Returns `None` if there is no child corresponding to token ACCESSTYPE_kw
    fn ACCESSTYPE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ACCESSTYPE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ADDRESSINGTYPE_kw
    /// Returns `None` if there is no child corresponding to token ADDRESSINGTYPE_kw
    fn ADDRESSINGTYPE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ADDRESSINGTYPE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ONREADTYPE_kw
    /// Returns `None` if there is no child corresponding to token ONREADTYPE_kw
    fn ONREADTYPE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ONREADTYPE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ONWRITETYPE_kw
    /// Returns `None` if there is no child corresponding to token ONWRITETYPE_kw
    fn ONWRITETYPE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ONWRITETYPE_kw, 0)
    }
}

impl<'input> Data_typeContextAttrs<'input> for Data_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn data_type(&mut self) -> Result<Rc<Data_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Data_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_data_type);
        let mut _localctx: Rc<Data_typeContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(466);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_BOOLEAN_kw | SystemRDL_BIT_kw | SystemRDL_LONGINT_kw
                | SystemRDL_STRING_kw | SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule basic_data_type*/
                        recog.base.set_state(464);
                        recog.basic_data_type()?;
                    }
                }

                SystemRDL_ACCESSTYPE_kw
                | SystemRDL_ADDRESSINGTYPE_kw
                | SystemRDL_ONREADTYPE_kw
                | SystemRDL_ONWRITETYPE_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(465);
                        cast_mut::<_, Data_typeContext>(&mut _localctx).kw =
                            recog.base.input.lt(1).cloned();

                        _la = recog.base.input.la(1);
                        if { !(((_la) & !0x3f) == 0 && ((1usize << _la) & 125829120) != 0) } {
                            let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                            cast_mut::<_, Data_typeContext>(&mut _localctx).kw = Some(tmp.clone());
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- basic_data_type ----------------
pub type Basic_data_typeContextAll<'input> = Basic_data_typeContext<'input>;

pub type Basic_data_typeContext<'input> =
    BaseParserRuleContext<'input, Basic_data_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Basic_data_typeContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Basic_data_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Basic_data_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_basic_data_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_basic_data_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Basic_data_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_basic_data_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Basic_data_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_basic_data_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_basic_data_type }
}
antlr4rust::tid! {Basic_data_typeContextExt<'a>}

impl<'input> Basic_data_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Basic_data_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Basic_data_typeContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Basic_data_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Basic_data_typeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BIT_kw
    /// Returns `None` if there is no child corresponding to token BIT_kw
    fn BIT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BIT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LONGINT_kw
    /// Returns `None` if there is no child corresponding to token LONGINT_kw
    fn LONGINT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LONGINT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UNSIGNED_kw
    /// Returns `None` if there is no child corresponding to token UNSIGNED_kw
    fn UNSIGNED_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_UNSIGNED_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_kw
    /// Returns `None` if there is no child corresponding to token STRING_kw
    fn STRING_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_STRING_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BOOLEAN_kw
    /// Returns `None` if there is no child corresponding to token BOOLEAN_kw
    fn BOOLEAN_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BOOLEAN_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
}

impl<'input> Basic_data_typeContextAttrs<'input> for Basic_data_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn basic_data_type(&mut self) -> Result<Rc<Basic_data_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Basic_data_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 68, RULE_basic_data_type);
        let mut _localctx: Rc<Basic_data_typeContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(473);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_BIT_kw | SystemRDL_LONGINT_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        recog.base.set_state(468);
                        cast_mut::<_, Basic_data_typeContext>(&mut _localctx).kw =
                            recog.base.input.lt(1).cloned();

                        _la = recog.base.input.la(1);
                        if { !(_la == SystemRDL_BIT_kw || _la == SystemRDL_LONGINT_kw) } {
                            let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                            cast_mut::<_, Basic_data_typeContext>(&mut _localctx).kw =
                                Some(tmp.clone());
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        recog.base.set_state(470);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SystemRDL_UNSIGNED_kw {
                            {
                                recog.base.set_state(469);
                                recog
                                    .base
                                    .match_token(SystemRDL_UNSIGNED_kw, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                SystemRDL_BOOLEAN_kw | SystemRDL_STRING_kw | SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(472);
                        cast_mut::<_, Basic_data_typeContext>(&mut _localctx).kw =
                            recog.base.input.lt(1).cloned();

                        _la = recog.base.input.la(1);
                        if {
                            !(_la == SystemRDL_BOOLEAN_kw
                                || _la == SystemRDL_STRING_kw
                                || _la == SystemRDL_ID)
                        } {
                            let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                            cast_mut::<_, Basic_data_typeContext>(&mut _localctx).kw =
                                Some(tmp.clone());
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;

pub type LiteralContext<'input> = BaseParserRuleContext<'input, LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for LiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for LiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for LiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr4rust::tid! {LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<LiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait LiteralContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<LiteralContextExt<'input>>
{
    fn number(&self) -> Option<Rc<NumberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn string_literal(&self) -> Option<Rc<String_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn boolean_literal(&self) -> Option<Rc<Boolean_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn accesstype_literal(&self) -> Option<Rc<Accesstype_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn onreadtype_literal(&self) -> Option<Rc<Onreadtype_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn onwritetype_literal(&self) -> Option<Rc<Onwritetype_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn addressingtype_literal(&self) -> Option<Rc<Addressingtype_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn precedencetype_literal(&self) -> Option<Rc<Precedencetype_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn enum_literal(&self) -> Option<Rc<Enum_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn literal(&mut self) -> Result<Rc<LiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(484);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_INT | SystemRDL_HEX_INT | SystemRDL_VLOG_INT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule number*/
                        recog.base.set_state(475);
                        recog.number()?;
                    }
                }

                SystemRDL_STRING => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule string_literal*/
                        recog.base.set_state(476);
                        recog.string_literal()?;
                    }
                }

                SystemRDL_TRUE_kw | SystemRDL_FALSE_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule boolean_literal*/
                        recog.base.set_state(477);
                        recog.boolean_literal()?;
                    }
                }

                SystemRDL_NA_kw | SystemRDL_RW_kw | SystemRDL_WR_kw | SystemRDL_R_kw
                | SystemRDL_W_kw | SystemRDL_RW1_kw | SystemRDL_W1_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule accesstype_literal*/
                        recog.base.set_state(478);
                        recog.accesstype_literal()?;
                    }
                }

                SystemRDL_RCLR_kw | SystemRDL_RSET_kw | SystemRDL_RUSER_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5)?;
                    recog.base.enter_outer_alt(None, 5)?;
                    {
                        /*InvokeRule onreadtype_literal*/
                        recog.base.set_state(479);
                        recog.onreadtype_literal()?;
                    }
                }

                SystemRDL_WOSET_kw | SystemRDL_WOCLR_kw | SystemRDL_WOT_kw | SystemRDL_WZS_kw
                | SystemRDL_WZC_kw | SystemRDL_WZT_kw | SystemRDL_WCLR_kw | SystemRDL_WSET_kw
                | SystemRDL_WUSER_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6)?;
                    recog.base.enter_outer_alt(None, 6)?;
                    {
                        /*InvokeRule onwritetype_literal*/
                        recog.base.set_state(480);
                        recog.onwritetype_literal()?;
                    }
                }

                SystemRDL_COMPACT_kw | SystemRDL_REGALIGN_kw | SystemRDL_FULLALIGN_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7)?;
                    recog.base.enter_outer_alt(None, 7)?;
                    {
                        /*InvokeRule addressingtype_literal*/
                        recog.base.set_state(481);
                        recog.addressingtype_literal()?;
                    }
                }

                SystemRDL_HW_kw | SystemRDL_SW_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8)?;
                    recog.base.enter_outer_alt(None, 8)?;
                    {
                        /*InvokeRule precedencetype_literal*/
                        recog.base.set_state(482);
                        recog.precedencetype_literal()?;
                    }
                }

                SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9)?;
                    recog.base.enter_outer_alt(None, 9)?;
                    {
                        /*InvokeRule enum_literal*/
                        recog.base.set_state(483);
                        recog.enum_literal()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- number ----------------
#[derive(Debug)]
pub enum NumberContextAll<'input> {
    NumberHexContext(NumberHexContext<'input>),
    NumberVerilogContext(NumberVerilogContext<'input>),
    NumberIntContext(NumberIntContext<'input>),
    Error(NumberContext<'input>),
}
antlr4rust::tid! {NumberContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for NumberContextAll<'input> {}

impl<'input> SystemRDLParserContext<'input> for NumberContextAll<'input> {}

impl<'input> Deref for NumberContextAll<'input> {
    type Target = dyn NumberContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use NumberContextAll::*;
        match self {
            NumberHexContext(inner) => inner,
            NumberVerilogContext(inner) => inner,
            NumberIntContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for NumberContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for NumberContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        self.deref().exit(listener)
    }
}

pub type NumberContext<'input> = BaseParserRuleContext<'input, NumberContextExt<'input>>;

#[derive(Clone)]
pub struct NumberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for NumberContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for NumberContext<'input> {}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for NumberContext<'input> {}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr4rust::tid! {NumberContextExt<'a>}

impl<'input> NumberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<NumberContextAll<'input>> {
        Rc::new(NumberContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                NumberContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait NumberContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<NumberContextExt<'input>>
{
}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input> {}

pub type NumberHexContext<'input> = BaseParserRuleContext<'input, NumberHexContextExt<'input>>;

pub trait NumberHexContextAttrs<'input>: SystemRDLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token HEX_INT
    /// Returns `None` if there is no child corresponding to token HEX_INT
    fn HEX_INT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_HEX_INT, 0)
    }
}

impl<'input> NumberHexContextAttrs<'input> for NumberHexContext<'input> {}

pub struct NumberHexContextExt<'input> {
    base: NumberContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {NumberHexContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for NumberHexContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for NumberHexContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_NumberHex(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_NumberHex(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for NumberHexContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_NumberHex(self);
    }
}

impl<'input> CustomRuleContext<'input> for NumberHexContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_number }
}

impl<'input> Borrow<NumberContextExt<'input>> for NumberHexContext<'input> {
    fn borrow(&self) -> &NumberContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NumberContextExt<'input>> for NumberHexContext<'input> {
    fn borrow_mut(&mut self) -> &mut NumberContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NumberContextAttrs<'input> for NumberHexContext<'input> {}

impl<'input> NumberHexContextExt<'input> {
    fn new(ctx: &dyn NumberContextAttrs<'input>) -> Rc<NumberContextAll<'input>> {
        Rc::new(NumberContextAll::NumberHexContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NumberHexContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NumberVerilogContext<'input> =
    BaseParserRuleContext<'input, NumberVerilogContextExt<'input>>;

pub trait NumberVerilogContextAttrs<'input>: SystemRDLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token VLOG_INT
    /// Returns `None` if there is no child corresponding to token VLOG_INT
    fn VLOG_INT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_VLOG_INT, 0)
    }
}

impl<'input> NumberVerilogContextAttrs<'input> for NumberVerilogContext<'input> {}

pub struct NumberVerilogContextExt<'input> {
    base: NumberContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {NumberVerilogContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for NumberVerilogContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for NumberVerilogContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_NumberVerilog(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_NumberVerilog(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for NumberVerilogContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_NumberVerilog(self);
    }
}

impl<'input> CustomRuleContext<'input> for NumberVerilogContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_number }
}

impl<'input> Borrow<NumberContextExt<'input>> for NumberVerilogContext<'input> {
    fn borrow(&self) -> &NumberContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NumberContextExt<'input>> for NumberVerilogContext<'input> {
    fn borrow_mut(&mut self) -> &mut NumberContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NumberContextAttrs<'input> for NumberVerilogContext<'input> {}

impl<'input> NumberVerilogContextExt<'input> {
    fn new(ctx: &dyn NumberContextAttrs<'input>) -> Rc<NumberContextAll<'input>> {
        Rc::new(NumberContextAll::NumberVerilogContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NumberVerilogContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NumberIntContext<'input> = BaseParserRuleContext<'input, NumberIntContextExt<'input>>;

pub trait NumberIntContextAttrs<'input>: SystemRDLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_INT, 0)
    }
}

impl<'input> NumberIntContextAttrs<'input> for NumberIntContext<'input> {}

pub struct NumberIntContextExt<'input> {
    base: NumberContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr4rust::tid! {NumberIntContextExt<'a>}

impl<'input> SystemRDLParserContext<'input> for NumberIntContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for NumberIntContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_NumberInt(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_NumberInt(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for NumberIntContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_NumberInt(self);
    }
}

impl<'input> CustomRuleContext<'input> for NumberIntContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_number }
}

impl<'input> Borrow<NumberContextExt<'input>> for NumberIntContext<'input> {
    fn borrow(&self) -> &NumberContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NumberContextExt<'input>> for NumberIntContext<'input> {
    fn borrow_mut(&mut self) -> &mut NumberContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NumberContextAttrs<'input> for NumberIntContext<'input> {}

impl<'input> NumberIntContextExt<'input> {
    fn new(ctx: &dyn NumberContextAttrs<'input>) -> Rc<NumberContextAll<'input>> {
        Rc::new(NumberContextAll::NumberIntContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NumberIntContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn number(&mut self) -> Result<Rc<NumberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(489);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_INT => {
                    let tmp = NumberIntContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
                    _localctx = tmp;
                    {
                        recog.base.set_state(486);
                        recog
                            .base
                            .match_token(SystemRDL_INT, &mut recog.err_handler)?;
                    }
                }

                SystemRDL_HEX_INT => {
                    let tmp = NumberHexContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
                    _localctx = tmp;
                    {
                        recog.base.set_state(487);
                        recog
                            .base
                            .match_token(SystemRDL_HEX_INT, &mut recog.err_handler)?;
                    }
                }

                SystemRDL_VLOG_INT => {
                    let tmp = NumberVerilogContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
                    _localctx = tmp;
                    {
                        recog.base.set_state(488);
                        recog
                            .base
                            .match_token(SystemRDL_VLOG_INT, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- string_literal ----------------
pub type String_literalContextAll<'input> = String_literalContext<'input>;

pub type String_literalContext<'input> =
    BaseParserRuleContext<'input, String_literalContextExt<'input>>;

#[derive(Clone)]
pub struct String_literalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for String_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for String_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_string_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_string_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for String_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_string_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for String_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_string_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_string_literal }
}
antlr4rust::tid! {String_literalContextExt<'a>}

impl<'input> String_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<String_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            String_literalContextExt { ph: PhantomData },
        ))
    }
}

pub trait String_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<String_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_STRING, 0)
    }
}

impl<'input> String_literalContextAttrs<'input> for String_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn string_literal(&mut self) -> Result<Rc<String_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            String_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 74, RULE_string_literal);
        let mut _localctx: Rc<String_literalContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(491);
                recog
                    .base
                    .match_token(SystemRDL_STRING, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- boolean_literal ----------------
pub type Boolean_literalContextAll<'input> = Boolean_literalContext<'input>;

pub type Boolean_literalContext<'input> =
    BaseParserRuleContext<'input, Boolean_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Boolean_literalContextExt<'input> {
    pub val: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Boolean_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Boolean_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_boolean_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_boolean_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Boolean_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_boolean_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Boolean_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_boolean_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_boolean_literal }
}
antlr4rust::tid! {Boolean_literalContextExt<'a>}

impl<'input> Boolean_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Boolean_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Boolean_literalContextExt {
                val: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Boolean_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Boolean_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token TRUE_kw
    /// Returns `None` if there is no child corresponding to token TRUE_kw
    fn TRUE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_TRUE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FALSE_kw
    /// Returns `None` if there is no child corresponding to token FALSE_kw
    fn FALSE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_FALSE_kw, 0)
    }
}

impl<'input> Boolean_literalContextAttrs<'input> for Boolean_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn boolean_literal(&mut self) -> Result<Rc<Boolean_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Boolean_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 76, RULE_boolean_literal);
        let mut _localctx: Rc<Boolean_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(493);
                cast_mut::<_, Boolean_literalContext>(&mut _localctx).val =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(_la == SystemRDL_TRUE_kw || _la == SystemRDL_FALSE_kw) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Boolean_literalContext>(&mut _localctx).val = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- array_literal ----------------
pub type Array_literalContextAll<'input> = Array_literalContext<'input>;

pub type Array_literalContext<'input> =
    BaseParserRuleContext<'input, Array_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Array_literalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Array_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Array_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_array_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_array_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Array_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_array_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Array_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_array_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_array_literal }
}
antlr4rust::tid! {Array_literalContextExt<'a>}

impl<'input> Array_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Array_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Array_literalContextExt { ph: PhantomData },
        ))
    }
}

pub trait Array_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Array_literalContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Array_literalContextAttrs<'input> for Array_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn array_literal(&mut self) -> Result<Rc<Array_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Array_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 78, RULE_array_literal);
        let mut _localctx: Rc<Array_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(510);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(37, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        recog.base.set_state(495);
                        recog
                            .base
                            .match_token(SystemRDL_T__10, &mut recog.err_handler)?;

                        recog.base.set_state(496);
                        recog
                            .base
                            .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                        recog.base.set_state(497);
                        recog
                            .base
                            .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(498);
                        recog
                            .base
                            .match_token(SystemRDL_T__10, &mut recog.err_handler)?;

                        recog.base.set_state(499);
                        recog
                            .base
                            .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(500);
                        recog.expr_rec(0)?;

                        recog.base.set_state(505);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == SystemRDL_T__3 {
                            {
                                {
                                    recog.base.set_state(501);
                                    recog
                                        .base
                                        .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                                    /*InvokeRule expr*/
                                    recog.base.set_state(502);
                                    recog.expr_rec(0)?;
                                }
                            }
                            recog.base.set_state(507);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(508);
                        recog
                            .base
                            .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- struct_literal ----------------
pub type Struct_literalContextAll<'input> = Struct_literalContext<'input>;

pub type Struct_literalContext<'input> =
    BaseParserRuleContext<'input, Struct_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Struct_literalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Struct_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Struct_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_struct_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_struct_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Struct_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_struct_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Struct_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_struct_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_struct_literal }
}
antlr4rust::tid! {Struct_literalContextExt<'a>}

impl<'input> Struct_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Struct_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Struct_literalContextExt { ph: PhantomData },
        ))
    }
}

pub trait Struct_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Struct_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn struct_kv_all(&self) -> Vec<Rc<Struct_kvContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn struct_kv(&self, i: usize) -> Option<Rc<Struct_kvContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Struct_literalContextAttrs<'input> for Struct_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn struct_literal(&mut self) -> Result<Rc<Struct_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Struct_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 80, RULE_struct_literal);
        let mut _localctx: Rc<Struct_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(529);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(39, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        recog.base.set_state(512);
                        recog
                            .base
                            .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                        recog.base.set_state(513);
                        recog
                            .base
                            .match_token(SystemRDL_T__10, &mut recog.err_handler)?;

                        recog.base.set_state(514);
                        recog
                            .base
                            .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                        recog.base.set_state(515);
                        recog
                            .base
                            .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(516);
                        recog
                            .base
                            .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                        recog.base.set_state(517);
                        recog
                            .base
                            .match_token(SystemRDL_T__10, &mut recog.err_handler)?;

                        recog.base.set_state(518);
                        recog
                            .base
                            .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                        /*InvokeRule struct_kv*/
                        recog.base.set_state(519);
                        recog.struct_kv()?;

                        recog.base.set_state(524);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == SystemRDL_T__3 {
                            {
                                {
                                    recog.base.set_state(520);
                                    recog
                                        .base
                                        .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                                    /*InvokeRule struct_kv*/
                                    recog.base.set_state(521);
                                    recog.struct_kv()?;
                                }
                            }
                            recog.base.set_state(526);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(527);
                        recog
                            .base
                            .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- struct_kv ----------------
pub type Struct_kvContextAll<'input> = Struct_kvContext<'input>;

pub type Struct_kvContext<'input> = BaseParserRuleContext<'input, Struct_kvContextExt<'input>>;

#[derive(Clone)]
pub struct Struct_kvContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Struct_kvContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Struct_kvContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_struct_kv(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_struct_kv(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Struct_kvContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_struct_kv(self);
    }
}

impl<'input> CustomRuleContext<'input> for Struct_kvContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_struct_kv
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_struct_kv }
}
antlr4rust::tid! {Struct_kvContextExt<'a>}

impl<'input> Struct_kvContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Struct_kvContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Struct_kvContextExt { ph: PhantomData },
        ))
    }
}

pub trait Struct_kvContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Struct_kvContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Struct_kvContextAttrs<'input> for Struct_kvContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn struct_kv(&mut self) -> Result<Rc<Struct_kvContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Struct_kvContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_struct_kv);
        let mut _localctx: Rc<Struct_kvContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(531);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(532);
                recog
                    .base
                    .match_token(SystemRDL_T__9, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(533);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- enum_literal ----------------
pub type Enum_literalContextAll<'input> = Enum_literalContext<'input>;

pub type Enum_literalContext<'input> =
    BaseParserRuleContext<'input, Enum_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Enum_literalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Enum_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Enum_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_enum_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_enum_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Enum_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_enum_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Enum_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_enum_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_enum_literal }
}
antlr4rust::tid! {Enum_literalContextExt<'a>}

impl<'input> Enum_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Enum_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Enum_literalContextExt { ph: PhantomData },
        ))
    }
}

pub trait Enum_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Enum_literalContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
    fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, i)
    }
}

impl<'input> Enum_literalContextAttrs<'input> for Enum_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn enum_literal(&mut self) -> Result<Rc<Enum_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Enum_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 84, RULE_enum_literal);
        let mut _localctx: Rc<Enum_literalContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(535);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(536);
                recog
                    .base
                    .match_token(SystemRDL_T__13, &mut recog.err_handler)?;

                recog.base.set_state(537);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- accesstype_literal ----------------
pub type Accesstype_literalContextAll<'input> = Accesstype_literalContext<'input>;

pub type Accesstype_literalContext<'input> =
    BaseParserRuleContext<'input, Accesstype_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Accesstype_literalContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Accesstype_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Accesstype_literalContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_accesstype_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_accesstype_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Accesstype_literalContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_accesstype_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Accesstype_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_accesstype_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_accesstype_literal }
}
antlr4rust::tid! {Accesstype_literalContextExt<'a>}

impl<'input> Accesstype_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Accesstype_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Accesstype_literalContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Accesstype_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Accesstype_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NA_kw
    /// Returns `None` if there is no child corresponding to token NA_kw
    fn NA_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NA_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RW_kw
    /// Returns `None` if there is no child corresponding to token RW_kw
    fn RW_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RW_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WR_kw
    /// Returns `None` if there is no child corresponding to token WR_kw
    fn WR_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WR_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token R_kw
    /// Returns `None` if there is no child corresponding to token R_kw
    fn R_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_R_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token W_kw
    /// Returns `None` if there is no child corresponding to token W_kw
    fn W_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_W_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RW1_kw
    /// Returns `None` if there is no child corresponding to token RW1_kw
    fn RW1_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RW1_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token W1_kw
    /// Returns `None` if there is no child corresponding to token W1_kw
    fn W1_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_W1_kw, 0)
    }
}

impl<'input> Accesstype_literalContextAttrs<'input> for Accesstype_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn accesstype_literal(
        &mut self,
    ) -> Result<Rc<Accesstype_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Accesstype_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 86, RULE_accesstype_literal);
        let mut _localctx: Rc<Accesstype_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(539);
                cast_mut::<_, Accesstype_literalContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 38) & !0x3f) == 0 && ((1usize << (_la - 38)) & 127) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Accesstype_literalContext>(&mut _localctx).kw = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- onreadtype_literal ----------------
pub type Onreadtype_literalContextAll<'input> = Onreadtype_literalContext<'input>;

pub type Onreadtype_literalContext<'input> =
    BaseParserRuleContext<'input, Onreadtype_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Onreadtype_literalContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Onreadtype_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Onreadtype_literalContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_onreadtype_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_onreadtype_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Onreadtype_literalContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_onreadtype_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Onreadtype_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_onreadtype_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_onreadtype_literal }
}
antlr4rust::tid! {Onreadtype_literalContextExt<'a>}

impl<'input> Onreadtype_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Onreadtype_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Onreadtype_literalContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Onreadtype_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Onreadtype_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RCLR_kw
    /// Returns `None` if there is no child corresponding to token RCLR_kw
    fn RCLR_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RCLR_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RSET_kw
    /// Returns `None` if there is no child corresponding to token RSET_kw
    fn RSET_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RSET_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RUSER_kw
    /// Returns `None` if there is no child corresponding to token RUSER_kw
    fn RUSER_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RUSER_kw, 0)
    }
}

impl<'input> Onreadtype_literalContextAttrs<'input> for Onreadtype_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn onreadtype_literal(
        &mut self,
    ) -> Result<Rc<Onreadtype_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Onreadtype_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 88, RULE_onreadtype_literal);
        let mut _localctx: Rc<Onreadtype_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(541);
                cast_mut::<_, Onreadtype_literalContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 45) & !0x3f) == 0 && ((1usize << (_la - 45)) & 7) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Onreadtype_literalContext>(&mut _localctx).kw = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- onwritetype_literal ----------------
pub type Onwritetype_literalContextAll<'input> = Onwritetype_literalContext<'input>;

pub type Onwritetype_literalContext<'input> =
    BaseParserRuleContext<'input, Onwritetype_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Onwritetype_literalContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Onwritetype_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Onwritetype_literalContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_onwritetype_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_onwritetype_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Onwritetype_literalContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_onwritetype_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Onwritetype_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_onwritetype_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_onwritetype_literal }
}
antlr4rust::tid! {Onwritetype_literalContextExt<'a>}

impl<'input> Onwritetype_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Onwritetype_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Onwritetype_literalContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Onwritetype_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Onwritetype_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token WOSET_kw
    /// Returns `None` if there is no child corresponding to token WOSET_kw
    fn WOSET_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WOSET_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WOCLR_kw
    /// Returns `None` if there is no child corresponding to token WOCLR_kw
    fn WOCLR_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WOCLR_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WOT_kw
    /// Returns `None` if there is no child corresponding to token WOT_kw
    fn WOT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WOT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WZS_kw
    /// Returns `None` if there is no child corresponding to token WZS_kw
    fn WZS_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WZS_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WZC_kw
    /// Returns `None` if there is no child corresponding to token WZC_kw
    fn WZC_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WZC_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WZT_kw
    /// Returns `None` if there is no child corresponding to token WZT_kw
    fn WZT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WZT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WCLR_kw
    /// Returns `None` if there is no child corresponding to token WCLR_kw
    fn WCLR_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WCLR_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WSET_kw
    /// Returns `None` if there is no child corresponding to token WSET_kw
    fn WSET_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WSET_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WUSER_kw
    /// Returns `None` if there is no child corresponding to token WUSER_kw
    fn WUSER_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WUSER_kw, 0)
    }
}

impl<'input> Onwritetype_literalContextAttrs<'input> for Onwritetype_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn onwritetype_literal(
        &mut self,
    ) -> Result<Rc<Onwritetype_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Onwritetype_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 90, RULE_onwritetype_literal);
        let mut _localctx: Rc<Onwritetype_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(543);
                cast_mut::<_, Onwritetype_literalContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 48) & !0x3f) == 0 && ((1usize << (_la - 48)) & 511) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Onwritetype_literalContext>(&mut _localctx).kw =
                        Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- addressingtype_literal ----------------
pub type Addressingtype_literalContextAll<'input> = Addressingtype_literalContext<'input>;

pub type Addressingtype_literalContext<'input> =
    BaseParserRuleContext<'input, Addressingtype_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Addressingtype_literalContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Addressingtype_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Addressingtype_literalContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_addressingtype_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_addressingtype_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Addressingtype_literalContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_addressingtype_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Addressingtype_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_addressingtype_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_addressingtype_literal }
}
antlr4rust::tid! {Addressingtype_literalContextExt<'a>}

impl<'input> Addressingtype_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Addressingtype_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Addressingtype_literalContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Addressingtype_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Addressingtype_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMPACT_kw
    /// Returns `None` if there is no child corresponding to token COMPACT_kw
    fn COMPACT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_COMPACT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REGALIGN_kw
    /// Returns `None` if there is no child corresponding to token REGALIGN_kw
    fn REGALIGN_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_REGALIGN_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FULLALIGN_kw
    /// Returns `None` if there is no child corresponding to token FULLALIGN_kw
    fn FULLALIGN_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_FULLALIGN_kw, 0)
    }
}

impl<'input> Addressingtype_literalContextAttrs<'input> for Addressingtype_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn addressingtype_literal(
        &mut self,
    ) -> Result<Rc<Addressingtype_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Addressingtype_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 92, RULE_addressingtype_literal);
        let mut _localctx: Rc<Addressingtype_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(545);
                cast_mut::<_, Addressingtype_literalContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 57) & !0x3f) == 0 && ((1usize << (_la - 57)) & 7) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Addressingtype_literalContext>(&mut _localctx).kw =
                        Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- precedencetype_literal ----------------
pub type Precedencetype_literalContextAll<'input> = Precedencetype_literalContext<'input>;

pub type Precedencetype_literalContext<'input> =
    BaseParserRuleContext<'input, Precedencetype_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Precedencetype_literalContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Precedencetype_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Precedencetype_literalContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_precedencetype_literal(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_precedencetype_literal(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Precedencetype_literalContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_precedencetype_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Precedencetype_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_precedencetype_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_precedencetype_literal }
}
antlr4rust::tid! {Precedencetype_literalContextExt<'a>}

impl<'input> Precedencetype_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Precedencetype_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Precedencetype_literalContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Precedencetype_literalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Precedencetype_literalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token HW_kw
    /// Returns `None` if there is no child corresponding to token HW_kw
    fn HW_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_HW_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SW_kw
    /// Returns `None` if there is no child corresponding to token SW_kw
    fn SW_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_SW_kw, 0)
    }
}

impl<'input> Precedencetype_literalContextAttrs<'input> for Precedencetype_literalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn precedencetype_literal(
        &mut self,
    ) -> Result<Rc<Precedencetype_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Precedencetype_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 94, RULE_precedencetype_literal);
        let mut _localctx: Rc<Precedencetype_literalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(547);
                cast_mut::<_, Precedencetype_literalContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(_la == SystemRDL_HW_kw || _la == SystemRDL_SW_kw) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Precedencetype_literalContext>(&mut _localctx).kw =
                        Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- instance_ref ----------------
pub type Instance_refContextAll<'input> = Instance_refContext<'input>;

pub type Instance_refContext<'input> =
    BaseParserRuleContext<'input, Instance_refContextExt<'input>>;

#[derive(Clone)]
pub struct Instance_refContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Instance_refContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Instance_refContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_instance_ref(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_instance_ref(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Instance_refContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_instance_ref(self);
    }
}

impl<'input> CustomRuleContext<'input> for Instance_refContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_instance_ref
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_instance_ref }
}
antlr4rust::tid! {Instance_refContextExt<'a>}

impl<'input> Instance_refContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Instance_refContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Instance_refContextExt { ph: PhantomData },
        ))
    }
}

pub trait Instance_refContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Instance_refContextExt<'input>>
{
    fn instance_ref_element_all(&self) -> Vec<Rc<Instance_ref_elementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn instance_ref_element(&self, i: usize) -> Option<Rc<Instance_ref_elementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Instance_refContextAttrs<'input> for Instance_refContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn instance_ref(&mut self) -> Result<Rc<Instance_refContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Instance_refContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 96, RULE_instance_ref);
        let mut _localctx: Rc<Instance_refContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: i32;
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule instance_ref_element*/
                recog.base.set_state(549);
                recog.instance_ref_element()?;

                recog.base.set_state(554);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(40, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(550);
                                recog
                                    .base
                                    .match_token(SystemRDL_T__7, &mut recog.err_handler)?;

                                /*InvokeRule instance_ref_element*/
                                recog.base.set_state(551);
                                recog.instance_ref_element()?;
                            }
                        }
                    }
                    recog.base.set_state(556);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(40, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- instance_ref_element ----------------
pub type Instance_ref_elementContextAll<'input> = Instance_ref_elementContext<'input>;

pub type Instance_ref_elementContext<'input> =
    BaseParserRuleContext<'input, Instance_ref_elementContextExt<'input>>;

#[derive(Clone)]
pub struct Instance_ref_elementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Instance_ref_elementContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Instance_ref_elementContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_instance_ref_element(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_instance_ref_element(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Instance_ref_elementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_instance_ref_element(self);
    }
}

impl<'input> CustomRuleContext<'input> for Instance_ref_elementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_instance_ref_element
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_instance_ref_element }
}
antlr4rust::tid! {Instance_ref_elementContextExt<'a>}

impl<'input> Instance_ref_elementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Instance_ref_elementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Instance_ref_elementContextExt { ph: PhantomData },
        ))
    }
}

pub trait Instance_ref_elementContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Instance_ref_elementContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn array_suffix_all(&self) -> Vec<Rc<Array_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn array_suffix(&self, i: usize) -> Option<Rc<Array_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Instance_ref_elementContextAttrs<'input> for Instance_ref_elementContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn instance_ref_element(
        &mut self,
    ) -> Result<Rc<Instance_ref_elementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Instance_ref_elementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 98, RULE_instance_ref_element);
        let mut _localctx: Rc<Instance_ref_elementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: i32;
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(557);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(561);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(41, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule array_suffix*/
                                recog.base.set_state(558);
                                recog.array_suffix()?;
                            }
                        }
                    }
                    recog.base.set_state(563);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(41, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- prop_ref ----------------
pub type Prop_refContextAll<'input> = Prop_refContext<'input>;

pub type Prop_refContext<'input> = BaseParserRuleContext<'input, Prop_refContextExt<'input>>;

#[derive(Clone)]
pub struct Prop_refContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Prop_refContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Prop_refContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_prop_ref(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_prop_ref(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Prop_refContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_prop_ref(self);
    }
}

impl<'input> CustomRuleContext<'input> for Prop_refContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_prop_ref
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_prop_ref }
}
antlr4rust::tid! {Prop_refContextExt<'a>}

impl<'input> Prop_refContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Prop_refContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Prop_refContextExt { ph: PhantomData },
        ))
    }
}

pub trait Prop_refContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Prop_refContextExt<'input>>
{
    fn instance_ref(&self) -> Option<Rc<Instance_refContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn prop_keyword(&self) -> Option<Rc<Prop_keywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
}

impl<'input> Prop_refContextAttrs<'input> for Prop_refContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn prop_ref(&mut self) -> Result<Rc<Prop_refContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Prop_refContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_prop_ref);
        let mut _localctx: Rc<Prop_refContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule instance_ref*/
                recog.base.set_state(564);
                recog.instance_ref()?;

                recog.base.set_state(565);
                recog
                    .base
                    .match_token(SystemRDL_T__14, &mut recog.err_handler)?;

                recog.base.set_state(568);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    SystemRDL_RCLR_kw | SystemRDL_RSET_kw | SystemRDL_WOSET_kw
                    | SystemRDL_WOCLR_kw | SystemRDL_HW_kw | SystemRDL_SW_kw => {
                        {
                            /*InvokeRule prop_keyword*/
                            recog.base.set_state(566);
                            recog.prop_keyword()?;
                        }
                    }

                    SystemRDL_ID => {
                        recog.base.set_state(567);
                        recog
                            .base
                            .match_token(SystemRDL_ID, &mut recog.err_handler)?;
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- local_property_assignment ----------------
pub type Local_property_assignmentContextAll<'input> = Local_property_assignmentContext<'input>;

pub type Local_property_assignmentContext<'input> =
    BaseParserRuleContext<'input, Local_property_assignmentContextExt<'input>>;

#[derive(Clone)]
pub struct Local_property_assignmentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Local_property_assignmentContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Local_property_assignmentContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_local_property_assignment(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_local_property_assignment(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Local_property_assignmentContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_local_property_assignment(self);
    }
}

impl<'input> CustomRuleContext<'input> for Local_property_assignmentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_local_property_assignment
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_local_property_assignment }
}
antlr4rust::tid! {Local_property_assignmentContextExt<'a>}

impl<'input> Local_property_assignmentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Local_property_assignmentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Local_property_assignmentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Local_property_assignmentContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Local_property_assignmentContextExt<'input>>
{
    fn normal_prop_assign(&self) -> Option<Rc<Normal_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DEFAULT_kw
    /// Returns `None` if there is no child corresponding to token DEFAULT_kw
    fn DEFAULT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_DEFAULT_kw, 0)
    }
    fn encode_prop_assign(&self) -> Option<Rc<Encode_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn prop_mod_assign(&self) -> Option<Rc<Prop_mod_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Local_property_assignmentContextAttrs<'input>
    for Local_property_assignmentContext<'input>
{
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn local_property_assignment(
        &mut self,
    ) -> Result<Rc<Local_property_assignmentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Local_property_assignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 102, RULE_local_property_assignment);
        let mut _localctx: Rc<Local_property_assignmentContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(582);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(46, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        recog.base.set_state(571);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SystemRDL_DEFAULT_kw {
                            {
                                recog.base.set_state(570);
                                recog
                                    .base
                                    .match_token(SystemRDL_DEFAULT_kw, &mut recog.err_handler)?;
                            }
                        }

                        /*InvokeRule normal_prop_assign*/
                        recog.base.set_state(573);
                        recog.normal_prop_assign()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(575);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SystemRDL_DEFAULT_kw {
                            {
                                recog.base.set_state(574);
                                recog
                                    .base
                                    .match_token(SystemRDL_DEFAULT_kw, &mut recog.err_handler)?;
                            }
                        }

                        /*InvokeRule encode_prop_assign*/
                        recog.base.set_state(577);
                        recog.encode_prop_assign()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        recog.base.set_state(579);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SystemRDL_DEFAULT_kw {
                            {
                                recog.base.set_state(578);
                                recog
                                    .base
                                    .match_token(SystemRDL_DEFAULT_kw, &mut recog.err_handler)?;
                            }
                        }

                        /*InvokeRule prop_mod_assign*/
                        recog.base.set_state(581);
                        recog.prop_mod_assign()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- dynamic_property_assignment ----------------
pub type Dynamic_property_assignmentContextAll<'input> = Dynamic_property_assignmentContext<'input>;

pub type Dynamic_property_assignmentContext<'input> =
    BaseParserRuleContext<'input, Dynamic_property_assignmentContextExt<'input>>;

#[derive(Clone)]
pub struct Dynamic_property_assignmentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Dynamic_property_assignmentContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Dynamic_property_assignmentContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_dynamic_property_assignment(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_dynamic_property_assignment(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Dynamic_property_assignmentContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_dynamic_property_assignment(self);
    }
}

impl<'input> CustomRuleContext<'input> for Dynamic_property_assignmentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dynamic_property_assignment
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dynamic_property_assignment }
}
antlr4rust::tid! {Dynamic_property_assignmentContextExt<'a>}

impl<'input> Dynamic_property_assignmentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Dynamic_property_assignmentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Dynamic_property_assignmentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Dynamic_property_assignmentContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Dynamic_property_assignmentContextExt<'input>>
{
    fn instance_ref(&self) -> Option<Rc<Instance_refContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn normal_prop_assign(&self) -> Option<Rc<Normal_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn encode_prop_assign(&self) -> Option<Rc<Encode_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Dynamic_property_assignmentContextAttrs<'input>
    for Dynamic_property_assignmentContext<'input>
{
}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn dynamic_property_assignment(
        &mut self,
    ) -> Result<Rc<Dynamic_property_assignmentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Dynamic_property_assignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 104, RULE_dynamic_property_assignment);
        let mut _localctx: Rc<Dynamic_property_assignmentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(592);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(47, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule instance_ref*/
                        recog.base.set_state(584);
                        recog.instance_ref()?;

                        recog.base.set_state(585);
                        recog
                            .base
                            .match_token(SystemRDL_T__14, &mut recog.err_handler)?;

                        /*InvokeRule normal_prop_assign*/
                        recog.base.set_state(586);
                        recog.normal_prop_assign()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule instance_ref*/
                        recog.base.set_state(588);
                        recog.instance_ref()?;

                        recog.base.set_state(589);
                        recog
                            .base
                            .match_token(SystemRDL_T__14, &mut recog.err_handler)?;

                        /*InvokeRule encode_prop_assign*/
                        recog.base.set_state(590);
                        recog.encode_prop_assign()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- normal_prop_assign ----------------
pub type Normal_prop_assignContextAll<'input> = Normal_prop_assignContext<'input>;

pub type Normal_prop_assignContext<'input> =
    BaseParserRuleContext<'input, Normal_prop_assignContextExt<'input>>;

#[derive(Clone)]
pub struct Normal_prop_assignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Normal_prop_assignContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Normal_prop_assignContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_normal_prop_assign(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_normal_prop_assign(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Normal_prop_assignContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_normal_prop_assign(self);
    }
}

impl<'input> CustomRuleContext<'input> for Normal_prop_assignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_normal_prop_assign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_normal_prop_assign }
}
antlr4rust::tid! {Normal_prop_assignContextExt<'a>}

impl<'input> Normal_prop_assignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Normal_prop_assignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Normal_prop_assignContextExt { ph: PhantomData },
        ))
    }
}

pub trait Normal_prop_assignContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Normal_prop_assignContextExt<'input>>
{
    fn prop_keyword(&self) -> Option<Rc<Prop_keywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn prop_assignment_rhs(&self) -> Option<Rc<Prop_assignment_rhsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Normal_prop_assignContextAttrs<'input> for Normal_prop_assignContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn normal_prop_assign(
        &mut self,
    ) -> Result<Rc<Normal_prop_assignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Normal_prop_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 106, RULE_normal_prop_assign);
        let mut _localctx: Rc<Normal_prop_assignContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(596);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    SystemRDL_RCLR_kw | SystemRDL_RSET_kw | SystemRDL_WOSET_kw
                    | SystemRDL_WOCLR_kw | SystemRDL_HW_kw | SystemRDL_SW_kw => {
                        {
                            /*InvokeRule prop_keyword*/
                            recog.base.set_state(594);
                            recog.prop_keyword()?;
                        }
                    }

                    SystemRDL_ID => {
                        recog.base.set_state(595);
                        recog
                            .base
                            .match_token(SystemRDL_ID, &mut recog.err_handler)?;
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(600);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ASSIGN {
                    {
                        recog.base.set_state(598);
                        recog
                            .base
                            .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule prop_assignment_rhs*/
                        recog.base.set_state(599);
                        recog.prop_assignment_rhs()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- encode_prop_assign ----------------
pub type Encode_prop_assignContextAll<'input> = Encode_prop_assignContext<'input>;

pub type Encode_prop_assignContext<'input> =
    BaseParserRuleContext<'input, Encode_prop_assignContextExt<'input>>;

#[derive(Clone)]
pub struct Encode_prop_assignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Encode_prop_assignContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Encode_prop_assignContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_encode_prop_assign(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_encode_prop_assign(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Encode_prop_assignContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_encode_prop_assign(self);
    }
}

impl<'input> CustomRuleContext<'input> for Encode_prop_assignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_encode_prop_assign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_encode_prop_assign }
}
antlr4rust::tid! {Encode_prop_assignContextExt<'a>}

impl<'input> Encode_prop_assignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Encode_prop_assignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Encode_prop_assignContextExt { ph: PhantomData },
        ))
    }
}

pub trait Encode_prop_assignContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Encode_prop_assignContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ENCODE_kw
    /// Returns `None` if there is no child corresponding to token ENCODE_kw
    fn ENCODE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ENCODE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
}

impl<'input> Encode_prop_assignContextAttrs<'input> for Encode_prop_assignContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn encode_prop_assign(
        &mut self,
    ) -> Result<Rc<Encode_prop_assignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Encode_prop_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 108, RULE_encode_prop_assign);
        let mut _localctx: Rc<Encode_prop_assignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(602);
                recog
                    .base
                    .match_token(SystemRDL_ENCODE_kw, &mut recog.err_handler)?;

                recog.base.set_state(603);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                recog.base.set_state(604);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- prop_mod_assign ----------------
pub type Prop_mod_assignContextAll<'input> = Prop_mod_assignContext<'input>;

pub type Prop_mod_assignContext<'input> =
    BaseParserRuleContext<'input, Prop_mod_assignContextExt<'input>>;

#[derive(Clone)]
pub struct Prop_mod_assignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Prop_mod_assignContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Prop_mod_assignContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_prop_mod_assign(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_prop_mod_assign(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Prop_mod_assignContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_prop_mod_assign(self);
    }
}

impl<'input> CustomRuleContext<'input> for Prop_mod_assignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_prop_mod_assign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_prop_mod_assign }
}
antlr4rust::tid! {Prop_mod_assignContextExt<'a>}

impl<'input> Prop_mod_assignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Prop_mod_assignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Prop_mod_assignContextExt { ph: PhantomData },
        ))
    }
}

pub trait Prop_mod_assignContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Prop_mod_assignContextExt<'input>>
{
    fn prop_mod(&self) -> Option<Rc<Prop_modContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
}

impl<'input> Prop_mod_assignContextAttrs<'input> for Prop_mod_assignContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn prop_mod_assign(&mut self) -> Result<Rc<Prop_mod_assignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Prop_mod_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 110, RULE_prop_mod_assign);
        let mut _localctx: Rc<Prop_mod_assignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule prop_mod*/
                recog.base.set_state(606);
                recog.prop_mod()?;

                recog.base.set_state(607);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- prop_assignment_rhs ----------------
pub type Prop_assignment_rhsContextAll<'input> = Prop_assignment_rhsContext<'input>;

pub type Prop_assignment_rhsContext<'input> =
    BaseParserRuleContext<'input, Prop_assignment_rhsContextExt<'input>>;

#[derive(Clone)]
pub struct Prop_assignment_rhsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Prop_assignment_rhsContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Prop_assignment_rhsContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_prop_assignment_rhs(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_prop_assignment_rhs(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Prop_assignment_rhsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_prop_assignment_rhs(self);
    }
}

impl<'input> CustomRuleContext<'input> for Prop_assignment_rhsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_prop_assignment_rhs
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_prop_assignment_rhs }
}
antlr4rust::tid! {Prop_assignment_rhsContextExt<'a>}

impl<'input> Prop_assignment_rhsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Prop_assignment_rhsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Prop_assignment_rhsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Prop_assignment_rhsContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Prop_assignment_rhsContextExt<'input>>
{
    fn precedencetype_literal(&self) -> Option<Rc<Precedencetype_literalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Prop_assignment_rhsContextAttrs<'input> for Prop_assignment_rhsContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn prop_assignment_rhs(
        &mut self,
    ) -> Result<Rc<Prop_assignment_rhsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Prop_assignment_rhsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 112, RULE_prop_assignment_rhs);
        let mut _localctx: Rc<Prop_assignment_rhsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(611);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(50, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule precedencetype_literal*/
                        recog.base.set_state(609);
                        recog.precedencetype_literal()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule expr*/
                        recog.base.set_state(610);
                        recog.expr_rec(0)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- prop_keyword ----------------
pub type Prop_keywordContextAll<'input> = Prop_keywordContext<'input>;

pub type Prop_keywordContext<'input> =
    BaseParserRuleContext<'input, Prop_keywordContextExt<'input>>;

#[derive(Clone)]
pub struct Prop_keywordContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Prop_keywordContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Prop_keywordContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_prop_keyword(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_prop_keyword(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Prop_keywordContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_prop_keyword(self);
    }
}

impl<'input> CustomRuleContext<'input> for Prop_keywordContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_prop_keyword
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_prop_keyword }
}
antlr4rust::tid! {Prop_keywordContextExt<'a>}

impl<'input> Prop_keywordContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Prop_keywordContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Prop_keywordContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Prop_keywordContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Prop_keywordContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SW_kw
    /// Returns `None` if there is no child corresponding to token SW_kw
    fn SW_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_SW_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token HW_kw
    /// Returns `None` if there is no child corresponding to token HW_kw
    fn HW_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_HW_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RCLR_kw
    /// Returns `None` if there is no child corresponding to token RCLR_kw
    fn RCLR_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RCLR_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RSET_kw
    /// Returns `None` if there is no child corresponding to token RSET_kw
    fn RSET_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_RSET_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WOCLR_kw
    /// Returns `None` if there is no child corresponding to token WOCLR_kw
    fn WOCLR_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WOCLR_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WOSET_kw
    /// Returns `None` if there is no child corresponding to token WOSET_kw
    fn WOSET_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_WOSET_kw, 0)
    }
}

impl<'input> Prop_keywordContextAttrs<'input> for Prop_keywordContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn prop_keyword(&mut self) -> Result<Rc<Prop_keywordContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Prop_keywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 114, RULE_prop_keyword);
        let mut _localctx: Rc<Prop_keywordContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(613);
                cast_mut::<_, Prop_keywordContext>(&mut _localctx).kw =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 45) & !0x3f) == 0 && ((1usize << (_la - 45)) & 98331) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Prop_keywordContext>(&mut _localctx).kw = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- prop_mod ----------------
pub type Prop_modContextAll<'input> = Prop_modContext<'input>;

pub type Prop_modContext<'input> = BaseParserRuleContext<'input, Prop_modContextExt<'input>>;

#[derive(Clone)]
pub struct Prop_modContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Prop_modContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Prop_modContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_prop_mod(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_prop_mod(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Prop_modContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_prop_mod(self);
    }
}

impl<'input> CustomRuleContext<'input> for Prop_modContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_prop_mod
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_prop_mod }
}
antlr4rust::tid! {Prop_modContextExt<'a>}

impl<'input> Prop_modContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Prop_modContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Prop_modContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Prop_modContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Prop_modContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token POSEDGE_kw
    /// Returns `None` if there is no child corresponding to token POSEDGE_kw
    fn POSEDGE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_POSEDGE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NEGEDGE_kw
    /// Returns `None` if there is no child corresponding to token NEGEDGE_kw
    fn NEGEDGE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NEGEDGE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BOTHEDGE_kw
    /// Returns `None` if there is no child corresponding to token BOTHEDGE_kw
    fn BOTHEDGE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_BOTHEDGE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEVEL_kw
    /// Returns `None` if there is no child corresponding to token LEVEL_kw
    fn LEVEL_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LEVEL_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NONSTICKY_kw
    /// Returns `None` if there is no child corresponding to token NONSTICKY_kw
    fn NONSTICKY_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NONSTICKY_kw, 0)
    }
}

impl<'input> Prop_modContextAttrs<'input> for Prop_modContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn prop_mod(&mut self) -> Result<Rc<Prop_modContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Prop_modContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_prop_mod);
        let mut _localctx: Rc<Prop_modContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(615);
                cast_mut::<_, Prop_modContext>(&mut _localctx).kw = recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 62) & !0x3f) == 0 && ((1usize << (_la - 62)) & 31) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Prop_modContext>(&mut _localctx).kw = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_def ----------------
pub type Udp_defContextAll<'input> = Udp_defContext<'input>;

pub type Udp_defContext<'input> = BaseParserRuleContext<'input, Udp_defContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_def }
}
antlr4rust::tid! {Udp_defContextExt<'a>}

impl<'input> Udp_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Udp_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_defContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PROPERTY_kw
    /// Returns `None` if there is no child corresponding to token PROPERTY_kw
    fn PROPERTY_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_PROPERTY_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn udp_attr_all(&self) -> Vec<Rc<Udp_attrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn udp_attr(&self, i: usize) -> Option<Rc<Udp_attrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Udp_defContextAttrs<'input> for Udp_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_def(&mut self) -> Result<Rc<Udp_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Udp_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_udp_def);
        let mut _localctx: Rc<Udp_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(617);
                recog
                    .base
                    .match_token(SystemRDL_PROPERTY_kw, &mut recog.err_handler)?;

                recog.base.set_state(618);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(619);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                recog.base.set_state(623);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule udp_attr*/
                            recog.base.set_state(620);
                            recog.udp_attr()?;

                            recog.base.set_state(621);
                            recog
                                .base
                                .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(625);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la - 69) & !0x3f) == 0 && ((1usize << (_la - 69)) & 4109) != 0) {
                        break;
                    }
                }
                recog.base.set_state(627);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_attr ----------------
pub type Udp_attrContextAll<'input> = Udp_attrContext<'input>;

pub type Udp_attrContext<'input> = BaseParserRuleContext<'input, Udp_attrContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_attrContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_attrContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_attrContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_attr(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_attr(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_attrContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_attr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_attrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_attr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_attr }
}
antlr4rust::tid! {Udp_attrContextExt<'a>}

impl<'input> Udp_attrContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_attrContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_attrContextExt { ph: PhantomData },
        ))
    }
}

pub trait Udp_attrContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_attrContextExt<'input>>
{
    fn udp_type(&self) -> Option<Rc<Udp_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn udp_usage(&self) -> Option<Rc<Udp_usageContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn udp_default(&self) -> Option<Rc<Udp_defaultContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn udp_constraint(&self) -> Option<Rc<Udp_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Udp_attrContextAttrs<'input> for Udp_attrContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_attr(&mut self) -> Result<Rc<Udp_attrContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Udp_attrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_udp_attr);
        let mut _localctx: Rc<Udp_attrContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(633);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_TYPE_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule udp_type*/
                        recog.base.set_state(629);
                        recog.udp_type()?;
                    }
                }

                SystemRDL_COMPONENT_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule udp_usage*/
                        recog.base.set_state(630);
                        recog.udp_usage()?;
                    }
                }

                SystemRDL_DEFAULT_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule udp_default*/
                        recog.base.set_state(631);
                        recog.udp_default()?;
                    }
                }

                SystemRDL_CONSTRAINT_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule udp_constraint*/
                        recog.base.set_state(632);
                        recog.udp_constraint()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_type ----------------
pub type Udp_typeContextAll<'input> = Udp_typeContext<'input>;

pub type Udp_typeContext<'input> = BaseParserRuleContext<'input, Udp_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_typeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_type }
}
antlr4rust::tid! {Udp_typeContextExt<'a>}

impl<'input> Udp_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_typeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Udp_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_typeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token TYPE_kw
    /// Returns `None` if there is no child corresponding to token TYPE_kw
    fn TYPE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_TYPE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn udp_data_type(&self) -> Option<Rc<Udp_data_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn array_type_suffix(&self) -> Option<Rc<Array_type_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Udp_typeContextAttrs<'input> for Udp_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_type(&mut self) -> Result<Rc<Udp_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Udp_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_udp_type);
        let mut _localctx: Rc<Udp_typeContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(635);
                recog
                    .base
                    .match_token(SystemRDL_TYPE_kw, &mut recog.err_handler)?;

                recog.base.set_state(636);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule udp_data_type*/
                recog.base.set_state(637);
                recog.udp_data_type()?;

                recog.base.set_state(639);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__11 {
                    {
                        /*InvokeRule array_type_suffix*/
                        recog.base.set_state(638);
                        recog.array_type_suffix()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_data_type ----------------
pub type Udp_data_typeContextAll<'input> = Udp_data_typeContext<'input>;

pub type Udp_data_typeContext<'input> =
    BaseParserRuleContext<'input, Udp_data_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_data_typeContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_data_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_data_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_data_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_data_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_data_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_data_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_data_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_data_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_data_type }
}
antlr4rust::tid! {Udp_data_typeContextExt<'a>}

impl<'input> Udp_data_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_data_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_data_typeContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Udp_data_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_data_typeContextExt<'input>>
{
    fn component_type_primary(&self) -> Option<Rc<Component_type_primaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token REF_kw
    /// Returns `None` if there is no child corresponding to token REF_kw
    fn REF_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_REF_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NUMBER_kw
    /// Returns `None` if there is no child corresponding to token NUMBER_kw
    fn NUMBER_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NUMBER_kw, 0)
    }
    fn basic_data_type(&self) -> Option<Rc<Basic_data_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Udp_data_typeContextAttrs<'input> for Udp_data_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_data_type(&mut self) -> Result<Rc<Udp_data_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Udp_data_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 124, RULE_udp_data_type);
        let mut _localctx: Rc<Udp_data_typeContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(644);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_ADDRMAP_kw | SystemRDL_REGFILE_kw | SystemRDL_REG_kw
                | SystemRDL_FIELD_kw | SystemRDL_MEM_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule component_type_primary*/
                        recog.base.set_state(641);
                        recog.component_type_primary()?;
                    }
                }

                SystemRDL_NUMBER_kw | SystemRDL_REF_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(642);
                        cast_mut::<_, Udp_data_typeContext>(&mut _localctx).kw =
                            recog.base.input.lt(1).cloned();

                        _la = recog.base.input.la(1);
                        if { !(_la == SystemRDL_NUMBER_kw || _la == SystemRDL_REF_kw) } {
                            let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                            cast_mut::<_, Udp_data_typeContext>(&mut _localctx).kw =
                                Some(tmp.clone());
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                SystemRDL_BOOLEAN_kw | SystemRDL_BIT_kw | SystemRDL_LONGINT_kw
                | SystemRDL_STRING_kw | SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule basic_data_type*/
                        recog.base.set_state(643);
                        recog.basic_data_type()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_usage ----------------
pub type Udp_usageContextAll<'input> = Udp_usageContext<'input>;

pub type Udp_usageContext<'input> = BaseParserRuleContext<'input, Udp_usageContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_usageContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_usageContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_usageContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_usage(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_usage(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_usageContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_usage(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_usageContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_usage
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_usage }
}
antlr4rust::tid! {Udp_usageContextExt<'a>}

impl<'input> Udp_usageContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_usageContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_usageContextExt { ph: PhantomData },
        ))
    }
}

pub trait Udp_usageContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_usageContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMPONENT_kw
    /// Returns `None` if there is no child corresponding to token COMPONENT_kw
    fn COMPONENT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_COMPONENT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn udp_comp_type_all(&self) -> Vec<Rc<Udp_comp_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn udp_comp_type(&self, i: usize) -> Option<Rc<Udp_comp_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
    fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_OR, i)
    }
}

impl<'input> Udp_usageContextAttrs<'input> for Udp_usageContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_usage(&mut self) -> Result<Rc<Udp_usageContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Udp_usageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 126, RULE_udp_usage);
        let mut _localctx: Rc<Udp_usageContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(646);
                recog
                    .base
                    .match_token(SystemRDL_COMPONENT_kw, &mut recog.err_handler)?;

                recog.base.set_state(647);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule udp_comp_type*/
                recog.base.set_state(648);
                recog.udp_comp_type()?;

                recog.base.set_state(653);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_OR {
                    {
                        {
                            recog.base.set_state(649);
                            recog
                                .base
                                .match_token(SystemRDL_OR, &mut recog.err_handler)?;

                            /*InvokeRule udp_comp_type*/
                            recog.base.set_state(650);
                            recog.udp_comp_type()?;
                        }
                    }
                    recog.base.set_state(655);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_comp_type ----------------
pub type Udp_comp_typeContextAll<'input> = Udp_comp_typeContext<'input>;

pub type Udp_comp_typeContext<'input> =
    BaseParserRuleContext<'input, Udp_comp_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_comp_typeContextExt<'input> {
    pub kw: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_comp_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_comp_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_comp_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_comp_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_comp_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_comp_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_comp_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_comp_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_comp_type }
}
antlr4rust::tid! {Udp_comp_typeContextExt<'a>}

impl<'input> Udp_comp_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_comp_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_comp_typeContextExt {
                kw: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Udp_comp_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_comp_typeContextExt<'input>>
{
    fn component_type(&self) -> Option<Rc<Component_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CONSTRAINT_kw
    /// Returns `None` if there is no child corresponding to token CONSTRAINT_kw
    fn CONSTRAINT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_CONSTRAINT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ALL_kw
    /// Returns `None` if there is no child corresponding to token ALL_kw
    fn ALL_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ALL_kw, 0)
    }
}

impl<'input> Udp_comp_typeContextAttrs<'input> for Udp_comp_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_comp_type(&mut self) -> Result<Rc<Udp_comp_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Udp_comp_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 128, RULE_udp_comp_type);
        let mut _localctx: Rc<Udp_comp_typeContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(658);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_ADDRMAP_kw | SystemRDL_REGFILE_kw | SystemRDL_REG_kw
                | SystemRDL_FIELD_kw | SystemRDL_MEM_kw | SystemRDL_SIGNAL_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule component_type*/
                        recog.base.set_state(656);
                        recog.component_type()?;
                    }
                }

                SystemRDL_ALL_kw | SystemRDL_CONSTRAINT_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(657);
                        cast_mut::<_, Udp_comp_typeContext>(&mut _localctx).kw =
                            recog.base.input.lt(1).cloned();

                        _la = recog.base.input.la(1);
                        if { !(_la == SystemRDL_ALL_kw || _la == SystemRDL_CONSTRAINT_kw) } {
                            let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                            cast_mut::<_, Udp_comp_typeContext>(&mut _localctx).kw =
                                Some(tmp.clone());
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_default ----------------
pub type Udp_defaultContextAll<'input> = Udp_defaultContext<'input>;

pub type Udp_defaultContext<'input> = BaseParserRuleContext<'input, Udp_defaultContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_defaultContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_defaultContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_defaultContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_default(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_default(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_defaultContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_default(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_defaultContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_default
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_default }
}
antlr4rust::tid! {Udp_defaultContextExt<'a>}

impl<'input> Udp_defaultContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_defaultContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_defaultContextExt { ph: PhantomData },
        ))
    }
}

pub trait Udp_defaultContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_defaultContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEFAULT_kw
    /// Returns `None` if there is no child corresponding to token DEFAULT_kw
    fn DEFAULT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_DEFAULT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Udp_defaultContextAttrs<'input> for Udp_defaultContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_default(&mut self) -> Result<Rc<Udp_defaultContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Udp_defaultContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 130, RULE_udp_default);
        let mut _localctx: Rc<Udp_defaultContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(660);
                recog
                    .base
                    .match_token(SystemRDL_DEFAULT_kw, &mut recog.err_handler)?;

                recog.base.set_state(661);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(662);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- udp_constraint ----------------
pub type Udp_constraintContextAll<'input> = Udp_constraintContext<'input>;

pub type Udp_constraintContext<'input> =
    BaseParserRuleContext<'input, Udp_constraintContextExt<'input>>;

#[derive(Clone)]
pub struct Udp_constraintContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Udp_constraintContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Udp_constraintContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_udp_constraint(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_udp_constraint(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Udp_constraintContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_udp_constraint(self);
    }
}

impl<'input> CustomRuleContext<'input> for Udp_constraintContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_udp_constraint
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_udp_constraint }
}
antlr4rust::tid! {Udp_constraintContextExt<'a>}

impl<'input> Udp_constraintContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Udp_constraintContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Udp_constraintContextExt { ph: PhantomData },
        ))
    }
}

pub trait Udp_constraintContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Udp_constraintContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CONSTRAINT_kw
    /// Returns `None` if there is no child corresponding to token CONSTRAINT_kw
    fn CONSTRAINT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_CONSTRAINT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMPONENTWIDTH_kw
    /// Returns `None` if there is no child corresponding to token COMPONENTWIDTH_kw
    fn COMPONENTWIDTH_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_COMPONENTWIDTH_kw, 0)
    }
}

impl<'input> Udp_constraintContextAttrs<'input> for Udp_constraintContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn udp_constraint(&mut self) -> Result<Rc<Udp_constraintContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Udp_constraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 132, RULE_udp_constraint);
        let mut _localctx: Rc<Udp_constraintContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(664);
                recog
                    .base
                    .match_token(SystemRDL_CONSTRAINT_kw, &mut recog.err_handler)?;

                recog.base.set_state(665);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                recog.base.set_state(666);
                recog
                    .base
                    .match_token(SystemRDL_COMPONENTWIDTH_kw, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- enum_def ----------------
pub type Enum_defContextAll<'input> = Enum_defContext<'input>;

pub type Enum_defContext<'input> = BaseParserRuleContext<'input, Enum_defContextExt<'input>>;

#[derive(Clone)]
pub struct Enum_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Enum_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Enum_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_enum_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_enum_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Enum_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_enum_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Enum_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_enum_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_enum_def }
}
antlr4rust::tid! {Enum_defContextExt<'a>}

impl<'input> Enum_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Enum_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Enum_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Enum_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Enum_defContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ENUM_kw
    /// Returns `None` if there is no child corresponding to token ENUM_kw
    fn ENUM_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ENUM_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn enum_entry_all(&self) -> Vec<Rc<Enum_entryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn enum_entry(&self, i: usize) -> Option<Rc<Enum_entryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Enum_defContextAttrs<'input> for Enum_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn enum_def(&mut self) -> Result<Rc<Enum_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Enum_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_enum_def);
        let mut _localctx: Rc<Enum_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(668);
                recog
                    .base
                    .match_token(SystemRDL_ENUM_kw, &mut recog.err_handler)?;

                recog.base.set_state(669);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(670);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                recog.base.set_state(674);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule enum_entry*/
                            recog.base.set_state(671);
                            recog.enum_entry()?;

                            recog.base.set_state(672);
                            recog
                                .base
                                .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(676);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == SystemRDL_ID) {
                        break;
                    }
                }
                recog.base.set_state(678);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- enum_entry ----------------
pub type Enum_entryContextAll<'input> = Enum_entryContext<'input>;

pub type Enum_entryContext<'input> = BaseParserRuleContext<'input, Enum_entryContextExt<'input>>;

#[derive(Clone)]
pub struct Enum_entryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Enum_entryContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Enum_entryContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_enum_entry(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_enum_entry(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Enum_entryContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_enum_entry(self);
    }
}

impl<'input> CustomRuleContext<'input> for Enum_entryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_enum_entry
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_enum_entry }
}
antlr4rust::tid! {Enum_entryContextExt<'a>}

impl<'input> Enum_entryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Enum_entryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Enum_entryContextExt { ph: PhantomData },
        ))
    }
}

pub trait Enum_entryContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Enum_entryContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn enum_prop_assign_all(&self) -> Vec<Rc<Enum_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn enum_prop_assign(&self, i: usize) -> Option<Rc<Enum_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Enum_entryContextAttrs<'input> for Enum_entryContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn enum_entry(&mut self) -> Result<Rc<Enum_entryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Enum_entryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 136, RULE_enum_entry);
        let mut _localctx: Rc<Enum_entryContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(680);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(683);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ASSIGN {
                    {
                        recog.base.set_state(681);
                        recog
                            .base
                            .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(682);
                        recog.expr_rec(0)?;
                    }
                }

                recog.base.set_state(695);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__1 {
                    {
                        recog.base.set_state(685);
                        recog
                            .base
                            .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                        recog.base.set_state(691);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == SystemRDL_ID {
                            {
                                {
                                    /*InvokeRule enum_prop_assign*/
                                    recog.base.set_state(686);
                                    recog.enum_prop_assign()?;

                                    recog.base.set_state(687);
                                    recog
                                        .base
                                        .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(693);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(694);
                        recog
                            .base
                            .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- enum_prop_assign ----------------
pub type Enum_prop_assignContextAll<'input> = Enum_prop_assignContext<'input>;

pub type Enum_prop_assignContext<'input> =
    BaseParserRuleContext<'input, Enum_prop_assignContextExt<'input>>;

#[derive(Clone)]
pub struct Enum_prop_assignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Enum_prop_assignContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Enum_prop_assignContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_enum_prop_assign(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_enum_prop_assign(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Enum_prop_assignContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_enum_prop_assign(self);
    }
}

impl<'input> CustomRuleContext<'input> for Enum_prop_assignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_enum_prop_assign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_enum_prop_assign }
}
antlr4rust::tid! {Enum_prop_assignContextExt<'a>}

impl<'input> Enum_prop_assignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Enum_prop_assignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Enum_prop_assignContextExt { ph: PhantomData },
        ))
    }
}

pub trait Enum_prop_assignContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Enum_prop_assignContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Enum_prop_assignContextAttrs<'input> for Enum_prop_assignContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn enum_prop_assign(
        &mut self,
    ) -> Result<Rc<Enum_prop_assignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Enum_prop_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 138, RULE_enum_prop_assign);
        let mut _localctx: Rc<Enum_prop_assignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(697);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(698);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(699);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- struct_def ----------------
pub type Struct_defContextAll<'input> = Struct_defContext<'input>;

pub type Struct_defContext<'input> = BaseParserRuleContext<'input, Struct_defContextExt<'input>>;

#[derive(Clone)]
pub struct Struct_defContextExt<'input> {
    pub name: Option<TokenType<'input>>,
    pub base: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Struct_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Struct_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_struct_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_struct_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Struct_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_struct_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Struct_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_struct_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_struct_def }
}
antlr4rust::tid! {Struct_defContextExt<'a>}

impl<'input> Struct_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Struct_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Struct_defContextExt {
                name: None,
                base: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Struct_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Struct_defContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRUCT_kw
    /// Returns `None` if there is no child corresponding to token STRUCT_kw
    fn STRUCT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_STRUCT_kw, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
    fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, i)
    }
    /// Retrieves first TerminalNode corresponding to token ABSTRACT_kw
    /// Returns `None` if there is no child corresponding to token ABSTRACT_kw
    fn ABSTRACT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ABSTRACT_kw, 0)
    }
    fn struct_elem_all(&self) -> Vec<Rc<Struct_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn struct_elem(&self, i: usize) -> Option<Rc<Struct_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Struct_defContextAttrs<'input> for Struct_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn struct_def(&mut self) -> Result<Rc<Struct_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Struct_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 140, RULE_struct_def);
        let mut _localctx: Rc<Struct_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(702);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_ABSTRACT_kw {
                    {
                        recog.base.set_state(701);
                        recog
                            .base
                            .match_token(SystemRDL_ABSTRACT_kw, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(704);
                recog
                    .base
                    .match_token(SystemRDL_STRUCT_kw, &mut recog.err_handler)?;

                recog.base.set_state(705);
                let tmp = recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;
                cast_mut::<_, Struct_defContext>(&mut _localctx).name = Some(tmp.clone());

                recog.base.set_state(708);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__9 {
                    {
                        recog.base.set_state(706);
                        recog
                            .base
                            .match_token(SystemRDL_T__9, &mut recog.err_handler)?;

                        recog.base.set_state(707);
                        let tmp = recog
                            .base
                            .match_token(SystemRDL_ID, &mut recog.err_handler)?;
                        cast_mut::<_, Struct_defContext>(&mut _localctx).base = Some(tmp.clone());
                    }
                }

                recog.base.set_state(710);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                recog.base.set_state(716);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 18) & !0x3f) == 0 && ((1usize << (_la - 18)) & 258551) != 0)
                    || _la == SystemRDL_ID
                {
                    {
                        {
                            /*InvokeRule struct_elem*/
                            recog.base.set_state(711);
                            recog.struct_elem()?;

                            recog.base.set_state(712);
                            recog
                                .base
                                .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(718);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(719);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- struct_elem ----------------
pub type Struct_elemContextAll<'input> = Struct_elemContext<'input>;

pub type Struct_elemContext<'input> = BaseParserRuleContext<'input, Struct_elemContextExt<'input>>;

#[derive(Clone)]
pub struct Struct_elemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Struct_elemContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Struct_elemContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_struct_elem(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_struct_elem(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Struct_elemContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_struct_elem(self);
    }
}

impl<'input> CustomRuleContext<'input> for Struct_elemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_struct_elem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_struct_elem }
}
antlr4rust::tid! {Struct_elemContextExt<'a>}

impl<'input> Struct_elemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Struct_elemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Struct_elemContextExt { ph: PhantomData },
        ))
    }
}

pub trait Struct_elemContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Struct_elemContextExt<'input>>
{
    fn struct_type(&self) -> Option<Rc<Struct_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn array_type_suffix(&self) -> Option<Rc<Array_type_suffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Struct_elemContextAttrs<'input> for Struct_elemContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn struct_elem(&mut self) -> Result<Rc<Struct_elemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Struct_elemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 142, RULE_struct_elem);
        let mut _localctx: Rc<Struct_elemContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule struct_type*/
                recog.base.set_state(721);
                recog.struct_type()?;

                recog.base.set_state(722);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(724);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SystemRDL_T__11 {
                    {
                        /*InvokeRule array_type_suffix*/
                        recog.base.set_state(723);
                        recog.array_type_suffix()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- struct_type ----------------
pub type Struct_typeContextAll<'input> = Struct_typeContext<'input>;

pub type Struct_typeContext<'input> = BaseParserRuleContext<'input, Struct_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Struct_typeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Struct_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Struct_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_struct_type(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_struct_type(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Struct_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_struct_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Struct_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_struct_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_struct_type }
}
antlr4rust::tid! {Struct_typeContextExt<'a>}

impl<'input> Struct_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Struct_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Struct_typeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Struct_typeContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Struct_typeContextExt<'input>>
{
    fn data_type(&self) -> Option<Rc<Data_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn component_type(&self) -> Option<Rc<Component_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Struct_typeContextAttrs<'input> for Struct_typeContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn struct_type(&mut self) -> Result<Rc<Struct_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Struct_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 144, RULE_struct_type);
        let mut _localctx: Rc<Struct_typeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(728);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_BOOLEAN_kw
                | SystemRDL_BIT_kw
                | SystemRDL_LONGINT_kw
                | SystemRDL_STRING_kw
                | SystemRDL_ACCESSTYPE_kw
                | SystemRDL_ADDRESSINGTYPE_kw
                | SystemRDL_ONREADTYPE_kw
                | SystemRDL_ONWRITETYPE_kw
                | SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule data_type*/
                        recog.base.set_state(726);
                        recog.data_type()?;
                    }
                }

                SystemRDL_ADDRMAP_kw | SystemRDL_REGFILE_kw | SystemRDL_REG_kw
                | SystemRDL_FIELD_kw | SystemRDL_MEM_kw | SystemRDL_SIGNAL_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule component_type*/
                        recog.base.set_state(727);
                        recog.component_type()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constraint_def ----------------
pub type Constraint_defContextAll<'input> = Constraint_defContext<'input>;

pub type Constraint_defContext<'input> =
    BaseParserRuleContext<'input, Constraint_defContextExt<'input>>;

#[derive(Clone)]
pub struct Constraint_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constraint_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Constraint_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constraint_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constraint_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Constraint_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constraint_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constraint_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constraint_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constraint_def }
}
antlr4rust::tid! {Constraint_defContextExt<'a>}

impl<'input> Constraint_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constraint_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constraint_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constraint_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constraint_defContextExt<'input>>
{
    fn constraint_named_def(&self) -> Option<Rc<Constraint_named_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constraint_insts(&self) -> Option<Rc<Constraint_instsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constraint_anon_def(&self) -> Option<Rc<Constraint_anon_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Constraint_defContextAttrs<'input> for Constraint_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constraint_def(&mut self) -> Result<Rc<Constraint_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constraint_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 146, RULE_constraint_def);
        let mut _localctx: Rc<Constraint_defContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(737);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(67, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule constraint_named_def*/
                        recog.base.set_state(730);
                        recog.constraint_named_def()?;

                        recog.base.set_state(732);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SystemRDL_ID {
                            {
                                /*InvokeRule constraint_insts*/
                                recog.base.set_state(731);
                                recog.constraint_insts()?;
                            }
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule constraint_anon_def*/
                        recog.base.set_state(734);
                        recog.constraint_anon_def()?;

                        /*InvokeRule constraint_insts*/
                        recog.base.set_state(735);
                        recog.constraint_insts()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constraint_named_def ----------------
pub type Constraint_named_defContextAll<'input> = Constraint_named_defContext<'input>;

pub type Constraint_named_defContext<'input> =
    BaseParserRuleContext<'input, Constraint_named_defContextExt<'input>>;

#[derive(Clone)]
pub struct Constraint_named_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constraint_named_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constraint_named_defContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constraint_named_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constraint_named_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constraint_named_defContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constraint_named_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constraint_named_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constraint_named_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constraint_named_def }
}
antlr4rust::tid! {Constraint_named_defContextExt<'a>}

impl<'input> Constraint_named_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constraint_named_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constraint_named_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constraint_named_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constraint_named_defContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CONSTRAINT_kw
    /// Returns `None` if there is no child corresponding to token CONSTRAINT_kw
    fn CONSTRAINT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_CONSTRAINT_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    fn constraint_body(&self) -> Option<Rc<Constraint_bodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Constraint_named_defContextAttrs<'input> for Constraint_named_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constraint_named_def(
        &mut self,
    ) -> Result<Rc<Constraint_named_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constraint_named_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 148, RULE_constraint_named_def);
        let mut _localctx: Rc<Constraint_named_defContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(739);
                recog
                    .base
                    .match_token(SystemRDL_CONSTRAINT_kw, &mut recog.err_handler)?;

                recog.base.set_state(740);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                /*InvokeRule constraint_body*/
                recog.base.set_state(741);
                recog.constraint_body()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constraint_anon_def ----------------
pub type Constraint_anon_defContextAll<'input> = Constraint_anon_defContext<'input>;

pub type Constraint_anon_defContext<'input> =
    BaseParserRuleContext<'input, Constraint_anon_defContextExt<'input>>;

#[derive(Clone)]
pub struct Constraint_anon_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constraint_anon_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constraint_anon_defContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constraint_anon_def(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constraint_anon_def(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constraint_anon_defContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constraint_anon_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constraint_anon_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constraint_anon_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constraint_anon_def }
}
antlr4rust::tid! {Constraint_anon_defContextExt<'a>}

impl<'input> Constraint_anon_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constraint_anon_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constraint_anon_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constraint_anon_defContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constraint_anon_defContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CONSTRAINT_kw
    /// Returns `None` if there is no child corresponding to token CONSTRAINT_kw
    fn CONSTRAINT_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_CONSTRAINT_kw, 0)
    }
    fn constraint_body(&self) -> Option<Rc<Constraint_bodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Constraint_anon_defContextAttrs<'input> for Constraint_anon_defContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constraint_anon_def(
        &mut self,
    ) -> Result<Rc<Constraint_anon_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constraint_anon_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 150, RULE_constraint_anon_def);
        let mut _localctx: Rc<Constraint_anon_defContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(743);
                recog
                    .base
                    .match_token(SystemRDL_CONSTRAINT_kw, &mut recog.err_handler)?;

                /*InvokeRule constraint_body*/
                recog.base.set_state(744);
                recog.constraint_body()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constraint_body ----------------
pub type Constraint_bodyContextAll<'input> = Constraint_bodyContext<'input>;

pub type Constraint_bodyContext<'input> =
    BaseParserRuleContext<'input, Constraint_bodyContextExt<'input>>;

#[derive(Clone)]
pub struct Constraint_bodyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constraint_bodyContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Constraint_bodyContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constraint_body(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constraint_body(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Constraint_bodyContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constraint_body(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constraint_bodyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constraint_body
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constraint_body }
}
antlr4rust::tid! {Constraint_bodyContextExt<'a>}

impl<'input> Constraint_bodyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constraint_bodyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constraint_bodyContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constraint_bodyContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constraint_bodyContextExt<'input>>
{
    fn constraint_body_elem_all(&self) -> Vec<Rc<Constraint_body_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn constraint_body_elem(&self, i: usize) -> Option<Rc<Constraint_body_elemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Constraint_bodyContextAttrs<'input> for Constraint_bodyContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constraint_body(&mut self) -> Result<Rc<Constraint_bodyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constraint_bodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 152, RULE_constraint_body);
        let mut _localctx: Rc<Constraint_bodyContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(746);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                recog.base.set_state(752);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0 && ((1usize << _la) & 1837124) != 0)
                    || (((_la - 36) & !0x3f) == 0 && ((1usize << (_la - 36)) & 67108863) != 0)
                    || (((_la - 80) & !0x3f) == 0 && ((1usize << (_la - 80)) & 250605569) != 0)
                    || _la == SystemRDL_ID
                {
                    {
                        {
                            /*InvokeRule constraint_body_elem*/
                            recog.base.set_state(747);
                            recog.constraint_body_elem()?;

                            recog.base.set_state(748);
                            recog
                                .base
                                .match_token(SystemRDL_T__0, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(754);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(755);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constraint_body_elem ----------------
pub type Constraint_body_elemContextAll<'input> = Constraint_body_elemContext<'input>;

pub type Constraint_body_elemContext<'input> =
    BaseParserRuleContext<'input, Constraint_body_elemContextExt<'input>>;

#[derive(Clone)]
pub struct Constraint_body_elemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constraint_body_elemContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constraint_body_elemContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constraint_body_elem(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constraint_body_elem(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constraint_body_elemContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constraint_body_elem(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constraint_body_elemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constraint_body_elem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constraint_body_elem }
}
antlr4rust::tid! {Constraint_body_elemContextExt<'a>}

impl<'input> Constraint_body_elemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constraint_body_elemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constraint_body_elemContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constraint_body_elemContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constraint_body_elemContextExt<'input>>
{
    fn constr_relational(&self) -> Option<Rc<Constr_relationalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constr_prop_assign(&self) -> Option<Rc<Constr_prop_assignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constr_inside_values(&self) -> Option<Rc<Constr_inside_valuesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn constr_inside_enum(&self) -> Option<Rc<Constr_inside_enumContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Constraint_body_elemContextAttrs<'input> for Constraint_body_elemContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constraint_body_elem(
        &mut self,
    ) -> Result<Rc<Constraint_body_elemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constraint_body_elemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 154, RULE_constraint_body_elem);
        let mut _localctx: Rc<Constraint_body_elemContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(761);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(69, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule constr_relational*/
                        recog.base.set_state(757);
                        recog.constr_relational()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule constr_prop_assign*/
                        recog.base.set_state(758);
                        recog.constr_prop_assign()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3)?;
                    recog.base.enter_outer_alt(None, 3)?;
                    {
                        /*InvokeRule constr_inside_values*/
                        recog.base.set_state(759);
                        recog.constr_inside_values()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4)?;
                    recog.base.enter_outer_alt(None, 4)?;
                    {
                        /*InvokeRule constr_inside_enum*/
                        recog.base.set_state(760);
                        recog.constr_inside_enum()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constraint_insts ----------------
pub type Constraint_instsContextAll<'input> = Constraint_instsContext<'input>;

pub type Constraint_instsContext<'input> =
    BaseParserRuleContext<'input, Constraint_instsContextExt<'input>>;

#[derive(Clone)]
pub struct Constraint_instsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constraint_instsContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constraint_instsContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constraint_insts(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constraint_insts(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Constraint_instsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constraint_insts(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constraint_instsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constraint_insts
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constraint_insts }
}
antlr4rust::tid! {Constraint_instsContextExt<'a>}

impl<'input> Constraint_instsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constraint_instsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constraint_instsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constraint_instsContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constraint_instsContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
    fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, i)
    }
}

impl<'input> Constraint_instsContextAttrs<'input> for Constraint_instsContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constraint_insts(
        &mut self,
    ) -> Result<Rc<Constraint_instsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constraint_instsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 156, RULE_constraint_insts);
        let mut _localctx: Rc<Constraint_instsContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(763);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(768);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_T__3 {
                    {
                        {
                            recog.base.set_state(764);
                            recog
                                .base
                                .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                            recog.base.set_state(765);
                            recog
                                .base
                                .match_token(SystemRDL_ID, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(770);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constr_relational ----------------
pub type Constr_relationalContextAll<'input> = Constr_relationalContext<'input>;

pub type Constr_relationalContext<'input> =
    BaseParserRuleContext<'input, Constr_relationalContextExt<'input>>;

#[derive(Clone)]
pub struct Constr_relationalContextExt<'input> {
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constr_relationalContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constr_relationalContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constr_relational(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constr_relational(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Constr_relationalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constr_relational(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constr_relationalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constr_relational
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constr_relational }
}
antlr4rust::tid! {Constr_relationalContextExt<'a>}

impl<'input> Constr_relationalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constr_relationalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constr_relationalContextExt {
                op: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Constr_relationalContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constr_relationalContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEQ
    /// Returns `None` if there is no child corresponding to token LEQ
    fn LEQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_LEQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_GT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GEQ
    /// Returns `None` if there is no child corresponding to token GEQ
    fn GEQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_GEQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NEQ
    /// Returns `None` if there is no child corresponding to token NEQ
    fn NEQ(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_NEQ, 0)
    }
}

impl<'input> Constr_relationalContextAttrs<'input> for Constr_relationalContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constr_relational(
        &mut self,
    ) -> Result<Rc<Constr_relationalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constr_relationalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 158, RULE_constr_relational);
        let mut _localctx: Rc<Constr_relationalContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule expr*/
                recog.base.set_state(771);
                recog.expr_rec(0)?;

                recog.base.set_state(772);
                cast_mut::<_, Constr_relationalContext>(&mut _localctx).op =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if { !(((_la - 114) & !0x3f) == 0 && ((1usize << (_la - 114)) & 125) != 0) } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Constr_relationalContext>(&mut _localctx).op = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                /*InvokeRule expr*/
                recog.base.set_state(773);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constr_prop_assign ----------------
pub type Constr_prop_assignContextAll<'input> = Constr_prop_assignContext<'input>;

pub type Constr_prop_assignContext<'input> =
    BaseParserRuleContext<'input, Constr_prop_assignContextExt<'input>>;

#[derive(Clone)]
pub struct Constr_prop_assignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constr_prop_assignContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constr_prop_assignContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constr_prop_assign(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constr_prop_assign(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constr_prop_assignContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constr_prop_assign(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constr_prop_assignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constr_prop_assign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constr_prop_assign }
}
antlr4rust::tid! {Constr_prop_assignContextExt<'a>}

impl<'input> Constr_prop_assignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constr_prop_assignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constr_prop_assignContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constr_prop_assignContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constr_prop_assignContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ASSIGN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Constr_prop_assignContextAttrs<'input> for Constr_prop_assignContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constr_prop_assign(
        &mut self,
    ) -> Result<Rc<Constr_prop_assignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constr_prop_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 160, RULE_constr_prop_assign);
        let mut _localctx: Rc<Constr_prop_assignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                recog.base.set_state(775);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;

                recog.base.set_state(776);
                recog
                    .base
                    .match_token(SystemRDL_ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(777);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constr_inside_values ----------------
pub type Constr_inside_valuesContextAll<'input> = Constr_inside_valuesContext<'input>;

pub type Constr_inside_valuesContext<'input> =
    BaseParserRuleContext<'input, Constr_inside_valuesContextExt<'input>>;

#[derive(Clone)]
pub struct Constr_inside_valuesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constr_inside_valuesContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constr_inside_valuesContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constr_inside_values(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constr_inside_values(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constr_inside_valuesContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constr_inside_values(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constr_inside_valuesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constr_inside_values
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constr_inside_values }
}
antlr4rust::tid! {Constr_inside_valuesContextExt<'a>}

impl<'input> Constr_inside_valuesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constr_inside_valuesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constr_inside_valuesContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constr_inside_valuesContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constr_inside_valuesContextExt<'input>>
{
    fn constr_lhs(&self) -> Option<Rc<Constr_lhsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INSIDE_kw
    /// Returns `None` if there is no child corresponding to token INSIDE_kw
    fn INSIDE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_INSIDE_kw, 0)
    }
    fn constr_inside_value_all(&self) -> Vec<Rc<Constr_inside_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn constr_inside_value(&self, i: usize) -> Option<Rc<Constr_inside_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Constr_inside_valuesContextAttrs<'input> for Constr_inside_valuesContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constr_inside_values(
        &mut self,
    ) -> Result<Rc<Constr_inside_valuesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constr_inside_valuesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 162, RULE_constr_inside_values);
        let mut _localctx: Rc<Constr_inside_valuesContextAll> = _localctx;
        let mut _la: i32 = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule constr_lhs*/
                recog.base.set_state(779);
                recog.constr_lhs()?;

                recog.base.set_state(780);
                recog
                    .base
                    .match_token(SystemRDL_INSIDE_kw, &mut recog.err_handler)?;

                recog.base.set_state(781);
                recog
                    .base
                    .match_token(SystemRDL_T__1, &mut recog.err_handler)?;

                /*InvokeRule constr_inside_value*/
                recog.base.set_state(782);
                recog.constr_inside_value()?;

                recog.base.set_state(787);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SystemRDL_T__3 {
                    {
                        {
                            recog.base.set_state(783);
                            recog
                                .base
                                .match_token(SystemRDL_T__3, &mut recog.err_handler)?;

                            /*InvokeRule constr_inside_value*/
                            recog.base.set_state(784);
                            recog.constr_inside_value()?;
                        }
                    }
                    recog.base.set_state(789);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(790);
                recog
                    .base
                    .match_token(SystemRDL_T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constr_inside_enum ----------------
pub type Constr_inside_enumContextAll<'input> = Constr_inside_enumContext<'input>;

pub type Constr_inside_enumContext<'input> =
    BaseParserRuleContext<'input, Constr_inside_enumContextExt<'input>>;

#[derive(Clone)]
pub struct Constr_inside_enumContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constr_inside_enumContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constr_inside_enumContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constr_inside_enum(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constr_inside_enum(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constr_inside_enumContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constr_inside_enum(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constr_inside_enumContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constr_inside_enum
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constr_inside_enum }
}
antlr4rust::tid! {Constr_inside_enumContextExt<'a>}

impl<'input> Constr_inside_enumContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constr_inside_enumContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constr_inside_enumContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constr_inside_enumContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constr_inside_enumContextExt<'input>>
{
    fn constr_lhs(&self) -> Option<Rc<Constr_lhsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INSIDE_kw
    /// Returns `None` if there is no child corresponding to token INSIDE_kw
    fn INSIDE_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_INSIDE_kw, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_ID, 0)
    }
}

impl<'input> Constr_inside_enumContextAttrs<'input> for Constr_inside_enumContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constr_inside_enum(
        &mut self,
    ) -> Result<Rc<Constr_inside_enumContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constr_inside_enumContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 164, RULE_constr_inside_enum);
        let mut _localctx: Rc<Constr_inside_enumContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
            recog.base.enter_outer_alt(None, 1)?;
            {
                /*InvokeRule constr_lhs*/
                recog.base.set_state(792);
                recog.constr_lhs()?;

                recog.base.set_state(793);
                recog
                    .base
                    .match_token(SystemRDL_INSIDE_kw, &mut recog.err_handler)?;

                recog.base.set_state(794);
                recog
                    .base
                    .match_token(SystemRDL_ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constr_lhs ----------------
pub type Constr_lhsContextAll<'input> = Constr_lhsContext<'input>;

pub type Constr_lhsContext<'input> = BaseParserRuleContext<'input, Constr_lhsContextExt<'input>>;

#[derive(Clone)]
pub struct Constr_lhsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constr_lhsContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a> for Constr_lhsContext<'input> {
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constr_lhs(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constr_lhs(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a> for Constr_lhsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constr_lhs(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constr_lhsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constr_lhs
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constr_lhs }
}
antlr4rust::tid! {Constr_lhsContextExt<'a>}

impl<'input> Constr_lhsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constr_lhsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constr_lhsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constr_lhsContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constr_lhsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token THIS_kw
    /// Returns `None` if there is no child corresponding to token THIS_kw
    fn THIS_kw(&self) -> Option<Rc<TerminalNode<'input, SystemRDLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SystemRDL_THIS_kw, 0)
    }
    fn instance_ref(&self) -> Option<Rc<Instance_refContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Constr_lhsContextAttrs<'input> for Constr_lhsContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constr_lhs(&mut self) -> Result<Rc<Constr_lhsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Constr_lhsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 166, RULE_constr_lhs);
        let mut _localctx: Rc<Constr_lhsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(798);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_THIS_kw => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        recog.base.set_state(796);
                        recog
                            .base
                            .match_token(SystemRDL_THIS_kw, &mut recog.err_handler)?;
                    }
                }

                SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        /*InvokeRule instance_ref*/
                        recog.base.set_state(797);
                        recog.instance_ref()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
//------------------- constr_inside_value ----------------
pub type Constr_inside_valueContextAll<'input> = Constr_inside_valueContext<'input>;

pub type Constr_inside_valueContext<'input> =
    BaseParserRuleContext<'input, Constr_inside_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Constr_inside_valueContextExt<'input> {
    pub val: Option<Rc<ExprContextAll<'input>>>,
    pub l_val: Option<Rc<ExprContextAll<'input>>>,
    pub r_val: Option<Rc<ExprContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SystemRDLParserContext<'input> for Constr_inside_valueContext<'input> {}

impl<'input, 'a> Listenable<dyn SystemRDLListener<'input> + 'a>
    for Constr_inside_valueContext<'input>
{
    fn enter(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.enter_every_rule(self)?;
        listener.enter_constr_inside_value(self);
        Ok(())
    }
    fn exit(&self, listener: &mut (dyn SystemRDLListener<'input> + 'a)) -> Result<(), ANTLRError> {
        listener.exit_constr_inside_value(self);
        listener.exit_every_rule(self)?;
        Ok(())
    }
}

impl<'input, 'a> Visitable<dyn SystemRDLVisitor<'input> + 'a>
    for Constr_inside_valueContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SystemRDLVisitor<'input> + 'a)) {
        visitor.visit_constr_inside_value(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constr_inside_valueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SystemRDLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constr_inside_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constr_inside_value }
}
antlr4rust::tid! {Constr_inside_valueContextExt<'a>}

impl<'input> Constr_inside_valueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SystemRDLParserContext<'input> + 'input>>,
        invoking_state: i32,
    ) -> Rc<Constr_inside_valueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constr_inside_valueContextExt {
                val: None,
                l_val: None,
                r_val: None,

                ph: PhantomData,
            },
        ))
    }
}

pub trait Constr_inside_valueContextAttrs<'input>:
    SystemRDLParserContext<'input> + BorrowMut<Constr_inside_valueContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Constr_inside_valueContextAttrs<'input> for Constr_inside_valueContext<'input> {}

impl<'input, I> SystemRDLParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn constr_inside_value(
        &mut self,
    ) -> Result<Rc<Constr_inside_valueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constr_inside_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 168, RULE_constr_inside_value);
        let mut _localctx: Rc<Constr_inside_valueContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(807);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SystemRDL_T__1
                | SystemRDL_T__5
                | SystemRDL_T__10
                | SystemRDL_BOOLEAN_kw
                | SystemRDL_BIT_kw
                | SystemRDL_LONGINT_kw
                | SystemRDL_TRUE_kw
                | SystemRDL_FALSE_kw
                | SystemRDL_NA_kw
                | SystemRDL_RW_kw
                | SystemRDL_WR_kw
                | SystemRDL_R_kw
                | SystemRDL_W_kw
                | SystemRDL_RW1_kw
                | SystemRDL_W1_kw
                | SystemRDL_RCLR_kw
                | SystemRDL_RSET_kw
                | SystemRDL_RUSER_kw
                | SystemRDL_WOSET_kw
                | SystemRDL_WOCLR_kw
                | SystemRDL_WOT_kw
                | SystemRDL_WZS_kw
                | SystemRDL_WZC_kw
                | SystemRDL_WZT_kw
                | SystemRDL_WCLR_kw
                | SystemRDL_WSET_kw
                | SystemRDL_WUSER_kw
                | SystemRDL_COMPACT_kw
                | SystemRDL_REGALIGN_kw
                | SystemRDL_FULLALIGN_kw
                | SystemRDL_HW_kw
                | SystemRDL_SW_kw
                | SystemRDL_INT
                | SystemRDL_HEX_INT
                | SystemRDL_VLOG_INT
                | SystemRDL_STRING
                | SystemRDL_PLUS
                | SystemRDL_MINUS
                | SystemRDL_BNOT
                | SystemRDL_NOT
                | SystemRDL_NAND
                | SystemRDL_AND
                | SystemRDL_OR
                | SystemRDL_NOR
                | SystemRDL_XOR
                | SystemRDL_XNOR
                | SystemRDL_ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1)?;
                    recog.base.enter_outer_alt(None, 1)?;
                    {
                        /*InvokeRule expr*/
                        recog.base.set_state(800);
                        let tmp = recog.expr_rec(0)?;
                        cast_mut::<_, Constr_inside_valueContext>(&mut _localctx).val =
                            Some(tmp.clone());
                    }
                }

                SystemRDL_T__11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2)?;
                    recog.base.enter_outer_alt(None, 2)?;
                    {
                        recog.base.set_state(801);
                        recog
                            .base
                            .match_token(SystemRDL_T__11, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(802);
                        let tmp = recog.expr_rec(0)?;
                        cast_mut::<_, Constr_inside_valueContext>(&mut _localctx).l_val =
                            Some(tmp.clone());

                        recog.base.set_state(803);
                        recog
                            .base
                            .match_token(SystemRDL_T__9, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(804);
                        let tmp = recog.expr_rec(0)?;
                        cast_mut::<_, Constr_inside_valueContext>(&mut _localctx).r_val =
                            Some(tmp.clone());

                        recog.base.set_state(805);
                        recog
                            .base
                            .match_token(SystemRDL_T__12, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule()?;

        Ok(_localctx)
    }
}
lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i).into())
        }
        Arc::new(dfa)
    };
    static ref _serializedATN: Vec<i32> = vec![
        4, 1, 125, 810, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 4, 2, 5, 7, 5, 2,
        6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2,
        13, 7, 13, 2, 14, 7, 14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7,
        19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 7, 25, 2,
        26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7,
        32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2,
        39, 7, 39, 2, 40, 7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 7,
        45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 7, 50, 2, 51, 7, 51, 2,
        52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7,
        58, 2, 59, 7, 59, 2, 60, 7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2,
        65, 7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 7, 70, 2, 71, 7,
        71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2,
        78, 7, 78, 2, 79, 7, 79, 2, 80, 7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 84, 7,
        84, 1, 0, 1, 0, 1, 0, 5, 0, 174, 8, 0, 10, 0, 12, 0, 177, 9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1,
        1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 192, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3,
        1, 3, 3, 3, 199, 8, 3, 3, 3, 201, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 208, 8, 3, 1,
        3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 218, 8, 3, 1, 4, 3, 4, 221, 8, 4, 1, 4,
        3, 4, 224, 8, 4, 1, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 3, 6, 235, 8, 6, 1,
        6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 5, 8, 246, 8, 8, 10, 8, 12, 8, 249, 9,
        8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 3, 9, 260, 8, 9, 1, 10, 3, 10,
        263, 8, 10, 1, 10, 1, 10, 1, 10, 5, 10, 268, 8, 10, 10, 10, 12, 10, 271, 9, 10, 1, 11, 1,
        11, 4, 11, 275, 8, 11, 11, 11, 12, 11, 276, 1, 11, 3, 11, 280, 8, 11, 1, 11, 3, 11, 283, 8,
        11, 1, 11, 3, 11, 286, 8, 11, 1, 11, 3, 11, 289, 8, 11, 1, 11, 3, 11, 292, 8, 11, 1, 12, 1,
        12, 1, 12, 1, 13, 1, 13, 1, 13, 1, 14, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 16, 1, 16, 1,
        17, 1, 17, 3, 17, 310, 8, 17, 1, 18, 1, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 5, 19, 319,
        8, 19, 10, 19, 12, 19, 322, 9, 19, 1, 19, 1, 19, 1, 20, 1, 20, 1, 20, 3, 20, 329, 8, 20, 1,
        20, 1, 20, 3, 20, 333, 8, 20, 1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 5, 21, 340, 8, 21, 10, 21,
        12, 21, 343, 9, 21, 1, 21, 1, 21, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 23, 1, 23,
        1, 23, 1, 23, 3, 23, 357, 8, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1,
        23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1,
        23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1,
        23, 1, 23, 1, 23, 1, 23, 1, 23, 5, 23, 398, 8, 23, 10, 23, 12, 23, 401, 9, 23, 1, 24, 1,
        24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 3, 24, 412, 8, 24, 1, 25, 1, 25, 1,
        25, 1, 25, 5, 25, 418, 8, 25, 10, 25, 12, 25, 421, 9, 25, 1, 25, 1, 25, 1, 26, 1, 26, 1,
        26, 1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 27, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 1,
        28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 28, 3, 28, 446, 8, 28, 1, 29, 1, 29, 3, 29, 450, 8, 29,
        1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 30, 1, 31, 1, 31, 1, 31, 1, 31, 1, 32, 1, 32, 1, 32,
        1, 33, 1, 33, 3, 33, 467, 8, 33, 1, 34, 1, 34, 3, 34, 471, 8, 34, 1, 34, 3, 34, 474, 8, 34,
        1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 3, 35, 485, 8, 35, 1, 36, 1,
        36, 1, 36, 3, 36, 490, 8, 36, 1, 37, 1, 37, 1, 38, 1, 38, 1, 39, 1, 39, 1, 39, 1, 39, 1,
        39, 1, 39, 1, 39, 1, 39, 5, 39, 504, 8, 39, 10, 39, 12, 39, 507, 9, 39, 1, 39, 1, 39, 3,
        39, 511, 8, 39, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 5,
        40, 523, 8, 40, 10, 40, 12, 40, 526, 9, 40, 1, 40, 1, 40, 3, 40, 530, 8, 40, 1, 41, 1, 41,
        1, 41, 1, 41, 1, 42, 1, 42, 1, 42, 1, 42, 1, 43, 1, 43, 1, 44, 1, 44, 1, 45, 1, 45, 1, 46,
        1, 46, 1, 47, 1, 47, 1, 48, 1, 48, 1, 48, 5, 48, 553, 8, 48, 10, 48, 12, 48, 556, 9, 48, 1,
        49, 1, 49, 5, 49, 560, 8, 49, 10, 49, 12, 49, 563, 9, 49, 1, 50, 1, 50, 1, 50, 1, 50, 3,
        50, 569, 8, 50, 1, 51, 3, 51, 572, 8, 51, 1, 51, 1, 51, 3, 51, 576, 8, 51, 1, 51, 1, 51, 3,
        51, 580, 8, 51, 1, 51, 3, 51, 583, 8, 51, 1, 52, 1, 52, 1, 52, 1, 52, 1, 52, 1, 52, 1, 52,
        1, 52, 3, 52, 593, 8, 52, 1, 53, 1, 53, 3, 53, 597, 8, 53, 1, 53, 1, 53, 3, 53, 601, 8, 53,
        1, 54, 1, 54, 1, 54, 1, 54, 1, 55, 1, 55, 1, 55, 1, 56, 1, 56, 3, 56, 612, 8, 56, 1, 57, 1,
        57, 1, 58, 1, 58, 1, 59, 1, 59, 1, 59, 1, 59, 1, 59, 1, 59, 4, 59, 624, 8, 59, 11, 59, 12,
        59, 625, 1, 59, 1, 59, 1, 60, 1, 60, 1, 60, 1, 60, 3, 60, 634, 8, 60, 1, 61, 1, 61, 1, 61,
        1, 61, 3, 61, 640, 8, 61, 1, 62, 1, 62, 1, 62, 3, 62, 645, 8, 62, 1, 63, 1, 63, 1, 63, 1,
        63, 1, 63, 5, 63, 652, 8, 63, 10, 63, 12, 63, 655, 9, 63, 1, 64, 1, 64, 3, 64, 659, 8, 64,
        1, 65, 1, 65, 1, 65, 1, 65, 1, 66, 1, 66, 1, 66, 1, 66, 1, 67, 1, 67, 1, 67, 1, 67, 1, 67,
        1, 67, 4, 67, 675, 8, 67, 11, 67, 12, 67, 676, 1, 67, 1, 67, 1, 68, 1, 68, 1, 68, 3, 68,
        684, 8, 68, 1, 68, 1, 68, 1, 68, 1, 68, 5, 68, 690, 8, 68, 10, 68, 12, 68, 693, 9, 68, 1,
        68, 3, 68, 696, 8, 68, 1, 69, 1, 69, 1, 69, 1, 69, 1, 70, 3, 70, 703, 8, 70, 1, 70, 1, 70,
        1, 70, 1, 70, 3, 70, 709, 8, 70, 1, 70, 1, 70, 1, 70, 1, 70, 5, 70, 715, 8, 70, 10, 70, 12,
        70, 718, 9, 70, 1, 70, 1, 70, 1, 71, 1, 71, 1, 71, 3, 71, 725, 8, 71, 1, 72, 1, 72, 3, 72,
        729, 8, 72, 1, 73, 1, 73, 3, 73, 733, 8, 73, 1, 73, 1, 73, 1, 73, 3, 73, 738, 8, 73, 1, 74,
        1, 74, 1, 74, 1, 74, 1, 75, 1, 75, 1, 75, 1, 76, 1, 76, 1, 76, 1, 76, 5, 76, 751, 8, 76,
        10, 76, 12, 76, 754, 9, 76, 1, 76, 1, 76, 1, 77, 1, 77, 1, 77, 1, 77, 3, 77, 762, 8, 77, 1,
        78, 1, 78, 1, 78, 5, 78, 767, 8, 78, 10, 78, 12, 78, 770, 9, 78, 1, 79, 1, 79, 1, 79, 1,
        79, 1, 80, 1, 80, 1, 80, 1, 80, 1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 5, 81, 786, 8,
        81, 10, 81, 12, 81, 789, 9, 81, 1, 81, 1, 81, 1, 82, 1, 82, 1, 82, 1, 82, 1, 83, 1, 83, 3,
        83, 799, 8, 83, 1, 84, 1, 84, 1, 84, 1, 84, 1, 84, 1, 84, 1, 84, 3, 84, 808, 8, 84, 1, 84,
        0, 1, 46, 85, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38,
        40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84,
        86, 88, 90, 92, 94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122,
        124, 126, 128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158,
        160, 162, 164, 166, 168, 0, 24, 1, 0, 28, 29, 1, 0, 30, 34, 3, 0, 96, 99, 101, 103, 105,
        107, 2, 0, 110, 110, 112, 113, 1, 0, 96, 97, 1, 0, 108, 109, 1, 0, 117, 120, 2, 0, 114,
        114, 116, 116, 1, 0, 106, 107, 1, 0, 18, 20, 1, 0, 23, 26, 1, 0, 19, 20, 3, 0, 18, 18, 22,
        22, 125, 125, 1, 0, 36, 37, 1, 0, 38, 44, 1, 0, 45, 47, 1, 0, 48, 56, 1, 0, 57, 59, 1, 0,
        60, 61, 3, 0, 45, 46, 48, 49, 60, 61, 1, 0, 62, 66, 2, 0, 76, 76, 78, 78, 2, 0, 68, 68, 71,
        71, 2, 0, 114, 114, 116, 120, 843, 0, 175, 1, 0, 0, 0, 2, 180, 1, 0, 0, 0, 4, 191, 1, 0, 0,
        0, 6, 217, 1, 0, 0, 0, 8, 220, 1, 0, 0, 0, 10, 228, 1, 0, 0, 0, 12, 231, 1, 0, 0, 0, 14,
        238, 1, 0, 0, 0, 16, 241, 1, 0, 0, 0, 18, 259, 1, 0, 0, 0, 20, 262, 1, 0, 0, 0, 22, 272, 1,
        0, 0, 0, 24, 293, 1, 0, 0, 0, 26, 296, 1, 0, 0, 0, 28, 299, 1, 0, 0, 0, 30, 302, 1, 0, 0,
        0, 32, 305, 1, 0, 0, 0, 34, 309, 1, 0, 0, 0, 36, 311, 1, 0, 0, 0, 38, 313, 1, 0, 0, 0, 40,
        325, 1, 0, 0, 0, 42, 334, 1, 0, 0, 0, 44, 346, 1, 0, 0, 0, 46, 356, 1, 0, 0, 0, 48, 411, 1,
        0, 0, 0, 50, 413, 1, 0, 0, 0, 52, 424, 1, 0, 0, 0, 54, 429, 1, 0, 0, 0, 56, 445, 1, 0, 0,
        0, 58, 449, 1, 0, 0, 0, 60, 451, 1, 0, 0, 0, 62, 457, 1, 0, 0, 0, 64, 461, 1, 0, 0, 0, 66,
        466, 1, 0, 0, 0, 68, 473, 1, 0, 0, 0, 70, 484, 1, 0, 0, 0, 72, 489, 1, 0, 0, 0, 74, 491, 1,
        0, 0, 0, 76, 493, 1, 0, 0, 0, 78, 510, 1, 0, 0, 0, 80, 529, 1, 0, 0, 0, 82, 531, 1, 0, 0,
        0, 84, 535, 1, 0, 0, 0, 86, 539, 1, 0, 0, 0, 88, 541, 1, 0, 0, 0, 90, 543, 1, 0, 0, 0, 92,
        545, 1, 0, 0, 0, 94, 547, 1, 0, 0, 0, 96, 549, 1, 0, 0, 0, 98, 557, 1, 0, 0, 0, 100, 564,
        1, 0, 0, 0, 102, 582, 1, 0, 0, 0, 104, 592, 1, 0, 0, 0, 106, 596, 1, 0, 0, 0, 108, 602, 1,
        0, 0, 0, 110, 606, 1, 0, 0, 0, 112, 611, 1, 0, 0, 0, 114, 613, 1, 0, 0, 0, 116, 615, 1, 0,
        0, 0, 118, 617, 1, 0, 0, 0, 120, 633, 1, 0, 0, 0, 122, 635, 1, 0, 0, 0, 124, 644, 1, 0, 0,
        0, 126, 646, 1, 0, 0, 0, 128, 658, 1, 0, 0, 0, 130, 660, 1, 0, 0, 0, 132, 664, 1, 0, 0, 0,
        134, 668, 1, 0, 0, 0, 136, 680, 1, 0, 0, 0, 138, 697, 1, 0, 0, 0, 140, 702, 1, 0, 0, 0,
        142, 721, 1, 0, 0, 0, 144, 728, 1, 0, 0, 0, 146, 737, 1, 0, 0, 0, 148, 739, 1, 0, 0, 0,
        150, 743, 1, 0, 0, 0, 152, 746, 1, 0, 0, 0, 154, 761, 1, 0, 0, 0, 156, 763, 1, 0, 0, 0,
        158, 771, 1, 0, 0, 0, 160, 775, 1, 0, 0, 0, 162, 779, 1, 0, 0, 0, 164, 792, 1, 0, 0, 0,
        166, 798, 1, 0, 0, 0, 168, 807, 1, 0, 0, 0, 170, 171, 3, 4, 2, 0, 171, 172, 5, 1, 0, 0,
        172, 174, 1, 0, 0, 0, 173, 170, 1, 0, 0, 0, 174, 177, 1, 0, 0, 0, 175, 173, 1, 0, 0, 0,
        175, 176, 1, 0, 0, 0, 176, 178, 1, 0, 0, 0, 177, 175, 1, 0, 0, 0, 178, 179, 5, 0, 0, 1,
        179, 1, 1, 0, 0, 0, 180, 181, 3, 46, 23, 0, 181, 182, 5, 0, 0, 1, 182, 3, 1, 0, 0, 0, 183,
        192, 3, 6, 3, 0, 184, 192, 3, 134, 67, 0, 185, 192, 3, 118, 59, 0, 186, 192, 3, 140, 70, 0,
        187, 192, 3, 146, 73, 0, 188, 192, 3, 8, 4, 0, 189, 192, 3, 102, 51, 0, 190, 192, 3, 104,
        52, 0, 191, 183, 1, 0, 0, 0, 191, 184, 1, 0, 0, 0, 191, 185, 1, 0, 0, 0, 191, 186, 1, 0, 0,
        0, 191, 187, 1, 0, 0, 0, 191, 188, 1, 0, 0, 0, 191, 189, 1, 0, 0, 0, 191, 190, 1, 0, 0, 0,
        192, 5, 1, 0, 0, 0, 193, 200, 3, 12, 6, 0, 194, 195, 3, 32, 16, 0, 195, 196, 3, 20, 10, 0,
        196, 201, 1, 0, 0, 0, 197, 199, 3, 20, 10, 0, 198, 197, 1, 0, 0, 0, 198, 199, 1, 0, 0, 0,
        199, 201, 1, 0, 0, 0, 200, 194, 1, 0, 0, 0, 200, 198, 1, 0, 0, 0, 201, 218, 1, 0, 0, 0,
        202, 207, 3, 14, 7, 0, 203, 204, 3, 32, 16, 0, 204, 205, 3, 20, 10, 0, 205, 208, 1, 0, 0,
        0, 206, 208, 3, 20, 10, 0, 207, 203, 1, 0, 0, 0, 207, 206, 1, 0, 0, 0, 208, 218, 1, 0, 0,
        0, 209, 210, 3, 32, 16, 0, 210, 211, 3, 12, 6, 0, 211, 212, 3, 20, 10, 0, 212, 218, 1, 0,
        0, 0, 213, 214, 3, 32, 16, 0, 214, 215, 3, 14, 7, 0, 215, 216, 3, 20, 10, 0, 216, 218, 1,
        0, 0, 0, 217, 193, 1, 0, 0, 0, 217, 202, 1, 0, 0, 0, 217, 209, 1, 0, 0, 0, 217, 213, 1, 0,
        0, 0, 218, 7, 1, 0, 0, 0, 219, 221, 3, 32, 16, 0, 220, 219, 1, 0, 0, 0, 220, 221, 1, 0, 0,
        0, 221, 223, 1, 0, 0, 0, 222, 224, 3, 10, 5, 0, 223, 222, 1, 0, 0, 0, 223, 224, 1, 0, 0, 0,
        224, 225, 1, 0, 0, 0, 225, 226, 5, 125, 0, 0, 226, 227, 3, 20, 10, 0, 227, 9, 1, 0, 0, 0,
        228, 229, 5, 27, 0, 0, 229, 230, 5, 125, 0, 0, 230, 11, 1, 0, 0, 0, 231, 232, 3, 34, 17, 0,
        232, 234, 5, 125, 0, 0, 233, 235, 3, 38, 19, 0, 234, 233, 1, 0, 0, 0, 234, 235, 1, 0, 0, 0,
        235, 236, 1, 0, 0, 0, 236, 237, 3, 16, 8, 0, 237, 13, 1, 0, 0, 0, 238, 239, 3, 34, 17, 0,
        239, 240, 3, 16, 8, 0, 240, 15, 1, 0, 0, 0, 241, 247, 5, 2, 0, 0, 242, 243, 3, 18, 9, 0,
        243, 244, 5, 1, 0, 0, 244, 246, 1, 0, 0, 0, 245, 242, 1, 0, 0, 0, 246, 249, 1, 0, 0, 0,
        247, 245, 1, 0, 0, 0, 247, 248, 1, 0, 0, 0, 248, 250, 1, 0, 0, 0, 249, 247, 1, 0, 0, 0,
        250, 251, 5, 3, 0, 0, 251, 17, 1, 0, 0, 0, 252, 260, 3, 6, 3, 0, 253, 260, 3, 134, 67, 0,
        254, 260, 3, 140, 70, 0, 255, 260, 3, 146, 73, 0, 256, 260, 3, 8, 4, 0, 257, 260, 3, 102,
        51, 0, 258, 260, 3, 104, 52, 0, 259, 252, 1, 0, 0, 0, 259, 253, 1, 0, 0, 0, 259, 254, 1, 0,
        0, 0, 259, 255, 1, 0, 0, 0, 259, 256, 1, 0, 0, 0, 259, 257, 1, 0, 0, 0, 259, 258, 1, 0, 0,
        0, 260, 19, 1, 0, 0, 0, 261, 263, 3, 42, 21, 0, 262, 261, 1, 0, 0, 0, 262, 263, 1, 0, 0, 0,
        263, 264, 1, 0, 0, 0, 264, 269, 3, 22, 11, 0, 265, 266, 5, 4, 0, 0, 266, 268, 3, 22, 11, 0,
        267, 265, 1, 0, 0, 0, 268, 271, 1, 0, 0, 0, 269, 267, 1, 0, 0, 0, 269, 270, 1, 0, 0, 0,
        270, 21, 1, 0, 0, 0, 271, 269, 1, 0, 0, 0, 272, 279, 5, 125, 0, 0, 273, 275, 3, 62, 31, 0,
        274, 273, 1, 0, 0, 0, 275, 276, 1, 0, 0, 0, 276, 274, 1, 0, 0, 0, 276, 277, 1, 0, 0, 0,
        277, 280, 1, 0, 0, 0, 278, 280, 3, 60, 30, 0, 279, 274, 1, 0, 0, 0, 279, 278, 1, 0, 0, 0,
        279, 280, 1, 0, 0, 0, 280, 282, 1, 0, 0, 0, 281, 283, 3, 24, 12, 0, 282, 281, 1, 0, 0, 0,
        282, 283, 1, 0, 0, 0, 283, 285, 1, 0, 0, 0, 284, 286, 3, 26, 13, 0, 285, 284, 1, 0, 0, 0,
        285, 286, 1, 0, 0, 0, 286, 288, 1, 0, 0, 0, 287, 289, 3, 28, 14, 0, 288, 287, 1, 0, 0, 0,
        288, 289, 1, 0, 0, 0, 289, 291, 1, 0, 0, 0, 290, 292, 3, 30, 15, 0, 291, 290, 1, 0, 0, 0,
        291, 292, 1, 0, 0, 0, 292, 23, 1, 0, 0, 0, 293, 294, 5, 115, 0, 0, 294, 295, 3, 46, 23, 0,
        295, 25, 1, 0, 0, 0, 296, 297, 5, 121, 0, 0, 297, 298, 3, 46, 23, 0, 298, 27, 1, 0, 0, 0,
        299, 300, 5, 122, 0, 0, 300, 301, 3, 46, 23, 0, 301, 29, 1, 0, 0, 0, 302, 303, 5, 123, 0,
        0, 303, 304, 3, 46, 23, 0, 304, 31, 1, 0, 0, 0, 305, 306, 7, 0, 0, 0, 306, 33, 1, 0, 0, 0,
        307, 310, 3, 36, 18, 0, 308, 310, 5, 35, 0, 0, 309, 307, 1, 0, 0, 0, 309, 308, 1, 0, 0, 0,
        310, 35, 1, 0, 0, 0, 311, 312, 7, 1, 0, 0, 312, 37, 1, 0, 0, 0, 313, 314, 5, 5, 0, 0, 314,
        315, 5, 6, 0, 0, 315, 320, 3, 40, 20, 0, 316, 317, 5, 4, 0, 0, 317, 319, 3, 40, 20, 0, 318,
        316, 1, 0, 0, 0, 319, 322, 1, 0, 0, 0, 320, 318, 1, 0, 0, 0, 320, 321, 1, 0, 0, 0, 321,
        323, 1, 0, 0, 0, 322, 320, 1, 0, 0, 0, 323, 324, 5, 7, 0, 0, 324, 39, 1, 0, 0, 0, 325, 326,
        3, 66, 33, 0, 326, 328, 5, 125, 0, 0, 327, 329, 3, 64, 32, 0, 328, 327, 1, 0, 0, 0, 328,
        329, 1, 0, 0, 0, 329, 332, 1, 0, 0, 0, 330, 331, 5, 115, 0, 0, 331, 333, 3, 46, 23, 0, 332,
        330, 1, 0, 0, 0, 332, 333, 1, 0, 0, 0, 333, 41, 1, 0, 0, 0, 334, 335, 5, 5, 0, 0, 335, 336,
        5, 6, 0, 0, 336, 341, 3, 44, 22, 0, 337, 338, 5, 4, 0, 0, 338, 340, 3, 44, 22, 0, 339, 337,
        1, 0, 0, 0, 340, 343, 1, 0, 0, 0, 341, 339, 1, 0, 0, 0, 341, 342, 1, 0, 0, 0, 342, 344, 1,
        0, 0, 0, 343, 341, 1, 0, 0, 0, 344, 345, 5, 7, 0, 0, 345, 43, 1, 0, 0, 0, 346, 347, 5, 8,
        0, 0, 347, 348, 5, 125, 0, 0, 348, 349, 5, 6, 0, 0, 349, 350, 3, 46, 23, 0, 350, 351, 5, 7,
        0, 0, 351, 45, 1, 0, 0, 0, 352, 353, 6, 23, -1, 0, 353, 354, 7, 2, 0, 0, 354, 357, 3, 48,
        24, 0, 355, 357, 3, 48, 24, 0, 356, 352, 1, 0, 0, 0, 356, 355, 1, 0, 0, 0, 357, 399, 1, 0,
        0, 0, 358, 359, 10, 13, 0, 0, 359, 360, 5, 111, 0, 0, 360, 398, 3, 46, 23, 14, 361, 362,
        10, 12, 0, 0, 362, 363, 7, 3, 0, 0, 363, 398, 3, 46, 23, 13, 364, 365, 10, 11, 0, 0, 365,
        366, 7, 4, 0, 0, 366, 398, 3, 46, 23, 12, 367, 368, 10, 10, 0, 0, 368, 369, 7, 5, 0, 0,
        369, 398, 3, 46, 23, 11, 370, 371, 10, 9, 0, 0, 371, 372, 7, 6, 0, 0, 372, 398, 3, 46, 23,
        10, 373, 374, 10, 8, 0, 0, 374, 375, 7, 7, 0, 0, 375, 398, 3, 46, 23, 9, 376, 377, 10, 7,
        0, 0, 377, 378, 5, 102, 0, 0, 378, 398, 3, 46, 23, 8, 379, 380, 10, 6, 0, 0, 380, 381, 7,
        8, 0, 0, 381, 398, 3, 46, 23, 7, 382, 383, 10, 5, 0, 0, 383, 384, 5, 103, 0, 0, 384, 398,
        3, 46, 23, 6, 385, 386, 10, 4, 0, 0, 386, 387, 5, 100, 0, 0, 387, 398, 3, 46, 23, 5, 388,
        389, 10, 3, 0, 0, 389, 390, 5, 104, 0, 0, 390, 398, 3, 46, 23, 4, 391, 392, 10, 2, 0, 0,
        392, 393, 5, 9, 0, 0, 393, 394, 3, 46, 23, 0, 394, 395, 5, 10, 0, 0, 395, 396, 3, 46, 23,
        2, 396, 398, 1, 0, 0, 0, 397, 358, 1, 0, 0, 0, 397, 361, 1, 0, 0, 0, 397, 364, 1, 0, 0, 0,
        397, 367, 1, 0, 0, 0, 397, 370, 1, 0, 0, 0, 397, 373, 1, 0, 0, 0, 397, 376, 1, 0, 0, 0,
        397, 379, 1, 0, 0, 0, 397, 382, 1, 0, 0, 0, 397, 385, 1, 0, 0, 0, 397, 388, 1, 0, 0, 0,
        397, 391, 1, 0, 0, 0, 398, 401, 1, 0, 0, 0, 399, 397, 1, 0, 0, 0, 399, 400, 1, 0, 0, 0,
        400, 47, 1, 0, 0, 0, 401, 399, 1, 0, 0, 0, 402, 412, 3, 70, 35, 0, 403, 412, 3, 50, 25, 0,
        404, 412, 3, 52, 26, 0, 405, 412, 3, 54, 27, 0, 406, 412, 3, 56, 28, 0, 407, 412, 3, 100,
        50, 0, 408, 412, 3, 96, 48, 0, 409, 412, 3, 80, 40, 0, 410, 412, 3, 78, 39, 0, 411, 402, 1,
        0, 0, 0, 411, 403, 1, 0, 0, 0, 411, 404, 1, 0, 0, 0, 411, 405, 1, 0, 0, 0, 411, 406, 1, 0,
        0, 0, 411, 407, 1, 0, 0, 0, 411, 408, 1, 0, 0, 0, 411, 409, 1, 0, 0, 0, 411, 410, 1, 0, 0,
        0, 412, 49, 1, 0, 0, 0, 413, 414, 5, 2, 0, 0, 414, 419, 3, 46, 23, 0, 415, 416, 5, 4, 0, 0,
        416, 418, 3, 46, 23, 0, 417, 415, 1, 0, 0, 0, 418, 421, 1, 0, 0, 0, 419, 417, 1, 0, 0, 0,
        419, 420, 1, 0, 0, 0, 420, 422, 1, 0, 0, 0, 421, 419, 1, 0, 0, 0, 422, 423, 5, 3, 0, 0,
        423, 51, 1, 0, 0, 0, 424, 425, 5, 2, 0, 0, 425, 426, 3, 46, 23, 0, 426, 427, 3, 50, 25, 0,
        427, 428, 5, 3, 0, 0, 428, 53, 1, 0, 0, 0, 429, 430, 5, 6, 0, 0, 430, 431, 3, 46, 23, 0,
        431, 432, 5, 7, 0, 0, 432, 55, 1, 0, 0, 0, 433, 434, 7, 9, 0, 0, 434, 435, 5, 11, 0, 0,
        435, 436, 5, 6, 0, 0, 436, 437, 3, 46, 23, 0, 437, 438, 5, 7, 0, 0, 438, 446, 1, 0, 0, 0,
        439, 440, 3, 58, 29, 0, 440, 441, 5, 11, 0, 0, 441, 442, 5, 6, 0, 0, 442, 443, 3, 46, 23,
        0, 443, 444, 5, 7, 0, 0, 444, 446, 1, 0, 0, 0, 445, 433, 1, 0, 0, 0, 445, 439, 1, 0, 0, 0,
        446, 57, 1, 0, 0, 0, 447, 450, 3, 70, 35, 0, 448, 450, 3, 54, 27, 0, 449, 447, 1, 0, 0, 0,
        449, 448, 1, 0, 0, 0, 450, 59, 1, 0, 0, 0, 451, 452, 5, 12, 0, 0, 452, 453, 3, 46, 23, 0,
        453, 454, 5, 10, 0, 0, 454, 455, 3, 46, 23, 0, 455, 456, 5, 13, 0, 0, 456, 61, 1, 0, 0, 0,
        457, 458, 5, 12, 0, 0, 458, 459, 3, 46, 23, 0, 459, 460, 5, 13, 0, 0, 460, 63, 1, 0, 0, 0,
        461, 462, 5, 12, 0, 0, 462, 463, 5, 13, 0, 0, 463, 65, 1, 0, 0, 0, 464, 467, 3, 68, 34, 0,
        465, 467, 7, 10, 0, 0, 466, 464, 1, 0, 0, 0, 466, 465, 1, 0, 0, 0, 467, 67, 1, 0, 0, 0,
        468, 470, 7, 11, 0, 0, 469, 471, 5, 21, 0, 0, 470, 469, 1, 0, 0, 0, 470, 471, 1, 0, 0, 0,
        471, 474, 1, 0, 0, 0, 472, 474, 7, 12, 0, 0, 473, 468, 1, 0, 0, 0, 473, 472, 1, 0, 0, 0,
        474, 69, 1, 0, 0, 0, 475, 485, 3, 72, 36, 0, 476, 485, 3, 74, 37, 0, 477, 485, 3, 76, 38,
        0, 478, 485, 3, 86, 43, 0, 479, 485, 3, 88, 44, 0, 480, 485, 3, 90, 45, 0, 481, 485, 3, 92,
        46, 0, 482, 485, 3, 94, 47, 0, 483, 485, 3, 84, 42, 0, 484, 475, 1, 0, 0, 0, 484, 476, 1,
        0, 0, 0, 484, 477, 1, 0, 0, 0, 484, 478, 1, 0, 0, 0, 484, 479, 1, 0, 0, 0, 484, 480, 1, 0,
        0, 0, 484, 481, 1, 0, 0, 0, 484, 482, 1, 0, 0, 0, 484, 483, 1, 0, 0, 0, 485, 71, 1, 0, 0,
        0, 486, 490, 5, 92, 0, 0, 487, 490, 5, 93, 0, 0, 488, 490, 5, 94, 0, 0, 489, 486, 1, 0, 0,
        0, 489, 487, 1, 0, 0, 0, 489, 488, 1, 0, 0, 0, 490, 73, 1, 0, 0, 0, 491, 492, 5, 95, 0, 0,
        492, 75, 1, 0, 0, 0, 493, 494, 7, 13, 0, 0, 494, 77, 1, 0, 0, 0, 495, 496, 5, 11, 0, 0,
        496, 497, 5, 2, 0, 0, 497, 511, 5, 3, 0, 0, 498, 499, 5, 11, 0, 0, 499, 500, 5, 2, 0, 0,
        500, 505, 3, 46, 23, 0, 501, 502, 5, 4, 0, 0, 502, 504, 3, 46, 23, 0, 503, 501, 1, 0, 0, 0,
        504, 507, 1, 0, 0, 0, 505, 503, 1, 0, 0, 0, 505, 506, 1, 0, 0, 0, 506, 508, 1, 0, 0, 0,
        507, 505, 1, 0, 0, 0, 508, 509, 5, 3, 0, 0, 509, 511, 1, 0, 0, 0, 510, 495, 1, 0, 0, 0,
        510, 498, 1, 0, 0, 0, 511, 79, 1, 0, 0, 0, 512, 513, 5, 125, 0, 0, 513, 514, 5, 11, 0, 0,
        514, 515, 5, 2, 0, 0, 515, 530, 5, 3, 0, 0, 516, 517, 5, 125, 0, 0, 517, 518, 5, 11, 0, 0,
        518, 519, 5, 2, 0, 0, 519, 524, 3, 82, 41, 0, 520, 521, 5, 4, 0, 0, 521, 523, 3, 82, 41, 0,
        522, 520, 1, 0, 0, 0, 523, 526, 1, 0, 0, 0, 524, 522, 1, 0, 0, 0, 524, 525, 1, 0, 0, 0,
        525, 527, 1, 0, 0, 0, 526, 524, 1, 0, 0, 0, 527, 528, 5, 3, 0, 0, 528, 530, 1, 0, 0, 0,
        529, 512, 1, 0, 0, 0, 529, 516, 1, 0, 0, 0, 530, 81, 1, 0, 0, 0, 531, 532, 5, 125, 0, 0,
        532, 533, 5, 10, 0, 0, 533, 534, 3, 46, 23, 0, 534, 83, 1, 0, 0, 0, 535, 536, 5, 125, 0, 0,
        536, 537, 5, 14, 0, 0, 537, 538, 5, 125, 0, 0, 538, 85, 1, 0, 0, 0, 539, 540, 7, 14, 0, 0,
        540, 87, 1, 0, 0, 0, 541, 542, 7, 15, 0, 0, 542, 89, 1, 0, 0, 0, 543, 544, 7, 16, 0, 0,
        544, 91, 1, 0, 0, 0, 545, 546, 7, 17, 0, 0, 546, 93, 1, 0, 0, 0, 547, 548, 7, 18, 0, 0,
        548, 95, 1, 0, 0, 0, 549, 554, 3, 98, 49, 0, 550, 551, 5, 8, 0, 0, 551, 553, 3, 98, 49, 0,
        552, 550, 1, 0, 0, 0, 553, 556, 1, 0, 0, 0, 554, 552, 1, 0, 0, 0, 554, 555, 1, 0, 0, 0,
        555, 97, 1, 0, 0, 0, 556, 554, 1, 0, 0, 0, 557, 561, 5, 125, 0, 0, 558, 560, 3, 62, 31, 0,
        559, 558, 1, 0, 0, 0, 560, 563, 1, 0, 0, 0, 561, 559, 1, 0, 0, 0, 561, 562, 1, 0, 0, 0,
        562, 99, 1, 0, 0, 0, 563, 561, 1, 0, 0, 0, 564, 565, 3, 96, 48, 0, 565, 568, 5, 15, 0, 0,
        566, 569, 3, 114, 57, 0, 567, 569, 5, 125, 0, 0, 568, 566, 1, 0, 0, 0, 568, 567, 1, 0, 0,
        0, 569, 101, 1, 0, 0, 0, 570, 572, 5, 72, 0, 0, 571, 570, 1, 0, 0, 0, 571, 572, 1, 0, 0, 0,
        572, 573, 1, 0, 0, 0, 573, 583, 3, 106, 53, 0, 574, 576, 5, 72, 0, 0, 575, 574, 1, 0, 0, 0,
        575, 576, 1, 0, 0, 0, 576, 577, 1, 0, 0, 0, 577, 583, 3, 108, 54, 0, 578, 580, 5, 72, 0, 0,
        579, 578, 1, 0, 0, 0, 579, 580, 1, 0, 0, 0, 580, 581, 1, 0, 0, 0, 581, 583, 3, 110, 55, 0,
        582, 571, 1, 0, 0, 0, 582, 575, 1, 0, 0, 0, 582, 579, 1, 0, 0, 0, 583, 103, 1, 0, 0, 0,
        584, 585, 3, 96, 48, 0, 585, 586, 5, 15, 0, 0, 586, 587, 3, 106, 53, 0, 587, 593, 1, 0, 0,
        0, 588, 589, 3, 96, 48, 0, 589, 590, 5, 15, 0, 0, 590, 591, 3, 108, 54, 0, 591, 593, 1, 0,
        0, 0, 592, 584, 1, 0, 0, 0, 592, 588, 1, 0, 0, 0, 593, 105, 1, 0, 0, 0, 594, 597, 3, 114,
        57, 0, 595, 597, 5, 125, 0, 0, 596, 594, 1, 0, 0, 0, 596, 595, 1, 0, 0, 0, 597, 600, 1, 0,
        0, 0, 598, 599, 5, 115, 0, 0, 599, 601, 3, 112, 56, 0, 600, 598, 1, 0, 0, 0, 600, 601, 1,
        0, 0, 0, 601, 107, 1, 0, 0, 0, 602, 603, 5, 74, 0, 0, 603, 604, 5, 115, 0, 0, 604, 605, 5,
        125, 0, 0, 605, 109, 1, 0, 0, 0, 606, 607, 3, 116, 58, 0, 607, 608, 5, 125, 0, 0, 608, 111,
        1, 0, 0, 0, 609, 612, 3, 94, 47, 0, 610, 612, 3, 46, 23, 0, 611, 609, 1, 0, 0, 0, 611, 610,
        1, 0, 0, 0, 612, 113, 1, 0, 0, 0, 613, 614, 7, 19, 0, 0, 614, 115, 1, 0, 0, 0, 615, 616, 7,
        20, 0, 0, 616, 117, 1, 0, 0, 0, 617, 618, 5, 77, 0, 0, 618, 619, 5, 125, 0, 0, 619, 623, 5,
        2, 0, 0, 620, 621, 3, 120, 60, 0, 621, 622, 5, 1, 0, 0, 622, 624, 1, 0, 0, 0, 623, 620, 1,
        0, 0, 0, 624, 625, 1, 0, 0, 0, 625, 623, 1, 0, 0, 0, 625, 626, 1, 0, 0, 0, 626, 627, 1, 0,
        0, 0, 627, 628, 5, 3, 0, 0, 628, 119, 1, 0, 0, 0, 629, 634, 3, 122, 61, 0, 630, 634, 3,
        126, 63, 0, 631, 634, 3, 130, 65, 0, 632, 634, 3, 132, 66, 0, 633, 629, 1, 0, 0, 0, 633,
        630, 1, 0, 0, 0, 633, 631, 1, 0, 0, 0, 633, 632, 1, 0, 0, 0, 634, 121, 1, 0, 0, 0, 635,
        636, 5, 81, 0, 0, 636, 637, 5, 115, 0, 0, 637, 639, 3, 124, 62, 0, 638, 640, 3, 64, 32, 0,
        639, 638, 1, 0, 0, 0, 639, 640, 1, 0, 0, 0, 640, 123, 1, 0, 0, 0, 641, 645, 3, 36, 18, 0,
        642, 645, 7, 21, 0, 0, 643, 645, 3, 68, 34, 0, 644, 641, 1, 0, 0, 0, 644, 642, 1, 0, 0, 0,
        644, 643, 1, 0, 0, 0, 645, 125, 1, 0, 0, 0, 646, 647, 5, 69, 0, 0, 647, 648, 5, 115, 0, 0,
        648, 653, 3, 128, 64, 0, 649, 650, 5, 103, 0, 0, 650, 652, 3, 128, 64, 0, 651, 649, 1, 0,
        0, 0, 652, 655, 1, 0, 0, 0, 653, 651, 1, 0, 0, 0, 653, 654, 1, 0, 0, 0, 654, 127, 1, 0, 0,
        0, 655, 653, 1, 0, 0, 0, 656, 659, 3, 34, 17, 0, 657, 659, 7, 22, 0, 0, 658, 656, 1, 0, 0,
        0, 658, 657, 1, 0, 0, 0, 659, 129, 1, 0, 0, 0, 660, 661, 5, 72, 0, 0, 661, 662, 5, 115, 0,
        0, 662, 663, 3, 46, 23, 0, 663, 131, 1, 0, 0, 0, 664, 665, 5, 71, 0, 0, 665, 666, 5, 115,
        0, 0, 666, 667, 5, 70, 0, 0, 667, 133, 1, 0, 0, 0, 668, 669, 5, 73, 0, 0, 669, 670, 5, 125,
        0, 0, 670, 674, 5, 2, 0, 0, 671, 672, 3, 136, 68, 0, 672, 673, 5, 1, 0, 0, 673, 675, 1, 0,
        0, 0, 674, 671, 1, 0, 0, 0, 675, 676, 1, 0, 0, 0, 676, 674, 1, 0, 0, 0, 676, 677, 1, 0, 0,
        0, 677, 678, 1, 0, 0, 0, 678, 679, 5, 3, 0, 0, 679, 135, 1, 0, 0, 0, 680, 683, 5, 125, 0,
        0, 681, 682, 5, 115, 0, 0, 682, 684, 3, 46, 23, 0, 683, 681, 1, 0, 0, 0, 683, 684, 1, 0, 0,
        0, 684, 695, 1, 0, 0, 0, 685, 691, 5, 2, 0, 0, 686, 687, 3, 138, 69, 0, 687, 688, 5, 1, 0,
        0, 688, 690, 1, 0, 0, 0, 689, 686, 1, 0, 0, 0, 690, 693, 1, 0, 0, 0, 691, 689, 1, 0, 0, 0,
        691, 692, 1, 0, 0, 0, 692, 694, 1, 0, 0, 0, 693, 691, 1, 0, 0, 0, 694, 696, 5, 3, 0, 0,
        695, 685, 1, 0, 0, 0, 695, 696, 1, 0, 0, 0, 696, 137, 1, 0, 0, 0, 697, 698, 5, 125, 0, 0,
        698, 699, 5, 115, 0, 0, 699, 700, 3, 46, 23, 0, 700, 139, 1, 0, 0, 0, 701, 703, 5, 67, 0,
        0, 702, 701, 1, 0, 0, 0, 702, 703, 1, 0, 0, 0, 703, 704, 1, 0, 0, 0, 704, 705, 5, 79, 0, 0,
        705, 708, 5, 125, 0, 0, 706, 707, 5, 10, 0, 0, 707, 709, 5, 125, 0, 0, 708, 706, 1, 0, 0,
        0, 708, 709, 1, 0, 0, 0, 709, 710, 1, 0, 0, 0, 710, 716, 5, 2, 0, 0, 711, 712, 3, 142, 71,
        0, 712, 713, 5, 1, 0, 0, 713, 715, 1, 0, 0, 0, 714, 711, 1, 0, 0, 0, 715, 718, 1, 0, 0, 0,
        716, 714, 1, 0, 0, 0, 716, 717, 1, 0, 0, 0, 717, 719, 1, 0, 0, 0, 718, 716, 1, 0, 0, 0,
        719, 720, 5, 3, 0, 0, 720, 141, 1, 0, 0, 0, 721, 722, 3, 144, 72, 0, 722, 724, 5, 125, 0,
        0, 723, 725, 3, 64, 32, 0, 724, 723, 1, 0, 0, 0, 724, 725, 1, 0, 0, 0, 725, 143, 1, 0, 0,
        0, 726, 729, 3, 66, 33, 0, 727, 729, 3, 34, 17, 0, 728, 726, 1, 0, 0, 0, 728, 727, 1, 0, 0,
        0, 729, 145, 1, 0, 0, 0, 730, 732, 3, 148, 74, 0, 731, 733, 3, 156, 78, 0, 732, 731, 1, 0,
        0, 0, 732, 733, 1, 0, 0, 0, 733, 738, 1, 0, 0, 0, 734, 735, 3, 150, 75, 0, 735, 736, 3,
        156, 78, 0, 736, 738, 1, 0, 0, 0, 737, 730, 1, 0, 0, 0, 737, 734, 1, 0, 0, 0, 738, 147, 1,
        0, 0, 0, 739, 740, 5, 71, 0, 0, 740, 741, 5, 125, 0, 0, 741, 742, 3, 152, 76, 0, 742, 149,
        1, 0, 0, 0, 743, 744, 5, 71, 0, 0, 744, 745, 3, 152, 76, 0, 745, 151, 1, 0, 0, 0, 746, 752,
        5, 2, 0, 0, 747, 748, 3, 154, 77, 0, 748, 749, 5, 1, 0, 0, 749, 751, 1, 0, 0, 0, 750, 747,
        1, 0, 0, 0, 751, 754, 1, 0, 0, 0, 752, 750, 1, 0, 0, 0, 752, 753, 1, 0, 0, 0, 753, 755, 1,
        0, 0, 0, 754, 752, 1, 0, 0, 0, 755, 756, 5, 3, 0, 0, 756, 153, 1, 0, 0, 0, 757, 762, 3,
        158, 79, 0, 758, 762, 3, 160, 80, 0, 759, 762, 3, 162, 81, 0, 760, 762, 3, 164, 82, 0, 761,
        757, 1, 0, 0, 0, 761, 758, 1, 0, 0, 0, 761, 759, 1, 0, 0, 0, 761, 760, 1, 0, 0, 0, 762,
        155, 1, 0, 0, 0, 763, 768, 5, 125, 0, 0, 764, 765, 5, 4, 0, 0, 765, 767, 5, 125, 0, 0, 766,
        764, 1, 0, 0, 0, 767, 770, 1, 0, 0, 0, 768, 766, 1, 0, 0, 0, 768, 769, 1, 0, 0, 0, 769,
        157, 1, 0, 0, 0, 770, 768, 1, 0, 0, 0, 771, 772, 3, 46, 23, 0, 772, 773, 7, 23, 0, 0, 773,
        774, 3, 46, 23, 0, 774, 159, 1, 0, 0, 0, 775, 776, 5, 125, 0, 0, 776, 777, 5, 115, 0, 0,
        777, 778, 3, 46, 23, 0, 778, 161, 1, 0, 0, 0, 779, 780, 3, 166, 83, 0, 780, 781, 5, 75, 0,
        0, 781, 782, 5, 2, 0, 0, 782, 787, 3, 168, 84, 0, 783, 784, 5, 4, 0, 0, 784, 786, 3, 168,
        84, 0, 785, 783, 1, 0, 0, 0, 786, 789, 1, 0, 0, 0, 787, 785, 1, 0, 0, 0, 787, 788, 1, 0, 0,
        0, 788, 790, 1, 0, 0, 0, 789, 787, 1, 0, 0, 0, 790, 791, 5, 3, 0, 0, 791, 163, 1, 0, 0, 0,
        792, 793, 3, 166, 83, 0, 793, 794, 5, 75, 0, 0, 794, 795, 5, 125, 0, 0, 795, 165, 1, 0, 0,
        0, 796, 799, 5, 80, 0, 0, 797, 799, 3, 96, 48, 0, 798, 796, 1, 0, 0, 0, 798, 797, 1, 0, 0,
        0, 799, 167, 1, 0, 0, 0, 800, 808, 3, 46, 23, 0, 801, 802, 5, 12, 0, 0, 802, 803, 3, 46,
        23, 0, 803, 804, 5, 10, 0, 0, 804, 805, 3, 46, 23, 0, 805, 806, 5, 13, 0, 0, 806, 808, 1,
        0, 0, 0, 807, 800, 1, 0, 0, 0, 807, 801, 1, 0, 0, 0, 808, 169, 1, 0, 0, 0, 74, 175, 191,
        198, 200, 207, 217, 220, 223, 234, 247, 259, 262, 269, 276, 279, 282, 285, 288, 291, 309,
        320, 328, 332, 341, 356, 397, 399, 411, 419, 445, 449, 466, 470, 473, 484, 489, 505, 510,
        524, 529, 554, 561, 568, 571, 575, 579, 582, 592, 596, 600, 611, 625, 633, 639, 644, 653,
        658, 676, 683, 691, 695, 702, 708, 716, 724, 728, 732, 737, 752, 761, 768, 787, 798, 807
    ];
}
