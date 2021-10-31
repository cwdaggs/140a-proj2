use std::fs;
// use std::io;
// use std::convert::TryFrom;
// use std::io::prelude::*;


pub struct CharStream {
	text: Vec<char>
}

impl CharStream {

	pub fn new(f: &str) -> CharStream {
		let file_contents = fs::read_to_string(f).expect("Unable to read file");
		CharStream {
			text: file_contents.chars().collect()
		}
	}
	
	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
		!self.text.is_empty()
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available. 
	pub fn peek_next_char(&self) -> Option<char> {
		// if self.more_available() {
			Some(self.text[0])
		// } 
		// None
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: u32) -> Option<char> {
		// if self.more_available() {
			Some(self.text[k as usize])
		// } 
		// None
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {
		// if self.more_available() {
			let next_char = Some(self.text[0]);
			self.text.remove(0);
			next_char
		// } 
		// None
	}
}



