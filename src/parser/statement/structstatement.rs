use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;

#[derive(Clone, Debug)]
struct StructStatement {
    name: String,
    typ: String,
    fields: HashMap<String, Type>,
    methods: HashMap<String, FunctionDefinitionStatement>,
}
impl ToAny for StructStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}
impl Statement for StructStatement {
    fn execute(&self, runtime: &mut Runtime) {
        todo!()
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        todo!()
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        todo!()
    }

    fn has_errors(&self) -> bool {
        todo!()
    }
}
impl StructStatement {
    pub fn new(name: String, typ: String) -> StructStatement {
        Self {
            name,
            typ,
            fields: HashMap::new(),
            methods: HashMap::new()
        }
    }

    pub fn add_field(&mut self, field_name: String, typ: Type) {
        self.fields.insert(field_name, typ);
    }

    pub fn add_method(&mut self, method_name: String, method: FunctionDefinitionStatement) {
        self.methods.insert(method_name, method);
    }
}