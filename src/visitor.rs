use crate::grammar::{self, Visitable};

pub trait Visitor<R> {
    fn visit_literal_expr(&self, expr: &grammar::Literal) -> R;
    fn visit_grouping_expr(&self, expr: &grammar::Grouping) -> R;
    fn visit_unary_expr(&self, expr: &grammar::Unary) -> R;
    fn visit_binary_expr(&self, expr: &grammar::Binary) -> R;
    fn visit_ternary_expr(&self, expr: &grammar::Ternary) -> R;
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_literal_expr(&self, expr: &grammar::Literal) -> String {
        return format!("{}", expr);
    }
    fn visit_grouping_expr(&self, expr: &grammar::Grouping) -> String {
        return format!("{}", expr);
    }
    fn visit_unary_expr(&self, expr: &grammar::Unary) -> String {
        return format!("{}", expr);
    }
    fn visit_binary_expr(&self, expr: &grammar::Binary) -> String {
        return format!("{}", expr);
    }
    fn visit_ternary_expr(&self, expr: &grammar::Ternary) -> String {
        return format!("{}", expr);
    }
}

impl AstPrinter {
    pub fn print(&self, expr: grammar::Expression) {
        println!("{}", expr.accept(self))
    }
}
