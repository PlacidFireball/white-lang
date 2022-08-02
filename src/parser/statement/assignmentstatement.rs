use crate::parser::expression::identifierexpression::IdentifierExpression;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;

use crate::javascript::JavaScript;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct AssignmentStatement {
    variable: Box<dyn Expression>,
    expr: Box<dyn Expression>,
}

impl ToAny for AssignmentStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for AssignmentStatement {
    fn execute(&self, runtime: &mut Runtime) {
        let ident = self
            .variable
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .unwrap();
        if let Some(boolean) = any_into_bool_literal(&self.expr.evaluate(runtime)) {
            runtime.set_value(ident.debug(), Box::new(boolean.clone()));
        } else if let Some(integer) = any_into_int_literal(&self.expr.evaluate(runtime)) {
            runtime.set_value(ident.debug(), Box::new(integer.clone()));
        } else if let Some(float) = any_into_f64_literal(&self.expr.evaluate(runtime)) {
            runtime.set_value(ident.debug(), Box::new(float.clone()));
        } else {
            runtime.set_value(ident.debug(), self.expr.clone());
        }
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append("".to_string());
        self.variable.transpile(javascript);
        javascript.append_no_tabs(format!(" = "));
        self.expr.transpile(javascript);
        javascript.append_no_tabs(String::from(";")).newline();
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.variable.validate(st);
        self.expr.validate(st);
        if self
            .variable
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .is_none()
        {
            add_parser_error(
                ParserErrorType::UnexpectedExpression(self.variable.clone()),
                format!(
                    "Variable: [{}] is not defined in this scope",
                    self.variable.debug()
                ),
            );
        }
        if !self
            .expr
            .get_white_type()
            .is_assignable_to(self.variable.get_white_type())
        {
            add_parser_error(
                ParserErrorType::IncompatibleTypes(
                    self.expr.clone().get_white_type(),
                    self.variable.clone().get_white_type(),
                ),
                format!(
                    "You cannot assign {:?} to {:?}",
                    self.expr.get_white_type(),
                    self.variable.get_white_type()
                ),
            );
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("AssignmentStatement")
    }
}

impl AssignmentStatement {
    pub fn new() -> Self {
        AssignmentStatement {
            variable: Box::new(SyntaxErrorExpression::new()),
            expr: Box::new(SyntaxErrorExpression::new()),
        }
    }

    pub fn set_variable(&mut self, identifier: Box<dyn Expression>) {
        self.variable = identifier;
    }

    pub fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
}
