#[derive(Clone, Copy)]
pub enum TokenType {
    NONE,
	INTCONSTANT,
	FLOATCONSTANT,
    OPERATOR,
    KEYWORD,
    VARIABLE,
	FUNCTION,
    INVALID
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            TokenType::NONE => "None",
            TokenType::INTCONSTANT => "IntConstant",
            TokenType::FLOATCONSTANT => "FloatConstant",
            TokenType::OPERATOR => "Operator",
            TokenType::KEYWORD => "Keyword",
            TokenType::VARIABLE => "Variable",
            TokenType::FUNCTION => "Function",
            TokenType::INVALID => "Invalid"
        }   
    }   
}

#[derive(Clone)]
pub struct Token {
    text: String,
    token_type: TokenType,
    line_number: i32,
    token_id: i32
}

impl Token {
    pub fn new(s: String, t: TokenType, linenum: i32, id: i32) -> Token {
        Token {
            text: s,
            token_type: t,
            line_number: linenum,
            token_id: id
        }   
    }   

    pub fn get_text(&self) -> &str {
        &self.text
    }   

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }   

    pub fn get_line_number(&self) -> i32 {
        self.line_number
    }

    pub fn get_id(&self) -> i32 {
        self.token_id
    }
    
    pub fn get_type_mut(&mut self) -> &mut TokenType {
        &mut self.token_type
    } 
}
