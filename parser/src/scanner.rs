use crate::CharStream;
use crate::Token;

const KEYWORDS: [&'static str; 12] = ["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
const OPERATORS: [char; 14] = ['(', ',', ')', '{', '}', '=', '<', '>', '+', '-', '*', '/', ';', '!']; //also ==, !=, <=, >=
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Scanner {
    stream: CharStream,
    linenum: i32,
    charnum: i32,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(s: CharStream, t: Vec<Token>) -> Scanner {
        Scanner {
            stream: s,
            linenum: 0,
            charnum: 0,
            tokens: t
        }
    }

    //if next char newline,
    // return token

    pub fn get_next_token(&mut self) {
        let char_vec: Vec<char>;
        while self.stream.more_available() {
            //maybe just peek and then put get next in the arguments
            let next_char = self.stream.get_next_char().unwrap();
            if OPERATORS.contains(&next_char) {
                self.operator(next_char);
            } else if next_char == '\n' {
                self.linenum += 1;
                self.charnum = 0;
            } else if next_char == ' ' {
                self.charnum += 1;
            } else {
                let temp_string: String = char_vec.iter().collect();
                self.keyword(temp_string);
            }
            self.charnum += 1;
        }
    }

    fn operator(&self, next_char: char) {
        // Add new token to vector, fix this later (simplify)
        let temp_string: String;
        match next_char {
            '+' => temp_string = "+".to_string(),
            '*' => self.tokens.push(Token::new("*".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            '/' => self.tokens.push(Token::new("/".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            '(' => self.tokens.push(Token::new("(".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            ')' => self.tokens.push(Token::new(")".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            ',' => self.tokens.push(Token::new(",".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            '{' => self.tokens.push(Token::new("{".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            '}' => self.tokens.push(Token::new("}".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            ';' => self.tokens.push(Token::new(";".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum)),
            '=' => if self.stream.peek_next_char() == Some('=') {
                    self.tokens.push(Token::new("==".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            } else {
                self.tokens.push(Token::new("=".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            },
            '!' => if self.stream.peek_next_char() == Some('=') {
                self.tokens.push(Token::new("==".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
                },
            '<' => if self.stream.peek_next_char() == Some('=') {
                self.tokens.push(Token::new("<=".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            } else {
                self.tokens.push(Token::new("<".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            },  
            '>' => if self.stream.peek_next_char() == Some('=') {
                self.tokens.push(Token::new(">=".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            } else {
                self.tokens.push(Token::new(">".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            },
            '-' => if self.stream.peek_next_char().unwrap().is_digit(10) { // this aint even right
                self.tokens.push(Token::new(">=".to_string(), crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
            },    
        }
        self.tokens.push(Token::new(temp_string, crate::token::TokenType::OPERATOR, self.linenum, self.charnum))
    }

    fn keyword(&self, temp_string: String) {
        // return bool?
        let temp_slice: &str = &&temp_string;
        if KEYWORDS.contains(&temp_slice) {
            self.tokens.push(Token::new(temp_string, crate::token::TokenType::KEYWORD, self.linenum, self.charnum));
        }
    }

    fn check_string(&self, temp_string: String) {
        self.tokens.push(Token::new(temp_string, crate::token::TokenType::VARIABLE, self.linenum, self.charnum));
    }

    fn check_num(&self, temp_string: String) {
        //nest inside checkstring?
        if temp_string.contains(".") {
            self.tokens.push(Token::new(temp_string, crate::token::TokenType::FLOATCONSTANT, self.linenum, self.charnum));
        } else {
            self.tokens.push(Token::new(temp_string, crate::token::TokenType::INTCONSTANT, self.linenum, self.charnum));
        }
    }

    fn check_other(&self, next_char: char) {
        // move spaces and newlines here
        if next_char == '\n' {
            self.tokens.push(Token::new("\n".to_string(), crate::token::TokenType::NONE, self.linenum, self.charnum));
        } else if next_char == ' ' {
            self.tokens.push(Token::new(" ".to_string(), crate::token::TokenType::NONE, self.linenum, self.charnum));
        }
    }
}