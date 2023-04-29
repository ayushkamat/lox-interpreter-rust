use std::{collections::HashMap, fmt::Display};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    // todo(ayush): lexical scoping here
    pub bindings: HashMap<String, Value>,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (k, v) in self.bindings.iter() {
            s = format!("{}{}: {}\n", s, k, v);
        }
        write!(f, "{}", s)
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
        }
    }

    pub fn add_name(&mut self, name: String) {
        self.bindings.insert(name, Value::Void);
    }

    pub fn set_value(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<Value> {
        match self.bindings.get(&name) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }
}
