use crate::parser::parser::Parser;
use crate::parser::statement_ast::*;
use crate::lexer::token::TokenKind;

pub fn parse_stmt(parser: &mut Parser) -> Stmt{
    let chose = stmtLu[parser.current_tokenkind()];
    if chose {
        return  stmtLu[parser.current_tokenkind()](parser)
    }
    return parse_expr_stmt(parser)
}

pub fn  parse_expr_stmt(parser: &mut Parser) -> Stmt {
    let expr = parse_expr(parser, BindingPower::defqult_bp);
    parser.is_one_of_many(vec!(TokenKind::NewLine, TokenKind::EOF));
    Stmt::new_expression(expr)
}

pub fn parse_block(parser: &mut Parser) -> Stmt {
    parser.expect(TokenKind::LBrace);
    let mut body = Vec::new();
    while parser.hasToken() && parser.current_tokenkind() != TokenKind::RBrace{
        body.push(Parse_Stmt(parser));
    }
    parser.expect(TokenKind::RBrace);
    Stmt::new_block(body)
}

pub fn parse_var(parser: &mut Parser) -> Stmt{
    let constant = parser.advance().kind == TokenKind::ExclamationMark;
    let name = parser.expect_error(TokenKind::Identifier, Some("Expected Identifier".to_owned())).value;
    parser.expect(TokenKind::Colon);
    let explicite_type = parse_type(parser, BindingPower.default_bp)
    parser.expect_error(TokenKind::Equal, Some("Expected an equals sign".to_string()));
    let value = parse_expr(parser, BindingPower::Assignment);
    Stmt::new_variable_decl(
        name,
        value,
        constant,
        explicite_type
    )
}

pub fn parse_funct(parser: &mut Parser) -> Stmt {
    parser.advance();
    let name = parser.expect_error(TokenKind::Identifier, Some("Expect an identifier".to_string())).value;
    parser.expect(TokenKind::Verbar);
    let param = parse_parameter(parser);
    let retype;
    if (parser.current_tokenkind() == TokenKind::Colon){
        parser.advance();
        if (parser.current_tokenkind() == TokenKind::RBracket){
            retype = parse_type(parser, BindingPower::Member);
        } else {
            retype = parse_type(parser, BindingPower::Primary); 
        }
    } else {
		retype = None;
	}
    

    let body = (parse_block(parser) as Stmt::Block).body;
	Stmt::new_function_decl(
		name,
		param,
		 retype,
		body)
}