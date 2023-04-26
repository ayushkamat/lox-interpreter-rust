use std::{
    fs::File,
    io::{stdin, Read},
};

use crate::{grammar::Visitable, parser::Parser, scanner, visitor::AstPrinter};

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        return Self { had_error: false };
    }

    pub fn run_file(&mut self, path: &String) {
        let mut f = File::open(path).unwrap_or_else(|e| panic!("error opening file: {:?}", e));
        let mut s: String = String::new();
        File::read_to_string(&mut f, &mut s)
            .unwrap_or_else(|e| panic!("error reading file into memory: {:?}", e));
        self.run(s);
    }

    pub fn run_prompt(&mut self) {
        loop {
            self.had_error = false;
            let mut buf = String::new();
            let s = stdin()
                .read_line(&mut buf)
                .unwrap_or_else(|e| panic!("error reading line: {:?}", e));
            if s == 0 {
                break;
            }
            self.run(buf);
        }
    }

    pub fn run(&self, s: String) {
        println!("{}", s);
        let mut scanner = scanner::Scanner::new(s.as_str());

        scanner.tokenize();

        // for token in scanner.tokens.iter() {
        //     println!("{}", token.token_type);
        // }

        let mut parser = Parser::new(scanner.tokens);
        let printer = AstPrinter {};

        match parser.parse() {
            Ok(expr) => printer.print(expr),
            Err(err) => panic!("parser error: {}", err),
        };
    }
}
