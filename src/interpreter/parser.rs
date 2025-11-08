use std::mem::take;

use anyhow::{Result, anyhow};

use crate::interpreter::tokenizer::{Token, TokenKind};
use crate::interpreter::ast::{AST, Statement, Expr, BinaryOp, UnaryOp};

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    // Helper method to create error messages with position information
    fn error_at(&self, pos: usize, message: &str) -> anyhow::Error {
        if pos < self.tokens.len() {
            anyhow!("{} at line {}, column {}", message, self.tokens[pos].line, self.tokens[pos].col)
        } else {
            anyhow!("{} at end of file", message)
        }
    }

    pub fn parse(mut self) -> Result<AST>{

        let mut current = 0;  
        let mut statements: Vec<Statement> = Vec::new();

        while current < self.tokens.len() {
            match &self.tokens[current].kind {
                TokenKind::EOF => {
                    break;
                }
                _ => {
                    let (statement, next_pos) = self.parse_statement(current)?;
                    statements.push(statement);
                    current = next_pos;
                }
            }
        }

        Ok(AST { statements })
    }

    // Parse an expression
    // lowest precedence -> highest precedence
    // from primitive to logical_or
    // logical_or <- logical_and <- comparison <- sum <- mult <- primitive (func calls | identifiers | numbers ...)
    pub fn parse_expression(&mut self, start: usize) -> Result<(Expr, usize)> {
        self.parse_logical_or(start)
    }

    // Parse logical OR (lowest precedence)
    fn parse_logical_or(&mut self, start: usize) -> Result<(Expr, usize)> {
        let (mut left, mut pos) = self.parse_logical_and(start)?;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::Or => {
                    pos += 1;
                    let (right, next_pos) = self.parse_logical_and(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Or,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                _ => break,
            }
        }

        Ok((left, pos))
    }

    // Parse logical AND (higher precedence than OR)
    fn parse_logical_and(&mut self, start: usize) -> Result<(Expr, usize)> {
        let (mut left, mut pos) = self.parse_comparison(start)?;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::And => {
                    pos += 1;
                    let (right, next_pos) = self.parse_comparison(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::And,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                _ => break,
            }
        }

        Ok((left, pos))
    }

    // Parse comparison operators (higher precedence than logical operators)
    fn parse_comparison(&mut self, start: usize) -> Result<(Expr, usize)> {
        let (mut left, mut pos) = self.parse_additive(start)?;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::Eq => {
                    pos += 1;
                    let (right, next_pos) = self.parse_additive(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Eq,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Neq => {
                    pos += 1;
                    let (right, next_pos) = self.parse_additive(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Neq,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Gt => {
                    pos += 1;
                    let (right, next_pos) = self.parse_additive(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Gt,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Lt => {
                    pos += 1;
                    let (right, next_pos) = self.parse_additive(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Lt,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Gte => {
                    pos += 1;
                    let (right, next_pos) = self.parse_additive(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Gte,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Lte => {
                    pos += 1;
                    let (right, next_pos) = self.parse_additive(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Lte,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                _ => break,
            }
        }

        Ok((left, pos))
    }

    // Parse + and -
    fn parse_additive(&mut self, start: usize) -> Result<(Expr, usize)> {
        let (mut left, mut pos) = self.parse_multiplicative(start)?;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::Plus => {
                    pos += 1;
                    let (right, next_pos) = self.parse_multiplicative(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Add,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Minus => {
                    pos += 1;
                    let (right, next_pos) = self.parse_multiplicative(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Sub,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                _ => break,
            }
        }

        Ok((left, pos))
    }

    // Parse multiplication and division (higher precedence)
    fn parse_multiplicative(&mut self, start: usize) -> Result<(Expr, usize)> {
        let (mut left, mut pos) = self.parse_unary(start)?;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::Multiply => {
                    pos += 1;
                    let (right, next_pos) = self.parse_unary(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Mul,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Divide => {
                    pos += 1;
                    let (right, next_pos) = self.parse_unary(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Div,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                _ => break,
            }
        }

        Ok((left, pos))
    }

    // Parse unary operators (higher precedence than multiplication)
    fn parse_unary(&mut self, start: usize) -> Result<(Expr, usize)> {
        if start >= self.tokens.len() {
            return Err(self.error_at(start.saturating_sub(1), "Unexpected end of input"));
        }

        match &self.tokens[start].kind {
            TokenKind::Minus => {
                let (operand, pos) = self.parse_unary(start + 1)?;
                Ok((Expr::Unary {
                    op: UnaryOp::Neg,
                    operand: Box::new(operand),
                }, pos))
            }
            _ => self.parse_primitive_expr(start),
        }
    }

    // Parse primary expressions
    fn parse_primitive_expr(&mut self, start: usize) -> Result<(Expr, usize)> {
        if start >= self.tokens.len() {
            return Err(self.error_at(start.saturating_sub(1), "Unexpected end of input"));
        }

        match &mut self.tokens[start].kind {
            TokenKind::Number(n) => {
                Ok((Expr::Number(*n), start + 1))
            }
            TokenKind::Identifier(name) => {
                // take, moves the String out of the TokenKind and replaces with empty String
                // empty String has no heap allocation (special pointer to "")
                let name_str = take(name);
                let mut pos = start + 1;
                
                // Check if this is a function call
                if pos < self.tokens.len() && self.tokens[pos].kind == TokenKind::LParen {
                    pos += 1; // consume '('
                    
                    let mut args = Vec::new();
                    
                    // Parse arguments
                    if pos < self.tokens.len() && self.tokens[pos].kind != TokenKind::RParen {
                        loop {
                            let (arg, next_pos) = self.parse_expression(pos)?;
                            args.push(arg);
                            pos = next_pos;
                            
                            if pos >= self.tokens.len() {
                                return Err(anyhow!("Unexpected end of input in function call"));
                            }
                            
                            match self.tokens[pos].kind {
                                TokenKind::Comma => {
                                    pos += 1;
                                    continue;
                                }
                                TokenKind::RParen => {
                                    break;
                                }
                                _ => return Err(anyhow!("Expected ',' or ')' in function call, found {:?}", self.tokens[pos].kind)),
                            }
                        }
                    }
                    
                    // Expect ')'
                    if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::RParen {
                        return Err(anyhow!("Expected ')' after function arguments"));
                    }
                    pos += 1;
                    
                    Ok((Expr::Call { name: name_str, args }, pos))
                } else {
                    // Just an identifier
                    Ok((Expr::Identifier(name_str), pos))
                }
            }
            TokenKind::String(s) => {
                // take, moves the String out of the TokenKind and replaces with empty String
                // empty String has no heap allocation (special pointer to "")
                Ok((Expr::String(take(s)), start + 1))
            }
            TokenKind::Boolean(b) => {
                Ok((Expr::Boolean(*b), start + 1))
            }
            TokenKind::LParen => {
                let (expr, pos) = self.parse_expression(start + 1)?;
                
                if pos >= self.tokens.len() {
                    return Err(self.error_at(pos.saturating_sub(1), "Expected closing parenthesis, found end of input"));
                }
                
                match &self.tokens[pos].kind {
                    TokenKind::RParen => Ok((expr, pos + 1)),
                    _ => Err(self.error_at(pos, &format!("Expected closing parenthesis, found {:?}", self.tokens[pos].kind)))
                }
            }
            _ => Err(self.error_at(start, &format!("Expected expression, found {:?}", self.tokens[start].kind)))
        }
    }

    // parse get statement, to a given start, it must correspond to a get token.
    fn get_parse(&self, start: usize) -> Result<Statement> {

        Err(anyhow!("Not implemented"))
    }

    fn set_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    // Parse where statement: where <condition>;
    fn where_parse(&mut self, start: usize) -> Result<(Statement, usize)> {
        if start >= self.tokens.len() {
            return Err(anyhow!("Start index out of bounds"));
        }

        if self.tokens[start].kind != TokenKind::Where {
            return Err(anyhow!("Expected 'where' token"));
        }

        let mut pos = start + 1;

        // Parse the condition expression
        let (condition, next_pos) = self.parse_expression(pos)?;
        pos = next_pos;

        // Optionally consume semicolon
        if pos < self.tokens.len() && self.tokens[pos].kind == TokenKind::Semicolon {
            pos += 1;
        }

        Ok((Statement::Where { condition }, pos))
    }

    fn delete_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    // Parse let statement: let <identifier> = <expr>;
    fn let_parse(&mut self, start: usize) -> Result<(Statement, usize)> {
        if start >= self.tokens.len() {
            return Err(self.error_at(start.saturating_sub(1), "Start index out of bounds"));
        }

        if self.tokens[start].kind != TokenKind::Let {
            return Err(self.error_at(start, "Expected 'let' token"));
        }

        let mut pos = start + 1;

        // Parse variable name
        let name = match &mut self.tokens[pos].kind {
            TokenKind::Identifier(n) => {
                let name = take(n);
                pos += 1;
                name
            }
            _ => return Err(self.error_at(pos, &format!("Expected identifier after 'let', found {:?}", self.tokens[pos].kind))),
        };

        // Expect '='
        if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::Assign {
            return Err(self.error_at(pos, &format!("Expected '=' after variable name, found {:?}", self.tokens.get(pos).map(|t| &t.kind))));
        }
        pos += 1;

        // Parse the value expression
        let (value, next_pos) = self.parse_expression(pos)?;
        pos = next_pos;

        // Optionally consume semicolon
        if pos < self.tokens.len() && self.tokens[pos].kind == TokenKind::Semicolon {
            pos += 1;
        }

        Ok((Statement::Let { name, value }, pos))
    }

    // Parse a single statement at the given position
    // Returns (Statement, next_position)
    fn parse_statement(&mut self, pos: usize) -> Result<(Statement, usize)> {
        if pos >= self.tokens.len() {
            return Err(anyhow!("Unexpected end of input"));
        }

        match &self.tokens[pos].kind {
            TokenKind::Func => {
                self.func_parse(pos)
            }
            TokenKind::Get => {
                let stmt = self.get_parse(pos)?;
                Ok((stmt, pos + 1))
            }
            TokenKind::Set => {
                let stmt = self.set_parse(pos)?;
                Ok((stmt, pos + 1))
            }
            TokenKind::Delete => {
                let stmt = self.delete_parse(pos)?;
                Ok((stmt, pos + 1))
            }
            TokenKind::Where => {
                self.where_parse(pos)
            }
            TokenKind::Let => {
                self.let_parse(pos)
            }
            _ => {
                // Parse as expression statement
                let (expr, next_pos) = self.parse_expression(pos)?;
                let mut final_pos = next_pos;
                
                // Optionally consume semicolon
                if final_pos < self.tokens.len() && self.tokens[final_pos].kind == TokenKind::Semicolon {
                    final_pos += 1;
                }
                
                Ok((Statement::Expr(expr), final_pos))
            }
        }
    }

    // Parse function definition: func name(param1, param2) { ... }
    fn func_parse(&mut self, start: usize) -> Result<(Statement, usize)> {
        if start >= self.tokens.len() {
            return Err(self.error_at(start.saturating_sub(1), "Start index out of bounds"));
        }

        if self.tokens[start].kind != TokenKind::Func {
            return Err(self.error_at(start, "Expected 'func' token"));
        }

        let mut pos = start + 1;

        // Parse function name
        let name = match &mut self.tokens[pos].kind {
            TokenKind::Identifier(n) => {
                let name = take(n);
                pos += 1;
                name
            }
            _ => return Err(self.error_at(pos, &format!("Expected function name after 'func', found {:?}", self.tokens[pos].kind))),
        };

        // Expect '('
        if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::LParen {
            return Err(self.error_at(pos, &format!("Expected '(' after function name, found {:?}", self.tokens.get(pos).map(|t| &t.kind))));
        }
        pos += 1;

        // Parse parameters
        let mut params = Vec::new();
        
        // Check if there are any parameters
        if pos < self.tokens.len() && self.tokens[pos].kind != TokenKind::RParen {
            loop {
                match &self.tokens[pos].kind {
                    TokenKind::Identifier(param_name) => {
                        params.push(param_name.clone());
                        pos += 1;
                    }
                    _ => return Err(anyhow!("Expected parameter name, found {:?}", self.tokens[pos].kind)),
                }

                // Check for comma or closing paren
                if pos >= self.tokens.len() {
                    return Err(anyhow!("Unexpected end of input in parameter list"));
                }

                match self.tokens[pos].kind {
                    TokenKind::Comma => {
                        pos += 1;
                        continue;
                    }
                    TokenKind::RParen => {
                        break;
                    }
                    _ => return Err(anyhow!("Expected ',' or ')' in parameter list, found {:?}", self.tokens[pos].kind)),
                }
            }
        }

        // Expect ')'
        if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::RParen {
            return Err(anyhow!("Expected ')' after parameters"));
        }
        pos += 1;

        // Expect '{'
        if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::LBrace {
            return Err(anyhow!("Expected '{{' after function signature"));
        }
        pos += 1;

        // Parse function body
        let mut body = Vec::new();
        
        while pos < self.tokens.len() && self.tokens[pos].kind != TokenKind::RBrace {
            let (stmt, next_pos) = self.parse_statement(pos)?;
            body.push(stmt);
            pos = next_pos;
        }

        // Expect '}'
        if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::RBrace {
            return Err(anyhow!("Expected '}}' at end of function body"));
        }
        pos += 1;

        Ok((Statement::FuncDef { name, params, body }, pos))
    }

}