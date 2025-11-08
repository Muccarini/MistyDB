// Example usage of the arithmetic parser

use misty_db::interpreter::tokenizer::Tokenizer;
use misty_db::interpreter::parser::Parser;

fn main() {
    // Example expressions to parse
    let examples = vec![
        "2 + 3",
        "10 - 5",
        "4 * 7",
        "20 / 4",
        "2 + 3 * 4",        // Should parse as 2 + (3 * 4) = 14
        "(2 + 3) * 4",      // Should parse as (2 + 3) * 4 = 20
        "10 / 2 - 3",       // Should parse as (10 / 2) - 3 = 2
        "1 + 2 + 3 + 4",    // Should parse as ((1 + 2) + 3) + 4 = 10
        "100 / 10 / 2",     // Should parse as (100 / 10) / 2 = 5
    ];

    for expr in examples {
        println!("\nParsing: {}", expr);
        
        match Tokenizer::tokenize(expr.to_string()) {
            Ok(tokens) => {
                println!("Tokens: {:?}", tokens);
                
                let mut parser = Parser::new(tokens);
                match parser.parse_expression(0) {
                    Ok((expr, pos)) => {
                        println!("Parsed AST: {:?}", expr);
                        println!("Next position: {}", pos);
                    }
                    Err(e) => {
                        println!("Parse error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Tokenization error: {}", e);
            }
        }
    }
}
