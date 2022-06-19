use std::fs::File;
use std::io::{BufReader, Read};

use crate::scanner::Scanner;
pub struct Lox;

// hadError not implmented.

impl Lox {
    pub fn run(f: Vec<&str>) {
        if f.len() > 1 {
            eprintln!("Usage rlox: [script]");
        } else if f.len() == 1 {
            Lox::run_file(f[0]);
        } else {
            Lox::run_prompt();
        }
    }

    fn run_file(path: &str) {
        let mut buffer = String::new();
        let file = File::open(path).expect("File not found");
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut buffer).expect("Read error");
        Lox::run_line(&buffer);
    }

    fn run_prompt() {
        loop {
            print!("> ");
            let mut buffer = String::new();
            let input = std::io::stdin()
                .read_line(&mut buffer);
            match input {
                Err(_) => {
                    break;
                },
                Ok(_) => {
                    Lox::run_line(&buffer);
                }
            }
            
        }
    }

    fn run_line(src: &str) {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token.to_string())
        }
    }

    pub fn error(line: u64, message: &str) {
        Lox::report(line, "", message)
    }

    fn report(line: u64, _wh: &str, message: &str) {
        eprintln!("[line {}] Error: {}", line, message)
    }

}
