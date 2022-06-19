use crate::parser::{Expr, ExprVisitor, Visitor};

pub struct AstPrinter {}

impl ExprVisitor<Option<String>> for AstPrinter {
    fn visit_binary(&mut self, e: &Expr) -> Option<String> {
        match e {
            Expr::Binary(e) => {
                Some(self.parenthesize(&e.operator.lexeme, &[e.clone().left, e.clone().right]))
            }
            _ => None,
        }
    }

    fn visit_grouping(&mut self, e: &Expr) -> Option<String> {
        match e {
            Expr::Grouping(e) => Some(self.parenthesize("group", &[e.clone().expression])),
            _ => None,
        }
    }

    fn visit_literal(&mut self, e: &Expr) -> Option<String> {
        match e {
            Expr::Literal(e) => {
                if e.value == None {
                    return Some("nil".to_owned());
                }
                Some(e.clone().value.unwrap())
            }
            _ => None,
        }
    }

    fn visit_unary(&mut self, e: &Expr) -> Option<String> {
        match e {
            Expr::Unary(e) => Some(self.parenthesize(&e.operator.lexeme, &[e.clone().right])),
            _ => None
        }
    }
}

// impl Visitor<Option<String>> for AstPrinter {

// }
impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print(&mut self, expr: &Expr) -> Option<String> {
        expr.accept(self)
    }
    fn parenthesize(&mut self, name: &str, exprs: &[Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            match &expr.accept(self) {
                None => continue,
                Some(v) => {
                    builder.push_str(v);
                } 
            }
        }
        builder.push(')');
        builder
    }
}
