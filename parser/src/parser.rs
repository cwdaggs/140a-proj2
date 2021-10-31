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
        if !self.scan.tokens.is_empty() {
            self.program();
        }
    }
    // All declarations have float or int as return type, no void
    fn program(&mut self) {
        // self.handle_spaces();
        while self.scan.peek_next_token().unwrap().get_text() != "void" {
            // self.handle_spaces();
            self.declaration();
        }
        if self.scan.more_tokens_available() {
            self.main_declaration();
        } //else return error
        while self.scan.more_tokens_available() {
            self.function_definition();
        }
    }

    fn declaration(&mut self) {
        self.declaration_type();
        //if variable token {
            // variable_declaration()
        // } else if function token {
            // function_declaration()
        // } else {
            // error
        // }
    }

    // DONE- May need more space handling
    fn main_declaration(&mut self) {
        // self.handle_spaces();
        let mut next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "void" {
            panic!("Main function missing void on line {}", next_token.get_line_number());
        }
        // self.handle_spaces();
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "main" {
            panic!("Main function missing main on line {}", next_token.get_line_number());
        }
        // self.handle_spaces();
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "(" {
            panic!("Main function missing open parenthesis on line {}", next_token.get_line_number());
        }
        // self.handle_spaces();
        next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != ")" {
            panic!("Main function missing closing parenthesis on line {}", next_token.get_line_number());
        }
        self.block();
    }

    // DONE
    fn function_definition(&mut self) {
        self.declaration_type();
        self.parameter_block();
        self.block();
    }

    // DONE
    fn declaration_type(&mut self) {
        self.data_type();
        self.identifier();
    }

    fn variable_declaration(&mut self) {
        // // self.handle_spaces();
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::OPERATOR.as_str() || (next_token.get_text() != ";" && next_token.get_text() != ";") {
            panic!("Invalid operator in variable declaration on line {}: use = if initializing or ; if declaring", next_token.get_line_number());
        }

        if next_token.get_text() == "=" {
            // self.handle_spaces();
            self.constant();
            // self.handle_spaces();
            let final_token = self.scan.peek_next_token().unwrap();
            if final_token.get_type().as_str() != TokenType::OPERATOR.as_str() || final_token.get_text() != ";" {
                panic!("Must end variable initialization with ';' on line {}", next_token.get_line_number());
            } else {
                self.scan.get_next_token();
            }
        }
    }

    fn function_declaration(&mut self) {
        self.parameter_block();
        // self.handle_spaces();
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_type().as_str() != TokenType::OPERATOR.as_str() || next_token.get_text() != ";" {
            panic!("Must end variable initialization with ';' on line {}", next_token.get_line_number());
        }
    }

    fn block(&mut self) {
        // self.handle_spaces();
        let mut bracket = self.scan.get_next_token().unwrap();
        if bracket.get_text() != "{" {
            panic!("Missing open bracket for code block on line {}", bracket.get_line_number());
        }
        // self.handle_spaces();
        while self.scan.peek_next_token().unwrap().get_text() != "}" && self.scan.more_tokens_available() {
            // declaration, statement, and/or function definition
        }
        if !self.scan.more_tokens_available() {
            panic!("Used all available tokens")
        }
        // self.handle_spaces();
        bracket = self.scan.get_next_token().unwrap();
        if bracket.get_text() != "}" {
            panic!("Missing closing bracket for code block on line {}", bracket.get_line_number());
        }
    }

    fn parameter_block(&mut self) {
        // self.handle_spaces();
        let mut parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != "(" {
            panic!("Missing open parenthesis for parameter block on line {}", parenthesis.get_line_number());
        }
        // self.handle_spaces();
        if self.scan.peek_next_token().unwrap().get_type().as_str() == TokenType::VARIABLE.as_str() {
            self.parameter();
        }
        // self.handle_spaces();
        while self.scan.peek_next_token().unwrap().get_text() == "," {
            self.scan.get_next_token();
            self.parameter();
            // self.handle_spaces();
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

    }

    fn statement(&mut self) {
        let mut next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() == "while" {
            self.while_loop();
        } else if next_token.get_text() == "return" {
            self.return_statement();
        } else if next_token.get_text() == "if" {
            self.if_statement();
        } else {
            // assignment or expression
        }
    }

    fn assignment(&mut self) {
        // self.handle_spaces();
        self.identifier();
        // self.handle_spaces();
        let mut next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != "=" {
            panic!("Missing '=' for assignment on line {}", next_token.get_line_number());
        }
        // How to check if more than one identifier? 
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
            self.scan.get_next_token();
            let possible_inttype = self.scan.get_next_token().unwrap();
            if !INT_TYPES.contains(&possible_inttype.get_text()) {
                panic!("Invalid unsigned type on line {}", possible_inttype.get_line_number());
            }
        }
        if INT_TYPES.contains(&next_token.get_text()) {
            self.scan.get_next_token();
            is_integer_type = true;
        }
        is_integer_type
    }
    
    //Need to handle spaces
    fn float_type(&mut self) {
        // self.handle_spaces();
        let next_token = self.scan.get_next_token().unwrap();
        if !FLOAT_TYPES.contains(&next_token.get_text()) {
            panic!("Invalid float type on line {}: use float or double", next_token.get_line_number());
        } 
    }

    fn identifier(&mut self) {
        // self.handle_spaces();
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
        self.scan.get_next_token(); //gets while, may not need if got in statement
        // self.handle_spaces();
        let mut parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != "(" {
            panic!("Missing open parenthesis for while loop on line {}", parenthesis.get_line_number());
        }
        // self.handle_spaces();
        self.expression();
        // self.handle_spaces();
        parenthesis = self.scan.get_next_token().unwrap();
        if parenthesis.get_text() != ")" {
            panic!("Missing closing parenthesis for while loop on line {}", parenthesis.get_line_number());
        }
        self.block();
    }

    fn if_statement(&mut self) {
        self.scan.get_next_token(); //gets if
        // self.handle_spaces();
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
        self.scan.get_next_token(); //gets return
        // self.handle_spaces();
        self.expression();
        let next_token = self.scan.get_next_token().unwrap();
        if next_token.get_text() != ";" {
            panic!("Missing ';' for return statement on line {}", next_token.get_line_number());
        }
    }

    //handle spaces
    fn expression(&mut self) {
        self.simple_expression();
        let next_token = self.scan.peek_next_token().unwrap();
        if next_token.get_text() == ";" {
            self.scan.get_next_token();
        } else {
            self.relation_operator();
            self.simple_expression();
        }
    }

// DIFFICULT TO IMPLEMENT

    fn simple_expression(&mut self) {
        self.term();
        //handle spaces
        let mut next_token = self.scan.peek_next_token().unwrap();
        while ADD_OPS.contains(&next_token.get_text()) {
            self.add_operator();
        }
    }

    //handle spaces
    fn term(&mut self) {
        self.factor();
    }

    fn factor(&mut self) {

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

    fn handle_spaces(&mut self) {
        while self.scan.peek_next_token().unwrap().get_text() == " "  || self.scan.peek_next_token().unwrap().get_text() == "\n"{
            self.scan.get_next_token();
        }
    }
}