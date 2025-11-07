use anyhow::Result;

use crate::interpreter::tokenizer::{Token};

#[derive(Debug)]
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
        args: Vec<Expr>
    },
}

#[derive(Debug)]
enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Lt, Gte, Lte,
}

#[derive(Debug)]
enum Statement {
    Get { table: String, filter: Option<Expr> },
    Set { table: String, key: String, value: Expr },
    Delete { table: String, key: String },
    Where { condition: Expr },
}

#[derive(Debug)]
pub struct AST {
    statements: Vec<Statement>
}

pub struct Parser{
    token: Vec<Token>
}

impl Parser {
    pub fn parse(_tokens: Vec<Token>) -> Result<AST>{
        // Placeholder implementation
        Ok(AST{ statements: vec![] } )
    }
}