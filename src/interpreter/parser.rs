use std::ops::Index;

use anyhow::Result;

use crate::interpreter::tokenizer::{Token};
use crate::interpreter::ast::{AST};

pub struct Parser{
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<AST>{

        let position = 0;


        for token in tokens{

        }

        // Placeholder implementation
        Ok(AST { statements: vec![] })
    }
}