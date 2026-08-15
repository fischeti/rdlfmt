#![allow(nonstandard_style)]
// Generated from SystemRDL.g4 by ANTLR 4.13.2
use super::systemrdlparser::*;
use antlr4rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SystemRDLParser}.
 */
pub trait SystemRDLVisitor<'input>: ParseTreeVisitor<'input, SystemRDLParserContextType> {
    /**
     * Visit a parse tree produced by {@link SystemRDLParser#root}.
     * @param ctx the parse tree
     */
    fn visit_root(&mut self, ctx: &RootContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#eval_expr_root}.
     * @param ctx the parse tree
     */
    fn visit_eval_expr_root(&mut self, ctx: &Eval_expr_rootContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#root_elem}.
     * @param ctx the parse tree
     */
    fn visit_root_elem(&mut self, ctx: &Root_elemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_def}.
     * @param ctx the parse tree
     */
    fn visit_component_def(&mut self, ctx: &Component_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#explicit_component_inst}.
     * @param ctx the parse tree
     */
    fn visit_explicit_component_inst(&mut self, ctx: &Explicit_component_instContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_inst_alias}.
     * @param ctx the parse tree
     */
    fn visit_component_inst_alias(&mut self, ctx: &Component_inst_aliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_named_def}.
     * @param ctx the parse tree
     */
    fn visit_component_named_def(&mut self, ctx: &Component_named_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_anon_def}.
     * @param ctx the parse tree
     */
    fn visit_component_anon_def(&mut self, ctx: &Component_anon_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_body}.
     * @param ctx the parse tree
     */
    fn visit_component_body(&mut self, ctx: &Component_bodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_body_elem}.
     * @param ctx the parse tree
     */
    fn visit_component_body_elem(&mut self, ctx: &Component_body_elemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_insts}.
     * @param ctx the parse tree
     */
    fn visit_component_insts(&mut self, ctx: &Component_instsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_inst}.
     * @param ctx the parse tree
     */
    fn visit_component_inst(&mut self, ctx: &Component_instContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#field_inst_reset}.
     * @param ctx the parse tree
     */
    fn visit_field_inst_reset(&mut self, ctx: &Field_inst_resetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#inst_addr_fixed}.
     * @param ctx the parse tree
     */
    fn visit_inst_addr_fixed(&mut self, ctx: &Inst_addr_fixedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#inst_addr_stride}.
     * @param ctx the parse tree
     */
    fn visit_inst_addr_stride(&mut self, ctx: &Inst_addr_strideContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#inst_addr_align}.
     * @param ctx the parse tree
     */
    fn visit_inst_addr_align(&mut self, ctx: &Inst_addr_alignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_inst_type}.
     * @param ctx the parse tree
     */
    fn visit_component_inst_type(&mut self, ctx: &Component_inst_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_type}.
     * @param ctx the parse tree
     */
    fn visit_component_type(&mut self, ctx: &Component_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_type_primary}.
     * @param ctx the parse tree
     */
    fn visit_component_type_primary(&mut self, ctx: &Component_type_primaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_def}.
     * @param ctx the parse tree
     */
    fn visit_param_def(&mut self, ctx: &Param_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_def_elem}.
     * @param ctx the parse tree
     */
    fn visit_param_def_elem(&mut self, ctx: &Param_def_elemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_inst}.
     * @param ctx the parse tree
     */
    fn visit_param_inst(&mut self, ctx: &Param_instContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_assignment}.
     * @param ctx the parse tree
     */
    fn visit_param_assignment(&mut self, ctx: &Param_assignmentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code BinaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_BinaryExpr(&mut self, ctx: &BinaryExprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code UnaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_UnaryExpr(&mut self, ctx: &UnaryExprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NOP}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_NOP(&mut self, ctx: &NOPContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TernaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_TernaryExpr(&mut self, ctx: &TernaryExprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#expr_primary}.
     * @param ctx the parse tree
     */
    fn visit_expr_primary(&mut self, ctx: &Expr_primaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#concatenate}.
     * @param ctx the parse tree
     */
    fn visit_concatenate(&mut self, ctx: &ConcatenateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#replicate}.
     * @param ctx the parse tree
     */
    fn visit_replicate(&mut self, ctx: &ReplicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#paren_expr}.
     * @param ctx the parse tree
     */
    fn visit_paren_expr(&mut self, ctx: &Paren_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CastType}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn visit_CastType(&mut self, ctx: &CastTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CastWidth}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn visit_CastWidth(&mut self, ctx: &CastWidthContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#cast_width_expr}.
     * @param ctx the parse tree
     */
    fn visit_cast_width_expr(&mut self, ctx: &Cast_width_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#range_suffix}.
     * @param ctx the parse tree
     */
    fn visit_range_suffix(&mut self, ctx: &Range_suffixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#array_suffix}.
     * @param ctx the parse tree
     */
    fn visit_array_suffix(&mut self, ctx: &Array_suffixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#array_type_suffix}.
     * @param ctx the parse tree
     */
    fn visit_array_type_suffix(&mut self, ctx: &Array_type_suffixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#data_type}.
     * @param ctx the parse tree
     */
    fn visit_data_type(&mut self, ctx: &Data_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#basic_data_type}.
     * @param ctx the parse tree
     */
    fn visit_basic_data_type(&mut self, ctx: &Basic_data_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_literal(&mut self, ctx: &LiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NumberInt}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn visit_NumberInt(&mut self, ctx: &NumberIntContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NumberHex}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn visit_NumberHex(&mut self, ctx: &NumberHexContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NumberVerilog}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn visit_NumberVerilog(&mut self, ctx: &NumberVerilogContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#string_literal}.
     * @param ctx the parse tree
     */
    fn visit_string_literal(&mut self, ctx: &String_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#boolean_literal}.
     * @param ctx the parse tree
     */
    fn visit_boolean_literal(&mut self, ctx: &Boolean_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#array_literal}.
     * @param ctx the parse tree
     */
    fn visit_array_literal(&mut self, ctx: &Array_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_literal}.
     * @param ctx the parse tree
     */
    fn visit_struct_literal(&mut self, ctx: &Struct_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_kv}.
     * @param ctx the parse tree
     */
    fn visit_struct_kv(&mut self, ctx: &Struct_kvContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_literal}.
     * @param ctx the parse tree
     */
    fn visit_enum_literal(&mut self, ctx: &Enum_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#accesstype_literal}.
     * @param ctx the parse tree
     */
    fn visit_accesstype_literal(&mut self, ctx: &Accesstype_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#onreadtype_literal}.
     * @param ctx the parse tree
     */
    fn visit_onreadtype_literal(&mut self, ctx: &Onreadtype_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#onwritetype_literal}.
     * @param ctx the parse tree
     */
    fn visit_onwritetype_literal(&mut self, ctx: &Onwritetype_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#addressingtype_literal}.
     * @param ctx the parse tree
     */
    fn visit_addressingtype_literal(&mut self, ctx: &Addressingtype_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#precedencetype_literal}.
     * @param ctx the parse tree
     */
    fn visit_precedencetype_literal(&mut self, ctx: &Precedencetype_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#instance_ref}.
     * @param ctx the parse tree
     */
    fn visit_instance_ref(&mut self, ctx: &Instance_refContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#instance_ref_element}.
     * @param ctx the parse tree
     */
    fn visit_instance_ref_element(&mut self, ctx: &Instance_ref_elementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_ref}.
     * @param ctx the parse tree
     */
    fn visit_prop_ref(&mut self, ctx: &Prop_refContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#local_property_assignment}.
     * @param ctx the parse tree
     */
    fn visit_local_property_assignment(&mut self, ctx: &Local_property_assignmentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#dynamic_property_assignment}.
     * @param ctx the parse tree
     */
    fn visit_dynamic_property_assignment(
        &mut self,
        ctx: &Dynamic_property_assignmentContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#normal_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_normal_prop_assign(&mut self, ctx: &Normal_prop_assignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#encode_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_encode_prop_assign(&mut self, ctx: &Encode_prop_assignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_mod_assign}.
     * @param ctx the parse tree
     */
    fn visit_prop_mod_assign(&mut self, ctx: &Prop_mod_assignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_assignment_rhs}.
     * @param ctx the parse tree
     */
    fn visit_prop_assignment_rhs(&mut self, ctx: &Prop_assignment_rhsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_keyword}.
     * @param ctx the parse tree
     */
    fn visit_prop_keyword(&mut self, ctx: &Prop_keywordContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_mod}.
     * @param ctx the parse tree
     */
    fn visit_prop_mod(&mut self, ctx: &Prop_modContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_def}.
     * @param ctx the parse tree
     */
    fn visit_udp_def(&mut self, ctx: &Udp_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_attr}.
     * @param ctx the parse tree
     */
    fn visit_udp_attr(&mut self, ctx: &Udp_attrContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_type}.
     * @param ctx the parse tree
     */
    fn visit_udp_type(&mut self, ctx: &Udp_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_data_type}.
     * @param ctx the parse tree
     */
    fn visit_udp_data_type(&mut self, ctx: &Udp_data_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_usage}.
     * @param ctx the parse tree
     */
    fn visit_udp_usage(&mut self, ctx: &Udp_usageContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_comp_type}.
     * @param ctx the parse tree
     */
    fn visit_udp_comp_type(&mut self, ctx: &Udp_comp_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_default}.
     * @param ctx the parse tree
     */
    fn visit_udp_default(&mut self, ctx: &Udp_defaultContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_constraint}.
     * @param ctx the parse tree
     */
    fn visit_udp_constraint(&mut self, ctx: &Udp_constraintContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_def}.
     * @param ctx the parse tree
     */
    fn visit_enum_def(&mut self, ctx: &Enum_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_entry}.
     * @param ctx the parse tree
     */
    fn visit_enum_entry(&mut self, ctx: &Enum_entryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_enum_prop_assign(&mut self, ctx: &Enum_prop_assignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_def}.
     * @param ctx the parse tree
     */
    fn visit_struct_def(&mut self, ctx: &Struct_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_elem}.
     * @param ctx the parse tree
     */
    fn visit_struct_elem(&mut self, ctx: &Struct_elemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_type}.
     * @param ctx the parse tree
     */
    fn visit_struct_type(&mut self, ctx: &Struct_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_def}.
     * @param ctx the parse tree
     */
    fn visit_constraint_def(&mut self, ctx: &Constraint_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_named_def}.
     * @param ctx the parse tree
     */
    fn visit_constraint_named_def(&mut self, ctx: &Constraint_named_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_anon_def}.
     * @param ctx the parse tree
     */
    fn visit_constraint_anon_def(&mut self, ctx: &Constraint_anon_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_body}.
     * @param ctx the parse tree
     */
    fn visit_constraint_body(&mut self, ctx: &Constraint_bodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_body_elem}.
     * @param ctx the parse tree
     */
    fn visit_constraint_body_elem(&mut self, ctx: &Constraint_body_elemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_insts}.
     * @param ctx the parse tree
     */
    fn visit_constraint_insts(&mut self, ctx: &Constraint_instsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_relational}.
     * @param ctx the parse tree
     */
    fn visit_constr_relational(&mut self, ctx: &Constr_relationalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_constr_prop_assign(&mut self, ctx: &Constr_prop_assignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_inside_values}.
     * @param ctx the parse tree
     */
    fn visit_constr_inside_values(&mut self, ctx: &Constr_inside_valuesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_inside_enum}.
     * @param ctx the parse tree
     */
    fn visit_constr_inside_enum(&mut self, ctx: &Constr_inside_enumContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_lhs}.
     * @param ctx the parse tree
     */
    fn visit_constr_lhs(&mut self, ctx: &Constr_lhsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_inside_value}.
     * @param ctx the parse tree
     */
    fn visit_constr_inside_value(&mut self, ctx: &Constr_inside_valueContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait SystemRDLVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = SystemRDLParserContextType>
{
    /**
     * Visit a parse tree produced by {@link SystemRDLParser#root}.
     * @param ctx the parse tree
     */
    fn visit_root(&mut self, ctx: &RootContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#eval_expr_root}.
     * @param ctx the parse tree
     */
    fn visit_eval_expr_root(&mut self, ctx: &Eval_expr_rootContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#root_elem}.
     * @param ctx the parse tree
     */
    fn visit_root_elem(&mut self, ctx: &Root_elemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_def}.
     * @param ctx the parse tree
     */
    fn visit_component_def(&mut self, ctx: &Component_defContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#explicit_component_inst}.
     * @param ctx the parse tree
     */
    fn visit_explicit_component_inst(
        &mut self,
        ctx: &Explicit_component_instContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_inst_alias}.
     * @param ctx the parse tree
     */
    fn visit_component_inst_alias(
        &mut self,
        ctx: &Component_inst_aliasContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_named_def}.
     * @param ctx the parse tree
     */
    fn visit_component_named_def(
        &mut self,
        ctx: &Component_named_defContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_anon_def}.
     * @param ctx the parse tree
     */
    fn visit_component_anon_def(
        &mut self,
        ctx: &Component_anon_defContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_body}.
     * @param ctx the parse tree
     */
    fn visit_component_body(&mut self, ctx: &Component_bodyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_body_elem}.
     * @param ctx the parse tree
     */
    fn visit_component_body_elem(
        &mut self,
        ctx: &Component_body_elemContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_insts}.
     * @param ctx the parse tree
     */
    fn visit_component_insts(&mut self, ctx: &Component_instsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_inst}.
     * @param ctx the parse tree
     */
    fn visit_component_inst(&mut self, ctx: &Component_instContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#field_inst_reset}.
     * @param ctx the parse tree
     */
    fn visit_field_inst_reset(&mut self, ctx: &Field_inst_resetContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#inst_addr_fixed}.
     * @param ctx the parse tree
     */
    fn visit_inst_addr_fixed(&mut self, ctx: &Inst_addr_fixedContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#inst_addr_stride}.
     * @param ctx the parse tree
     */
    fn visit_inst_addr_stride(&mut self, ctx: &Inst_addr_strideContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#inst_addr_align}.
     * @param ctx the parse tree
     */
    fn visit_inst_addr_align(&mut self, ctx: &Inst_addr_alignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_inst_type}.
     * @param ctx the parse tree
     */
    fn visit_component_inst_type(
        &mut self,
        ctx: &Component_inst_typeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_type}.
     * @param ctx the parse tree
     */
    fn visit_component_type(&mut self, ctx: &Component_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#component_type_primary}.
     * @param ctx the parse tree
     */
    fn visit_component_type_primary(
        &mut self,
        ctx: &Component_type_primaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_def}.
     * @param ctx the parse tree
     */
    fn visit_param_def(&mut self, ctx: &Param_defContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_def_elem}.
     * @param ctx the parse tree
     */
    fn visit_param_def_elem(&mut self, ctx: &Param_def_elemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_inst}.
     * @param ctx the parse tree
     */
    fn visit_param_inst(&mut self, ctx: &Param_instContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#param_assignment}.
     * @param ctx the parse tree
     */
    fn visit_param_assignment(&mut self, ctx: &Param_assignmentContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code BinaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_BinaryExpr(&mut self, ctx: &BinaryExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code UnaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_UnaryExpr(&mut self, ctx: &UnaryExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NOP}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_NOP(&mut self, ctx: &NOPContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TernaryExpr}
     * labeled alternative in {@link SystemRDLParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_TernaryExpr(&mut self, ctx: &TernaryExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#expr_primary}.
     * @param ctx the parse tree
     */
    fn visit_expr_primary(&mut self, ctx: &Expr_primaryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#concatenate}.
     * @param ctx the parse tree
     */
    fn visit_concatenate(&mut self, ctx: &ConcatenateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#replicate}.
     * @param ctx the parse tree
     */
    fn visit_replicate(&mut self, ctx: &ReplicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#paren_expr}.
     * @param ctx the parse tree
     */
    fn visit_paren_expr(&mut self, ctx: &Paren_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CastType}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn visit_CastType(&mut self, ctx: &CastTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CastWidth}
     * labeled alternative in {@link SystemRDLParser#cast}.
     * @param ctx the parse tree
     */
    fn visit_CastWidth(&mut self, ctx: &CastWidthContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#cast_width_expr}.
     * @param ctx the parse tree
     */
    fn visit_cast_width_expr(&mut self, ctx: &Cast_width_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#range_suffix}.
     * @param ctx the parse tree
     */
    fn visit_range_suffix(&mut self, ctx: &Range_suffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#array_suffix}.
     * @param ctx the parse tree
     */
    fn visit_array_suffix(&mut self, ctx: &Array_suffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#array_type_suffix}.
     * @param ctx the parse tree
     */
    fn visit_array_type_suffix(&mut self, ctx: &Array_type_suffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#data_type}.
     * @param ctx the parse tree
     */
    fn visit_data_type(&mut self, ctx: &Data_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#basic_data_type}.
     * @param ctx the parse tree
     */
    fn visit_basic_data_type(&mut self, ctx: &Basic_data_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NumberInt}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn visit_NumberInt(&mut self, ctx: &NumberIntContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NumberHex}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn visit_NumberHex(&mut self, ctx: &NumberHexContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NumberVerilog}
     * labeled alternative in {@link SystemRDLParser#number}.
     * @param ctx the parse tree
     */
    fn visit_NumberVerilog(&mut self, ctx: &NumberVerilogContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#string_literal}.
     * @param ctx the parse tree
     */
    fn visit_string_literal(&mut self, ctx: &String_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#boolean_literal}.
     * @param ctx the parse tree
     */
    fn visit_boolean_literal(&mut self, ctx: &Boolean_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#array_literal}.
     * @param ctx the parse tree
     */
    fn visit_array_literal(&mut self, ctx: &Array_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_literal}.
     * @param ctx the parse tree
     */
    fn visit_struct_literal(&mut self, ctx: &Struct_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_kv}.
     * @param ctx the parse tree
     */
    fn visit_struct_kv(&mut self, ctx: &Struct_kvContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_literal}.
     * @param ctx the parse tree
     */
    fn visit_enum_literal(&mut self, ctx: &Enum_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#accesstype_literal}.
     * @param ctx the parse tree
     */
    fn visit_accesstype_literal(
        &mut self,
        ctx: &Accesstype_literalContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#onreadtype_literal}.
     * @param ctx the parse tree
     */
    fn visit_onreadtype_literal(
        &mut self,
        ctx: &Onreadtype_literalContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#onwritetype_literal}.
     * @param ctx the parse tree
     */
    fn visit_onwritetype_literal(
        &mut self,
        ctx: &Onwritetype_literalContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#addressingtype_literal}.
     * @param ctx the parse tree
     */
    fn visit_addressingtype_literal(
        &mut self,
        ctx: &Addressingtype_literalContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#precedencetype_literal}.
     * @param ctx the parse tree
     */
    fn visit_precedencetype_literal(
        &mut self,
        ctx: &Precedencetype_literalContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#instance_ref}.
     * @param ctx the parse tree
     */
    fn visit_instance_ref(&mut self, ctx: &Instance_refContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#instance_ref_element}.
     * @param ctx the parse tree
     */
    fn visit_instance_ref_element(
        &mut self,
        ctx: &Instance_ref_elementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_ref}.
     * @param ctx the parse tree
     */
    fn visit_prop_ref(&mut self, ctx: &Prop_refContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#local_property_assignment}.
     * @param ctx the parse tree
     */
    fn visit_local_property_assignment(
        &mut self,
        ctx: &Local_property_assignmentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#dynamic_property_assignment}.
     * @param ctx the parse tree
     */
    fn visit_dynamic_property_assignment(
        &mut self,
        ctx: &Dynamic_property_assignmentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#normal_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_normal_prop_assign(
        &mut self,
        ctx: &Normal_prop_assignContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#encode_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_encode_prop_assign(
        &mut self,
        ctx: &Encode_prop_assignContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_mod_assign}.
     * @param ctx the parse tree
     */
    fn visit_prop_mod_assign(&mut self, ctx: &Prop_mod_assignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_assignment_rhs}.
     * @param ctx the parse tree
     */
    fn visit_prop_assignment_rhs(
        &mut self,
        ctx: &Prop_assignment_rhsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_keyword}.
     * @param ctx the parse tree
     */
    fn visit_prop_keyword(&mut self, ctx: &Prop_keywordContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#prop_mod}.
     * @param ctx the parse tree
     */
    fn visit_prop_mod(&mut self, ctx: &Prop_modContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_def}.
     * @param ctx the parse tree
     */
    fn visit_udp_def(&mut self, ctx: &Udp_defContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_attr}.
     * @param ctx the parse tree
     */
    fn visit_udp_attr(&mut self, ctx: &Udp_attrContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_type}.
     * @param ctx the parse tree
     */
    fn visit_udp_type(&mut self, ctx: &Udp_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_data_type}.
     * @param ctx the parse tree
     */
    fn visit_udp_data_type(&mut self, ctx: &Udp_data_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_usage}.
     * @param ctx the parse tree
     */
    fn visit_udp_usage(&mut self, ctx: &Udp_usageContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_comp_type}.
     * @param ctx the parse tree
     */
    fn visit_udp_comp_type(&mut self, ctx: &Udp_comp_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_default}.
     * @param ctx the parse tree
     */
    fn visit_udp_default(&mut self, ctx: &Udp_defaultContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#udp_constraint}.
     * @param ctx the parse tree
     */
    fn visit_udp_constraint(&mut self, ctx: &Udp_constraintContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_def}.
     * @param ctx the parse tree
     */
    fn visit_enum_def(&mut self, ctx: &Enum_defContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_entry}.
     * @param ctx the parse tree
     */
    fn visit_enum_entry(&mut self, ctx: &Enum_entryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#enum_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_enum_prop_assign(&mut self, ctx: &Enum_prop_assignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_def}.
     * @param ctx the parse tree
     */
    fn visit_struct_def(&mut self, ctx: &Struct_defContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_elem}.
     * @param ctx the parse tree
     */
    fn visit_struct_elem(&mut self, ctx: &Struct_elemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#struct_type}.
     * @param ctx the parse tree
     */
    fn visit_struct_type(&mut self, ctx: &Struct_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_def}.
     * @param ctx the parse tree
     */
    fn visit_constraint_def(&mut self, ctx: &Constraint_defContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_named_def}.
     * @param ctx the parse tree
     */
    fn visit_constraint_named_def(
        &mut self,
        ctx: &Constraint_named_defContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_anon_def}.
     * @param ctx the parse tree
     */
    fn visit_constraint_anon_def(
        &mut self,
        ctx: &Constraint_anon_defContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_body}.
     * @param ctx the parse tree
     */
    fn visit_constraint_body(&mut self, ctx: &Constraint_bodyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_body_elem}.
     * @param ctx the parse tree
     */
    fn visit_constraint_body_elem(
        &mut self,
        ctx: &Constraint_body_elemContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constraint_insts}.
     * @param ctx the parse tree
     */
    fn visit_constraint_insts(&mut self, ctx: &Constraint_instsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_relational}.
     * @param ctx the parse tree
     */
    fn visit_constr_relational(&mut self, ctx: &Constr_relationalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_prop_assign}.
     * @param ctx the parse tree
     */
    fn visit_constr_prop_assign(
        &mut self,
        ctx: &Constr_prop_assignContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_inside_values}.
     * @param ctx the parse tree
     */
    fn visit_constr_inside_values(
        &mut self,
        ctx: &Constr_inside_valuesContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_inside_enum}.
     * @param ctx the parse tree
     */
    fn visit_constr_inside_enum(
        &mut self,
        ctx: &Constr_inside_enumContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_lhs}.
     * @param ctx the parse tree
     */
    fn visit_constr_lhs(&mut self, ctx: &Constr_lhsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SystemRDLParser#constr_inside_value}.
     * @param ctx the parse tree
     */
    fn visit_constr_inside_value(
        &mut self,
        ctx: &Constr_inside_valueContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> SystemRDLVisitor<'input> for T
where
    T: SystemRDLVisitorCompat<'input>,
{
    fn visit_root(&mut self, ctx: &RootContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_root(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_eval_expr_root(&mut self, ctx: &Eval_expr_rootContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_eval_expr_root(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_root_elem(&mut self, ctx: &Root_elemContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_root_elem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_def(&mut self, ctx: &Component_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_explicit_component_inst(&mut self, ctx: &Explicit_component_instContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_explicit_component_inst(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_inst_alias(&mut self, ctx: &Component_inst_aliasContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_inst_alias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_named_def(&mut self, ctx: &Component_named_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_named_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_anon_def(&mut self, ctx: &Component_anon_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_anon_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_body(&mut self, ctx: &Component_bodyContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_body(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_body_elem(&mut self, ctx: &Component_body_elemContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_body_elem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_insts(&mut self, ctx: &Component_instsContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_insts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_inst(&mut self, ctx: &Component_instContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_inst(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_field_inst_reset(&mut self, ctx: &Field_inst_resetContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_field_inst_reset(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_inst_addr_fixed(&mut self, ctx: &Inst_addr_fixedContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_inst_addr_fixed(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_inst_addr_stride(&mut self, ctx: &Inst_addr_strideContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_inst_addr_stride(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_inst_addr_align(&mut self, ctx: &Inst_addr_alignContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_inst_addr_align(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_inst_type(&mut self, ctx: &Component_inst_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_inst_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_type(&mut self, ctx: &Component_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_component_type_primary(&mut self, ctx: &Component_type_primaryContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_component_type_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_param_def(&mut self, ctx: &Param_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_param_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_param_def_elem(&mut self, ctx: &Param_def_elemContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_param_def_elem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_param_inst(&mut self, ctx: &Param_instContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_param_inst(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_param_assignment(&mut self, ctx: &Param_assignmentContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_param_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_BinaryExpr(&mut self, ctx: &BinaryExprContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_BinaryExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UnaryExpr(&mut self, ctx: &UnaryExprContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_UnaryExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NOP(&mut self, ctx: &NOPContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_NOP(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TernaryExpr(&mut self, ctx: &TernaryExprContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_TernaryExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expr_primary(&mut self, ctx: &Expr_primaryContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_expr_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_concatenate(&mut self, ctx: &ConcatenateContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_concatenate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_replicate(&mut self, ctx: &ReplicateContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_replicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_paren_expr(&mut self, ctx: &Paren_exprContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_paren_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CastType(&mut self, ctx: &CastTypeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_CastType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CastWidth(&mut self, ctx: &CastWidthContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_CastWidth(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_cast_width_expr(&mut self, ctx: &Cast_width_exprContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_cast_width_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_range_suffix(&mut self, ctx: &Range_suffixContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_range_suffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_array_suffix(&mut self, ctx: &Array_suffixContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_array_suffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_array_type_suffix(&mut self, ctx: &Array_type_suffixContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_array_type_suffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_data_type(&mut self, ctx: &Data_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_data_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_basic_data_type(&mut self, ctx: &Basic_data_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_basic_data_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_literal(&mut self, ctx: &LiteralContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NumberInt(&mut self, ctx: &NumberIntContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_NumberInt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NumberHex(&mut self, ctx: &NumberHexContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_NumberHex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NumberVerilog(&mut self, ctx: &NumberVerilogContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_NumberVerilog(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_string_literal(&mut self, ctx: &String_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_string_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_boolean_literal(&mut self, ctx: &Boolean_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_boolean_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_array_literal(&mut self, ctx: &Array_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_array_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_struct_literal(&mut self, ctx: &Struct_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_struct_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_struct_kv(&mut self, ctx: &Struct_kvContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_struct_kv(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_enum_literal(&mut self, ctx: &Enum_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_enum_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_accesstype_literal(&mut self, ctx: &Accesstype_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_accesstype_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_onreadtype_literal(&mut self, ctx: &Onreadtype_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_onreadtype_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_onwritetype_literal(&mut self, ctx: &Onwritetype_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_onwritetype_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_addressingtype_literal(&mut self, ctx: &Addressingtype_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_addressingtype_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_precedencetype_literal(&mut self, ctx: &Precedencetype_literalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_precedencetype_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_instance_ref(&mut self, ctx: &Instance_refContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_instance_ref(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_instance_ref_element(&mut self, ctx: &Instance_ref_elementContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_instance_ref_element(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_prop_ref(&mut self, ctx: &Prop_refContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_prop_ref(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_local_property_assignment(&mut self, ctx: &Local_property_assignmentContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_local_property_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dynamic_property_assignment(
        &mut self,
        ctx: &Dynamic_property_assignmentContext<'input>,
    ) {
        let result = <Self as SystemRDLVisitorCompat>::visit_dynamic_property_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_normal_prop_assign(&mut self, ctx: &Normal_prop_assignContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_normal_prop_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_encode_prop_assign(&mut self, ctx: &Encode_prop_assignContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_encode_prop_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_prop_mod_assign(&mut self, ctx: &Prop_mod_assignContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_prop_mod_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_prop_assignment_rhs(&mut self, ctx: &Prop_assignment_rhsContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_prop_assignment_rhs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_prop_keyword(&mut self, ctx: &Prop_keywordContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_prop_keyword(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_prop_mod(&mut self, ctx: &Prop_modContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_prop_mod(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_def(&mut self, ctx: &Udp_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_attr(&mut self, ctx: &Udp_attrContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_attr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_type(&mut self, ctx: &Udp_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_data_type(&mut self, ctx: &Udp_data_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_data_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_usage(&mut self, ctx: &Udp_usageContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_usage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_comp_type(&mut self, ctx: &Udp_comp_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_comp_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_default(&mut self, ctx: &Udp_defaultContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_udp_constraint(&mut self, ctx: &Udp_constraintContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_udp_constraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_enum_def(&mut self, ctx: &Enum_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_enum_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_enum_entry(&mut self, ctx: &Enum_entryContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_enum_entry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_enum_prop_assign(&mut self, ctx: &Enum_prop_assignContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_enum_prop_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_struct_def(&mut self, ctx: &Struct_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_struct_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_struct_elem(&mut self, ctx: &Struct_elemContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_struct_elem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_struct_type(&mut self, ctx: &Struct_typeContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_struct_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constraint_def(&mut self, ctx: &Constraint_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constraint_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constraint_named_def(&mut self, ctx: &Constraint_named_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constraint_named_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constraint_anon_def(&mut self, ctx: &Constraint_anon_defContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constraint_anon_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constraint_body(&mut self, ctx: &Constraint_bodyContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constraint_body(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constraint_body_elem(&mut self, ctx: &Constraint_body_elemContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constraint_body_elem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constraint_insts(&mut self, ctx: &Constraint_instsContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constraint_insts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constr_relational(&mut self, ctx: &Constr_relationalContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constr_relational(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constr_prop_assign(&mut self, ctx: &Constr_prop_assignContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constr_prop_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constr_inside_values(&mut self, ctx: &Constr_inside_valuesContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constr_inside_values(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constr_inside_enum(&mut self, ctx: &Constr_inside_enumContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constr_inside_enum(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constr_lhs(&mut self, ctx: &Constr_lhsContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constr_lhs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constr_inside_value(&mut self, ctx: &Constr_inside_valueContext<'input>) {
        let result = <Self as SystemRDLVisitorCompat>::visit_constr_inside_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
