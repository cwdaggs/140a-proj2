use crate::CharStream;
use crate::Token;

const KEYWORDS: [&'static str; 12] = ["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
const OPERATORS: [char; 14] = ['(', ',', ')', '{', '}', '=', '<', '>', '+', '-', '*', '/', ';', '!']; //also ==, !=, <=, >=
// const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Scanner {
    stream: CharStream,
    linenum: i32,
    charnum: i32,
    pub tokens: Vec<Token>
}

impl Scanner {
    pub fn new(s: CharStream) -> Scanner {
        Scanner {
            stream: s,
            linenum: 0,
            charnum: 0,
            tokens: Vec::new()
        }
    }

//Operator highest precedence (check first)
// Previous char to keep track of - op?
// check what type of string it is: var, num, or key
// use the bool returned to push

    pub fn print_tokens(&self) {
        for i in 0..self.tokens.len() {
            println!("{}", self.tokens[i].get_text());
        }
    }

    pub fn get_next_token(&self) -> Option<Token> {
        if self.more_tokens_available() {
            let next_token = Some(self.tokens[0]);
			self.tokens.remove(0);
			return next_token;
        }
        None
    }

    pub fn peek_next_token(&self) -> Option<Token> {
        if self.more_tokens_available() {
            return Some(self.tokens[0]);
        }
        None
    }

    pub fn more_tokens_available(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub fn tokenize(&mut self) {
        let mut char_vec = Vec::new();
        while self.stream.more_available() {
            let next_char = self.stream.peek_next_char().unwrap();

            // Handles 1/2 length ops, will move stream ahead inside function
            // if self.is_operator(next_char) || self.is_space(next_char) {

            // }
            if OPERATORS.contains(&next_char) {
                self.operator(next_char);
            // If next is space, gathers string to test
            } else if self.is_space(next_char) {
                let temp_string: String = char_vec.iter().collect();
                if !temp_string.is_empty() {
                    self.determine_string(&temp_string);
                }
                self.tokens.push(Token::new(self.stream.get_next_char().unwrap().to_string(), crate::token::TokenType::NONE, self.linenum, self.charnum));
                self.charnum += 1;
                char_vec.clear();
            } else if self.is_newline(next_char) {
                self.tokens.push(Token::new(self.stream.get_next_char().unwrap().to_string(), crate::token::TokenType::NONE, self.linenum, self.charnum));
                self.linenum += 1;
                self.charnum = 0;
            } else {
                char_vec.push(self.stream.get_next_char().unwrap());
                self.charnum += 1;
            }
        }
    }

    fn operator(&mut self, next_char: char) {
        let mut temp_string: String = "".to_string();
        match next_char {
            '+' => temp_string = "+".to_string(),
            '*' => temp_string = "*".to_string(),
            '/' => temp_string = "/".to_string(),
            '(' => temp_string = "(".to_string(),
            ')' => temp_string = ")".to_string(),
            ',' => temp_string = ",".to_string(),
            '{' => temp_string = "{".to_string(),
            '}' => temp_string = "}".to_string(),
            ';' => temp_string = ";".to_string(),
            '=' => if self.stream.peek_next_char() == Some('=') {
                temp_string = "==".to_string()
            } else {
                temp_string = "=".to_string()
            },
            '!' => if self.stream.peek_next_char() == Some('=') {
                temp_string = "!=".to_string()
            },
            '<' => if self.stream.peek_next_char() == Some('=') {
                temp_string = "<=".to_string()
            } else {
                temp_string = "<".to_string()
            },  
            '>' => if self.stream.peek_next_char() == Some('=') {
                temp_string = ">=".to_string()
            } else {
                temp_string = ">".to_string()
            },
            '-' => if self.stream.peek_next_char().unwrap().is_digit(10) { // this aint even right
                temp_string = "-".to_string()
            },
            _ => {}    
        }
        
        for _i in 0..temp_string.len() {
            self.stream.get_next_char();
            self.charnum += 1;
        }
        self.tokens.push(Token::new(temp_string, crate::token::TokenType::OPERATOR, self.linenum, self.charnum));
    }

    fn is_keyword(&self, temp_string: String) -> bool {
        let temp_slice: &str = &&temp_string;
        KEYWORDS.contains(&temp_slice)
    }

    fn is_operator(&self, next_char: char) -> bool {
        OPERATORS.contains(&next_char)
    }

    fn is_num(&self, temp_string: String) -> bool {
        for c in temp_string.chars() {
            if !c.is_digit(10) && c != '.' {
                return false;
            }
        }
        true
    }

    fn is_space(&self, next_char: char) -> bool {
        next_char == ' '
    }

    fn is_newline(&self, next_char: char) -> bool {
        next_char == '\n'
    }

    fn determine_string(&mut self, temp_string: &String) {
        if self.is_keyword(temp_string.to_string()) {
            self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::KEYWORD, self.linenum, self.charnum));
        } else if self.is_num(temp_string.to_string()) {
            if temp_string.contains('.') {
                self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::FLOATCONSTANT, self.linenum, self.charnum));
            } else {
                self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::INTCONSTANT, self.linenum, self.charnum));
            }
        } else {
            self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::VARIABLE, self.linenum, self.charnum));
        }
    }
}