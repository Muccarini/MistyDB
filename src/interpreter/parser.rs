use std::ffi::FromVecWithNulError;
use std::ops::Index;
use std::thread::current;
use std::mem::take;

use anyhow::{Result, anyhow};

use crate::interpreter::tokenizer::{Token, TokenKind};
use crate::interpreter::ast::{AST, Statement, Expr, BinaryOp};

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

    // Parse an expression and return (Expr, next_position)
    pub fn parse_expression(&mut self, start: usize) -> Result<(Expr, usize)> {
        self.parse_additive(start)
    }

    // Parse + and - (lowest precedence)
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
        let (mut left, mut pos) = self.parse_primary(start)?;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::Multiply => {
                    pos += 1;
                    let (right, next_pos) = self.parse_primary(pos)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Mul,
                        right: Box::new(right),
                    };
                    pos = next_pos;
                }
                TokenKind::Divide => {
                    pos += 1;
                    let (right, next_pos) = self.parse_primary(pos)?;
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

    // Parse primary expressions
    fn parse_primary(&mut self, start: usize) -> Result<(Expr, usize)> {
        if start >= self.tokens.len() {
            return Err(anyhow!("Unexpected end of input"));
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
                    return Err(anyhow!("Expected closing parenthesis, found end of input"));
                }
                
                match &self.tokens[pos].kind {
                    TokenKind::RParen => Ok((expr, pos + 1)),
                    _ => Err(anyhow!("Expected closing parenthesis, found {:?}", self.tokens[pos].kind))
                }
            }
            _ => Err(anyhow!("Expected expression, found {:?}", self.tokens[start].kind))
        }
    }

    fn binary_op_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    fn expr_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    fn statement_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    // parse get statement, to a given start, it must correspond to a get token.
    fn get_parse(&self, start: usize) -> Result<Statement> {
 
        if start >= self.tokens.len() {
            return Err(anyhow::anyhow!("Start index out of bounds"));
        }

        if(self.tokens[start].kind != TokenKind::Get){
            return Err(anyhow::anyhow!("Expected 'get' token"));
        }

        let mut pos = start + 1;

        let mut field_name = String::new();
        //let mut filter = None;

        while pos < self.tokens.len() {
            match &self.tokens[pos].kind {
                TokenKind::Identifier(name) => {
                    let table_name = name;
                    pos += 1;
                },
                TokenKind::Number(value) => {
                    // Handle number
                },
                TokenKind::Dot => {
                    // Handle dot
                },
                TokenKind::Semicolon => {
                    // End of statement
                },
                _ => {
                    pos += 1;
                }
            }
        }

        Err(anyhow!("Not implemented"))
    }

    fn set_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    fn where_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
    }

    fn delete_parse(&self, start: usize) -> Result<Statement> {
        // Placeholder implementation
        Err(anyhow::anyhow!("Not implemented"))
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
                let stmt = self.where_parse(pos)?;
                Ok((stmt, pos + 1))
            }
            TokenKind::Let => {
                // TODO: Implement let parsing
                Err(anyhow!("Let statements not yet implemented"))
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
            return Err(anyhow!("Start index out of bounds"));
        }

        if self.tokens[start].kind != TokenKind::Func {
            return Err(anyhow!("Expected 'func' token"));
        }

        let mut pos = start + 1;

        // Parse function name
        let name = match &mut self.tokens[pos].kind {
            TokenKind::Identifier(n) => {
                let name = take(n);
                pos += 1;
                name
            }
            _ => return Err(anyhow!("Expected function name after 'func', found {:?}", self.tokens[pos].kind)),
        };

        // Expect '('
        if pos >= self.tokens.len() || self.tokens[pos].kind != TokenKind::LParen {
            return Err(anyhow!("Expected '(' after function name, found {:?}", self.tokens.get(pos).map(|t| &t.kind)));
        }
        pos += 1;

        // Parse parameters
        let mut params = Vec::new();
        
        // Check if there are any parameters
        if pos < self.tokens.len() && self.tokens[pos].kind != TokenKind::RParen {
            while pos < self.tokens.len() {
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

            return Err(anyhow!("Unexpected end of input in parameter list"));
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