use std::fmt::Display;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, cause: &str, message: &str) {
    println!("[Line {line}] Error {cause}: {message}")
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
}

impl ParserError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
