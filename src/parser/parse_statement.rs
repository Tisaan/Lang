use crate::parser::parser::Parser;
use crate::parser::statement_ast::*;
use crate::lexer::token::TokenType;
use crate::parser::lookup::*;

pub fn parse_stmt(parser: &mut Parser) -> Stmt{
    let chose = get_stmt_handler(parser.current_tokenkind());
    if chose.is_some() {
        return  (chose.unwrap())(parser)
    }
    return parse_expr_stmt(parser)
}

pub fn  parse_expr_stmt(parser: &mut Parser) -> Stmt {
    let expr = parse_expr(parser, BindingPower::Default);
    parser.is_one_of_many(vec!(TokenType::NewLine, TokenType::EOF));
    Stmt::new_expression(expr)
}

pub fn parse_block(parser: &mut Parser) -> Stmt {
    parser.expect(TokenType::LBrace);
    let mut body = Vec::new();
    while parser.has_token() && parser.current_tokenkind() != TokenType::RBrace{
        body.push(parse_stmt(parser));
    }
    parser.expect(TokenType::RBrace);
    Stmt::new_block(body)
}

pub fn parse_var(parser: &mut Parser) -> Stmt{
    let constant = parser.advance().kind == TokenType::ExclamationMark;
    let name = parser.expect_error(TokenType::Identifier, Some("Expected Identifier".to_owned())).value;
    parser.expect(TokenType::Colon);
    let explicite_type = parse_type(parser, BindingPower::Default);
    parser.expect_error(TokenType::Equal, Some("Expected an equals sign".to_string()));
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
    let name = parser.expect_error(TokenType::Identifier, Some("Expect an identifier".to_string())).value;
    parser.expect(TokenType::Verbar);
    let param = parse_parameter(parser);
    let retype;
    if parser.current_tokenkind() == TokenType::Colon {
        parser.advance();
        if parser.current_tokenkind() == TokenType::RBracket {
            retype = parse_type(parser, BindingPower::Member);
        } else {
            retype = parse_type(parser, BindingPower::Primary); 
        }
    } else {
		retype = None;
	}
    

    let body = parse_block(parser).extractblock_body();
	Stmt::new_function_decl(
		name,
		param,
		 retype,
		body.cloned()
    )
}

pub fn parse_parameter(parser: &mut Parser) -> Vec<Stmt> {
    let param = Vec::new();
    while parser.has_token() && parser.current_tokenkind() != TokenType::RParen {
        let name = parser.expect(TokenType::Identifier).value;

        let ptype: Option<Type>;
        if parser.has_token() && parser.current_tokenkind() != TokenType::Colon {
            parser.advance();
            if parser.current_tokenkind() == TokenType::LBracket {
                ptype = parse_type(parser, BindingPower::Member);
            } else {
                ptype = parse_type(parser, BindingPower::Primary);
            }
        } else {
            parser.throw("Expected type description");
        }
        if parser.is_one_of_many(vec![TokenType::EOF, TokenType::RParen]){
            parser.expect(TokenType::Comma);
        }
        param.push(
            Stmt::new_parameter(
                name,
                ptype.unwrap())
        );
    }
    parser.expect(TokenType::RParen);
    param
}