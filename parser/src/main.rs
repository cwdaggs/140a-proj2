mod character_stream;
use character_stream::*;

mod token;
use token::*;

mod scanner;
use scanner::*;

mod parser;
use parser::*;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		panic!("Not enough arguments");
	}
	let filename = &args[1];
	if !filename.ends_with(".x") {
		panic!("Invalid file type");
	}
	
	// let char_stream = CharStream::new(filename);
	// let mut scanner = Scanner::new(char_stream);
	// scanner.tokenize();
	// scanner.print_tokens();
	let mut parser = Parser::new(Scanner::new(CharStream::new(filename)), filename);
	parser.parse();

}
