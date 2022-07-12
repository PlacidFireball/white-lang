use crate::config::*;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::UnexpectedToken;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub struct IfStatement {
    true_stmts: Vec<Box<dyn Statement>>,
    false_stmts: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
}

impl ToAny for IfStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}
impl Statement for IfStatement {
    fn execute(&self, runtime: &mut Runtime) {
        let eval = self.expr.evaluate(runtime);
        let downcast = *eval.downcast_ref::<WhiteLangBool>().unwrap();
        runtime.push_scope(String::from("if"));
        if downcast {
            for statement in &self.true_stmts {
                statement.execute(runtime);
            }
        } else {
            for statement in &self.false_stmts {
                statement.execute(runtime);
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
           add_parser_error(UnexpectedToken);
        }
        st.push_scope();
        if !self.true_stmts.is_empty() {
            for i in 0..self.true_stmts.len() {
                // TODO: Body Validation, variables within blocks and for statements (iterated vars)
                self.true_stmts[i].validate(st);
            }
        }
        st.pop_scope();
        st.push_scope();
        if !self.false_stmts.is_empty() {
            for i in 0..self.false_stmts.len() {
                self.false_stmts[i].validate(st);
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
impl IfStatement {
    pub fn new() -> Self {
        IfStatement {
            true_stmts: vec![],
            false_stmts: vec![],
            expr: Box::new(SyntaxErrorExpression::new()),
            errors: vec![],
        }
    }
    pub(crate) fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
    pub(crate) fn add_true_statement(&mut self, stmt: Box<dyn Statement>) {
        self.true_stmts.push(stmt);
    }
    pub(crate) fn add_false_statement(&mut self, stmt: Box<dyn Statement>) {
        self.false_stmts.push(stmt);
    }
    pub(crate) fn get_true_stmts(&self) -> &Vec<Box<dyn Statement>> {
        &self.true_stmts
    }
    pub(crate) fn get_false_stmts(&self) -> &Vec<Box<dyn Statement>> {
        &self.false_stmts
    }
}
