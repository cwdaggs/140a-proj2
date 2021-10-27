use crate::Token;
use crate::Scanner;
use crate::token::TokenType;

const INT_TYPES: [&'static str; 4] = ["char", "short", "int", "long"];
const FLOAT_TYPES:  [&'static str; 2] = ["float", "double"]; 

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
        while self.scan.peek_next_token().unwrap().get_text() != "void" {
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

    fn main_declaration(&self) {
        // print void main(), give error if that is not correct
        //block()
    }

    fn function_definition(&self) {

    }

    fn declaration_type(&mut self) {
        self.data_type();
        self.identifier();
    }

    fn variable_declaration(&self) {
        //if next token isnt =, _=, or ; then error
        //if not ; print (= constant;) 
    }

    fn function_declaration(&self) {
        //parameter_block()
        //print ;
    }

    fn block(&self) {
        //print {, error if not next token
    }

    fn parameter_block(&self) {
        //print (, if not there error
        // while token != ) {
            // parameter()
        //}
        //print ), if not there error
    }

    fn data_type(&mut self) {
        if !self.integer_type() {
            self.float_type();
        }
    }

    fn constant(&self) {

    }

    fn statement(&self) {

    }

    fn parameter(&self) {

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
            let possible_inttype = self.scan.peek_next_token().unwrap();
            if !INT_TYPES.contains(&possible_inttype.get_text()) {
                panic!("Invalid unsigned type on line {}", possible_inttype.get_line_number());
            } else {
                self.scan.get_next_token();
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
        let next_token = self.scan.peek_next_token().unwrap();
        if !FLOAT_TYPES.contains(&next_token.get_text()) {
            panic!("Invalid float type on line {}", next_token.get_line_number());
        } else {
            self.scan.get_next_token();
        }
    }

    fn identifier(&self) {
        let next_token = self.scan.peek_next_token().unwrap();
    }

    fn assignment(&self) {

    }

    fn while_loop(&self) {

    }

    fn if_statement(&self) {

    }

    fn expression(&self) {

    }

    fn simple_expression(&self) {

    }

    fn term(&self) {

    }

    fn factor(&self) {

    }

    fn relation_operator(&self) {

    }

    fn add_operator(&self) {

    }

    fn mult_operator(&self) {

    }

    fn handle_spaces(&self) {
        //loop through and go through space tokens until actual token
    }
}