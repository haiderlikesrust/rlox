use crate::token_type::TokenType;
use std::{string::ToString};
pub trait Object { }
#[derive(Debug, Clone)]
pub struct Token {
    pub tty: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: u64
}


impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{}  {} {}", &self.tty, &self.lexeme, &self.literal.as_ref().unwrap())
    }
}