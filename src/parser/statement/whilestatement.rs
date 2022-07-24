use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::{add_parser_error, Expression, Statement, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use crate::LOGGER;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct WhileStatement {
    body: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
}

impl ToAny for WhileStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}
impl Statement for WhileStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.push_scope(String::from("while"));
        let mut iterations: usize = 0;
        let mut is_broken = false;
        let mut cond: bool = *self.expr.evaluate(runtime).downcast_ref::<bool>().unwrap();
        while cond {
            for statement in self.body.iter() {
                statement.execute(runtime);
                if runtime.get_break() {
                    is_broken = true; // interrogate break state
                    runtime.set_break(false); // set break state back to false
                    break;
                }
            }
            if is_broken {
                break;
            }
            cond = *self.expr.evaluate(runtime).downcast_ref::<bool>().unwrap();
            iterations += 1;
            if iterations > usize::MAX - 1 {
                LOGGER.error("Infinite loop!".to_string()); // idk about this one but I feel like this should be a feature
            }
        }
        runtime.pop_scope();
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.expr.validate(st);
        if self.expr.get_white_type() != Type::Boolean {
            add_parser_error(
                ParserErrorType::BadType(self.expr.get_white_type()),
                format!("Expected a boolean type to loop on."),
            );
        }
        st.push_scope();
        if !self.body.is_empty() {
            for i in 0..self.body.len() {
                self.body[i].validate(st);
            }
        }
        st.pop_scope();
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        String::from("IfStatement")
    }
}
#[allow(dead_code)]
impl WhileStatement {
    pub fn new() -> Self {
        WhileStatement {
            body: vec![],
            expr: Box::new(SyntaxErrorExpression::new()),
        }
    }
    pub(crate) fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
    pub(crate) fn add_body_statement(&mut self, stmt: Box<dyn Statement>) {
        self.body.push(stmt);
    }
    pub(crate) fn get_body(&self) -> &Vec<Box<dyn Statement>> {
        &self.body
    }
}
