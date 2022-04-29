use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::UnexpectedToken;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub struct WhileStatement {
    body: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
}

impl ToAny for WhileStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}
impl Statement for WhileStatement {
    fn execute(&self, runtime: &mut Runtime) {
        let mut iterations : usize = 0;
        let mut cond : bool = *self.expr.evaluate(runtime).downcast_ref::<bool>().unwrap();
        while cond {
            for statement in self.body.iter() {
                statement.execute(runtime);
            }
            cond = *self.expr.evaluate(runtime).downcast_ref::<bool>().unwrap();
            iterations += 1;
            if iterations > usize::MAX - 1 {
                panic!("Infinite Loop!"); // idk about this one but I feel like this should be a feature
            }
        }
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
            self.errors.push(UnexpectedToken);
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

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
#[allow(dead_code)]
impl WhileStatement {
    pub fn new() -> Self {
        WhileStatement {
            body: vec![],
            expr: Box::new(SyntaxErrorExpression::new()),
            errors: vec![],
        }
    }
    pub(crate) fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
    pub(crate) fn add_body_statement(&mut self, stmt: Box<dyn Statement>) {
        self.body.push(stmt);
    }
    pub(crate) fn get_body(&self) -> &Vec<Box<dyn Statement>> { &self.body }
}
