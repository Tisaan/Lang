use crate::parser::statement_ast::{Stmt};

// First, define the Token type (assuming it's needed for Binary/Unary expressions)
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: String,
    pub value: String,
}



// Define the main Expr enum with all variants
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    AssignmentExpr {
        assigne: Box<Expr>, // Can be Identifier or Membre
        value: Box<Expr>,
    },
    BinaryExpr {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Boolean {
        value: bool,
    },
    CallExpr {
        args: Vec<Expr>,
        caller: Box<Expr>,
    },
    ComputedExpr {
        member: Box<Expr>,
        property: Box<Expr>,
    },
    ForExpr {
        var_name: Box<Expr>, // IdentifierExpr
        sequence: Box<Expr>, // IdentifierExpr | Membre | ListExpr
        iterator: Box<Expr>, // Call
    },
    Identifier {
        value: String,
    },
    IfExpr {
        body: Vec<(Expr, Vec<Stmt>)>, // Vector of tuples (condition, statements)
        else_branch: Vec<Expr>,      // Else expressions
    },
    Lambda {
        parameters: Vec<Stmt>,
        body: Box<Expr>,
    },
    List {
        value: Vec<Expr>,
        length: usize,
    },
    Map {
        // You might want to define a proper Map structure
        entries: Vec<(Expr, Expr)>,
    },
    MembreExpr {
        member: Box<Expr>,    // IdentifierExpr
        property: Box<Expr>, // IdentifierExpr
    },
    NCallExpr {
        args: Vec<Stmt>,
        caller: Box<Expr>,
    },
    Null,
    Number {
        value: f64, // or i64 depending on your needs
    },
    Property {
        // You might want to define what a Property contains
        name: String,
        value: Box<Expr>,
    },
    Return {
        value: Box<Expr>,
    },
    String {
        value: String,
        length: usize,
    },
    TernaryExpr {
        condition: Box<Expr>,
        true_value: Box<Expr>,
        false_value: Box<Expr>,
    },
    UnaryExpr {
        operator: Token,
        right: Box<Expr>,
    },
    WhileExpr {
        condition: Box<Expr>,
        body: Vec<Stmt>,
    },
}

// The ExpectExpr function (simplified since Rust has a strong type system)
pub fn expect_expr<T>(expr: T) -> T {
    expr
}

// Helper functions to create specific expression types
impl Expr {
    pub fn new_assignment(assigne: Expr, value: Expr) -> Self {
        Expr::AssignmentExpr {
            assigne: Box::new(assigne),
            value: Box::new(value),
        }
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_boolean(value: bool) -> Self {
        Expr::Boolean { value }
    }

    // Add more constructor functions as needed...
}

/*
// Example usage
fn main() {
    // Create a simple expression: 5 + 3
    let expr = Expr::new_binary(
        Expr::Number { value: 5.0 },
        Token { kind: "Plus".to_string(), value: "+".to_string() },
        Expr::Number { value: 3.0 }
    );

    println!("{:?}", expr);

    // Create an identifier
    let ident = Expr::Identifier { value: "x".to_string() };
    println!("{:?}", ident);

    // Create a ternary expression: condition ? true_value : false_value
    let ternary = Expr::TernaryExpr {
        condition: Box::new(Expr::Boolean { value: true }),
        true_value: Box::new(Expr::Number { value: 1.0 }),
        false_value: Box::new(Expr::Number { value: 0.0 }),
    };
    println!("{:?}", ternary);
}
*/