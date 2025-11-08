use anyhow::{Result, anyhow};
use std::collections::HashMap;
use crate::interpreter::ast::{AST, Statement, Expr, BinaryOp, UnaryOp};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Unit => write!(f, "()"),
        }
    }
}

pub struct Evaluator {
    // Store function definitions
    functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
    // Store variables
    variables: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn evaluate(mut self, ast: AST) -> Result<()> {
        for statement in ast.statements {
            self.eval_statement(&statement)?;
        }
        Ok(())
    }

    fn eval_statement(&mut self, stmt: &Statement) -> Result<Value> {
        match stmt {
            Statement::FuncDef { name, params, body } => {
                // Store the function definition
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                println!("Defined function: {}", name);
                Ok(Value::Unit)
            }
            Statement::Expr(expr) => {
                let value = self.eval_expr(expr)?;
                println!("{}", value);
                Ok(value)
            }
            Statement::Let { name, value } => {
                let eval_value = self.eval_expr(value)?;
                self.variables.insert(name.clone(), eval_value.clone());
                println!("Let {} = {}", name, eval_value);
                Ok(eval_value)
            }
            Statement::Get { .. } => {
                println!("GET statement (not yet implemented)");
                Ok(Value::Unit)
            }
            Statement::Set { .. } => {
                println!("SET statement (not yet implemented)");
                Ok(Value::Unit)
            }
            Statement::Delete { .. } => {
                println!("DELETE statement (not yet implemented)");
                Ok(Value::Unit)
            }
            Statement::Where { .. } => {
                println!("WHERE statement (not yet implemented)");
                Ok(Value::Unit)
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Identifier(name) => {
                // Look up variable
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Undefined variable: {}", name))
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary_op(&left_val, op, &right_val)
            }
            Expr::Unary { op, operand } => {
                let operand_val = self.eval_expr(operand)?;
                self.eval_unary_op(op, &operand_val)
            }
            Expr::Call { name, args } => {
                // Look up function
                let (params, body) = self.functions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Undefined function: {}", name))?;

                // Check argument count
                if args.len() != params.len() {
                    return Err(anyhow!(
                        "Function '{}' expects {} arguments, got {}",
                        name,
                        params.len(),
                        args.len()
                    ));
                }

                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }

                // Save current variables (simple scope handling)
                let saved_vars = self.variables.clone();

                // Bind parameters to arguments
                for (param, value) in params.iter().zip(arg_values.iter()) {
                    self.variables.insert(param.clone(), value.clone());
                }

                // Execute function body
                let mut result = Value::Unit;
                for stmt in &body {
                    result = self.eval_statement(stmt)?;
                }

                // Restore variables
                self.variables = saved_vars;

                Ok(result)
            }
            Expr::FieldAccess { .. } => {
                println!("Field access (not yet implemented)");
                Ok(Value::Unit)
            }
        }
    }

    fn eval_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value> {
        match (op, operand) {
            (UnaryOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            _ => Err(anyhow!(
                "Invalid unary operation: {:?} {:?}",
                op,
                operand
            )),
        }
    }

    fn eval_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value> {
        match (left, op, right) {
            (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), BinaryOp::Sub, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), BinaryOp::Mul, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), BinaryOp::Div, Value::Number(r)) => {
                if *r == 0.0 {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            (Value::Number(l), BinaryOp::Eq, Value::Number(r)) => Ok(Value::Boolean(l == r)),
            (Value::Number(l), BinaryOp::Neq, Value::Number(r)) => Ok(Value::Boolean(l != r)),
            (Value::Number(l), BinaryOp::Gt, Value::Number(r)) => Ok(Value::Boolean(l > r)),
            (Value::Number(l), BinaryOp::Lt, Value::Number(r)) => Ok(Value::Boolean(l < r)),
            (Value::Number(l), BinaryOp::Gte, Value::Number(r)) => Ok(Value::Boolean(l >= r)),
            (Value::Number(l), BinaryOp::Lte, Value::Number(r)) => Ok(Value::Boolean(l <= r)),
            
            // String concatenation
            (Value::String(l), BinaryOp::Add, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            
            // Boolean operations
            (Value::Boolean(l), BinaryOp::And, Value::Boolean(r)) => Ok(Value::Boolean(*l && *r)),
            (Value::Boolean(l), BinaryOp::Or, Value::Boolean(r)) => Ok(Value::Boolean(*l || *r)),
            (Value::Boolean(l), BinaryOp::Eq, Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
            (Value::Boolean(l), BinaryOp::Neq, Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
            
            _ => Err(anyhow!(
                "Invalid binary operation: {:?} {:?} {:?}",
                left,
                op,
                right
            )),
        }
    }
}