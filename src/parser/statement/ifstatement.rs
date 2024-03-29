use crate::config::*;
use crate::javascript::JavaScript;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::{parser_traits::*, ParserErrorType};
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct IfStatement {
    true_stmts: Vec<Box<dyn Statement>>,
    false_stmts: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
}

impl ToAny for IfStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
    fn to_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
impl Statement for IfStatement {
    fn execute(&mut self, runtime: &mut Runtime) {
        let eval = self.expr.evaluate(runtime);
        let downcast = *eval.downcast_ref::<WhiteLangBool>().unwrap();
        runtime.push_scope(String::from(uuid::Uuid::new_v4().to_string()));
        if downcast {
            for statement in &mut self.true_stmts {
                statement.execute(runtime);
            }
        } else {
            for statement in &mut self.false_stmts {
                statement.execute(runtime);
            }
        }
        //runtime.pop_scope(); // TODO: nested if in else causes double pop before return can evaluate the expression
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append(String::from("if ("));
        self.expr.transpile(javascript);
        javascript
            .append_no_tabs(String::from(") {"))
            .newline()
            .indent();
        for stmt in self.true_stmts.iter() {
            stmt.transpile(javascript);
        }
        javascript.newline().outdent().append(String::from("}"));
        if self.false_stmts.len() > 0 {
            javascript
                .newline()
                .append(String::from("else {"))
                .newline()
                .indent();
            for stmt in self.false_stmts.iter() {
                stmt.transpile(javascript);
            }
            javascript.outdent().newline().append(String::from("}"));
        }
        javascript.newline();
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.expr.validate(st);
        if self.expr.get_white_type() != Type::Boolean {
            add_parser_error(
                ParserErrorType::BadType(self.expr.get_white_type()),
                format!(
                    "You cannot branch based on type: {:?}",
                    self.expr.get_white_type()
                ),
            );
        }
        st.push_scope();
        if !self.true_stmts.is_empty() {
            for i in 0..self.true_stmts.len() {
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
}
#[allow(dead_code)]
impl IfStatement {
    pub fn new() -> Self {
        IfStatement {
            true_stmts: vec![],
            false_stmts: vec![],
            expr: Box::new(SyntaxErrorExpression::new()),
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
