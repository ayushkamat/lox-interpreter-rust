use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::grammar::{declaration, expression, statement};

use crate::{
    value::Value,
    visitors::visitor::{Visitable, Visitor},
};
pub struct Interpreter {
    pub environment: Environment,
}

impl Visitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_literal_expr(
        &mut self,
        expr: &mut expression::Literal,
    ) -> Result<Value, RuntimeError> {
        let res = match expr {
            expression::Literal::String(s) => Value::String(s.clone()),
            expression::Literal::Number(n) => Value::Number(*n),
            expression::Literal::True => Value::Boolean(true),
            expression::Literal::False => Value::Boolean(false),
            expression::Literal::Nil => Value::Nil,
        };
        Ok(res)
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &mut expression::Grouping,
    ) -> Result<Value, RuntimeError> {
        expr.expr.accept(self)
    }

    fn visit_unary_expr(&mut self, expr: &mut expression::Unary) -> Result<Value, RuntimeError> {
        let inner = expr.expr.accept(self)?;
        let res = match expr.operator {
            expression::UnaryOperator::Not => match inner {
                Value::Boolean(t) => Value::Boolean(!t),
                Value::Nil => Value::Boolean(true),
                _ => Value::Boolean(false), // every value except for `false` and `nil` is truthy
            },
            expression::UnaryOperator::Minus => match inner {
                Value::Number(n) => Value::Number(-n),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot negate non-numerical valuef".to_string(),
                        expression::Expression::Unary(expr.clone()),
                    ))
                }
            },
        };
        Ok(res)
    }

    fn visit_binary_expr(&mut self, expr: &mut expression::Binary) -> Result<Value, RuntimeError> {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;

        let res = match expr.operator {
            expression::BinaryOperator::Comma => right,
            expression::BinaryOperator::Divide => match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0.0 {
                        return Err(RuntimeError::new(
                            "cannot divide by 0".to_string(),
                            expression::Expression::Binary(expr.clone()),
                        ));
                    }
                    Value::Number(l / r)
                }
                _ => {
                    return Err(RuntimeError::new(
                        "cannot divide non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::Equal => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l == r),
                (Value::String(l), Value::String(r)) => Value::Boolean(l == r),
                (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l == r),
                (Value::Nil, Value::Nil) => Value::Boolean(true),
                _ => Value::Boolean(false),
            },
            expression::BinaryOperator::Greater => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l > r),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot compare non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::GreaterEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l >= r),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot compare non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::Less => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l < r),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot compare non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::LessEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l <= r),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot compare non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::Minus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot subtract non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::Multiply => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot multiply non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
            expression::BinaryOperator::NotEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l != r),
                (Value::String(l), Value::String(r)) => Value::Boolean(l != r),
                (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l != r),
                (Value::Nil, Value::Nil) => Value::Boolean(false),
                _ => Value::Boolean(true),
            },
            expression::BinaryOperator::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                (Value::String(l), Value::String(r)) => Value::String((l + &r).clone()),
                _ => {
                    return Err(RuntimeError::new(
                        "cannot add non-numeric types".to_string(),
                        expression::Expression::Binary(expr.clone()),
                    ))
                }
            },
        };
        Ok(res)
    }

    fn visit_ternary_expr(
        &mut self,
        expr: &mut expression::Ternary,
    ) -> Result<Value, RuntimeError> {
        let cond = expr.condition.accept(self)?;
        match cond {
            Value::Boolean(false) | Value::Nil => expr.right_branch.accept(self),
            _ => expr.left_branch.accept(self),
        }
    }

    fn visit_var_expr(&mut self, expr: &mut expression::Variable) -> Result<Value, RuntimeError> {
        let val = self.environment.get(expr.name.clone());
        match val {
            Some(val) => match val {
                Value::Void => Err(RuntimeError::new(
                    format!("variable `{}` has does not have a value", expr.name),
                    expression::Expression::Variable(expr.clone()),
                )),
                _ => Ok(val),
            },
            None => Err(RuntimeError::new(
                format!("unknown variable `{}`", expr.name),
                expression::Expression::Variable(expr.clone()),
            )),
        }
    }

    fn visit_expr_stmt(&mut self, stmt: &mut statement::Expression) -> Result<Value, RuntimeError> {
        let _ = stmt.expr.accept(self)?;
        Ok(Value::Void)
    }

    fn visit_print_stmt(&mut self, stmt: &mut statement::Print) -> Result<Value, RuntimeError> {
        let ret = stmt.expr.accept(self)?;
        println!("{}", ret);
        Ok(Value::Void)
    }

    fn visit_init_decl(
        &mut self,
        decl: &mut declaration::Initialization,
    ) -> Result<Value, RuntimeError> {
        self.environment.add_name(decl.name.clone());
        Ok(Value::Void)
    }

    fn visit_inst_decl(
        &mut self,
        decl: &mut declaration::Instantiation,
    ) -> Result<Value, RuntimeError> {
        let val = decl.definition.accept(self)?;
        self.environment.set_value(decl.name.clone(), val);
        Ok(Value::Void)
    }

    fn visit_stmt_decl(
        &mut self,
        decl: &mut declaration::Statement,
    ) -> Result<Value, RuntimeError> {
        decl.stmt.accept(self)?;
        Ok(Value::Void)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn evaluate(&mut self, decl: &mut declaration::Declaration) -> Result<Value, RuntimeError> {
        decl.accept(self)
    }

    pub fn interpret(&mut self, decl: &mut declaration::Declaration) {
        let res = self.evaluate(decl);
        match res {
            Ok(v) => {
                if v != Value::Void {
                    println!("{}", v)
                }
            }
            Err(e) => println!("{}", e),
        };
    }
}
