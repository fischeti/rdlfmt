//! The grammar rules, following `SystemRDL.g4` rule for rule.
//!
//! See the module docs in [`super`] for the trivia convention and the places
//! where this deliberately diverges from the reference grammar.

use super::Parser;
use crate::kind::SyntaxKind::{self, *};

//--------------------------------------------------------------------------
// Token sets
//--------------------------------------------------------------------------

const COMPONENT_TYPE_KW: &[SyntaxKind] =
    &[ADDRMAP_KW, REGFILE_KW, REG_KW, FIELD_KW, MEM_KW, SIGNAL_KW];
/// `component_type_primary`: everything above except `signal`.
const COMPONENT_TYPE_PRIMARY_KW: &[SyntaxKind] =
    &[ADDRMAP_KW, REGFILE_KW, REG_KW, FIELD_KW, MEM_KW];
const COMPONENT_INST_TYPE_KW: &[SyntaxKind] = &[EXTERNAL_KW, INTERNAL_KW];
const PROP_MOD_KW: &[SyntaxKind] = &[POSEDGE_KW, NEGEDGE_KW, BOTHEDGE_KW, LEVEL_KW, NONSTICKY_KW];
const PROP_KEYWORD_KW: &[SyntaxKind] = &[SW_KW, HW_KW, RCLR_KW, RSET_KW, WOCLR_KW, WOSET_KW];
const DATA_TYPE_KW: &[SyntaxKind] = &[
    ACCESSTYPE_KW,
    ADDRESSINGTYPE_KW,
    ONREADTYPE_KW,
    ONWRITETYPE_KW,
];
const NUMBER_TOKEN: &[SyntaxKind] = &[INT_NUMBER, HEX_NUMBER, VLOG_NUMBER];
const RELATIONAL_OP: &[SyntaxKind] = &[LT, LEQ, GT, GEQ, EQ, NEQ];
/// Reduction operators double as binary operators; position decides which.
const UNARY_OP: &[SyntaxKind] = &[PLUS, MINUS, BNOT, NOT, AND, NAND, OR, NOR, XOR, XNOR];

/// Keywords that stand for a value rather than a name: boolean, access type,
/// on-read/on-write type, addressing type and precedence type literals.
const LITERAL_KW: &[SyntaxKind] = &[
    TRUE_KW,
    FALSE_KW,
    NA_KW,
    RW_KW,
    WR_KW,
    R_KW,
    W_KW,
    RW1_KW,
    W1_KW,
    RCLR_KW,
    RSET_KW,
    RUSER_KW,
    WOSET_KW,
    WOCLR_KW,
    WOT_KW,
    WZS_KW,
    WZC_KW,
    WZT_KW,
    WCLR_KW,
    WSET_KW,
    WUSER_KW,
    COMPACT_KW,
    REGALIGN_KW,
    FULLALIGN_KW,
    HW_KW,
    SW_KW,
];

//--------------------------------------------------------------------------
// Entry point
//--------------------------------------------------------------------------

pub(super) fn source_file(p: &mut Parser) {
    p.start_node(SOURCE_FILE);
    item_list(p, EOF);
    // Trailing whitespace and comments after the last item. These must land
    // inside SOURCE_FILE -- a rowan tree has exactly one root.
    p.flush_trivia();
    p.finish_node();
}

/// Parses items until `stop` (or end of input), guaranteeing forward progress.
fn item_list(p: &mut Parser, stop: SyntaxKind) {
    while !p.at(stop) && !p.at_end() {
        let before = p.pos;
        item(p);
        if p.pos == before {
            // No rule consumed anything; force progress so this cannot hang.
            p.error_and_bump(format!("unexpected {:?}", p.current()));
        }
    }
}

//--------------------------------------------------------------------------
// Items
//--------------------------------------------------------------------------

fn item(p: &mut Parser) {
    match p.current() {
        ENUM_KW => enum_def(p),
        PROPERTY_KW => udp_def(p),
        ABSTRACT_KW | STRUCT_KW => struct_def(p),
        CONSTRAINT_KW => constraint_def(p),
        DEFAULT_KW | ENCODE_KW => local_property_assignment(p),
        ALIAS_KW => explicit_component_inst(p),
        // A stray terminator; keep it rather than erroring so the tree still
        // round-trips.
        SEMICOLON => p.bump(),
        k if PROP_MOD_KW.contains(&k) => local_property_assignment(p),
        _ if at_component_def(p) => component_def(p),
        EXTERNAL_KW | INTERNAL_KW => explicit_component_inst(p),
        k if k.is_ident_like() => ident_item(p),
        k => p.error_and_bump(format!("expected a declaration, found {k:?}")),
    }
}

