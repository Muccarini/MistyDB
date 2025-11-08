#[derive(Debug, Clone)]
pub struct AST {
    pub statements: Vec<Statement>
}

//expr returns a value
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    FieldAccess {
        field: String,
        object: Box<Expr>,
    },
    // Function call or command
    Call {
        name: String,
        args: Vec<Expr>
    },
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Lt, Gte, Lte,
    And, Or,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Get { field: Expr, filter: Option<Expr> },
    Set { field: Expr, key: String, value: Expr },
    Delete { field: Expr, key: String },
    Where { condition: Expr },
    Let { name: String, value: Expr },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Expr(Expr),
}