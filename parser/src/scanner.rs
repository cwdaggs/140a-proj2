mod character_stream;
use character_stream::*;

mod token;
use token::*;

// Needs to tokenize
pub struct Scanner {
    stream: CharStream,
    tokens: Vec<Token>
}

// Keywords: unsigned, char, short, int, long, float, double, while, if, return, void, main
// Operators: (, ,, ), {, }, =, ==, <, >, <=, >=, !=, +, -, *, /, ;

impl Scanner {
    pub fn new(s: &CharStream) -> Scanner {
        Scanner {
            stream: s
            // tokens: new Vec<Token>
        }
    }

    pub fn get_next_token(&self) -> Token {
        while true {

        }
    }
}