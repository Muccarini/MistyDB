mod tokenizer;
mod parser;
mod evaluator;
mod ast;

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
            .map_err(|_| anyhow!("Failed to tokenize input"))?;

        println!("Tokens: {:?}", tokens);

        let ast = Parser::parse(tokens)
            .map_err(|_| anyhow!("Failed to parse tokens"))?;

        println!("AST: {:?}", ast);

        let result = Evaluator::evaluate(ast)
            .map_err(|_| anyhow!("Failed to evaluate AST"))?;

        println!("Evaluation Result: {:?}", result);

        println!("Execution completed successfully.");

        Ok(())
    }
}