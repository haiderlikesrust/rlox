use crate::{
    lox::Lox,
    token::{Token},
    token_type::TokenType,
};
use std::{collections::HashMap};
use substring::Substring;

pub struct Scanner {
    pub source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            tty: crate::token_type::TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        });
        let mut t = vec![];
        for token in &self.tokens {
            t.push(token.clone())
        }
        t
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.check_match('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.check_match('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.check_match('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.check_match('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },
            '/' => match self.check_match('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash),
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            'o' => {
                if self.check_match('r') {
                    self.add_token(TokenType::Or)
                }
            }
            _ => {
                if self.is_digit(&c) {
                    self.number();
                } else if self.is_alpha(&c) {
                    self.identifier();
                } else {
                    Lox::error(self.line as u64, "Unexpected character")
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let b = self.source.chars().collect::<Vec<_>>()[self.current];
        self.current += 1;
        b
    }

    fn add_token(&mut self, tty: TokenType) {
        self.add_token2(tty, None);
    }
    fn add_token2(&mut self, tty: TokenType, literal: Option<String>) {
        let text = self.source.substring(self.start, self.current);
        self.tokens.push(Token {
            tty,
            lexeme: text.to_string(),
            literal,
            line: self.line as u64,
        });
    }
    // match in jlox
    fn check_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().collect::<Vec<_>>()[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().collect::<Vec<_>>()[self.current]
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Lox::error(self.line as u64, "Unterminated string");
        }
        self.advance();
        let value = self.source.substring(self.start - 1, self.current - 1).to_owned();
        self.add_token2(TokenType::String, Some(value));
    }
    fn is_digit(&self, c: &char) -> bool {
        (&'0'..=&'9').contains(&c)
    }

    fn number(&mut self) {
        while self.is_digit(&self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(&self.peek_next()) {
            self.advance();
            while self.is_digit(&self.peek()) {
                self.advance();
            }
        }
        let value = self.source.substring(self.start, self.current).to_string();
        let iv: u64 = value.parse().unwrap();
        self.add_token2(TokenType::Number, Some(format!("{}", iv)))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        self.source.chars().collect::<Vec<_>>()[self.current + 1]
    }
    fn identifier(&mut self) {
        while self.is_alpha_numeric(&self.peek()) {
            self.advance();
        }
        let text = self.source.substring(self.start, self.current);
        let tty = self.keywords.get(text);
        match tty {
            Some(t) => {
                let t = t.clone();
                self.add_token(t)
            },
            None => self.add_token(TokenType::Indentifier),
        }
    }
    fn is_alpha(&self, c: &char) -> bool {
        (&'a'..=&'z').contains(&c) || (&'A'..=&'Z').contains(&c) || c == &'_'
    }

    fn is_alpha_numeric(&self, c: &char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}
