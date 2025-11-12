
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenType {
    Number,		// 42
    Identifier,	// "x"

	QuestionMark,		// ?
	ExclamationMark,	// !

	BackQuote,			// `
	DoubleQuote,		// "
	Quote,				// '
	CircumFlex,			// ^
	Amper,				// &
	AmperAmper,			// &&
	Verbar,				// |
	VerbarVerbar,		// ||
	Percent,			// %
	Tild,				// ~
    Plus,				// +
	PlusEqual,			// +=
    Minus,				// -
	MinusEqual,			// -=
    Star,				// *
	StarEqual,			// *=
    Slash,				// /
	SlashEqual,			// /=
	BackSlash,			// \
    LParen,				// (
    RParen,				// )
	LBracket,			// [
	RBracket,			// ]
	LBrace,				// {
	RBrace,				// }
	At,					// @
	Dot,				// .
	Colon,				// :
	ColonEqual,			// :=
	SemiColon,			// ;
	Comma,				// ,
	Hash,				// #
	Equal,				// =
	EqualEqual,			// ==
	ExclEqual,			// !=
	Sup,				// >
	Inf,				// <
	SupEqual,			// >=
	InfEqual,			// <=
	Arrow,				// ->
	DoubleArrow,		// =>
	NewLine,			// \n

	// Non-context keyword
    Alias,
    Raise,
    Import,
    FromFile,

    // Function keyword
    Lambda,
    Exportable,
    Async,
    Func,
    Await,
    Return,

	// Control flow keyword
    IfConditional,
    ElseConditional,
    ForLoop,
    WhileLoop,
    BreakLoop,
    InLoop,
    Break,
    Continue,

    EOF,				// End Of File 
}

pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Variable Declaration keyword
    m.insert("!", TokenType::ExclamationMark);
    m.insert("?", TokenType::QuestionMark);

    // Non-context keyword
    m.insert("as", TokenType::Alias);
    m.insert("raise", TokenType::Raise);
    m.insert("import", TokenType::Import);
    m.insert("from", TokenType::FromFile);

    // Function keyword
    m.insert("lambda", TokenType::Lambda);
    m.insert("export", TokenType::Exportable);
    m.insert("async", TokenType::Async);
    m.insert("func", TokenType::Func);
    m.insert("await", TokenType::Await);
    m.insert("return", TokenType::Return);


    // Logical Keyword
    m.insert("and", TokenType::AmperAmper);
    m.insert("or", TokenType::VerbarVerbar);
    m.insert("not", TokenType::ExclamationMark);

    // Control flow keyword
    m.insert("if", TokenType::IfConditional);
    m.insert("else", TokenType::ElseConditional);
    m.insert("for", TokenType::ForLoop);
    m.insert("while", TokenType::WhileLoop);
    m.insert("loop", TokenType::BreakLoop);
    m.insert("in", TokenType::InLoop);
    m.insert("break", TokenType::Break);
    m.insert("continue", TokenType::Continue);

    m
});



#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,  // Stores the raw text (e.g., "42", "+", "x")
    // Optional: Add source location (e.g., for error reporting)
    // pub span: Span,
}

impl Token {
    /// Create a new token with a kind and value.
    pub fn new(kind: TokenType, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }

    /// Check if the token matches a specific kind.
    pub fn is(&self, kind: TokenType) -> bool {
        self.kind == kind
    }

    /// Check if the token matches a kind and a value.
    pub fn is_value(&self, kind: TokenType, value: &str) -> bool {
        self.kind == kind && self.value == value
    }
}


struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    // Peek the current character without advancing
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    // Advance to the next character
    fn advance(&mut self) {
        self.position += 1;
    }

    // Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    // Read a number (e.g., "123" -> `Token::Number(123)`)
    fn read_number(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }
        let num_str = &self.input[start..self.position];
        Token::new(TokenType::Number, num_str.parse::<String>().unwrap())
    }

    // Read an identifier (e.g., "x" -> `Token::Identifier("x")`)
    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.peek() {
            if !c.is_alphabetic() {
                break;
            }
            self.advance();
        }
        let ident = &self.input[start..self.position];
		if KEYWORDS.get(ident).is_some(){
			return Token::new(*KEYWORDS.get(ident).unwrap(), ident.parse::<String>().unwrap())
		}
        Token::new(TokenType::Identifier, ident.parse::<String>().unwrap())
    }

    // Main lexing logic
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

		if let Some(c1) = self.peek() {
            if let Some(c2) = self.input.chars().nth(self.position + 1) {
                match (c1, c2) {
					('=', '=') => { self.advance(); self.advance(); return Token::new(TokenType::EqualEqual,  "==")},
					('!', '=') => { self.advance(); self.advance(); return Token::new(TokenType::ExclEqual,  "!=")},
                	('<', '=') => { self.advance(); self.advance(); return Token::new(TokenType::InfEqual,  "<=")},
                    ('>', '=') => { self.advance(); self.advance(); return Token::new(TokenType::SupEqual,  ">=")},
					('&', '&') => { self.advance(); self.advance(); return Token::new(TokenType::AmperAmper,  "&&")},
					('+', '=') => { self.advance(); self.advance(); return Token::new(TokenType::PlusEqual,  "+=")},
					('-', '=') => { self.advance(); self.advance(); return Token::new(TokenType::MinusEqual,  "-=")},
					('*', '=') => { self.advance(); self.advance(); return Token::new(TokenType::StarEqual,  "*=")},
					('/', '=') => { self.advance(); self.advance(); return Token::new(TokenType::EqualEqual,  "/=")},
					(':', '=') => { self.advance(); self.advance(); return Token::new(TokenType::ColonEqual,  ":=")},
					('-', '>') => { self.advance(); self.advance(); return Token::new(TokenType::Arrow,  "->")},
					('=', '>') => { self.advance(); self.advance(); return Token::new(TokenType::DoubleArrow,  "=>")},
                    _ => (), // No match, fall through to single-character tokens
                }
            }
        }

    match self.peek() {
    	Some('?') => { self.advance(); Token::new(TokenType::QuestionMark, "?") },
    	Some('!') => { self.advance(); Token::new(TokenType::ExclamationMark, "!") },
    	Some('`') => { self.advance(); Token::new(TokenType::BackQuote, "`") },
    	Some('"') => { self.advance(); Token::new(TokenType::DoubleQuote, "\"") },
    	Some('\'') => { self.advance(); Token::new(TokenType::Quote, "'") },
    	Some('^') => { self.advance(); Token::new(TokenType::CircumFlex, "^") },
    	Some('&') => { self.advance(); Token::new(TokenType::Amper, "&") },
    	Some('|') => { self.advance(); Token::new(TokenType::Verbar, "|") },
    	Some('%') => { self.advance(); Token::new(TokenType::Percent, "%") },
    	Some('~') => { self.advance(); Token::new(TokenType::Tild, "~") },
    	Some('+') => { self.advance(); Token::new(TokenType::Plus, "+") },
    	Some('-') => { self.advance(); Token::new(TokenType::Minus, "-") },
    	Some('*') => { self.advance(); Token::new(TokenType::Star, "*") },
    	Some('/') => { self.advance(); Token::new(TokenType::Slash, "/") },
    	Some('\\') => { self.advance(); Token::new(TokenType::BackSlash, "\\") },
    	Some('(') => { self.advance(); Token::new(TokenType::LParen, "(") },
    	Some(')') => { self.advance(); Token::new(TokenType::RParen, ")") },
    	Some('[') => { self.advance(); Token::new(TokenType::LBracket, "[") },
    	Some(']') => { self.advance(); Token::new(TokenType::RBracket, "]") },
    	Some('{') => { self.advance(); Token::new(TokenType::LBrace, "{") },
    	Some('}') => { self.advance(); Token::new(TokenType::RBrace, "}") },
    	Some('@') => { self.advance(); Token::new(TokenType::At, "@") },
    	Some(':') => { self.advance(); Token::new(TokenType::Colon, ":") },
    	Some(';') => { self.advance(); Token::new(TokenType::SemiColon, ";") },
    	Some('#') => { self.advance(); Token::new(TokenType::Hash, "#") },
    	Some('=') => { self.advance(); Token::new(TokenType::Equal, "=") },
    	Some('>') => { self.advance(); Token::new(TokenType::Sup, ">") },
    	Some('<') => { self.advance(); Token::new(TokenType::Inf, "<") },
    	Some('\n') => { self.advance(); Token::new(TokenType::NewLine, "\n") },
    	Some(c) if c.is_ascii_digit() => self.read_number(),
    	Some(c) if c.is_alphabetic() => self.read_identifier(),
    	None => Token::new(TokenType::EOF, ""),
    _ => panic!("Unexpected character: {:?}", self.peek()),
}

    }

	pub fn tokenize(&mut self, input: String) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		while input.len() > self.position {
			tokens.push(self.next_token());
			self.advance();
		};
		tokens
	}
}