fn at_component_def(p: &Parser) -> bool {
    if COMPONENT_TYPE_KW.contains(&p.current()) {
        return true;
    }
    // `external reg foo {...} bar;` is a definition; `external my_reg bar;` is
    // an instantiation. The token after the modifier decides.
    COMPONENT_INST_TYPE_KW.contains(&p.current()) && COMPONENT_TYPE_KW.contains(&p.nth(1))
}

/// Disambiguates the three item forms that begin with a name.
///
/// After scanning the `instance_ref` prefix (`a.b[0].c`), the next token tells
/// them apart: `->` means a dynamic property assignment, `=` or `;` directly
/// after a bare name means a local property assignment, and anything else --
/// notably a second name -- means an instantiation.
fn ident_item(p: &mut Parser) {
    let mut n = 1;
    let mut bare = true;
    loop {
        match p.nth(n) {
            DOT => {
                bare = false;
                n += 2; // `.` and the name after it
            }
            L_BRACK => {
                bare = false;
                n += 1;
                let mut depth = 1usize;
                while depth > 0 {
                    match p.nth(n) {
                        L_BRACK => depth += 1,
                        R_BRACK => depth -= 1,
                        EOF => return explicit_component_inst(p),
                        _ => {}
                    }
                    n += 1;
                }
            }
            _ => break,
        }
    }
    match p.nth(n) {
        ARROW => dynamic_property_assignment(p),
        ASSIGN | SEMICOLON if bare => local_property_assignment(p),
        _ => explicit_component_inst(p),
    }
}

//--------------------------------------------------------------------------
// Components
//--------------------------------------------------------------------------

