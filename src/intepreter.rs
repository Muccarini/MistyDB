use anyhow::{Result, Error, anyhow};

struct Tokenizer{
    source: String
}

enum Token{
    Get,
    Set,
    Where,
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(f64),
    LParen,
    RParen,
    Semicolon,
    Comma,
    Expr,
    Identifier(String),
    Func,
    EOF,
}

impl Tokenizer{
    fn new(source: String) -> Self{
        let t = Tokenizer{
            source: source
        };

        return t;
    }

    //it takes the source code and splits in token
    fn tokenize(&self) -> Result<Vec<Token>>{

        let mut tokens: Vec<Token> =  Vec::new();
        // 1 alloc, but i like it more than peekable more c-like
        let chars = self.source.chars().collect::<Vec<char>>();

        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                ' ' | '\n' | '\t' => {
                    i += 1;
                },
                '+' => {
                    tokens.push(Token::Plus);
                    i += 1;
                },
                '-' => {
                    tokens.push(Token::Minus);
                    i += 1;
                },
                '*' => {
                    tokens.push(Token::Multiply);
                    i += 1;
                },
                '/' => {
                    tokens.push(Token::Divide);
                    i += 1;
                },
                '(' => {
                    tokens.push(Token::LParen);
                    i += 1;
                },
                ')' => {
                    tokens.push(Token::RParen);
                    i += 1;
                },
                ';' => {
                    tokens.push(Token::Semicolon);
                    i += 1;
                },
                '0'..='9' | '.' => {
                    let mut num_str = String::new();
                    let mut j = i;
                    while j < chars.len() && (chars[j].is_numeric() || chars[j] == '.') {
                        num_str.push(chars[j]);
                        j += 1;
                    }

                    let num = num_str.parse::<f64>()
                        .map_err(|_| anyhow!("Invalid Number; {}", num_str))?;

                    tokens.push(Token::Number(num));
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    // Read identifier or keyword
                    let mut word = String::new();
                    let mut j = i;

                    while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
                        word.push(chars[j]);
                        j += 1;
                    }

                    // Check if it's a keyword
                    let token = match word.to_lowercase().as_str() {
                        "get" => Token::Get,
                        "set" => Token::Set,
                        "where" => Token::Where,
                        _ => Token::Identifier(word),
                    };
                    tokens.push(token);
                }
                _ => return Err(anyhow!("Unexpected char: {}", chars[i])),
            }
        }
        return Ok(tokens);

    }
}

struct Parser {
    //this takes a list of tokens and generate an AST
}

struct AST{
    // abstratc syntax tree
}