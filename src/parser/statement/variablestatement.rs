use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::SymbolDefinitionError;
use crate::parser::parser_traits::default_expr;
use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::symbol_table::SymbolTable;
use std::any::Any;

#[derive(Clone)]
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
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.expr.validate(st);
        if st.has_symbol(self.name.clone()) {
            self.errors.push(SymbolDefinitionError);
        }
        if self.typ == Type::Error {
            self.errors.push(ParserErrorType::UnexpectedToken);
        }
        if !self.has_errors() {
            st.register_symbol(self.name.clone(), self.typ);
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        todo!()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
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
