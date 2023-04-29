use std::{
    fs::File,
    io::{stdin, Read},
};

use crate::{
    parser::Parser,
    scanner,
    visitors::{interpreter, printer, visitor},
};

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
            print!(">>> ");
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
        let mut scanner = scanner::Scanner::new(s.as_str());

        scanner.tokenize();

        let mut parser = Parser::new(scanner.tokens);
        let mut interpreter = interpreter::Interpreter::new();

        let parsed = parser.parse();

        // for res in parsed.iter() {
        //     match res {
        //         Ok(decl) => println!("{}", decl),
        //         Err(e) => println!("{}", e),
        //     }
        // }

        for stmt in parsed.iter() {
            match stmt {
                Ok(s) => interpreter.interpret(&mut s.clone()),
                Err(e) => println!("{}", e),
            }
        }
    }
}
