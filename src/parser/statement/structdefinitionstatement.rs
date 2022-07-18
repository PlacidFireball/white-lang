use crate::parser::ParserErrorType;
use crate::parser::parser_traits::{Expression, Statement, ToAny, add_parser_error};
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
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
    methods: HashMap<String, FunctionDefinitionStatement>,
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

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        if st.has_symbol(self.name.clone()) {
            add_parser_error(ParserErrorType::DuplicateName, format!("Duplicate name `{}`", self.name));
        }
        st.register_symbol(self.name.clone(), self.typ.clone());
        for (name, typ) in self.fields.iter() {
            st.register_symbol(format!("{}.{}", self.name, name), typ.clone());
        }
        for (name, method) in self.methods.iter() {
            let mut func = method.clone();
            func.name = format!("{}.{}", self.name, name);
            func.validate(st);
            st.register_function(func.name.clone(), func);
        }
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

    pub fn add_method(&mut self, method_name: String, method: FunctionDefinitionStatement) {
        self.methods.insert(method_name, method);
    }
}
