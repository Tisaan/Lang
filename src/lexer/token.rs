
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Number,		// 42
    Identifier,	// "x"

	QuestionMark,		// !
	ExclamationMark,	// ?

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
	Colon,				// :
	ColonEqual,			// :=
	SemiColon,			// ;
	Hash,				// #
	Equal,				// =
	EqualEqual,			// ==
	Inequal,			// !=
	Sup,				// >
	Inf,				// <
	SupEqual,			// >=
	InfEqual,			// <=
	Arrow,				// ->
	DoubleArrow,		// =>
	NewLine,			// \n
    EOF,				// End Of File 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,  // Stores the raw text (e.g., "42", "+", "x")
    // Optional: Add source location (e.g., for error reporting)
    // pub span: Span,
}

impl Token {
    /// Create a new token with a kind and value.
    pub fn new(kind: TokenKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }

    /// Check if the token matches a specific kind.
    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    /// Check if the token matches a kind and a value.
    pub fn is_value(&self, kind: TokenKind, value: &str) -> bool {
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
        Token::new(TokenKind::Number, num_str.parse::<String>().unwrap())
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
        Token::new(TokenKind::Identifier, ident.parse::<String>().unwrap())
    }

    // Main lexing logic
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

		if let Some(c1) = self.peek() {
            if let Some(c2) = self.input.chars().nth(self.position + 1) {
                match (c1, c2) {
					('=', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::EqualEqual,  "==")},
					('!', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::Inequal,  "!=")},
                	('<', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::InfEqual,  "<=")},
                    ('>', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::SupEqual,  ">=")},
					('&', '&') => { self.advance(); self.advance(); return Token::new(TokenKind::AmperAmper,  "&&")},
					('+', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::PlusEqual,  "+=")},
					('-', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::MinusEqual,  "-=")},
					('*', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::StarEqual,  "*=")},
					('/', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::EqualEqual,  "/=")},
					(':', '=') => { self.advance(); self.advance(); return Token::new(TokenKind::ColonEqual,  ":=")},
					('-', '>') => { self.advance(); self.advance(); return Token::new(TokenKind::Arrow,  "->")},
					('=', '>') => { self.advance(); self.advance(); return Token::new(TokenKind::DoubleArrow,  "=>")},
                    _ => (), // No match, fall through to single-character tokens
                }
            }
        }

    match self.peek() {
    	Some('?') => { self.advance(); Token::new(TokenKind::QuestionMark, "?") },
    	Some('!') => { self.advance(); Token::new(TokenKind::ExclamationMark, "!") },
    	Some('`') => { self.advance(); Token::new(TokenKind::BackQuote, "`") },
    	Some('"') => { self.advance(); Token::new(TokenKind::DoubleQuote, "\"") },
    	Some('\'') => { self.advance(); Token::new(TokenKind::Quote, "'") },
    	Some('^') => { self.advance(); Token::new(TokenKind::CircumFlex, "^") },
    	Some('&') => { self.advance(); Token::new(TokenKind::Amper, "&") },
    	Some('|') => { self.advance(); Token::new(TokenKind::Verbar, "|") },
    	Some('%') => { self.advance(); Token::new(TokenKind::Percent, "%") },
    	Some('~') => { self.advance(); Token::new(TokenKind::Tild, "~") },
    	Some('+') => { self.advance(); Token::new(TokenKind::Plus, "+") },
    	Some('-') => { self.advance(); Token::new(TokenKind::Minus, "-") },
    	Some('*') => { self.advance(); Token::new(TokenKind::Star, "*") },
    	Some('/') => { self.advance(); Token::new(TokenKind::Slash, "/") },
    	Some('\\') => { self.advance(); Token::new(TokenKind::BackSlash, "\\") },
    	Some('(') => { self.advance(); Token::new(TokenKind::LParen, "(") },
    	Some(')') => { self.advance(); Token::new(TokenKind::RParen, ")") },
    	Some('[') => { self.advance(); Token::new(TokenKind::LBracket, "[") },
    	Some(']') => { self.advance(); Token::new(TokenKind::RBracket, "]") },
    	Some('{') => { self.advance(); Token::new(TokenKind::LBrace, "{") },
    	Some('}') => { self.advance(); Token::new(TokenKind::RBrace, "}") },
    	Some('@') => { self.advance(); Token::new(TokenKind::At, "@") },
    	Some(':') => { self.advance(); Token::new(TokenKind::Colon, ":") },
    	Some(';') => { self.advance(); Token::new(TokenKind::SemiColon, ";") },
    	Some('#') => { self.advance(); Token::new(TokenKind::Hash, "#") },
    	Some('=') => { self.advance(); Token::new(TokenKind::Equal, "=") },
    	Some('>') => { self.advance(); Token::new(TokenKind::Sup, ">") },
    	Some('<') => { self.advance(); Token::new(TokenKind::Inf, "<") },
    	Some('\n') => { self.advance(); Token::new(TokenKind::NewLine, "\n") },
    	Some(c) if c.is_ascii_digit() => self.read_number(),
    	Some(c) if c.is_alphabetic() => self.read_identifier(),
    	None => Token::new(TokenKind::EOF, ""),
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
