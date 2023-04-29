use std::env;

mod environment;
mod error;
mod grammar;
mod lox;
mod parser;
mod scanner;
mod token;
mod util;
mod value;
mod visitors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    if args.len() == 1 {
        lox.run_prompt();
    } else if args.len() == 2 {
        lox.run_file(args.last().unwrap());
    }
}
