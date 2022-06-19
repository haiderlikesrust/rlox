use rlox::{
    scanner::Scanner, ast_printer::AstPrinter, parser::Parser
};
fn main() {
    let mut s = Scanner::new("1 + 1 + 1");
    let _tokens = s.scan_tokens();
    println!("{:?}", &_tokens);
    let mut parser = Parser::new(_tokens);
    let a = parser.parse();
    println!("{:#?}", a)
}
