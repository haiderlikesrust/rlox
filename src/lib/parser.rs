use crate::{token::Token, token_type::TokenType};
use strum::Display;

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, e: &Expr) -> T;
    fn visit_grouping(&mut self, e: &Expr) -> T;
    fn visit_literal(&mut self, e: &Expr) -> T;
    fn visit_unary(&mut self, e: &Expr) -> T;
    fn visit_print(&mut self, e: &StmtExpr) -> T;
    fn visit_expression(&mut self, e: &StmtExpr) -> T;
}

pub trait Visitor<T> {
    fn accept(&self, e: &mut dyn ExprVisitor<T>) -> T;
}

#[derive(Debug, Display, Clone)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(LiteralExpr),
    Unary(Box<UnaryExpr>),
    Stmt(Box<StmtExpr>),
}
#[derive(Debug, Display, Clone)]
pub enum StmtExpr {
    Expression {  expression: Expr },
    Print { expression: Expr },
}

impl StmtExpr {
    pub fn get_inner(&self) -> Expr {
        match self {
            StmtExpr::Expression { expression } => expression.clone(),
            StmtExpr::Print { expression } => expression.clone(),
        }
    }
}

impl Visitor<Option<String>> for Expr {
    fn accept(&self, e: &mut dyn ExprVisitor<Option<String>>) -> Option<String> {
        match self {
            Expr::Binary(_) => e.visit_binary(self),
            Expr::Grouping(_) => e.visit_grouping(self),
            Expr::Literal(_) => e.visit_literal(self),
            Expr::Unary(_) => e.visit_unary(self),
            Expr::Stmt(s) => {
                let s = *s.clone();
                match &s {
                    StmtExpr::Expression { .. } => e.visit_expression(&s),
                    StmtExpr::Print { .. } => e.visit_print(&s),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl Visitor<Option<String>> for BinaryExpr {
    fn accept(&self, e: &mut dyn ExprVisitor<Option<String>>) -> Option<String> {
        e.visit_binary(&Expr::Binary(Box::new(self.clone())))
    }
}

impl Visitor<Option<String>> for LiteralExpr {
    fn accept(&self, e: &mut dyn ExprVisitor<Option<String>>) -> Option<String> {
        e.visit_literal(&Expr::Literal(self.clone()))
    }
}

impl Visitor<Option<String>> for GroupingExpr {
    fn accept(&self, e: &mut dyn ExprVisitor<Option<String>>) -> Option<String> {
        e.visit_grouping(&Expr::Grouping(Box::new(self.clone())))
    }
}

impl Visitor<Option<String>> for UnaryExpr {
    fn accept(&self, e: &mut dyn ExprVisitor<Option<String>>) -> Option<String> {
        e.visit_unary(&Expr::Unary(Box::new(self.clone())))
    }
}

impl Visitor<Option<String>> for StmtExpr {
    fn accept(&self, e: &mut dyn ExprVisitor<Option<String>>) -> Option<String> {
        return match self {
            StmtExpr::Expression { .. } => e.visit_expression(self),
            StmtExpr::Print { .. } => e.visit_print(self),
        };
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug, Clone)]
pub struct Parser {
    current: u64,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: op,
                right,
            }));
        }
        expr
    }

    fn match_next(&mut self, types: &[TokenType]) -> bool {
        for tty in types {
            if self.check(tty.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, tty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tty == tty
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().tty == TokenType::Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    fn previous(&self) -> Token {
        let i = self.current - 1;
        self.tokens[i as usize].clone()
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let l = &[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_next(l) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: op,
                right,
            }));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let l = [TokenType::Minus, TokenType::Plus];
        while self.match_next(&l) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: op,
                right,
            }));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let l = [TokenType::Slash, TokenType::Star];
        while self.match_next(&l) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: op,
                right,
            }));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_next(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary(Box::new(UnaryExpr {
                operator: op,
                right,
            }));
        }
        self.primary().unwrap()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_next(&[TokenType::False]) {
            return Some(Expr::Literal(LiteralExpr {
                value: Some("false".to_string()),
            }));
        }

        if self.match_next(&[TokenType::True]) {
            return Some(Expr::Literal(LiteralExpr {
                value: Some("true".to_string()),
            }));
        }
        if self.match_next(&[TokenType::Nil]) {
            return Some(Expr::Literal(LiteralExpr {
                value: Some("null".to_string()),
            }));
        }

        if self.match_next(&[TokenType::Number, TokenType::String]) {
            return Some(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Some(Expr::Grouping(Box::new(GroupingExpr { expression: expr })));
        }

        None
    }
    fn expression_statement(&mut self) -> StmtExpr {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value");
        StmtExpr::Expression { expression: expr }
    }
    fn print_statement(&mut self) -> StmtExpr {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression");
        StmtExpr::Print { expression: expr }
    }

    fn statement(&mut self) -> StmtExpr {
        if self.match_next(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn consume(&mut self, tty: TokenType, message: &str) -> Token {
        if self.check(tty) {
            return self.advance();
        }
        panic!("{}", message)
    }

    pub fn parse(&mut self) -> Vec<StmtExpr> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        statements
        // self.expression()
    }

    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().tty == TokenType::Semicolon {
                return;
            }

            match self.peek().tty {
                TokenType::Class => (),
                TokenType::Fun => (),
                TokenType::Var => (),
                TokenType::For => (),
                TokenType::If => (),
                TokenType::While => (),
                TokenType::Print => (),
                TokenType::Return => (),
                _ => (),
            }
        }
        self.advance();
    }
}
