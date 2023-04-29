// --- Declarations ---

use std::fmt::Display;

use crate::{
    grammar::{expression, statement},
    visitors::visitor::{Visitable, Visitor},
};

// definitions

#[derive(Debug, Clone)]

pub struct Initialization {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Instantiation {
    pub name: String,
    pub definition: expression::Expression,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub stmt: statement::Statement,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Init(Initialization),
    Inst(Instantiation),
    Stmt(Statement),
}

// constructors

impl Declaration {
    pub fn initialization(name: String) -> Self {
        Declaration::Init(Initialization { name })
    }

    pub fn instantiation(name: String, definition: expression::Expression) -> Self {
        Declaration::Inst(Instantiation { name, definition })
    }

    pub fn statement(stmt: statement::Statement) -> Self {
        Declaration::Stmt(Statement { stmt })
    }
}

// impl Display

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Init(x) => x.fmt(f),
            Declaration::Inst(x) => x.fmt(f),
            Declaration::Stmt(x) => x.fmt(f),
        }
    }
}

impl Display for Initialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "var {};", self.name)
    }
}

impl Display for Instantiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "var {} = {};", self.name, self.definition)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stmt)
    }
}

// impl Visitable

impl<R> Visitable<R> for Declaration {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        match self {
            Declaration::Init(x) => x.accept(v),
            Declaration::Inst(x) => x.accept(v),
            Declaration::Stmt(x) => x.accept(v),
        }
    }
}

impl<R> Visitable<R> for Initialization {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_init_decl(self)
    }
}

impl<R> Visitable<R> for Instantiation {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_inst_decl(self)
    }
}

impl<R> Visitable<R> for Statement {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_stmt_decl(self)
    }
}
