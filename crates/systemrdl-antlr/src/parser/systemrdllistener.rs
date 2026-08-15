#![allow(nonstandard_style)]
// Generated from SystemRDL.g4 by ANTLR 4.13.2
use super::systemrdlparser::*;
use antlr4rust::tree::ParseTreeListener;

pub trait SystemRDLListener<'input>: ParseTreeListener<'input, SystemRDLParserContextType> {
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#root}.
     * @param ctx the parse tree
     */
    fn enter_root(&mut self, _ctx: &RootContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#root}.
     * @param ctx the parse tree
     */
    fn exit_root(&mut self, _ctx: &RootContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#eval_expr_root}.
     * @param ctx the parse tree
     */
    fn enter_eval_expr_root(&mut self, _ctx: &Eval_expr_rootContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#eval_expr_root}.
     * @param ctx the parse tree
     */
    fn exit_eval_expr_root(&mut self, _ctx: &Eval_expr_rootContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#root_elem}.
     * @param ctx the parse tree
     */
    fn enter_root_elem(&mut self, _ctx: &Root_elemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#root_elem}.
     * @param ctx the parse tree
     */
    fn exit_root_elem(&mut self, _ctx: &Root_elemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_def}.
     * @param ctx the parse tree
     */
    fn enter_component_def(&mut self, _ctx: &Component_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_def}.
     * @param ctx the parse tree
     */
    fn exit_component_def(&mut self, _ctx: &Component_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#explicit_component_inst}.
     * @param ctx the parse tree
     */
    fn enter_explicit_component_inst(&mut self, _ctx: &Explicit_component_instContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#explicit_component_inst}.
     * @param ctx the parse tree
     */
    fn exit_explicit_component_inst(&mut self, _ctx: &Explicit_component_instContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_inst_alias}.
     * @param ctx the parse tree
     */
    fn enter_component_inst_alias(&mut self, _ctx: &Component_inst_aliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_inst_alias}.
     * @param ctx the parse tree
     */
    fn exit_component_inst_alias(&mut self, _ctx: &Component_inst_aliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_named_def}.
     * @param ctx the parse tree
     */
    fn enter_component_named_def(&mut self, _ctx: &Component_named_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_named_def}.
     * @param ctx the parse tree
     */
    fn exit_component_named_def(&mut self, _ctx: &Component_named_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_anon_def}.
     * @param ctx the parse tree
     */
    fn enter_component_anon_def(&mut self, _ctx: &Component_anon_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_anon_def}.
     * @param ctx the parse tree
     */
    fn exit_component_anon_def(&mut self, _ctx: &Component_anon_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_body}.
     * @param ctx the parse tree
     */
    fn enter_component_body(&mut self, _ctx: &Component_bodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_body}.
     * @param ctx the parse tree
     */
    fn exit_component_body(&mut self, _ctx: &Component_bodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_body_elem}.
     * @param ctx the parse tree
     */
    fn enter_component_body_elem(&mut self, _ctx: &Component_body_elemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_body_elem}.
     * @param ctx the parse tree
     */
    fn exit_component_body_elem(&mut self, _ctx: &Component_body_elemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_insts}.
     * @param ctx the parse tree
     */
    fn enter_component_insts(&mut self, _ctx: &Component_instsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_insts}.
     * @param ctx the parse tree
     */
    fn exit_component_insts(&mut self, _ctx: &Component_instsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_inst}.
     * @param ctx the parse tree
     */
    fn enter_component_inst(&mut self, _ctx: &Component_instContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_inst}.
     * @param ctx the parse tree
     */
    fn exit_component_inst(&mut self, _ctx: &Component_instContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#field_inst_reset}.
     * @param ctx the parse tree
     */
    fn enter_field_inst_reset(&mut self, _ctx: &Field_inst_resetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#field_inst_reset}.
     * @param ctx the parse tree
     */
    fn exit_field_inst_reset(&mut self, _ctx: &Field_inst_resetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#inst_addr_fixed}.
     * @param ctx the parse tree
     */
    fn enter_inst_addr_fixed(&mut self, _ctx: &Inst_addr_fixedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#inst_addr_fixed}.
     * @param ctx the parse tree
     */
    fn exit_inst_addr_fixed(&mut self, _ctx: &Inst_addr_fixedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#inst_addr_stride}.
     * @param ctx the parse tree
     */
    fn enter_inst_addr_stride(&mut self, _ctx: &Inst_addr_strideContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#inst_addr_stride}.
     * @param ctx the parse tree
     */
    fn exit_inst_addr_stride(&mut self, _ctx: &Inst_addr_strideContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#inst_addr_align}.
     * @param ctx the parse tree
     */
    fn enter_inst_addr_align(&mut self, _ctx: &Inst_addr_alignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#inst_addr_align}.
     * @param ctx the parse tree
     */
    fn exit_inst_addr_align(&mut self, _ctx: &Inst_addr_alignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_inst_type}.
     * @param ctx the parse tree
     */
    fn enter_component_inst_type(&mut self, _ctx: &Component_inst_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_inst_type}.
     * @param ctx the parse tree
     */
    fn exit_component_inst_type(&mut self, _ctx: &Component_inst_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_type}.
     * @param ctx the parse tree
     */
    fn enter_component_type(&mut self, _ctx: &Component_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_type}.
     * @param ctx the parse tree
     */
    fn exit_component_type(&mut self, _ctx: &Component_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#component_type_primary}.
     * @param ctx the parse tree
     */
    fn enter_component_type_primary(&mut self, _ctx: &Component_type_primaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#component_type_primary}.
     * @param ctx the parse tree
     */
    fn exit_component_type_primary(&mut self, _ctx: &Component_type_primaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#param_def}.
     * @param ctx the parse tree
     */
    fn enter_param_def(&mut self, _ctx: &Param_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#param_def}.
     * @param ctx the parse tree
     */
    fn exit_param_def(&mut self, _ctx: &Param_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#param_def_elem}.
     * @param ctx the parse tree
     */
    fn enter_param_def_elem(&mut self, _ctx: &Param_def_elemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#param_def_elem}.
     * @param ctx the parse tree
     */
    fn exit_param_def_elem(&mut self, _ctx: &Param_def_elemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#param_inst}.
     * @param ctx the parse tree
     */
    fn enter_param_inst(&mut self, _ctx: &Param_instContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#param_inst}.
     * @param ctx the parse tree
     */
    fn exit_param_inst(&mut self, _ctx: &Param_instContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#param_assignment}.
     * @param ctx the parse tree
     */
    fn enter_param_assignment(&mut self, _ctx: &Param_assignmentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#param_assignment}.
     * @param ctx the parse tree
     */
    fn exit_param_assignment(&mut self, _ctx: &Param_assignmentContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code BinaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_BinaryExpr(&mut self, _ctx: &BinaryExprContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code BinaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_BinaryExpr(&mut self, _ctx: &BinaryExprContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code UnaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_UnaryExpr(&mut self, _ctx: &UnaryExprContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code UnaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_UnaryExpr(&mut self, _ctx: &UnaryExprContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code NOP}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_NOP(&mut self, _ctx: &NOPContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code NOP}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_NOP(&mut self, _ctx: &NOPContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TernaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_TernaryExpr(&mut self, _ctx: &TernaryExprContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TernaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_TernaryExpr(&mut self, _ctx: &TernaryExprContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#expr_primary}.
     * @param ctx the parse tree
     */
    fn enter_expr_primary(&mut self, _ctx: &Expr_primaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#expr_primary}.
     * @param ctx the parse tree
     */
    fn exit_expr_primary(&mut self, _ctx: &Expr_primaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#concatenate}.
     * @param ctx the parse tree
     */
    fn enter_concatenate(&mut self, _ctx: &ConcatenateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#concatenate}.
     * @param ctx the parse tree
     */
    fn exit_concatenate(&mut self, _ctx: &ConcatenateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#replicate}.
     * @param ctx the parse tree
     */
    fn enter_replicate(&mut self, _ctx: &ReplicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#replicate}.
     * @param ctx the parse tree
     */
    fn exit_replicate(&mut self, _ctx: &ReplicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#paren_expr}.
     * @param ctx the parse tree
     */
    fn enter_paren_expr(&mut self, _ctx: &Paren_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#paren_expr}.
     * @param ctx the parse tree
     */
    fn exit_paren_expr(&mut self, _ctx: &Paren_exprContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code CastType}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn enter_CastType(&mut self, _ctx: &CastTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code CastType}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn exit_CastType(&mut self, _ctx: &CastTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code CastWidth}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn enter_CastWidth(&mut self, _ctx: &CastWidthContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code CastWidth}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn exit_CastWidth(&mut self, _ctx: &CastWidthContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#cast_width_expr}.
     * @param ctx the parse tree
     */
    fn enter_cast_width_expr(&mut self, _ctx: &Cast_width_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#cast_width_expr}.
     * @param ctx the parse tree
     */
    fn exit_cast_width_expr(&mut self, _ctx: &Cast_width_exprContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#range_suffix}.
     * @param ctx the parse tree
     */
    fn enter_range_suffix(&mut self, _ctx: &Range_suffixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#range_suffix}.
     * @param ctx the parse tree
     */
    fn exit_range_suffix(&mut self, _ctx: &Range_suffixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#array_suffix}.
     * @param ctx the parse tree
     */
    fn enter_array_suffix(&mut self, _ctx: &Array_suffixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#array_suffix}.
     * @param ctx the parse tree
     */
    fn exit_array_suffix(&mut self, _ctx: &Array_suffixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#array_type_suffix}.
     * @param ctx the parse tree
     */
    fn enter_array_type_suffix(&mut self, _ctx: &Array_type_suffixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#array_type_suffix}.
     * @param ctx the parse tree
     */
    fn exit_array_type_suffix(&mut self, _ctx: &Array_type_suffixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#data_type}.
     * @param ctx the parse tree
     */
    fn enter_data_type(&mut self, _ctx: &Data_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#data_type}.
     * @param ctx the parse tree
     */
    fn exit_data_type(&mut self, _ctx: &Data_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#basic_data_type}.
     * @param ctx the parse tree
     */
    fn enter_basic_data_type(&mut self, _ctx: &Basic_data_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#basic_data_type}.
     * @param ctx the parse tree
     */
    fn exit_basic_data_type(&mut self, _ctx: &Basic_data_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code NumberInt}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn enter_NumberInt(&mut self, _ctx: &NumberIntContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code NumberInt}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn exit_NumberInt(&mut self, _ctx: &NumberIntContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code NumberHex}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn enter_NumberHex(&mut self, _ctx: &NumberHexContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code NumberHex}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn exit_NumberHex(&mut self, _ctx: &NumberHexContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code NumberVerilog}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn enter_NumberVerilog(&mut self, _ctx: &NumberVerilogContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code NumberVerilog}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn exit_NumberVerilog(&mut self, _ctx: &NumberVerilogContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#string_literal}.
     * @param ctx the parse tree
     */
    fn enter_string_literal(&mut self, _ctx: &String_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#string_literal}.
     * @param ctx the parse tree
     */
    fn exit_string_literal(&mut self, _ctx: &String_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#boolean_literal}.
     * @param ctx the parse tree
     */
    fn enter_boolean_literal(&mut self, _ctx: &Boolean_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#boolean_literal}.
     * @param ctx the parse tree
     */
    fn exit_boolean_literal(&mut self, _ctx: &Boolean_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#array_literal}.
     * @param ctx the parse tree
     */
    fn enter_array_literal(&mut self, _ctx: &Array_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#array_literal}.
     * @param ctx the parse tree
     */
    fn exit_array_literal(&mut self, _ctx: &Array_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#struct_literal}.
     * @param ctx the parse tree
     */
    fn enter_struct_literal(&mut self, _ctx: &Struct_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#struct_literal}.
     * @param ctx the parse tree
     */
    fn exit_struct_literal(&mut self, _ctx: &Struct_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#struct_kv}.
     * @param ctx the parse tree
     */
    fn enter_struct_kv(&mut self, _ctx: &Struct_kvContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#struct_kv}.
     * @param ctx the parse tree
     */
    fn exit_struct_kv(&mut self, _ctx: &Struct_kvContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#enum_literal}.
     * @param ctx the parse tree
     */
    fn enter_enum_literal(&mut self, _ctx: &Enum_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#enum_literal}.
     * @param ctx the parse tree
     */
    fn exit_enum_literal(&mut self, _ctx: &Enum_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#accesstype_literal}.
     * @param ctx the parse tree
     */
    fn enter_accesstype_literal(&mut self, _ctx: &Accesstype_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#accesstype_literal}.
     * @param ctx the parse tree
     */
    fn exit_accesstype_literal(&mut self, _ctx: &Accesstype_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#onreadtype_literal}.
     * @param ctx the parse tree
     */
    fn enter_onreadtype_literal(&mut self, _ctx: &Onreadtype_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#onreadtype_literal}.
     * @param ctx the parse tree
     */
    fn exit_onreadtype_literal(&mut self, _ctx: &Onreadtype_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#onwritetype_literal}.
     * @param ctx the parse tree
     */
    fn enter_onwritetype_literal(&mut self, _ctx: &Onwritetype_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#onwritetype_literal}.
     * @param ctx the parse tree
     */
    fn exit_onwritetype_literal(&mut self, _ctx: &Onwritetype_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#addressingtype_literal}.
     * @param ctx the parse tree
     */
    fn enter_addressingtype_literal(&mut self, _ctx: &Addressingtype_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#addressingtype_literal}.
     * @param ctx the parse tree
     */
    fn exit_addressingtype_literal(&mut self, _ctx: &Addressingtype_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#precedencetype_literal}.
     * @param ctx the parse tree
     */
    fn enter_precedencetype_literal(&mut self, _ctx: &Precedencetype_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#precedencetype_literal}.
     * @param ctx the parse tree
     */
    fn exit_precedencetype_literal(&mut self, _ctx: &Precedencetype_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#instance_ref}.
     * @param ctx the parse tree
     */
    fn enter_instance_ref(&mut self, _ctx: &Instance_refContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#instance_ref}.
     * @param ctx the parse tree
     */
    fn exit_instance_ref(&mut self, _ctx: &Instance_refContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#instance_ref_element}.
     * @param ctx the parse tree
     */
    fn enter_instance_ref_element(&mut self, _ctx: &Instance_ref_elementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#instance_ref_element}.
     * @param ctx the parse tree
     */
    fn exit_instance_ref_element(&mut self, _ctx: &Instance_ref_elementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#prop_ref}.
     * @param ctx the parse tree
     */
    fn enter_prop_ref(&mut self, _ctx: &Prop_refContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#prop_ref}.
     * @param ctx the parse tree
     */
    fn exit_prop_ref(&mut self, _ctx: &Prop_refContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#local_property_assignment}.
     * @param ctx the parse tree
     */
    fn enter_local_property_assignment(&mut self, _ctx: &Local_property_assignmentContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#local_property_assignment}.
     * @param ctx the parse tree
     */
    fn exit_local_property_assignment(&mut self, _ctx: &Local_property_assignmentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#dynamic_property_assignment}.
     * @param ctx the parse tree
     */
    fn enter_dynamic_property_assignment(
        &mut self,
        _ctx: &Dynamic_property_assignmentContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#dynamic_property_assignment}.
     * @param ctx the parse tree
     */
    fn exit_dynamic_property_assignment(
        &mut self,
        _ctx: &Dynamic_property_assignmentContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#normal_prop_assign}.
     * @param ctx the parse tree
     */
    fn enter_normal_prop_assign(&mut self, _ctx: &Normal_prop_assignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#normal_prop_assign}.
     * @param ctx the parse tree
     */
    fn exit_normal_prop_assign(&mut self, _ctx: &Normal_prop_assignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#encode_prop_assign}.
     * @param ctx the parse tree
     */
    fn enter_encode_prop_assign(&mut self, _ctx: &Encode_prop_assignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#encode_prop_assign}.
     * @param ctx the parse tree
     */
    fn exit_encode_prop_assign(&mut self, _ctx: &Encode_prop_assignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#prop_mod_assign}.
     * @param ctx the parse tree
     */
    fn enter_prop_mod_assign(&mut self, _ctx: &Prop_mod_assignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#prop_mod_assign}.
     * @param ctx the parse tree
     */
    fn exit_prop_mod_assign(&mut self, _ctx: &Prop_mod_assignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#prop_assignment_rhs}.
     * @param ctx the parse tree
     */
    fn enter_prop_assignment_rhs(&mut self, _ctx: &Prop_assignment_rhsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#prop_assignment_rhs}.
     * @param ctx the parse tree
     */
    fn exit_prop_assignment_rhs(&mut self, _ctx: &Prop_assignment_rhsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#prop_keyword}.
     * @param ctx the parse tree
     */
    fn enter_prop_keyword(&mut self, _ctx: &Prop_keywordContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#prop_keyword}.
     * @param ctx the parse tree
     */
    fn exit_prop_keyword(&mut self, _ctx: &Prop_keywordContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#prop_mod}.
     * @param ctx the parse tree
     */
    fn enter_prop_mod(&mut self, _ctx: &Prop_modContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#prop_mod}.
     * @param ctx the parse tree
     */
    fn exit_prop_mod(&mut self, _ctx: &Prop_modContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_def}.
     * @param ctx the parse tree
     */
    fn enter_udp_def(&mut self, _ctx: &Udp_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_def}.
     * @param ctx the parse tree
     */
    fn exit_udp_def(&mut self, _ctx: &Udp_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_attr}.
     * @param ctx the parse tree
     */
    fn enter_udp_attr(&mut self, _ctx: &Udp_attrContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_attr}.
     * @param ctx the parse tree
     */
    fn exit_udp_attr(&mut self, _ctx: &Udp_attrContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_type}.
     * @param ctx the parse tree
     */
    fn enter_udp_type(&mut self, _ctx: &Udp_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_type}.
     * @param ctx the parse tree
     */
    fn exit_udp_type(&mut self, _ctx: &Udp_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_data_type}.
     * @param ctx the parse tree
     */
    fn enter_udp_data_type(&mut self, _ctx: &Udp_data_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_data_type}.
     * @param ctx the parse tree
     */
    fn exit_udp_data_type(&mut self, _ctx: &Udp_data_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_usage}.
     * @param ctx the parse tree
     */
    fn enter_udp_usage(&mut self, _ctx: &Udp_usageContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_usage}.
     * @param ctx the parse tree
     */
    fn exit_udp_usage(&mut self, _ctx: &Udp_usageContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_comp_type}.
     * @param ctx the parse tree
     */
    fn enter_udp_comp_type(&mut self, _ctx: &Udp_comp_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_comp_type}.
     * @param ctx the parse tree
     */
    fn exit_udp_comp_type(&mut self, _ctx: &Udp_comp_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_default}.
     * @param ctx the parse tree
     */
    fn enter_udp_default(&mut self, _ctx: &Udp_defaultContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_default}.
     * @param ctx the parse tree
     */
    fn exit_udp_default(&mut self, _ctx: &Udp_defaultContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#udp_constraint}.
     * @param ctx the parse tree
     */
    fn enter_udp_constraint(&mut self, _ctx: &Udp_constraintContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#udp_constraint}.
     * @param ctx the parse tree
     */
    fn exit_udp_constraint(&mut self, _ctx: &Udp_constraintContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#enum_def}.
     * @param ctx the parse tree
     */
    fn enter_enum_def(&mut self, _ctx: &Enum_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#enum_def}.
     * @param ctx the parse tree
     */
    fn exit_enum_def(&mut self, _ctx: &Enum_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#enum_entry}.
     * @param ctx the parse tree
     */
    fn enter_enum_entry(&mut self, _ctx: &Enum_entryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#enum_entry}.
     * @param ctx the parse tree
     */
    fn exit_enum_entry(&mut self, _ctx: &Enum_entryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#enum_prop_assign}.
     * @param ctx the parse tree
     */
    fn enter_enum_prop_assign(&mut self, _ctx: &Enum_prop_assignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#enum_prop_assign}.
     * @param ctx the parse tree
     */
    fn exit_enum_prop_assign(&mut self, _ctx: &Enum_prop_assignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#struct_def}.
     * @param ctx the parse tree
     */
    fn enter_struct_def(&mut self, _ctx: &Struct_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#struct_def}.
     * @param ctx the parse tree
     */
    fn exit_struct_def(&mut self, _ctx: &Struct_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#struct_elem}.
     * @param ctx the parse tree
     */
    fn enter_struct_elem(&mut self, _ctx: &Struct_elemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#struct_elem}.
     * @param ctx the parse tree
     */
    fn exit_struct_elem(&mut self, _ctx: &Struct_elemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#struct_type}.
     * @param ctx the parse tree
     */
    fn enter_struct_type(&mut self, _ctx: &Struct_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#struct_type}.
     * @param ctx the parse tree
     */
    fn exit_struct_type(&mut self, _ctx: &Struct_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constraint_def}.
     * @param ctx the parse tree
     */
    fn enter_constraint_def(&mut self, _ctx: &Constraint_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constraint_def}.
     * @param ctx the parse tree
     */
    fn exit_constraint_def(&mut self, _ctx: &Constraint_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constraint_named_def}.
     * @param ctx the parse tree
     */
    fn enter_constraint_named_def(&mut self, _ctx: &Constraint_named_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constraint_named_def}.
     * @param ctx the parse tree
     */
    fn exit_constraint_named_def(&mut self, _ctx: &Constraint_named_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constraint_anon_def}.
     * @param ctx the parse tree
     */
    fn enter_constraint_anon_def(&mut self, _ctx: &Constraint_anon_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constraint_anon_def}.
     * @param ctx the parse tree
     */
    fn exit_constraint_anon_def(&mut self, _ctx: &Constraint_anon_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constraint_body}.
     * @param ctx the parse tree
     */
    fn enter_constraint_body(&mut self, _ctx: &Constraint_bodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constraint_body}.
     * @param ctx the parse tree
     */
    fn exit_constraint_body(&mut self, _ctx: &Constraint_bodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constraint_body_elem}.
     * @param ctx the parse tree
     */
    fn enter_constraint_body_elem(&mut self, _ctx: &Constraint_body_elemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constraint_body_elem}.
     * @param ctx the parse tree
     */
    fn exit_constraint_body_elem(&mut self, _ctx: &Constraint_body_elemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constraint_insts}.
     * @param ctx the parse tree
     */
    fn enter_constraint_insts(&mut self, _ctx: &Constraint_instsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constraint_insts}.
     * @param ctx the parse tree
     */
    fn exit_constraint_insts(&mut self, _ctx: &Constraint_instsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constr_relational}.
     * @param ctx the parse tree
     */
    fn enter_constr_relational(&mut self, _ctx: &Constr_relationalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constr_relational}.
     * @param ctx the parse tree
     */
    fn exit_constr_relational(&mut self, _ctx: &Constr_relationalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constr_prop_assign}.
     * @param ctx the parse tree
     */
    fn enter_constr_prop_assign(&mut self, _ctx: &Constr_prop_assignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constr_prop_assign}.
     * @param ctx the parse tree
     */
    fn exit_constr_prop_assign(&mut self, _ctx: &Constr_prop_assignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constr_inside_values}.
     * @param ctx the parse tree
     */
    fn enter_constr_inside_values(&mut self, _ctx: &Constr_inside_valuesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constr_inside_values}.
     * @param ctx the parse tree
     */
    fn exit_constr_inside_values(&mut self, _ctx: &Constr_inside_valuesContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constr_inside_enum}.
     * @param ctx the parse tree
     */
    fn enter_constr_inside_enum(&mut self, _ctx: &Constr_inside_enumContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constr_inside_enum}.
     * @param ctx the parse tree
     */
    fn exit_constr_inside_enum(&mut self, _ctx: &Constr_inside_enumContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constr_lhs}.
     * @param ctx the parse tree
     */
    fn enter_constr_lhs(&mut self, _ctx: &Constr_lhsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constr_lhs}.
     * @param ctx the parse tree
     */
    fn exit_constr_lhs(&mut self, _ctx: &Constr_lhsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SystemRDLParser#constr_inside_value}.
     * @param ctx the parse tree
     */
    fn enter_constr_inside_value(&mut self, _ctx: &Constr_inside_valueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SystemRDLParser#constr_inside_value}.
     * @param ctx the parse tree
     */
    fn exit_constr_inside_value(&mut self, _ctx: &Constr_inside_valueContext<'input>) {}
}

antlr4rust::coerce_from! { 'input : SystemRDLListener<'input> }
