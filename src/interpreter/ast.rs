#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>
}

//expr returns a value
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
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
    //also func def return an expr
    Func{
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Lt, Gte, Lte,
}

#[derive(Debug)]
pub enum Statement {
    Get { table: String, filter: Option<Expr> },
    Set { table: String, key: String, value: Expr },
    Delete { table: String, key: String },
    Where { condition: Expr },
}