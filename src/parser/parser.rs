use crate::parser::statement_ast::*;
use crate::parser::parse_Statement::*;
use crate::lexer::token::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        //createTokenLookup();
        //createTypeTokenLookup();
        Parser {pos: 0, tokens: tokens}
    }
    
    pub fn is_one_of_many(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens{
            if token == self.current_tokenkind(){
                return true
            }
        }
        return false
    }

    pub fn expect_error(&mut self, expected: TokenType, err: Option<String>) -> Token {
        if self.current_tokenkind() != expected {
            match err {
                Some(n) => panic!("ParsingError {}", n),
                None => panic!("{}", format!("Expected {:#?} but recieved {:#?}", expected, self.current_tokenkind())),
            }
        }
        self.advance()
    }

    pub fn expect(&mut self, expected: TokenType) -> Token {
        self.expect_error(expected, None)
    }

    pub fn current_tokenkind(&mut self) -> TokenType {
        match self.tokens.get(self.pos) {
            Some(n) => return n.kind,
            None => panic!("No token found at index {}", self.pos)
        }
    }

    pub fn clean_newline(&mut self){
        while self.has_token() && self.current_tokenkind() == TokenType::NewLine {
            self.advance();
        }
    }

    pub fn current_token(&mut self) -> &Token {
        match self.tokens.get(self.pos) {
            Some(n) => n,
            None => panic!("No Current Token")
        }
    }

    pub fn advance(&mut self) -> Token{
        let token = self.current_token().clone();
        self.pos += 1;
        token
    }

    pub fn has_token(&mut self) -> bool {
        self.pos < self.tokens.len() && self.current_tokenkind() != TokenType::EOF
    }

    pub fn throw(&mut self, mess: &str){
        panic!("Parsing: {}", mess);
    }

    pub fn parse(&mut self) -> Stmt {
        let mut program = Stmt::init_program();
        while self.has_token() {
            program.push_to_program(parse_stmt(self));

            while self.current_tokenkind() == TokenType::NewLine{
                self.advance();
            }
        }
        program
    }

}
