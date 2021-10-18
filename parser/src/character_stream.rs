use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;


pub struct CharStream {
}

impl CharStream {

	pub fn new(f: &str) -> CharStream {
	}
	
	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available. 
	pub fn peek_next_char(&self) -> Option<char> {
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: i32) -> Option<char> {
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {
	}
}



