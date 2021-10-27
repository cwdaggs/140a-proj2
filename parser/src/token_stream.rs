// use crate::Token;

// pub struct TokenStream {
// 	token_vec: Vec<Token>
// }

// impl TokenStream {

// 	pub fn new(v: Vec<Token>) -> TokenStream {
// 		TokenStream {
// 			token_vec: v
// 		}
// 	}
	
// 	// Returns true if more characters are available, false otherwise.
// 	pub fn more_available(&self) -> bool {
// 		!self.token_vec.is_empty()
// 	}

// 	// Returns the next character without consuming it.
// 	// Returns None if no more characters are available. 
// 	pub fn peek_next_char(&self) -> Option<Token> {
// 		if self.more_available() {
// 			return Some(self.token_vec[0]);
// 		} 
// 		None
// 	}

// 	// Returns the kth character ahead in the stream without consuming it.
// 	// peek_ahead_char(0) returns the same character as peek_next_char().
// 	// Returns None if no more characters are available at the position.
// 	// The input k cannot be negative.
// 	pub fn peek_ahead_char(&self, k: u32) -> Option<Token> {
// 		if self.more_available() {
// 			return Some(self.token_vec[k as usize]);
// 		} 
// 		None
// 	}

// 	// Returns the next character and consumes it.
// 	// Returns None if no more characters are available.
// 	pub fn get_next_char(&mut self) -> Option<Token> {
// 		if self.more_available() {
// 			let next_token = Some(self.token_vec[0]);
// 			self.token_vec.remove(0);
// 			return next_token;
// 		} 
// 		None
// 	}
// }