use crate::{parser::{Expr, ExprVisitor, Visitor, StmtExpr}, token_type::TokenType};

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

    

    fn visit_print(&mut self, e: &StmtExpr) -> Option<String> {
        None
    }

    fn visit_expression(&mut self, e: &StmtExpr) -> Option<String> {
        None
    }

    fn visit_var(&mut self, e: &crate::parser::VariableExpr) -> Option<String> {
        None
    }
}

// impl Visitor<Option<String>> for AstPrinter {

// }

pub fn ac(e: &Expr) -> String {
    let mut stri = String::new();
    match e {
        Expr::Binary(a) => {
            match a.clone().right {
                Expr::Literal(l) => {
                    stri.push_str(&format!(" {}", &l.value.unwrap()));
                },
                Expr::Unary(a) => {
                    ac(&Expr::Unary(a));
                }
                Expr::Binary(a) => {
                    ac(&Expr::Binary(a));

                },
                Expr::Grouping(a) => {
                    ac(&Expr::Grouping(a));
                },
                Expr::Stmt(_) => todo!(), 
                Expr::Variable(_) => todo!("Variable"),
                
            }

            match a.clone().left {
                Expr::Literal(l) => {
                    stri.push_str(&l.value.unwrap());
                },
                Expr::Unary(a) => {
                    ac(&Expr::Unary(a));
                }
                Expr::Binary(a) => {
                    ac(&Expr::Binary(a));

                },
                Expr::Grouping(a) => {
                    ac(&Expr::Grouping(a));
                },
                Expr::Stmt(_) => todo!(), 
                Expr::Variable(_) => todo!("Variable"),
                
            }

            match a.operator.tty {
                TokenType::Plus => {
                    stri.push_str(" + ")
                }
                _ => ()
            }
        }
        _ => ()
        // Expr::Grouping(a) => todo!(),
        // Expr::Literal(a) => todo!(),
        // Expr::Unary(a) => todo!(),
    }
    stri
}
impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }
   
    pub fn print(&mut self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Binary(e) => e.accept(self),
            Expr::Grouping(e) => e.accept(self),
            Expr::Literal(e) => e.accept(self),
            Expr::Unary(e) => e.accept(self),
            Expr::Stmt(e) => e.accept(self),
            Expr::Variable(e) => e.accept(self),
        }
    }
    fn parenthesize(&mut self, name: &str, exprs: &[Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            println!("{:#?}", &expr);
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
