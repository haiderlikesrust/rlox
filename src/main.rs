use rlox::{
    scanner::Scanner, parser::Parser, interpreter::Interpreter
};
fn main() {
    let mut s = Scanner::new("var a;");
    let _tokens = s.scan_tokens();
    let mut parser = Parser::new(_tokens);
    let a = parser.parse();
    let value = Interpreter::new().interpret(a);
}
