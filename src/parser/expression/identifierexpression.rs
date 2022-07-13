use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType::UnknownName;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct IdentifierExpression {
    name: String,
    typ: Type,
}

impl ToAny for IdentifierExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IdentifierExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let debug_has_key: bool = runtime.has_symbol(self.name.clone());
        if debug_has_key {
            println!("Runtime has symbol: {}", self.name);
        }
        if let Some(eval) = runtime.get_value(self.name.clone()) {
            return eval;
        }
        panic!("Undefined variable `{}`", self.name);
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        let opt_typ = st.get_symbol_type(self.name.clone());
        if opt_typ.is_some() {
            self.typ = opt_typ.unwrap();
        }
        if opt_typ.is_none() {
            self.typ = Type::Error;
            add_parser_error(UnknownName);
        }
    }

    fn debug(&self) -> String {
        self.name.clone()
    }

    fn get_white_type(&self) -> Type {
        self.typ
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
        }
    }
}
