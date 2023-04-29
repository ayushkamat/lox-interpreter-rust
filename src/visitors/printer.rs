use crate::{
    grammar::{declaration, expression, statement},
    visitors::visitor::{Visitable, Visitor},
};

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_literal_expr(&mut self, expr: &mut expression::Literal) -> String {
        format!("{}", expr)
    }
    fn visit_grouping_expr(&mut self, expr: &mut expression::Grouping) -> String {
        format!("{}", expr)
    }
    fn visit_unary_expr(&mut self, expr: &mut expression::Unary) -> String {
        format!("{}", expr)
    }
    fn visit_binary_expr(&mut self, expr: &mut expression::Binary) -> String {
        format!("{}", expr)
    }
    fn visit_ternary_expr(&mut self, expr: &mut expression::Ternary) -> String {
        format!("{}", expr)
    }
    fn visit_var_expr(&mut self, expr: &mut expression::Variable) -> String {
        format!("{}", expr)
    }

    fn visit_expr_stmt(&mut self, stmt: &mut statement::Expression) -> String {
        format!("{}", stmt)
    }
    fn visit_print_stmt(&mut self, stmt: &mut statement::Print) -> String {
        format!("{}", stmt)
    }

    fn visit_init_decl(&mut self, decl: &mut declaration::Initialization) -> String {
        format!("{}", decl)
    }
    fn visit_inst_decl(&mut self, decl: &mut declaration::Instantiation) -> String {
        format!("{}", decl)
    }
    fn visit_stmt_decl(&mut self, decl: &mut declaration::Statement) -> String {
        format!("{}", decl)
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: &mut expression::Expression) {
        println!("{}", expr.accept(self))
    }
}
