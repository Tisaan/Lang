use crate::parser::parser::Parser;
use crate::parser::statement_ast::*;
use crate::lexer::token::TokenType;
use crate::parser::lookup::*;

fn parse_expr(parser: &mut Parser, bp: BindingPower) -> Expr {
    let func = get_nud_handler(parser);
    if func.is_none() {
		//throw error
	}
	let left = (func.unwrap())(parser);
	while (get_binding_power(parser.current_tokenkind).unwrap() > bp){
		const tokenkind: TokenType = parser.current_tokenkind();
		let led = get_led_handler(tokenkind);
		if (led.is_none()){
			//throw error
		}
		left = (led.unwrap())(parser, left, bp);
	}
	left
}

