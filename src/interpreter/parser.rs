use crate::interpreter::tokenizer::{Token};

enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    // Field access: user.name
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    // Function call or command
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Lt, Gte, Lte,
}

enum Statement {
    Get { table: String, filter: Option<Expr> },
    Set { table: String, key: String, value: Expr },
}

pub struct Parser{
    token: Vec<Token>
}

impl Parser {
    
}