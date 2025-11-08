pub mod tokenizer;
pub mod parser;
pub mod evaluator;
pub mod ast;

use anyhow::{Error, Result, anyhow};
use tokenizer::Tokenizer;
use parser::Parser;
use evaluator::Evaluator;

pub struct Interpreter {
}

impl Interpreter{
    // Execute the full pipeline:
    // source -> tokenization -> parsing -> evaluation
    pub fn execute_full_pipeline(source: String) -> Result<(), Error>{
        let tokens = Tokenizer::tokenize(source)
            .map_err(|e| anyhow!("Failed to tokenize input: {}", e))?;

        //println!("Tokens: {:?}", tokens);

        let parser = Parser::new(tokens);

        let ast = parser.parse()
            .map_err(|e| anyhow!("Failed to parse tokens: {}", e))?;

        println!("AST: {:#?}", ast);

        let evaluator = Evaluator::new();
        evaluator.evaluate(ast)
            .map_err(|e| anyhow!("Failed to evaluate AST: {}", e))?;

        println!("Execution completed successfully.");

        Ok(())
    }
}