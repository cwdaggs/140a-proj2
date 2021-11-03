use crate::CharStream;
use crate::Token;

const KEYWORDS: [&'static str; 12] = ["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
const OPERATORS: [char; 14] = ['(', ',', ')', '{', '}', '=', '<', '>', '+', '-', '*', '/', ';', '!']; //also ==, !=, <=, >=

pub struct Scanner {
    stream: CharStream,
    linenum: i32,
    tokens: Vec<Token>,
    id_count: i32
}

impl Scanner {
    pub fn new(s: CharStream) -> Scanner {
        Scanner {
            stream: s,
            linenum: 1,
            tokens: Vec::new(),
            id_count: 0
        }
    }

//Operator highest precedence (check first)
// Previous char to keep track of - op?
// check what type of string it is: var, num, or key
// use the bool returned to push
// May need to have a function here that returns vector, use a parameter for parser

    // pub fn print_tokens(&self) {
    //     println!("{}", self.tokens.len());
    //     for i in 0..self.tokens.len() {
    //         println!("{} {} Line:{} ID:{}", self.tokens[i].get_text(), self.tokens[i].get_type().as_str(), self.tokens[i].get_line_number(), self.tokens[i].get_id());
    //     }
    // }

    // pub fn create_tokens_clone(&self) -> Vec<Token> {
    //     self.tokens.clone()
    // }

    pub fn tokens_length(&self) -> u32 {
        self.tokens.len() as u32
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let next_token = self.tokens[0].clone();
        self.tokens.remove(0);
        Some(next_token)
    }

    pub fn peek_next_token(&self) -> Option<Token> {
        Some(self.tokens[0].clone())
    }

    pub fn peek_ahead_token(&self, k: u32) -> Option<Token> {
        Some(self.tokens[k as usize].clone())
    }

    // Implement this like the char stream with peek at k token, use this for assignment
    // if k > len then error
 

    pub fn more_tokens_available(&self) -> bool {
        !self.tokens.is_empty()
    }
//implement last keyword to help integer or float type

    pub fn tokenize(&mut self) {
        let mut char_vec = Vec::new();
        while self.stream.more_available() {
            let next_char = self.stream.peek_next_char().unwrap();

            if self.is_operator(next_char) {
                if !char_vec.is_empty() {
                    let temp_string: String = char_vec.iter().collect();
                    if !temp_string.trim().is_empty() {
                        self.determine_string(&temp_string);
                    }
                    // self.stream.get_next_char();
                    char_vec.clear();
                } else {
                    self.operator(next_char);
                }
            // If next is space, gathers string to test
            } else if self.is_space_or_tab(next_char) {
                let temp_string: String = char_vec.iter().collect();
                if !temp_string.trim().is_empty() {
                    self.determine_string(&temp_string);
                }
                self.stream.get_next_char();
                char_vec.clear();
            } else if self.is_newline(next_char) {
                self.stream.get_next_char();
                self.linenum += 1;
            } else {
                char_vec.push(self.stream.get_next_char().unwrap());
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
            '=' => if self.stream.peek_ahead_char(1) == Some('=') {
                temp_string = "==".to_string()
            } else {
                temp_string = "=".to_string()
            },
            '!' => if self.stream.peek_ahead_char(1) == Some('=') {
                temp_string = "!=".to_string()
            },
            '<' => if self.stream.peek_ahead_char(1) == Some('=') {
                temp_string = "<=".to_string()
            } else {
                temp_string = "<".to_string()
            },  
            '>' => if self.stream.peek_ahead_char(1) == Some('=') {
                temp_string = ">=".to_string()
            } else {
                temp_string = ">".to_string()
            },
            '-' => if self.stream.peek_ahead_char(1).unwrap().is_digit(10) { // this aint even right
                temp_string = "-".to_string()
            },
            _ => {}    
        }
        
        for _i in 0..temp_string.len() {
            self.stream.get_next_char();
        }
        self.tokens.push(Token::new(temp_string, crate::token::TokenType::OPERATOR, self.linenum, self.id_count));
        self.id_count += 1;
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

    fn is_space_or_tab(&self, next_char: char) -> bool {
        next_char == ' ' || next_char == '\t'
    }

    fn is_newline(&self, next_char: char) -> bool {
        next_char == '\n'
    }

    fn determine_string(&mut self, temp_string: &String) {
        if self.is_keyword(temp_string.to_string()) {
            self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::KEYWORD, self.linenum, self.id_count));
        } else if self.is_num(temp_string.to_string()) {
            if temp_string.contains('.') {
                self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::FLOATCONSTANT, self.linenum, self.id_count));
            } else {
                self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::INTCONSTANT, self.linenum, self.id_count));
            }
        } else {
            self.tokens.push(Token::new(temp_string.to_string(), crate::token::TokenType::VARIABLE, self.linenum, self.id_count));
        }
        self.id_count += 1;
    }
}