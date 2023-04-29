use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    String(String),
    Number(f64),
    Nil,
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(t) => write!(f, "{}", *t),
            Value::Number(n) => write!(f, "{}", *n),
            Value::String(s) => write!(f, "\"{}\"", *s),
            Value::Nil => write!(f, "nil"),
            Value::Void => write!(f, "void"),
        }
    }
}
