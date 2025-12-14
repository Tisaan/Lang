use crate::parser::parser::Parser;
use crate::parser::expression_ast::*;
use crate::parser::statement_ast::*;
use crate::lexer::token::{TokenType, KEYWORDS};
use crate::parser::parse_Statement::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Copy, Clone)]
pub enum BindingPower {
    Default,
    Comma,
    Assignment,
    Conditional,//ternary
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary
}

type StmtHandler = fn(p: &mut Parser) -> Stmt;
type NudHandler = fn(p: &mut Parser) -> Stmt;
type LedHandler = fn(p: &mut Parser, left: Expr, bp: BindingPower) -> Expr;

// Define type aliases for the lookup tables
pub type BpLu = HashMap<TokenType, BindingPower>;
pub type NudLu = HashMap<TokenType, NudHandler>;
pub type LedLu = HashMap<TokenType, LedHandler>;
pub type StmtLu = HashMap<TokenType, StmtHandler>;

pub static BP_LU: Lazy<HashMap<TokenType, BindingPower>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(TokenType::Equal, BindingPower::Assignment);
    m.insert(TokenType::AmperAmper, BindingPower::Logical);
    m.insert(TokenType::VerbarVerbar, BindingPower::Logical);
    m.insert(TokenType::Inf, BindingPower::Relational);
    m.insert(TokenType::Sup, BindingPower::Relational);
    m.insert(TokenType::SupEqual, BindingPower::Relational);
    m.insert(TokenType::InfEqual, BindingPower::Relational);
    m.insert(TokenType::EqualEqual, BindingPower::Relational);
    m.insert(TokenType::ExclEqual, BindingPower::Relational);
    m.insert(TokenType::Minus, BindingPower::Additive);
    m.insert(TokenType::Plus, BindingPower::Additive);
    m.insert(TokenType::Star, BindingPower::Multiplicative);
    m.insert(TokenType::Slash, BindingPower::Multiplicative);
    m.insert(TokenType::Percent, BindingPower::Multiplicative);
    m.insert(TokenType::QuestionMark, BindingPower::Conditional);
    m.insert(TokenType::Dot, BindingPower::Member);
    m.insert(TokenType::LBracket, BindingPower::Member);
    m.insert(TokenType::LParen, BindingPower::Call);

        // ASSIGNMENT
   
    // LITERALS AND IDENTIFIER
    m.insert(TokenType::Identifier, BindingPower::Primary);
    m.insert(TokenType::DoubleQuote, BindingPower::Primary);
    m.insert(TokenType::Number, BindingPower::Primary);
    m.insert(TokenType::LBracket, BindingPower::Primary);

    // UNARY & PREFIX
    m.insert(TokenType::Plus, BindingPower::Primary);
    m.insert(TokenType::Minus, BindingPower::Primary);
    m.insert(TokenType::ExclamationMark, BindingPower::Primary);

    // GROUPING EXPR
    m.insert(TokenType::LParen, BindingPower::Primary);
    m.insert(TokenType::Lambda, BindingPower::Primary);
    m.insert(TokenType::Return, BindingPower::Primary);

    m
});
pub static NUD_LU: Lazy<HashMap<TokenType, NudHandler>> = Lazy::new(|| {
    let mut m = HashMap::new();
        // ASSIGNMENT
   
    // LITERALS AND IDENTIFIER
    m.insert(TokenType::Identifier, parse_primary_expr);
    m.insert(TokenType::DoubleQuote, parse_primary_expr);
    m.insert(TokenType::Number, parse_primary_expr);
    m.insert(TokenType::LBracket, parse_array_expr);

    // UNARY & PREFIX
    m.insert(TokenType::Plus, parse_prefix_expr);
    m.insert(TokenType::Minus, parse_prefix_expr);
    m.insert(TokenType::ExclamationMark, parse_prefix_expr);

    // GROUPING EXPR
    m.insert(TokenType::LParen, parse_grouping_expr);
    m.insert(TokenType::Lambda, parse_lambda_expr);
    m.insert(TokenType::Return, parse_return_decl);
    m
});

pub static LED_LU: Lazy<HashMap<TokenType, LedHandler>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(TokenType::Equal, parse_assignment_expr);

    // LOGICAL
    m.insert(TokenType::AmperAmper,parse_binary_expr);
    m.insert(TokenType::VerbarVerbar, parse_binary_expr);

    // RELATIONAL
    m.insert(TokenType::Inf, parse_binary_expr);
    m.insert(TokenType::Sup, parse_binary_expr);
    m.insert(TokenType::SupEqual, parse_binary_expr);
    m.insert(TokenType::InfEqual, parse_binary_expr);
    m.insert(TokenType::EqualEqual, parse_binary_expr);
    m.insert(TokenType::ExclEqual, parse_binary_expr);

    // ADDITIVE & MULTIPLICATIVE
    m.insert(TokenType::Minus, parse_binary_expr);
    m.insert(TokenType::Plus, parse_binary_expr);
    m.insert(TokenType::Star, parse_binary_expr);
    m.insert(TokenType::Slash, parse_binary_expr);
    m.insert(TokenType::Percent, parse_binary_expr);

    // CONDITIONAL
    m.insert(TokenType::QuestionMark, parse_binary_expr);
    
    // MEMBER, COMPUTED, CALL
    m.insert(TokenType::Dot, parse_member_expr);
    m.insert(TokenType::LBracket, parse_member_expr);
    m.insert(TokenType::LParen, parse_call_expr);

    m
});

pub static STMT_LU: Lazy<HashMap<TokenType, StmtHandler>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Register statement handlers here
    m.insert(TokenType::Func, parse_funct as StmtHandler);
    m.insert(TokenType::Return, parse_return_stmt as StmtHandler);
    m.insert(TokenType::IfConditional, parse_if_stmt as StmtHandler);
    m.insert(TokenType::ForLoop, parse_for_stmt as StmtHandler);
    m.insert(TokenType::WhileLoop, parse_while_stmt as StmtHandler);
    m.insert(TokenType::QuestionMark, parse_var as StmtHandler);
    m.insert(TokenType::ExclamationMark, parse_var as StmtHandler);

    m
});

// Helper functions for lookups
pub fn lookup_keyword(word: &str) -> Option<TokenType> {
    KEYWORDS.get(word).copied()
}

pub fn is_stmt_token(token_type: TokenType) -> bool {
    STMT_LU.contains_key(&token_type)
}

pub fn get_stmt_handler(token_type: TokenType) -> Option<StmtHandler> {
    STMT_LU.get(&token_type).copied()
}

pub fn get_binding_power(token_type: TokenType) -> Option<BindingPower> {
    BP_LU.get(&token_type).copied()
}

pub fn get_nud_handler(token_type: TokenType) -> Option<NudHandler> {
    NUD_LU.get(&token_type).copied()
}

pub fn get_led_handler(token_type: TokenType) -> Option<LedHandler> {
    LED_LU.get(&token_type).copied()
}

