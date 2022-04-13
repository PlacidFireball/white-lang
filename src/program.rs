use crate::config::WhiteLangList;
use crate::parser;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::{Expression, Statement};
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;
use std::ops::Deref;

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
    runtime: Runtime,
    pub output: String,
    errors: Vec<ParserErrorType>,
}
impl Program {
    pub fn from_parser(parser: &mut parser::Parser) -> Self {
        if let Some(statements) = parser.get_statements() {
            return Program {
                statements: statements.clone(),
                expr: Box::new(SyntaxErrorExpression::new()),
                runtime: Runtime::new(),
                output: String::new(),
                errors: vec![],
            };
        }
        if let Some(expr) = parser.get_expr() {
            return Program {
                statements: vec![],
                expr: expr.clone(),
                runtime: Runtime::new(),
                output: String::new(),
                errors: vec![],
            };
        }
        panic!("Uncaught parser error...")
    }

    pub fn execute(&mut self) {
        if self.statements.is_empty() {
            let eval = self.expr.evaluate(&self.runtime);
            self.try_print_output(&eval);
            self.output.push_str("\n");
        } else {
            for statement in &self.statements {
                statement.execute(&self.runtime);
            }
        }
    }

    fn try_print_output(&mut self, evaluated: &Box<dyn Any>) {
        if let Some(eval_f64) = evaluated.downcast_ref::<f64>() {
            let push = eval_f64.to_string();
            self.output.push_str(push.as_str());
        } else if let Some(eval_isize) = evaluated.downcast_ref::<isize>() {
            let push = eval_isize.to_string();
            self.output.push_str(push.as_str());
        } else if let Some(eval_bool) = evaluated.downcast_ref::<bool>() {
            let push = eval_bool.to_string();
            self.output.push_str(push.as_str());
        } else if let Some(eval_str) = evaluated.downcast_ref::<&str>() {
            let push = eval_str.to_string();
            self.output.push_str(push.as_str());
        } else if let Some(eval_string) = evaluated.downcast_ref::<String>() {
            self.output.push_str(eval_string.as_str());
        } else if let Some(eval_list) = evaluated.downcast_ref::<WhiteLangList<Box<dyn Any>>>() {
            self.output.push_str("[");
            for (i, thing) in eval_list.iter().enumerate() {
                self.try_print_output(thing);
                if i < eval_list.len() - 1 {
                    self.output.push_str(", ");
                }
            }
            self.output.push_str("]");
        }
    }
}
