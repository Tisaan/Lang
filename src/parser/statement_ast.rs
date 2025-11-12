use crate::parser::expression_ast::Expr;


// First, define the Type type (assuming it's needed)
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Null,
    Any,
    Custom(String),
    // Add other types as needed
}

// Define the Stmt enum with all variants
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Box<Expr>,  // Using the Expr enum from the previous conversion
    },
    Block {
        body: Vec<Stmt>,
    },
    Program {
        body: Vec<Stmt>,
    },
    FunctionDecl {
        name: String,
        parameters: Vec<Stmt>,
        return_type: Type,
        body: Option<Vec<Stmt>>,
    },
    Parameter {
        name: String,
        param_type: Type,
    },
    VariableDecl {
        name: String,
        value: Box<Expr>,
        constant: bool,
        explicit_type: Type,
    },
}

// Define Arity as a separate enum to handle both single number and range
#[derive(Debug, Clone, PartialEq)]
pub enum Arity {
    Exact(usize),
    Range(usize, usize),
}

// The ExpectStmt function (simplified since Rust has a strong type system)
pub fn expect_stmt<T>(stmt: T) -> T {
    stmt
}

// Helper functions to create specific statement types
impl Stmt {
    pub fn new_expression(expression: Expr) -> Self {
        Stmt::Expression {
            expression: Box::new(expression),
        }
    }

    pub fn new_block(body: Vec<Stmt>) -> Self {
        Stmt::Block { body }
    }

    pub fn init_program() -> Self {
        Stmt::Program {body: Vec::new()}
    }

    pub fn new_program(body: Vec<Stmt>) -> Self {
        Stmt::Program { body }
    }

    pub fn push_to_program(&mut self, stmt: Stmt) -> Result<(), &'static str> {
        if let Stmt::Program { body } = self {
            body.push(stmt);
            Ok(())
        } else {
            Err("Not a Program variant")
        }
    }


    pub fn new_function_decl(
        name: String,
        parameters: Vec<Stmt>,
        return_type: Type,
        body: Option<Vec<Stmt>>,
    ) -> Self {
        Stmt::FunctionDecl {
            name,
            parameters,
            return_type,
            body,
        }
    }

    pub fn new_parameter(name: String, param_type: Type) -> Self {
        Stmt::Parameter {
            name,
            param_type,
        }
    }

    pub fn new_variable_decl(
        name: String,
        value: Expr,
        constant: bool,
        explicit_type: Type,
    ) -> Self {
        Stmt::VariableDecl {
            name,
            value: Box::new(value),
            constant,
            explicit_type,
        }
    }
    
    pub fn extractblock_body(&self) -> Option<&Vec<Stmt>> {
        if let Stmt::Block { body } = self {
            Some(body)
        } else {
            None
        }
    }
}

/*
// Example usage
fn main() {
    // Create a simple program with a variable declaration
    let var_decl = Stmt::new_variable_decl(
        "x".to_string(),
        None,
        Expr::Number { value: 42.0 },
        false,
        Type::Number,
    );

    let program = Stmt::new_program(vec![var_decl]);

    println!("{:#?}", program);

    // Create a function declaration
    let func = Stmt::new_function_decl(
        "add".to_string(),
        None,
        Arity::Exact(2),
        vec![
            Parameter {
                name: "a".to_string(),
                param_type: Type::Number,
                constant: true,
            },
            Parameter {
                name: "b".to_string(),
                param_type: Type::Number,
                constant: true,
            },
        ],
        Type::Number,
        vec![
            Stmt::new_expression(Expr::BinaryExpr {
                left: Box::new(Expr::Identifier { value: "a".to_string() }),
                operator: Token { kind: "Plus".to_string(), value: "+".to_string() },
                right: Box::new(Expr::Identifier { value: "b".to_string() }),
            })
        ],
    );

    println!("{:#?}", func);

    // Create a block statement
    let block = Stmt::new_block(vec![
        Stmt::new_variable_decl(
            "y".to_string(),
            None,
            Expr::Number { value: 10.0 },
            true,
            Type::Number,
        ),
        Stmt::new_expression(Expr::Identifier { value: "y".to_string() }),
    ]);

    println!("{:#?}", block);
}
*/