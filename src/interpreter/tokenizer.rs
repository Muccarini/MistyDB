use anyhow::{Result, anyhow};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Token{
    kind: TokenKind,
    line: usize,
    col: usize
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?} at {}:{}", self.kind, self.line, self.col)
    }
}

#[derive(Debug)]
enum TokenKind{
    //keywords
    Get,
    Set,
    Where,
    True,
    False,

    //op
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,

    //compounds
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Dot,
    Semicolon,
    Comma,

    //comparison
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,

    //generic
    Expr,
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Func,
    EOF,
}

pub struct Tokenizer{

}

impl Tokenizer{
    //it takes the source code and splits in token
    pub fn tokenize(source: String) -> Result<Vec<Token>>{

        // rough estimate
        let mut tokens: Vec<Token> = Vec::with_capacity(source.len() / 4);

        // we work on bytes for easier indexing/slicing
        // this is safe as we only deal with ascii characters
        // non-ascii will be handled in string literals and identifiers using from_utf8
        let bytes = source.as_bytes();

        let mut i: usize = 0;
        let mut row: usize = 1;
        let mut col: usize = 1;

        while i < bytes.len() {
            match bytes[i] as char {
                ' ' => {
                    col += 1;
                    i += 1;
                },
                '\n' => {
                    row += 1;
                    col = 1;
                    i += 1;
                },
                '\r' => {
                    i += 1;  // window os uses \r\n for new lines, we just skip \r
                },
                '\t' => {
                    col += 4; // visual alignment
                    i += 1;   // still skip one 1 byte
                },
                '.' => {
                    tokens.push(Token { kind: TokenKind::Dot, line: row, col: col });
                    col += 1;   
                    i += 1;
                },
                '+' => {
                    tokens.push(Token { kind: TokenKind::Plus, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                '-' => {
                    tokens.push(Token { kind: TokenKind::Minus, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                '*' => {
                    tokens.push(Token { kind: TokenKind::Multiply, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                '(' => {
                    tokens.push(Token { kind: TokenKind::LParen, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                ')' => {
                    tokens.push(Token { kind: TokenKind::RParen, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                '{' => {
                    tokens.push(Token { kind: TokenKind::LBrace, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                '}' => {
                    tokens.push(Token { kind: TokenKind::RBrace, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                '[' => {
                    tokens.push(Token { kind: TokenKind::LBracket, line: row, col: col });
                    col += 1;
                    i += 1;
                }
                ']' => {
                    tokens.push(Token { kind: TokenKind::RBracket, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                ',' => {
                    tokens.push(Token { kind: TokenKind::Comma, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                ';' => {
                    tokens.push(Token { kind: TokenKind::Semicolon, line: row, col: col });
                    col += 1;
                    i += 1;
                },
                // Division, or Comments (skipped)
                '/' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                        i += 2;
                        col += 2;
                        while i < bytes.len() && bytes[i] != b'\n' {
                            i += 1;
                            col += 1;
                        }
                        // we do not move i, \n will be handled in the next iteration
                    } else if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                        i += 2;
                        col += 2;

                        while i + 1 < bytes.len() {
                            if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                                i += 2;
                                col += 2;
                                break;
                            }

                            if bytes[i] == b'\n' {
                                row += 1;
                                col = 1;
                                i += 1;
                            } else {
                                col += 1;
                                i += 1;
                            }
                        }

                        if i >= bytes.len() || (i == bytes.len() - 1 && bytes[i - 1] != b'/') {
                            return Err(anyhow!("Unterminated multi-line comment"));
                        }
                    } else {
                        // Division operator
                        tokens.push(Token { kind: TokenKind::Divide, line: row, col: col });
                        col += 1;
                        i += 1;
                    }
                },
                '=' => {
                    if i + 1 < bytes.len(){
                        if bytes[i + 1] as char == '=' {
                            tokens.push(Token { kind: TokenKind::Eq, line: row, col: col });
                            col += 2;
                            i += 2;
                        } else {
                            tokens.push(Token { kind: TokenKind::Assign, line: row, col: col });
                            col += 1;
                            i += 1;
                        }
                    } else {
                        tokens.push(Token { kind: TokenKind::Assign, line: row, col: col });
                        col += 1;
                        i += 1;
                    }
                },
                // Comparison operators
                '>' => {
                    if i + 1 < bytes.len() && bytes[i + 1] as char == '=' {
                        tokens.push(Token { kind: TokenKind::Gte, line: row, col: col });
                        col += 2;
                        i += 2;
                    } else {
                        tokens.push(Token { kind: TokenKind::Gt, line: row, col: col });
                        col += 1;
                        i += 1;
                    }
                },
                '<' => {
                    if i + 1 < bytes.len() && bytes[i + 1] as char == '=' {
                        tokens.push(Token { kind: TokenKind::Lte, line: row, col: col });
                        col += 2;
                        i += 2;
                    } else {
                        tokens.push(Token { kind: TokenKind::Lt, line: row, col: col });
                        col += 1;
                        i += 1;
                    }
                },
                '!' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                        tokens.push(Token { kind: TokenKind::Neq, line: row, col: col });
                        col += 2;
                        i += 2;
                    } else {
                        return Err(anyhow!("Expected char '='"));
                    }
                },

                // Numbers
                '0'..='9' => {
                    let mut j = i;
                    while j < bytes.len() && ((bytes[j] as char).is_ascii_digit() || bytes[j] as char == '.') {
                        j += 1;
                    }

                    let num_str = str::from_utf8(&bytes[i..j])
                        .map_err(|_| anyhow!("Invalid UTF-8 in number literal"))?;

                    let num = num_str.parse::<f64>()
                        .map_err(|_| anyhow!("Invalid Number: {}", num_str))?;

                    tokens.push(Token { kind: TokenKind::Number(num), line: row, col: col });
                    col += j - i;
                    i = j;
                },

                // Literal String
                '"' => {
                    let mut j = i + 1;

                    while j < bytes.len(){
                        if (bytes[j] == b'"'){
                            let mut num_backslashes = 0;
                            let mut k = j - 1;
                            
                            // goes backwards counting backslashes, if even it's the closing quote
                            while k > i && bytes[k] == b'\\'{
                                num_backslashes += 1;
                                k -= 1;
                            }

                            if num_backslashes % 2 == 0{
                                //quote not escaped
                                break;
                            }
                        }
                        j += 1;
                    }

                    if j >= bytes.len(){
                        return Err(anyhow!("Unterminated string literal"))
                    }

                    let word = &bytes[i + 1..j];

                    let str_lit = String::from_utf8(word.to_vec())
                        .map_err(|_| anyhow!("Invalid UTF-8 in string literal"))?;

                    tokens.push(Token { kind: TokenKind::String(str_lit), line: row, col: col });
                    // skipping final '"' 
                    // the parent "while" will spot the overflow i < bytes.len()
                    col += j - i + 1; 
                    i = j + 1;
                },

                // Identifiers and Keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut j = i;

                    while j < bytes.len() && ((bytes[j] as char).is_alphanumeric() || bytes[j] as char == '_') {
                        j += 1;
                    }

                    let word = &bytes[i..j];

                    // Check if it's a keyword
                    let token = match word.to_ascii_lowercase().as_slice() {
                        b"get" => Token { kind: TokenKind::Get, line: row, col: col },
                        b"set" => Token { kind: TokenKind::Set, line: row, col: col },
                        b"where" => Token { kind: TokenKind::Where, line: row, col: col },
                        b"true" => Token { kind: TokenKind::Boolean(true), line: row, col: col },
                        b"false" => Token { kind: TokenKind::Boolean(false), line: row, col: col },
                        _ => {
                            let ident_str = String::from_utf8(word.to_vec())
                                .map_err(|_| anyhow!("Invalid UTF-8 in identifier"))?;
                            Token { kind: TokenKind::Identifier(ident_str), line: row, col: col }
                        },
                    };
                    tokens.push(token);
                    col += j - i;
                    i = j;
                }
                _ => return Err(anyhow!("Unexpected char: {}", bytes[i] as char)),
            }
        }

        tokens.push(Token { kind: TokenKind::EOF, line: row, col: col });
        return Ok(tokens);

    }
}