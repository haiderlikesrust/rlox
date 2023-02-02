use crate::{
    parser::{BinaryExpr, Expr, ExprVisitor, LiteralExpr, StmtExpr, UnaryExpr, Visitor, VariableExpr},
    token_type::TokenType, env::Environment,
};
#[derive(Clone, Debug)]
pub struct Interpreter {
    environment: Environment,
}
impl ExprVisitor<Option<String>> for Interpreter {
    fn visit_binary(&mut self, e: &Expr) -> Option<String> {
        if let Expr::Binary(a) = e {
            let binary_expr = &**a;
            if let BinaryExpr {
                left,
                operator,
                right,
            } = binary_expr
            {
                // let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                // let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                match operator.tty {
                    TokenType::Greater => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left > parsed_right));
                    }
                    TokenType::GreaterEqual => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left >= parsed_right));
                    }
                    TokenType::Less => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left < parsed_right));
                    }
                    TokenType::LessEqual => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left <= parsed_right));
                    }
                    TokenType::BangEqual => {
                        let a = self.evaluate(left);
                        let b = self.evaluate(right);
                        let res = !self.is_equal(&a, &b).unwrap().parse::<bool>().unwrap();
                        return Some(format!("{}", res));
                    }
                    TokenType::EqualEqual => {
                        let a = self.evaluate(left);
                        let b = self.evaluate(right);
                        return self.is_equal(&a, &b);
                    }

                    TokenType::Minus => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left - parsed_right));
                    }
                    TokenType::Star => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left * parsed_right));
                    }
                    TokenType::Slash => {
                        let parsed_left: isize = self.evaluate(left).unwrap().parse().unwrap();
                        let parsed_right: isize = self.evaluate(right).unwrap().parse().unwrap();
                        return Some(format!("{}", parsed_left / parsed_right));
                    }
                    TokenType::Plus => {
                        let parsed_left = self.evaluate(left).unwrap().parse::<isize>();
                        let parsed_right = self.evaluate(right).unwrap().parse::<isize>();
                        if parsed_left.is_err() && parsed_right.is_err() {
                            let mut evaluated_left = self.evaluate(left).unwrap();
                            let evaluated_right = self.evaluate(right).unwrap();
                            evaluated_left.push_str(&evaluated_right);
                        }
                        return Some(format!("{}", parsed_left.unwrap() + parsed_right.unwrap()));
                    }
                    _ => (),
                }
            }
        }
        None
    }
    fn visit_literal(&mut self, e: &Expr) -> Option<String> {
        if let Expr::Literal(LiteralExpr { value }) = e {
            return value.clone();
        }
        None
    }
    fn visit_unary(&mut self, e: &Expr) -> Option<String> {
        if let Expr::Unary(a) = e {
            let unary_expr = &**a;
            if let UnaryExpr { operator, right } = unary_expr {
                let rig = self.evaluate(&right.clone());
                match operator.tty {
                    TokenType::Minus => {
                        return Some(format!("-{}", rig.unwrap()));
                    }
                    TokenType::Bang => return self.is_truthy(&rig),

                    _ => (),
                }
            }
        }
        None
    }
    fn visit_grouping(&mut self, e: &Expr) -> Option<String> {
        self.evaluate(e)
    }

    fn visit_print(&mut self, e: &crate::parser::StmtExpr) -> Option<String> {
        let value = self.evaluate(&e.get_inner());
        match value {
            Some(v) => {

                println!("{}", v)
            }
            None => (),
        }
        None
    }

    fn visit_expression(&mut self, e: &crate::parser::StmtExpr) -> Option<String> {
        self.evaluate(&e.get_inner());
        None
    }

    fn visit_var(&mut self, e: &VariableExpr) -> Option<String> {
        let name = e.get_name();
        let value = self.evaluate(e);
        self.environment.define(name, value.unwrap_or_else(|| "null".to_string()));
        None
    }

    
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Environment::new();
        Self { environment: env }
    }
    pub fn evaluate(&mut self, expr: &impl Visitor<Option<String>>) -> Option<String> {
        expr.accept(self)
    }
    pub fn is_truthy(&mut self, expr: &Option<String>) -> Option<String> {
        if expr.clone().unwrap() == "null" {
            return Some("false".to_string());
        }
        let to_bool: bool = expr.clone().unwrap().parse().unwrap();
        return Some(format!("{}", !to_bool));
    }
    pub fn is_equal(&mut self, a: &Option<String>, b: &Option<String>) -> Option<String> {
        if a.clone().unwrap() == "null" && b.clone().unwrap() == "null" {
            return Some("true".to_string());
        }
        if a.clone().unwrap() == "null" {
            return Some("false".to_string());
        }
        Some(format!("{}", a.clone().unwrap() == b.clone().unwrap()))
    }

    pub fn interpret(&mut self, statements: Vec<StmtExpr>) {
        for stmt_expr in statements {
            self.execute(stmt_expr);
        }
    }

    pub fn execute(&mut self, stmt_expr: StmtExpr) {
        stmt_expr.accept(self);
    }


}
