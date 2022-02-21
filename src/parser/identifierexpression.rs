use crate::parser::whitetypes::Type;
use crate::parser_traits::{Expression, ToAny};
use crate::symbol_table::SymbolTable;
use std::any::Any;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::UnexpectedToken;

#[derive(Clone)]
pub(crate) struct IdentifierExpression {
    name: String,
    typ: Type,
    errors: Vec<ParserErrorType>
}

impl ToAny for IdentifierExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IdentifierExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.name.clone())
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        let opt_typ = st.get_symbol_type(self.name.clone());
        if opt_typ.is_some() {
            self.typ = opt_typ.unwrap();
        }
        if opt_typ.is_none() {
            self.typ = Type::Error;
            self.errors.push(UnexpectedToken);
        }
    }

    fn debug(&self) -> String {
        self.name.clone()
    }

    fn get_white_type(&self) -> Type {
        self.typ
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        String::from("IdentifierExpression")
    }
}
impl IdentifierExpression {
    pub fn new(name: String) -> IdentifierExpression {
        IdentifierExpression {
            name,
            typ: Type::Initialized,
            errors: vec![]
        }
    }
}
