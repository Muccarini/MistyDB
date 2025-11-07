mod tokenizer;
mod parser;

use tokenizer::Tokenizer;


struct Interpreter {
    source: String
}


impl Interpreter{
    pub fn new(source: String) -> Self{
        Interpreter{ source }
    }

    pub fn execute(self){
        Tokenizer::tokenize(self.source);
    }
}