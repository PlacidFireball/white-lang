use crate::javascript::JavaScript;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType::*;
use crate::runtime::Runtime;
use crate::LOGGER;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct VariableStatement {
    name: String,
    expr: Box<dyn Expression>,
    typ: Type,
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

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append(format!("let {} = ", self.name));
        self.expr.transpile(javascript);
        javascript.semicolon().newline();
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.expr.validate(st);
        if st.has_symbol(self.name.clone()) {
            add_parser_error(
                DuplicateName(
                    self.name.clone(),
                    st.get_symbol_type(self.name.clone()).unwrap(),
                ),
                format!("Duplicate name: {}", self.name.clone()),
            );
        }
        if self.typ == Type::Error {
            add_parser_error(BadType(self.typ.clone()), format!("Got error type."));
        }
        LOGGER.info(format!(
            "Registering `{}` with type {:?}",
            self.name, self.typ
        ));
        st.register_symbol(self.name.clone(), self.typ.clone());
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
        }
    }
    pub fn set_type(&mut self, typ: Type) {
        if self.typ == Type::Initialized {
            LOGGER.info(format!("Set type of `{}` to {:?}", self.name, self.typ));
            self.typ = typ;
        } else if self.typ != typ {
            add_parser_error(
                MismatchedTypes(self.typ.clone(), typ),
                format!("Set bad type"),
            );
        }
    }

    pub fn set_expr(&mut self, expr: Box<dyn Expression>) {
        self.expr = expr;
    }
    #[allow(dead_code)]
    pub fn get_type(&self) -> Type {
        self.typ.clone()
    }
}