/// The reference grammar spells this as four alternatives; they collapse to one
/// linear form. See the divergence note in [`super`].
fn component_def(p: &mut Parser) {
    p.start_node(COMPONENT_DEF);
    if p.at_any(COMPONENT_INST_TYPE_KW) {
        component_inst_type(p);
    }

    let cp = p.checkpoint();
    component_type(p);
    let named = p.current().is_ident_like();
    if named {
        p.bump();
        if p.at(HASH) {
            param_def(p);
        }
    }
    component_body(p);
    p.start_node_at(
        cp,
        if named {
            COMPONENT_NAMED_DEF
        } else {
            COMPONENT_ANON_DEF
        },
    );
    p.finish_node();

    if p.at_any(COMPONENT_INST_TYPE_KW) {
        component_inst_type(p);
    }
    if !p.at(SEMICOLON) && !p.at_end() {
        component_insts(p);
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn component_type(p: &mut Parser) {
    p.start_node(COMPONENT_TYPE);
    if p.at_any(COMPONENT_TYPE_KW) {
        p.bump();
    } else {
        p.error(format!(
            "expected a component type, found {:?}",
            p.current()
        ));
    }
    p.finish_node();
}

fn component_inst_type(p: &mut Parser) {
    p.start_node(COMPONENT_INST_TYPE);
    p.bump();
    p.finish_node();
}

fn component_body(p: &mut Parser) {
    p.start_node(COMPONENT_BODY);
    p.expect(L_BRACE);
    item_list(p, R_BRACE);
    p.expect(R_BRACE);
    p.finish_node();
}

fn explicit_component_inst(p: &mut Parser) {
    p.start_node(EXPLICIT_COMPONENT_INST);
    if p.at_any(COMPONENT_INST_TYPE_KW) {
        component_inst_type(p);
    }
    if p.at(ALIAS_KW) {
        p.start_node(COMPONENT_INST_ALIAS);
        p.bump();
        expect_name(p);
        p.finish_node();
    }
    expect_name(p); // the type being instantiated
    component_insts(p);
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn component_insts(p: &mut Parser) {
    p.start_node(COMPONENT_INSTS);
    if p.at(HASH) {
        param_inst(p);
    }
    component_inst(p);
    while p.eat(COMMA) {
        component_inst(p);
    }
    p.finish_node();
}

fn component_inst(p: &mut Parser) {
    p.start_node(COMPONENT_INST);
    expect_name(p);
    if p.at(L_BRACK) {
        if at_range_suffix(p) {
            range_suffix(p);
        } else {
            while p.at(L_BRACK) {
                array_suffix(p);
            }
        }
    }
    for (op, node) in [
        (ASSIGN, FIELD_INST_RESET),
        (AT, INST_ADDR_FIXED),
        (INC, INST_ADDR_STRIDE),
        (ALIGN, INST_ADDR_ALIGN),
    ] {
        if p.at(op) {
            p.start_node(node);
            p.bump();
            expr(p);
            p.finish_node();
        }
    }
    p.finish_node();
}

//--------------------------------------------------------------------------
// Parameters
//--------------------------------------------------------------------------

fn param_def(p: &mut Parser) {
    p.start_node(PARAM_DEF);
    p.bump(); // #
    p.expect(L_PAREN);
    param_def_elem(p);
    while p.eat(COMMA) {
        param_def_elem(p);
    }
    p.expect(R_PAREN);
    p.finish_node();
}

fn param_def_elem(p: &mut Parser) {
    p.start_node(PARAM_DEF_ELEM);
    data_type(p);
    expect_name(p);
    if p.at(L_BRACK) {
        array_type_suffix(p);
    }
    if p.eat(ASSIGN) {
        expr(p);
    }
    p.finish_node();
}

fn param_inst(p: &mut Parser) {
    p.start_node(PARAM_INST);
    p.bump(); // #
    p.expect(L_PAREN);
    param_assignment(p);
    while p.eat(COMMA) {
        param_assignment(p);
    }
    p.expect(R_PAREN);
    p.finish_node();
}

fn param_assignment(p: &mut Parser) {
    p.start_node(PARAM_ASSIGNMENT);
    p.expect(DOT);
    expect_name(p);
    p.expect(L_PAREN);
    expr(p);
    p.expect(R_PAREN);
    p.finish_node();
}

//--------------------------------------------------------------------------
// Property assignments
//--------------------------------------------------------------------------

fn local_property_assignment(p: &mut Parser) {
    p.start_node(LOCAL_PROPERTY_ASSIGNMENT);
    p.eat(DEFAULT_KW);
    match p.current() {
        ENCODE_KW => encode_prop_assign(p),
        k if PROP_MOD_KW.contains(&k) => {
            p.start_node(PROP_MOD_ASSIGN);
            p.start_node(PROP_MOD);
            p.bump();
            p.finish_node();
            expect_name(p);
            p.finish_node();
        }
        _ => normal_prop_assign(p),
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn dynamic_property_assignment(p: &mut Parser) {
    p.start_node(DYNAMIC_PROPERTY_ASSIGNMENT);
    instance_ref(p);
    p.expect(ARROW);
    if p.at(ENCODE_KW) {
        encode_prop_assign(p);
    } else {
        normal_prop_assign(p);
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn normal_prop_assign(p: &mut Parser) {
    p.start_node(NORMAL_PROP_ASSIGN);
    if p.at_any(PROP_KEYWORD_KW) {
        p.start_node(PROP_KEYWORD);
        p.bump();
        p.finish_node();
    } else {
        expect_name(p);
    }
    // The right-hand side is optional: `donttest;` sets a boolean property.
    if p.eat(ASSIGN) {
        expr(p);
    }
    p.finish_node();
}

fn encode_prop_assign(p: &mut Parser) {
    p.start_node(ENCODE_PROP_ASSIGN);
    p.bump(); // encode
    p.expect(ASSIGN);
    expect_name(p);
    p.finish_node();
}

//--------------------------------------------------------------------------
// User-defined properties
//--------------------------------------------------------------------------

fn udp_def(p: &mut Parser) {
    p.start_node(UDP_DEF);
    p.bump(); // property
    expect_name(p);
    p.start_node(UDP_BODY);
    p.expect(L_BRACE);
    while !p.at(R_BRACE) && !p.at_end() {
        let before = p.pos;
        udp_attr(p);
        if p.pos == before {
            p.error_and_bump(format!(
                "expected a property attribute, found {:?}",
                p.current()
            ));
        }
    }
    p.expect(R_BRACE);
    p.finish_node();
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn udp_attr(p: &mut Parser) {
    match p.current() {
        TYPE_KW => {
            p.start_node(UDP_TYPE);
            p.bump();
            p.expect(ASSIGN);
            udp_data_type(p);
            if p.at(L_BRACK) {
                array_type_suffix(p);
            }
        }
        COMPONENT_KW => {
            p.start_node(UDP_USAGE);
            p.bump();
            p.expect(ASSIGN);
            udp_comp_type(p);
            while p.eat(OR) {
                udp_comp_type(p);
            }
        }
        DEFAULT_KW => {
            p.start_node(UDP_DEFAULT);
            p.bump();
            p.expect(ASSIGN);
            expr(p);
        }
        CONSTRAINT_KW => {
            p.start_node(UDP_CONSTRAINT);
            p.bump();
            p.expect(ASSIGN);
            p.expect(COMPONENTWIDTH_KW);
        }
        k => {
            p.error_and_bump(format!("expected a property attribute, found {k:?}"));
            return;
        }
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn udp_data_type(p: &mut Parser) {
    p.start_node(UDP_DATA_TYPE);
    if p.at_any(COMPONENT_TYPE_PRIMARY_KW) || p.at_any(&[REF_KW, NUMBER_KW]) {
        p.bump();
    } else {
        basic_data_type(p);
    }
    p.finish_node();
}

fn udp_comp_type(p: &mut Parser) {
    p.start_node(UDP_COMP_TYPE);
    match p.current() {
        CONSTRAINT_KW | ALL_KW => p.bump(),
        _ => component_type(p),
    }
    p.finish_node();
}

//--------------------------------------------------------------------------
// Enums, structs, constraints
//--------------------------------------------------------------------------

fn enum_def(p: &mut Parser) {
    p.start_node(ENUM_DEF);
    p.bump(); // enum
    expect_name(p);
    p.start_node(ENUM_BODY);
    p.expect(L_BRACE);
    while !p.at(R_BRACE) && !p.at_end() {
        let before = p.pos;
        enum_entry(p);
        if p.pos == before {
            p.error_and_bump(format!("expected an enum entry, found {:?}", p.current()));
        }
    }
    p.expect(R_BRACE);
    p.finish_node();
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn enum_entry(p: &mut Parser) {
    p.start_node(ENUM_ENTRY);
    expect_name(p);
    if p.eat(ASSIGN) {
        expr(p);
    }
    if p.at(L_BRACE) {
        p.start_node(ENUM_ENTRY_BODY);
        p.bump();
        while !p.at(R_BRACE) && !p.at_end() {
            let before = p.pos;
            p.start_node(ENUM_PROP_ASSIGN);
            expect_name(p);
            p.expect(ASSIGN);
            expr(p);
            p.expect(SEMICOLON);
            p.finish_stmt();
            if p.pos == before {
                p.error_and_bump(format!("expected a property, found {:?}", p.current()));
            }
        }
        p.expect(R_BRACE);
        p.finish_node();
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn struct_def(p: &mut Parser) {
    p.start_node(STRUCT_DEF);
    p.eat(ABSTRACT_KW);
    p.expect(STRUCT_KW);
    expect_name(p);
    if p.eat(COLON) {
        expect_name(p);
    }
    p.start_node(STRUCT_BODY);
    p.expect(L_BRACE);
    while !p.at(R_BRACE) && !p.at_end() {
        let before = p.pos;
        struct_elem(p);
        if p.pos == before {
            p.error_and_bump(format!("expected a struct member, found {:?}", p.current()));
        }
    }
    p.expect(R_BRACE);
    p.finish_node();
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn struct_elem(p: &mut Parser) {
    p.start_node(STRUCT_ELEM);
    if p.at_any(COMPONENT_TYPE_KW) {
        component_type(p);
    } else {
        data_type(p);
    }
    expect_name(p);
    if p.at(L_BRACK) {
        array_type_suffix(p);
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn constraint_def(p: &mut Parser) {
    p.start_node(CONSTRAINT_DEF);
    let cp = p.checkpoint();
    p.bump(); // constraint
    let named = p.current().is_ident_like();
    if named {
        p.bump();
    }
    constraint_body(p);
    p.start_node_at(
        cp,
        if named {
            CONSTRAINT_NAMED_DEF
        } else {
            CONSTRAINT_ANON_DEF
        },
    );
    p.finish_node();

    if p.current().is_ident_like() {
        p.start_node(CONSTRAINT_INSTS);
        p.bump();
        while p.eat(COMMA) {
            expect_name(p);
        }
        p.finish_node();
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn constraint_body(p: &mut Parser) {
    p.start_node(CONSTRAINT_BODY);
    p.expect(L_BRACE);
    while !p.at(R_BRACE) && !p.at_end() {
        let before = p.pos;
        constraint_body_elem(p);
        if p.pos == before {
            p.error_and_bump(format!("expected a constraint, found {:?}", p.current()));
        }
    }
    p.expect(R_BRACE);
    p.finish_node();
}

fn constraint_body_elem(p: &mut Parser) {
    if p.current().is_ident_like() && p.nth(1) == ASSIGN {
        p.start_node(CONSTR_PROP_ASSIGN);
        expect_name(p);
        p.bump();
        expr(p);
        p.expect(SEMICOLON);
        p.finish_stmt();
        return;
    }

    let cp = p.checkpoint();
    if p.at(THIS_KW) {
        p.start_node(CONSTR_LHS);
        p.bump();
        p.finish_node();
    } else {
        expr(p);
    }

    match p.current() {
        INSIDE_KW if p.nth(1) == L_BRACE => {
            p.start_node_at(cp, CONSTR_INSIDE_VALUES);
            p.bump(); // inside
            p.expect(L_BRACE);
            constr_inside_value(p);
            while p.eat(COMMA) {
                constr_inside_value(p);
            }
            p.expect(R_BRACE);
        }
        INSIDE_KW => {
            p.start_node_at(cp, CONSTR_INSIDE_ENUM);
            p.bump();
            expect_name(p);
        }
        // Only reachable via the `this` form, since `expr` does not accept it.
        k if RELATIONAL_OP.contains(&k) => {
            p.start_node_at(cp, CONSTR_RELATIONAL);
            p.bump();
            expr(p);
        }
        // `constr_relational` is spelled `expr op expr` in the grammar, but the
        // expression grammar already contains the relational operators, so the
        // call above consumed the whole comparison. Wrap what we have rather
        // than demanding an operator that is now nested inside it.
        _ => p.start_node_at(cp, CONSTR_RELATIONAL),
    }
    p.expect(SEMICOLON);
    p.finish_stmt();
}

fn constr_inside_value(p: &mut Parser) {
    p.start_node(CONSTR_INSIDE_VALUE);
    if p.eat(L_BRACK) {
        expr(p);
        p.expect(COLON);
        expr(p);
        p.expect(R_BRACK);
    } else {
        expr(p);
    }
    p.finish_node();
}

//--------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------

fn data_type(p: &mut Parser) {
    p.start_node(DATA_TYPE);
    if p.at_any(DATA_TYPE_KW) {
        p.bump();
    } else {
        basic_data_type(p);
    }
    p.finish_node();
}

fn basic_data_type(p: &mut Parser) {
    p.start_node(BASIC_DATA_TYPE);
    match p.current() {
        BIT_KW | LONGINT_KW => {
            p.bump();
            p.eat(UNSIGNED_KW);
        }
        // Spelled out to mirror the grammar. Both are keywords, so the arm
        // below would accept them anyway.
        STRING_KW | BOOLEAN_KW => p.bump(),
        // A bare identifier names a user-defined struct or enum type.
        k if k.is_ident_like() => p.bump(),
        k => p.error(format!("expected a data type, found {k:?}")),
    }
    p.finish_node();
}

//--------------------------------------------------------------------------
// Array and range suffixes
//--------------------------------------------------------------------------

/// Distinguishes `[7:0]` from `[4]`.
///
/// The colon of a ternary conditional would otherwise be mistaken for a range
/// separator, so `?` depth is tracked alongside bracket depth.
fn at_range_suffix(p: &Parser) -> bool {
    let mut n = 1;
    let mut brackets = 1usize;
    let mut ternaries = 0usize;
    loop {
        match p.nth(n) {
            L_BRACK => brackets += 1,
            R_BRACK => {
                brackets -= 1;
                if brackets == 0 {
                    return false;
                }
            }
            QUESTION => ternaries += 1,
            COLON if brackets == 1 && ternaries == 0 => return true,
            COLON => ternaries = ternaries.saturating_sub(1),
            EOF => return false,
            _ => {}
        }
        n += 1;
    }
}

fn array_suffix(p: &mut Parser) {
    p.start_node(ARRAY_SUFFIX);
    p.expect(L_BRACK);
    expr(p);
    p.expect(R_BRACK);
    p.finish_node();
}

fn range_suffix(p: &mut Parser) {
    p.start_node(RANGE_SUFFIX);
    p.expect(L_BRACK);
    expr(p);
    p.expect(COLON);
    expr(p);
    p.expect(R_BRACK);
    p.finish_node();
}

fn array_type_suffix(p: &mut Parser) {
    p.start_node(ARRAY_TYPE_SUFFIX);
    p.expect(L_BRACK);
    p.expect(R_BRACK);
    p.finish_node();
}

//--------------------------------------------------------------------------
// References
//--------------------------------------------------------------------------

fn instance_ref(p: &mut Parser) {
    p.start_node(INSTANCE_REF);
    instance_ref_element(p);
    while p.at(DOT) {
        p.bump();
        instance_ref_element(p);
    }
    p.finish_node();
}

fn instance_ref_element(p: &mut Parser) {
    p.start_node(INSTANCE_REF_ELEMENT);
    expect_name(p);
    while p.at(L_BRACK) {
        array_suffix(p);
    }
    p.finish_node();
}

//--------------------------------------------------------------------------
// Expressions
//--------------------------------------------------------------------------

/// Binding power of the ternary conditional -- the loosest-binding operator.
const TERNARY_BP: u8 = 1;

/// Binding powers, tightest last, matching the alternative order in the
/// reference grammar. All binary operators are left-associative there.
fn binary_bp(kind: SyntaxKind) -> Option<u8> {
    Some(match kind {
        BOR => 2,
        BAND => 3,
        OR => 4,
        XOR | XNOR => 5,
        AND => 6,
        EQ | NEQ => 7,
        LT | LEQ | GT | GEQ => 8,
        LSHIFT | RSHIFT => 9,
        PLUS | MINUS => 10,
        MULT | DIV | MOD => 11,
        EXP => 12,
        _ => return None,
    })
}

fn expr(p: &mut Parser) {
    expr_bp(p, 0);
}

/// Precedence climbing.
///
/// `checkpoint` is what makes this work in a CST: the left operand is already
/// in the tree by the time the operator is seen, so the enclosing
/// `BINARY_EXPR` is inserted *retroactively* around it. Looping rather than
/// recursing on the left gives left associativity.
fn expr_bp(p: &mut Parser, min_bp: u8) {
    let cp = p.checkpoint();
    unary_expr(p);

    loop {
        let kind = p.current();
        // An operator that binds too loosely for this call is left for the
        // caller to absorb, which is what `filter` folds in: both "not a
        // binary operator" and "binds too loosely" mean the same thing here.
        if let Some(bp) = binary_bp(kind).filter(|&bp| bp >= min_bp) {
            p.start_node_at(cp, BINARY_EXPR);
            p.bump();
            expr_bp(p, bp + 1);
            p.finish_node();
        } else if kind == QUESTION && min_bp <= TERNARY_BP {
            p.start_node_at(cp, TERNARY_EXPR);
            p.bump();
            expr_bp(p, 0);
            p.expect(COLON);
            // Same binding power on the right, which makes it right-associative.
            expr_bp(p, TERNARY_BP);
            p.finish_node();
        } else {
            break;
        }
    }
}

fn unary_expr(p: &mut Parser) {
    if p.at_any(UNARY_OP) {
        p.start_node(UNARY_EXPR);
        p.bump();
        // The grammar applies unary operators to `expr_primary`, not to a full
        // expression, so `-a ** b` groups as `(-a) ** b`.
        expr_primary(p);
        p.finish_node();
    } else {
        expr_primary(p);
    }
}

fn expr_primary(p: &mut Parser) {
    let cp = p.checkpoint();

    match p.current() {
        // `bit'(x)` -- a type cast.
        BOOLEAN_KW | BIT_KW | LONGINT_KW if p.nth(1) == TICK => {
            p.start_node(CAST_TYPE);
            p.bump(); // type
            p.bump(); // '
            p.expect(L_PAREN);
            expr(p);
            p.expect(R_PAREN);
            p.finish_node();
            return;
        }
        TICK => {
            array_literal(p);
            return;
        }
        L_BRACE => {
            concat_or_replicate(p);
            return;
        }
        L_PAREN => paren_expr(p),
        k if NUMBER_TOKEN.contains(&k) || k == STRING_LITERAL || LITERAL_KW.contains(&k) => {
            p.start_node(LITERAL);
            p.bump();
            p.finish_node();
        }
        k if k.is_ident_like() => {
            if p.nth(1) == DOUBLE_COLON {
                p.start_node(ENUM_LITERAL);
                p.bump();
                p.bump();
                expect_name(p);
                p.finish_node();
                return;
            }
            if p.nth(1) == TICK {
                struct_literal(p);
                return;
            }
            instance_ref(p);
            if p.at(ARROW) {
                p.start_node_at(cp, PROP_REF);
                p.bump();
                if p.at_any(PROP_KEYWORD_KW) {
                    p.start_node(PROP_KEYWORD);
                    p.bump();
                    p.finish_node();
                } else {
                    expect_name(p);
                }
                p.finish_node();
            }
            return;
        }
        k => {
            p.error_and_bump(format!("expected an expression, found {k:?}"));
            return;
        }
    }

    // `32'(x)` or `(w)'(x)`. Only a literal or a parenthesised expression may
    // carry a cast width, which is why this sits after those two arms only.
    if p.at(TICK) && p.nth(1) == L_PAREN {
        p.start_node_at(cp, CAST_WIDTH);
        p.bump(); // '
        p.expect(L_PAREN);
        expr(p);
        p.expect(R_PAREN);
        p.finish_node();
    }
}

fn paren_expr(p: &mut Parser) {
    p.start_node(PAREN_EXPR);
    p.expect(L_PAREN);
    expr(p);
    p.expect(R_PAREN);
    p.finish_node();
}

/// `{a, b}` is a concatenation; `{n {a, b}}` replicates one.
fn concat_or_replicate(p: &mut Parser) {
    let cp = p.checkpoint();
    p.bump(); // {
    if p.at(R_BRACE) {
        p.bump();
        p.start_node_at(cp, CONCATENATE);
        p.finish_node();
        return;
    }
    expr(p);
    if p.at(L_BRACE) {
        concat_or_replicate(p);
        p.expect(R_BRACE);
        p.start_node_at(cp, REPLICATE);
        p.finish_node();
        return;
    }
    while p.eat(COMMA) {
        expr(p);
    }
    p.expect(R_BRACE);
    p.start_node_at(cp, CONCATENATE);
    p.finish_node();
}

fn array_literal(p: &mut Parser) {
    p.start_node(ARRAY_LITERAL);
    p.bump(); // '
    p.expect(L_BRACE);
    if !p.at(R_BRACE) {
        expr(p);
        while p.eat(COMMA) {
            expr(p);
        }
    }
    p.expect(R_BRACE);
    p.finish_node();
}

fn struct_literal(p: &mut Parser) {
    p.start_node(STRUCT_LITERAL);
    expect_name(p);
    p.expect(TICK);
    p.expect(L_BRACE);
    if !p.at(R_BRACE) {
        struct_kv(p);
        while p.eat(COMMA) {
            struct_kv(p);
        }
    }
    p.expect(R_BRACE);
    p.finish_node();
}

fn struct_kv(p: &mut Parser) {
    p.start_node(STRUCT_KV);
    expect_name(p);
    p.expect(COLON);
    expr(p);
    p.finish_node();
}

//--------------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------------

/// Consumes an identifier, accepting a keyword in its place.
///
/// SystemRDL reserves enough short words that the grammar has to allow
/// keywords where names are expected; `\`-escaping is the author's way out.
fn expect_name(p: &mut Parser) {
    if p.current().is_ident_like() {
        p.bump();
    } else {
        p.error(format!("expected a name, found {:?}", p.current()));
    }
}
