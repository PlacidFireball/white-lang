use std::any::Any;
use crate::parser::ParserErrorType;
use crate::parser::whitetypes::Type;
use crate::parser_traits::{Expression, Statement, ToAny};
use crate::parser_traits::default_expr;
use crate::symbol_table::SymbolTable;

pub(crate) struct VariableStatement {
    name: String,
    expr: Box<dyn Expression>,
    typ: Type,
    errors: Vec<ParserErrorType>
}

impl ToAny for VariableStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for VariableStatement {
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) -> String {
        todo!()
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        todo!()
    }
}

impl VariableStatement {
    pub fn new(name: String) -> VariableStatement {
        VariableStatement {
            name,
            expr: default_expr(),
            typ: Type::Initialized,
            errors: vec![]
        }
    }
    pub fn set_type(&mut self, typ: Type) {
        if self.typ == Type::Initialized {
            self.typ = typ;
        }
        else if self.typ != typ {
            self.errors.push(ParserErrorType::MismatchedTypes);
        }

    }
    pub fn get_type(&self) -> Type {
        self.typ.clone()
    }
    pub fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
