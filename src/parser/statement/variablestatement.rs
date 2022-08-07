use crate::javascript::JavaScript;
use crate::parser::expression::functioncallexpression::FunctionCallExpression;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::whitetypes::Type::Initialized;
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
    fn to_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Statement for VariableStatement {
    fn execute(&mut self, runtime: &mut Runtime) {
        if self.expr.debug() == String::from("StructExpression") {
            self.expr.evaluate(runtime); // need to do this to get the functions and fields in the runtime
        }
        if let Some(fce) = self.expr.to_any().downcast_ref::<FunctionCallExpression>() {
            let eval = fce.evaluate(runtime);
            let mut tmp = self.expr.clone();
            if let Some(integer) = any_into_int_literal(&eval) {
                tmp = Box::new(integer);
            }
            if let Some(float) = any_into_f64_literal(&eval) {
                tmp = Box::new(float);
            }
            if let Some(boolean) = any_into_bool_literal(&eval) {
                tmp = Box::new(boolean);
            }
            if let Some(string) = any_into_string_literal(&eval) {
                tmp = Box::new(string);
            }
            self.expr = tmp;
        }
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
        if self.expr.debug() == "StructExpression" {
            self.expr.set_name(self.name.clone());
        }
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
        if self.typ == Type::Error || self.typ == Initialized {
            add_parser_error(BadType(self.typ.clone()), format!("Got error type."));
        }
        if !self.typ.is_assignable_to(self.expr.get_white_type()) {
            add_parser_error(
                MismatchedTypes(self.typ.clone(), self.expr.get_white_type()),
                format!("Types are not assignable"),
            );
        }
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
            typ: Initialized,
        }
    }
    pub fn set_type(&mut self, typ: Type) {
        let old_type = self.typ.clone();
        LOGGER.debug(
            format!("Set type of `{}` to {:?}", self.name, self.typ),
            false,
        );
        self.typ = typ;
        if old_type != Initialized {
            if old_type != self.typ.clone() {
                add_parser_error(
                    MismatchedTypes(old_type, self.typ.clone()),
                    format!("Attempt to set a bad type"),
                );
            }
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
