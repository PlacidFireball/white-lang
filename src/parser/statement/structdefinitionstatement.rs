use crate::javascript::JavaScript;
use crate::parser::parser_traits::{add_parser_error, Expression, Statement, ToAny};
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct StructDefinitionStatement {
    pub name: String,
    typ: Type,
    fields: HashMap<String, Type>,
    pub(crate) methods: HashMap<String, FunctionDefinitionStatement>,
}
impl ToAny for StructDefinitionStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}
#[allow(dead_code, unused_variables)]
impl Statement for StructDefinitionStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.add_struct(self.name.clone(), self.clone());
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        if st.has_symbol(self.name.clone()) {
            add_parser_error(
                ParserErrorType::DuplicateName(
                    self.name.clone(),
                    st.get_symbol_type(self.name.clone()).unwrap(),
                ),
                format!("Duplicate name `{}`", self.name),
            );
        }
        for (name, typ) in self.fields.iter() {
            st.register_symbol(format!("{}.{}", self.name, name), typ.clone()); // do this because we don't want to clash y and x.y
        }
        for (name, method) in self.methods.iter() {
            let mut func = method.clone();
            func.name = format!("{}.{}", self.name, name); // do this because we don't want to clash foo() and x.foo()
            func.validate(st);
            st.register_function(func.name.clone(), func);
        }
        st.register_struct(self.name.clone(), self.clone());
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        String::from("StructDefinitionStatement")
    }
}
#[allow(dead_code)]
impl StructDefinitionStatement {
    pub fn new(name: String) -> StructDefinitionStatement {
        Self {
            name: name.clone(),
            typ: Type::Struct(name.clone()),
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, field_name: String, typ: Type) {
        self.fields.insert(field_name, typ);
    }

    pub fn get_field_type(&self, field_name: String) -> Option<Type> {
        match self.fields.get(&field_name) {
            Some(typ) => Some(typ.clone()),
            None => {
                crate::LOGGER.warn(format!("No such field: {}", field_name));
                None
            }
        }
    }

    pub fn add_method(&mut self, method_name: String, method: FunctionDefinitionStatement) {
        self.methods.insert(method_name, method);
    }

    pub fn get_method(&self, method_name: String) -> Option<FunctionDefinitionStatement> {
        match self.methods.get(&method_name) {
            Some(fds) => Some(fds.clone()),
            None => {
                crate::LOGGER.warn(format!("No such method `{}`", method_name));
                None
            }
        }
    }
}
