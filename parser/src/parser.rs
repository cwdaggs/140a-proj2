use crate::Token;
use crate::Scanner;
use crate::token::TokenType;

const INT_TYPES: [&'static str; 4] = ["char", "short", "int", "long"];
const FLOAT_TYPES:  [&'static str; 2] = ["float", "double"]; 
const RELATION_OPS: [&'static str; 6] = ["==", "<", ">", "<=", ">=", "!="];
const ADD_OPS: [&'static str; 2] = ["+", "-"];
const MULT_OPS: [&'static str; 2] = ["*", "/"];

pub struct Parser {
    scan: Scanner
}

//Add a coloring and formatting file for xhtml
// Real question, how to handle spaces?
//Maintain list of variables/functions?

//{} for repetition
//[] for optional
// () for grouping

impl Parser {
    // has a while loop calling get next token or whatever from scanner
    pub fn new(s: Scanner) -> Parser {
        Parser {
            scan: s
        }
    }

    pub fn parse(&mut self) {
        //call tokenize from scanner here
        self.scan.tokenize();
        if !self.scan.more_tokens_available() {
            self.program();
        }
    }

    // All declarations have float or int as return type, no void
    fn program(&mut self) {
        while self.scan.peek_next_token().unwrap().get_text() != "void" {
            self.declaration();
        }
        if self.scan.more_tokens_available() {
            self.main_declaration();
        } 
        while self.scan.more_tokens_available() {
            self.function_definition();
        }
    }

    fn declaration(&mut self) {
        self.declaration_type();
        let mut next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_text() == "(" {
            self.function_declaration();
        } else {
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
        self.declaration_type();
        self.parameter_block();
        self.block();
    }

    fn declaration_type(&mut self) {
        self.data_type();
        self.identifier();
    }

    fn variable_declaration(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::OPERATOR.as_str() && next_token.get_text() != ";" {
            panic!("Invalid operator in variable declaration on line {}: use = if initializing or ; if declaring", next_token.get_line_number());
        }

        if next_token.get_text() == "=" {
            self.constant();
        }

        let final_token = self.scan.get_next_token().unwrap();
        if final_token.get_type().as_str() != TokenType::OPERATOR.as_str() && final_token.get_text() != ";" {
            panic!("Must end variable initialization with ';' on line {}", next_token.get_line_number());
        }
    }

    fn function_declaration(&mut self) {
        self.parameter_block();
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::OPERATOR.as_str() || next_token.get_text() != ";" {
            panic!("Must end variable initialization with ';' on line {}", next_token.get_line_number());
        }
    }

    // UNFINISHED
    fn block(&mut self) {
        let mut bracket = self.scan.get_next_token().unwrap();
        if bracket.get_text() != "{" {
            panic!("Missing open bracket for code block on line {}", bracket.get_line_number());
        }
        while self.scan.peek_next_token().unwrap().get_text() != "}" && self.scan.more_tokens_available() {
            // declaration, statement, and/or function definition
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
        if self.scan.peek_next_token().unwrap().get_type().as_str() == TokenType::VARIABLE.as_str() {
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
        let mut next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::FLOATCONSTANT.as_str() && next_token.get_type().as_str() != TokenType::INTCONSTANT.as_str() {
            panic!("Invalid type of constant on line {}", next_token.get_line_number());
        } else if !self.int_constant() && !self.float_constant() {
            panic!("Invalid constant on line {}", next_token.get_text());
        }
        self.scan.get_next_token().unwrap();
    }

    fn int_constant(&mut self) -> bool {
        let mut constant_string = self.scan.peek_next_token().unwrap().get_text().to_string();
        let first_char = constant_string.chars().nth(0).unwrap();
        if first_char != '-' && !first_char.is_digit(10) {
            return false;
        }
        for c in constant_string.chars() {
            if !c.is_digit(10) {
                return false;
            }
        }
        true
    }

    fn float_constant(&mut self) -> bool {
        let mut constant_string = self.scan.peek_next_token().unwrap().get_text().to_string();
        let first_char = constant_string.chars().nth(0).unwrap();
        if first_char != '-' && !first_char.is_digit(10) {
            panic!("Invalid constant on line {}", constant_string);
        }
        let mut period_count = 0;
        for c in constant_string.chars() {
            if c == '.' {
                period_count = period_count + 1;
                if period_count > 1 {
                    return false;
                }
            }
            if !c.is_digit(10) {
                return false;
            }
        }
        true
    }

    // UNFINISHED
    fn statement(&mut self) {
        let mut next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_text() == "while" {
            self.while_loop();
        } else if next_token.get_text() == "return" {
            self.return_statement();
        } else if next_token.get_text() == "if" {
            self.if_statement();
        } else {
            // assignment (peek 2 ahead to see if =) or expression
        }
    }

    // UNFINISHED
    fn assignment(&mut self) {
        self.identifier();
        let mut next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "=" {
            panic!("Missing '=' for assignment on line {}", next_token.get_line_number());
        }
        // How to check if more than one identifier? 
        // keep peeking two tokens ahead, if its = then call identifier and get the =
        self.expression();
       
        if next_token.get_text() != ";" {
            panic!("Missing ';' for assignment on line {}", next_token.get_line_number());
        }
    }

    fn parameter(&mut self) {
        self.data_type();
        self.identifier();
    }

    // Need to handle spaces
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

    fn identifier(&mut self) {
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::VARIABLE.as_str() {
            panic!("Invalid variable on line {}", next_token.get_line_number());
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
        self.scan.get_next_token().unwrap(); //gets while, may not need if got in statement
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
        self.scan.get_next_token().unwrap(); //gets if
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
        self.scan.get_next_token().unwrap(); //gets return
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
        // } else {
        //     self.scan.get_next_token().unwrap();
        // }
    }

// DIFFICULT TO IMPLEMENT

    fn simple_expression(&mut self) {
        self.term();
        let mut next_token = self.scan.peek_next_token().unwrap();
        while ADD_OPS.contains(&next_token.get_text()) {
            self.add_operator();
            self.term();
        }
    }

    fn term(&mut self) {
        self.factor();
        let mut next_token = self.scan.peek_next_token().unwrap();
        while MULT_OPS.contains(&next_token.get_text()) {
            self.mult_operator();
            self.factor();
        }
    }

    // UNFINISHED
    fn factor(&mut self) {
        //peek next
        //if (
            // expression
        // else if type is constant
            // constant
        // else
            // identifier
            // if next is (
                // while peek next != ) {
                    //expression
                // }
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

    // fn handle_spaces(&mut self) {
    //     while self.scan.peek_next_token().unwrap().get_text() == " "  || self.scan.peek_next_token().unwrap().get_text() == "\n"{
    //         self.scan.get_next_token().unwrap();
    //     }
    // }
}