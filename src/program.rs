use std::any::Any;
use crate::parser;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::{Expression, Statement};
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
    pub output: String,
    errors: Vec<ParserErrorType>,
}
impl Program {
    pub fn from_parser(parser: &mut parser::Parser) -> Self {
        if let Some(statements) = parser.get_statements() {
            return Program {
                statements: statements.clone(),
                expr: Box::new(SyntaxErrorExpression::new()),
                output: String::new(),
                errors: vec![]
            };
        }
        if let Some(expr) = parser.get_expr() {
            return Program {
                statements: vec![],
                expr: expr.clone(),
                output: String::new(),
                errors: vec![]
            };
        }
        panic!("Uncaught parser error...")
    }

    pub fn execute(&mut self) {
        if self.statements.is_empty() {
            let eval = self.expr.evaluate(&Runtime::new());
            self.try_print_output(eval);
        } else {
            for statement in &self.statements {
                statement.execute();
            }
        }
    }

    fn try_print_output(&mut self, evaluated : Box<dyn Any>) {
        if let Some(eval_f64) = evaluated.downcast_ref::<f64>() {
            self.output.push_str(eval_f64.to_string().as_str());
        }
        else if let Some(eval_isize) = evaluated.downcast_ref::<isize>() {
            self.output.push_str(eval_isize.to_string().as_str());
        }
        self.output.push_str("\n");
    }
}
