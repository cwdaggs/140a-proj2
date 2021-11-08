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
	if args.len() > 2 {
		panic!("Too many arguments");
	}
	let filename = &args[1];
	if !filename.ends_with(".x") {
		panic!("Invalid file type: use a file with the suffix .x");
	}

	let mut parser = Parser::new(Scanner::new(CharStream::new(filename)), filename);
	parser.parse();
}
