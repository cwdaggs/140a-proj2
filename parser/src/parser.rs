use crate::Token;
use crate::Scanner;


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
            program();
        }
    }
    // All declarations have float or int as return type, no void
    fn program() {
        // while token != void {
        // declaration()  
        // }
        //main_declaration()
        //while !empty {
//          function_definition()
        // }
    }

    fn declaration() {
        //declaration_type
        //if variable token {
            // variable_declaration()
        // } else if function token {
            // function_declaration()
        // } else {
            // error
        // }
    }

    fn main_declaration() {
        // print void main(), give error if that is not correct
        //block()
    }

    fn function_definition() {

    }

    fn declaration_type() {

    }

    fn variable_declaration() {
        //if next token isnt =, _=, or ; then error
        //if not ; print (= constant;) 
    }

    fn function_declaration() {
        //parameter_block()
        //print ;
    }

    fn block() {
        //print {, error if not next token
    }

    fn parameter_block() {
        //print (, if not there error
        // while token != ) {
            // parameter()
        //}
        //print ), if not there error
    }

    fn data_type() {

    }

    fn constant() {

    }

    fn statement() {

    }

    fn parameter() {

    }

    fn integer_type() {

    }
    
    fn float_type() {

    }

    fn assignment() {

    }

    fn while_loop() {

    }

    fn if_statement() {

    }

    fn expression() {

    }

    fn simple_expression() {

    }

    fn term() {

    }

    fn factor() {

    }

    fn relation_operator() {

    }

    fn add_operator() {

    }

    fn mult_operator() {

    }
}