use crate::grammar::declaration;
use crate::grammar::expression;
use crate::grammar::statement;

pub trait Visitor<R> {
    fn visit_literal_expr(&mut self, expr: &mut expression::Literal) -> R;
    fn visit_grouping_expr(&mut self, expr: &mut expression::Grouping) -> R;
    fn visit_unary_expr(&mut self, expr: &mut expression::Unary) -> R;
    fn visit_binary_expr(&mut self, expr: &mut expression::Binary) -> R;
    fn visit_ternary_expr(&mut self, expr: &mut expression::Ternary) -> R;
    fn visit_var_expr(&mut self, expr: &mut expression::Variable) -> R;

    fn visit_print_stmt(&mut self, stmt: &mut statement::Print) -> R;
    fn visit_expr_stmt(&mut self, stmt: &mut statement::Expression) -> R;

    fn visit_init_decl(&mut self, decl: &mut declaration::Initialization) -> R;
    fn visit_inst_decl(&mut self, decl: &mut declaration::Instantiation) -> R;
    fn visit_stmt_decl(&mut self, decl: &mut declaration::Statement) -> R;
}

pub trait Visitable<R> {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R;
}
