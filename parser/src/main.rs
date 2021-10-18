//mod character_stream;
//use character_stream::*;

mod token;
use token::*;



fn main() {
	let tt = TokenType::OPERATOR;
	let token = Token::new("+".to_string(), tt, 2, 30);
	println!("text: {}", token.get_text());
	println!("token type: {}", token.get_type().as_str());
	println!("line number: {}", token.get_line_number());
	println!("char position: {}", token.get_char_pos());
}
