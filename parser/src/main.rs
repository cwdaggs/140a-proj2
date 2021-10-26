mod character_stream;
use character_stream::*;

mod token;
use token::*;

mod scanner;
use scanner::*;

use std::env;

// Needs to take in .x source file
fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		panic!("Not enough arguments");
	}
	let filename = &args[1];
	if !filename.ends_with(".x") {
		panic!("Invalid file type");
	}
	
	let mut char_stream = CharStream::new(filename);
	println!("{}", char_stream.peek_next_char().unwrap());
	println!("{}", char_stream.peek_ahead_char(5).unwrap());
	// println!("{}", char_stream.peek_ahead_char(1).unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.peek_next_char().unwrap());
	let scanner = Scanner::new(char_stream);



	let tt = TokenType::OPERATOR;
	let token = Token::new("+".to_string(), tt, 2, 30);
	println!("text: {}", token.get_text());
	println!("token type: {}", token.get_type().as_str());
	println!("line number: {}", token.get_line_number());
	println!("char position: {}", token.get_char_pos());
}
