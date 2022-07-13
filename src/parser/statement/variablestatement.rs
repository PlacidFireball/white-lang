use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::{MismatchedTypes, SymbolDefinitionError, UnexpectedToken};
use crate::runtime::Runtime;
use crate::IS_TESTING;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct VariableStatement {
    name: String,
    expr: Box<dyn Expression>,
    typ: Type,
    errors: Vec<ParserErrorType>,
}

impl ToAny for VariableStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for VariableStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.set_value(self.name.clone(), self.expr.clone());
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.expr.validate(st);
        if st.has_symbol(self.name.clone()) {
            add_parser_error(SymbolDefinitionError);
        }
        if self.typ == Type::Error {
            add_parser_error(UnexpectedToken);
        }
        if !self.has_errors() {
            st.register_symbol(self.name.clone(), self.typ);
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("VariableStatement")
    }
}

impl VariableStatement {
    pub fn new(name: String) -> VariableStatement {
        VariableStatement {
            name,
            expr: default_expr(),
            typ: Type::Initialized,
            errors: vec![],
        }
    }
    pub fn set_type(&mut self, typ: Type) {
        if self.typ == Type::Initialized {
            self.typ = typ;
        } else if self.typ != typ {
            if !IS_TESTING.with(|test| test.get()) {
                add_parser_error(MismatchedTypes);
            }
        }
    }

    pub fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn get_type(&self) -> Type {
        self.typ
    }
}
