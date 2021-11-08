use crate::Token;
use crate::Scanner;
use crate::token::TokenType;
use std::fs::File;
use std::io::Write;

const INT_TYPES: [&'static str; 4] = ["char", "short", "int", "long"];
const FLOAT_TYPES:  [&'static str; 2] = ["float", "double"]; 
const RELATION_OPS: [&'static str; 6] = ["==", "<", ">", "<=", ">=", "!="];
const ADD_OPS: [&'static str; 2] = ["+", "-"];
const MULT_OPS: [&'static str; 2] = ["*", "/"];

pub struct Parser {
    scan: Scanner,
    token_clone: Vec<Token>,
    filename: String
}

impl Parser {
    pub fn new(s: Scanner, f: &str) -> Parser {
        Parser {
            scan: s,
            token_clone: Vec::new(),
            filename: f.to_string()
        }
    }

    fn alter_filename(&self) -> String {
        let mut new_filename = self.filename.clone();
        new_filename.pop();
        new_filename.pop();
        format!("{}.xhtml", new_filename)
    }

    fn match_color(&self, token_type: &str) -> &str{
        let color: &str;
        match token_type {
            "FloatConstant" | "IntConstant" => color = "aqua",
            "Function" => color = "orange",
            "Variable" => color = "yellow",
            _ => color = "white"
        }
        color
    } 

    fn bold_style(&self, token_type: &str) -> bool {
        token_type != TokenType::FUNCTION.as_str() && token_type != TokenType::VARIABLE.as_str()
    }

    fn write_to_xhtml(&self) {
        let mut file = File::create(self.alter_filename()).expect("Failed to create file");
        file.write_all("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1
        -transitional.dtd\">\n".as_bytes()).expect("write failed");
        file.write_all("<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n".as_bytes()).expect("write failed");
        file.write_all("<head>\n<title>\nX Formatted file</title>\n</head>\n<body bgcolor=\"navy\" text=\"yellow\" link=\"yellow\" vlink=\"yellow\">".as_bytes()).expect("write failed");
        file.write_all("<font face=\"Courier New\">\n".as_bytes()).expect("write failed");
        let mut tab_count = 0;
        let mut prev_newline = false;
        for i in 0..self.token_clone.len() {
            let mut token_text = self.token_clone[i as usize].get_text();
            if token_text == ">" {
                token_text = "&gt;";
            } else if token_text == "<" {
                token_text = "&lt;";
            }
            let token_type = self.token_clone[i as usize].get_type().as_str();
            let mut next_token_text = "";
            if prev_newline {
                if token_text == "}" {
                    tab_count -= 1;
                }
                for _j in 0..(tab_count * 4) {
                    file.write_all("&nbsp;".as_bytes()).expect("write failed");
                }
                prev_newline = false;
            }
            if (i + 1) < self.token_clone.len() {
                next_token_text = self.token_clone[(i+1) as usize].get_text();
            }
            if token_text == "{" || token_text == "}" || token_text == ";" {
                if token_text == "{" {
                    tab_count += 1;
                }
                
                let buf = format!("<font color=\"white\"><b>{}</b></font><br />\n", token_text);
                file.write_all(buf.as_bytes()).expect("write failed");
                prev_newline = true;
            } else if token_text == "(" || token_text == ")" {
                let buf = format!("<font color=\"white\"><b>{}</b></font>", token_text);
                file.write_all(buf.as_bytes()).expect("write failed");
            } else if next_token_text == "}" || next_token_text == "(" || next_token_text == ")" || next_token_text == ";" {
                if self.bold_style(token_type) {
                    let buf = format!("<font color=\"{}\"><b>{}</b></font>", self.match_color(token_type), token_text);
                    file.write_all(buf.as_bytes()).expect("write failed");
                } else {
                    let buf = format!("<font color=\"{}\">{}</font>", self.match_color(token_type), token_text);
                    file.write_all(buf.as_bytes()).expect("write failed");
                }
                
            } else {
                if self.bold_style(token_type) {
                    let buf = format!("<font color=\"{}\"><b>{}</b></font> ", self.match_color(token_type), token_text);
                    file.write_all(buf.as_bytes()).expect("write failed");
                } else {
                    let buf = format!("<font color=\"{}\">{}</font> ", self.match_color(token_type), token_text);
                    file.write_all(buf.as_bytes()).expect("write failed");
                }
                
            }
        }
        file.write_all("</font>\n</body>\n</html>\n".as_bytes()).expect("write failed");
    }

    fn initialize_clone(&mut self) {
        let length = self.scan.tokens_length();
        for i in 0..length {
            self.token_clone.push(self.scan.peek_ahead_token(i).unwrap());
        }
    }

    pub fn parse(&mut self) {
        self.scan.tokenize();
        self.initialize_clone();
        if self.scan.more_tokens_available() {
            self.program();
        }
        self.write_to_xhtml();
    }

    fn program(&mut self) {
        while self.scan.peek_next_token().unwrap().get_text() != "void" {
            self.declaration();
        }
        if self.scan.more_tokens_available() {
            self.main_declaration();
        } else {
            panic!("No main declaration");
        }
        while self.scan.more_tokens_available() {
            self.function_definition();
        }
    }

    fn declaration(&mut self) {
        let next_token = self.scan.peek_ahead_token(2).unwrap();
        if next_token.get_text() == "(" || (self.scan.peek_next_token().unwrap().get_text() == "unsigned" && self.scan.peek_ahead_token(3).unwrap().get_text() == "("){
            self.declaration_type(true);
            self.function_declaration();
        } else {
            self.declaration_type(false);
            self.variable_declaration();
        }
    }

    fn main_declaration(&mut self) {
        let mut next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "void" {
            panic!("Main function missing void on line {}", next_token.get_line_number());
        }
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "main" {
            panic!("Main function missing main on line {}", next_token.get_line_number());
        }
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "(" {
            panic!("Main function missing open parenthesis on line {}", next_token.get_line_number());
        }
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != ")" {
            panic!("Main function missing closing parenthesis on line {}", next_token.get_line_number());
        }
        self.block();
    }

    fn function_definition(&mut self) {
        self.declaration_type(true);
        self.parameter_block();
        self.block();
    }

    fn declaration_type(&mut self, is_func: bool) {
        self.data_type();
        self.identifier(is_func);
    }

    fn variable_declaration(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() == "=" {
            self.constant();
            let final_token = self.scan.get_next_token().unwrap();
            if final_token.get_text() != ";" {
                panic!("Must end variable initialization with ';' on line {}", next_token.get_line_number());
            }
        } else if next_token.get_text() != ";" {
            panic!("Invalid operator in variable declaration on line {}: use = if initializing or ; if declaring", next_token.get_line_number());
        }
    }

    fn function_declaration(&mut self) {
        self.parameter_block();
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != ";" {
            panic!("Must end variable initialization with ';' on line {}", next_token.get_line_number());
        }
    }

    fn block(&mut self) {
        let mut bracket = self.scan.get_next_token().unwrap();
        if bracket.get_text() != "{" {
            panic!("Missing open bracket for code block on line {}", bracket.get_line_number());
        }
        while FLOAT_TYPES.contains(&self.scan.peek_next_token().unwrap().get_text()) || INT_TYPES.contains(&self.scan.peek_next_token().unwrap().get_text()) ||
        self.scan.peek_next_token().unwrap().get_text() == "unsigned" {
            self.declaration();
        } 
        while !FLOAT_TYPES.contains(&self.scan.peek_next_token().unwrap().get_text()) && !INT_TYPES.contains(&self.scan.peek_next_token().unwrap().get_text()) &&
        self.scan.peek_next_token().unwrap().get_text() != "unsigned" && self.scan.peek_next_token().unwrap().get_text() != "}" {
            self.statement();
        }
        while self.scan.peek_next_token().unwrap().get_text() != "}" {
            self.function_definition();
        } 
        if !self.scan.more_tokens_available() {
            panic!("Used all available tokens")
        }
        bracket = self.scan.get_next_token().unwrap();
        if bracket.get_text() != "}" {
            panic!("Missing closing bracket for code block on line {}", bracket.get_line_number());
        }
    }

    fn parameter_block(&mut self) {
        let mut parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != "(" {
            panic!("Missing open parenthesis for parameter block on line {}", parenthesis.get_line_number());
        }
        if self.scan.peek_next_token().unwrap().get_type().as_str() == TokenType::KEYWORD.as_str() {
            self.parameter();
        }
        while self.scan.peek_next_token().unwrap().get_text() == "," {
            self.scan.get_next_token().unwrap();
            self.parameter();
        }
        parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != ")" {
            panic!("Missing closing parenthesis for code block on line {}", parenthesis.get_line_number());
        }
    }

    fn data_type(&mut self) {
        if !self.integer_type() {
            self.float_type();
        }
    }

    fn constant(&mut self) {
        let next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::FLOATCONSTANT.as_str() && next_token.get_type().as_str() != TokenType::INTCONSTANT.as_str() {
            panic!("Invalid type of constant on line {}", next_token.get_line_number());
        } else if !self.int_constant() && !self.float_constant() {
            panic!("Invalid constant {} on line {}", next_token.get_text(), next_token.get_line_number());
        }
        self.scan.get_next_token();
    }

    fn int_constant(&mut self) -> bool {
        let constant_string = self.scan.peek_next_token().unwrap().get_text().to_string();
        for (i, c) in constant_string.chars().enumerate() {
            if (i == 0 && c != '-' && !c.is_digit(10)) || (i != 0 && !c.is_digit(10)) {
                return false;
            }
        }
        true
    }

    fn float_constant(&mut self) -> bool {
        let constant_string = self.scan.peek_next_token().unwrap().get_text().to_string();
        let mut period_count = 0;
        for (i, c) in constant_string.chars().enumerate() {
            if c == '.' {
                period_count = period_count + 1;
                if period_count > 1 {
                    return false;
                }
            } else if  (i == 0 && c != '-' && !c.is_digit(10)) || (i != 0 && !c.is_digit(10)) {
                return false;
            }
        }
        true
    }

    fn statement(&mut self) {
        let mut next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_text() == "while" {
            self.while_loop();
        } else if next_token.get_text() == "return" {
            self.return_statement();
        } else if next_token.get_text() == "if" {
            self.if_statement();
        } else if self.scan.peek_ahead_token(1).unwrap().get_text() == "=" {
            self.assignment();
        } else {
            self.expression();
            next_token = self.scan.get_next_token().unwrap();
            if next_token.get_text() != ";" {
                panic!("Missing ; after expression on line {}", next_token.get_line_number());
            }
        }
    }

    fn assignment(&mut self) {
        self.identifier(false);
        let mut next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "=" {
            panic!("Missing '=' for assignment on line {}", next_token.get_line_number());
        }
        while self.scan.peek_ahead_token(1).unwrap().get_text() == "=" {
            self.identifier(false);
            self.scan.get_next_token();
        }
        self.expression();
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != ";" {
            panic!("Missing ';' for assignment on line {}", next_token.get_line_number());
        }
    }

    fn parameter(&mut self) {
        self.data_type();
        self.identifier(false);
    }

    fn integer_type(&mut self) -> bool {
        let mut is_integer_type = false;
        let next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::KEYWORD.as_str() {
            panic!("Invalid keyword on line {}", next_token.get_line_number());
        }
        if next_token.get_text() == "unsigned" {
            is_integer_type = true;
            self.scan.get_next_token().unwrap();
            let possible_inttype = self.scan.get_next_token().unwrap();
            if !INT_TYPES.contains(&possible_inttype.get_text()) {
                panic!("Invalid unsigned type on line {}", possible_inttype.get_line_number());
            }
        }
        if INT_TYPES.contains(&next_token.get_text()) {
            self.scan.get_next_token().unwrap();
            is_integer_type = true;
        }
        is_integer_type
    }
    
    fn float_type(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if !FLOAT_TYPES.contains(&next_token.get_text()) {
            panic!("Invalid float type on line {}: use float or double", next_token.get_line_number());
        } 
    }

    fn identifier(&mut self, is_func: bool) {
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::VARIABLE.as_str() {
            panic!("Invalid variable on line {}", next_token.get_line_number());
        }
        if is_func {
            let next_token_id = next_token.get_id() as usize;
            *self.token_clone[next_token_id].get_type_mut() = TokenType::FUNCTION;
        }
        for (i, c) in next_token.get_text().chars().enumerate() {
            if i == 0 && !c.is_ascii_alphabetic() && c != '_' {
                panic!("Invalid identifier on line {}: Must start with _ or alphabetic char", next_token.get_line_number());
            }
            if i != 0 && !c.is_ascii_alphanumeric() && c != '_' {
                panic!("Invalid identifier on line {}: Contains invalid char", next_token.get_line_number());
            }
        }
    }

    fn while_loop(&mut self) {
        self.scan.get_next_token().unwrap(); 
        let mut parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != "(" {
            panic!("Missing open parenthesis for while loop on line {}", parenthesis.get_line_number());
        }
        self.expression();
        parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != ")" {
            panic!("Missing closing parenthesis for while loop on line {}", parenthesis.get_line_number());
        }
        self.block();
    }

    fn if_statement(&mut self) {
        self.scan.get_next_token().unwrap(); 
        let mut parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != "(" {
            panic!("Missing open parenthesis for if statement on line {}", parenthesis.get_line_number());
        }
        self.expression();
        parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != ")" {
            panic!("Missing closing parenthesis for if statement on line {}", parenthesis.get_line_number());
        }
        self.block();
    }

    fn return_statement(&mut self) {
        self.scan.get_next_token().unwrap(); 
        self.expression();
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != ";" {
            panic!("Missing ';' for return statement on line {}", next_token.get_line_number());
        }
    }

    fn expression(&mut self) {
        self.simple_expression();
        let next_token = self.scan.peek_next_token().unwrap();
        if RELATION_OPS.contains(&next_token.get_text()) {
            self.relation_operator();
            self.simple_expression();
        }
    }

    fn simple_expression(&mut self) {
        self.term();
        while ADD_OPS.contains(&self.scan.peek_next_token().unwrap().get_text()) {
            self.add_operator();
            self.term();
        }
    }

    fn term(&mut self) {
        self.factor();
        while MULT_OPS.contains(&self.scan.peek_next_token().unwrap().get_text()) {
            self.mult_operator();
            self.factor();
        }
    }

    fn factor(&mut self) {
        let next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_text() == "(" {
            self.scan.get_next_token();
            self.expression();
            if self.scan.peek_next_token().unwrap().get_text() != ")" {
                panic!("Invalid factor on line {}", self.scan.peek_next_token().unwrap().get_line_number());
            }
        } else if next_token.get_type().as_str() == TokenType::FLOATCONSTANT.as_str() || next_token.get_type().as_str() == TokenType::INTCONSTANT.as_str() {
            self.constant();
        } else {
            
            if self.scan.peek_ahead_token(1).unwrap().get_text() == "(" {
                self.identifier(true);
                self.scan.get_next_token();
                while self.scan.peek_next_token().unwrap().get_text() != ")" {
                    self.expression();
                }
                self.scan.get_next_token();
            } else if next_token.get_text() == "," {
                self.scan.get_next_token();
            } else {
                self.identifier(false);
            }
        }
    }

    fn relation_operator(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if !RELATION_OPS.contains(&next_token.get_text()) {
            panic!("Invalid relation operator on line {}", next_token.get_line_number());
        }
    }

    fn add_operator(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if !ADD_OPS.contains(&next_token.get_text()) {
            panic!("Invalid addition operator on line {}", next_token.get_line_number());
        }
    }

    fn mult_operator(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if !MULT_OPS.contains(&next_token.get_text()) {
            panic!("Invalid multiplication operator on line {}", next_token.get_line_number());
        }
    }
}