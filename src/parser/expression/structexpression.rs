use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

/*
// struct definition
struct __NAME__ {
    __field__: type,
    __field__: type,
    ...
} implement __NAME__ {
    fn some_fn(args) {}
    ...
}

// struct expression
let x : __NAME__ = __NAME__ ( __field__ = value , __field__ = value ... )
x.some_fn(args)
*/

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct StructExpression {
    name: String,
    typ: String,
    fields: HashMap<String, Box<dyn Expression>>,
    methods: HashMap<String, FunctionDefinitionStatement>,
}
impl ToAny for StructExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

#[allow(dead_code, unused_variables)]
impl Expression for StructExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        todo!()
    }

    fn debug(&self) -> String {
        todo!()
    }

    fn get_white_type(&self) -> Type {
        todo!()
    }

    fn get_expr_type(&self) -> String {
        todo!()
    }
}

#[allow(dead_code)]
impl StructExpression {
    pub fn new(name: String, typ: String) -> StructExpression {
        Self {
            name,
            typ,
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, field_name: String, expression: Box<dyn Expression>) {
        self.fields.insert(field_name, expression.clone());
    }

    pub fn add_method(&mut self, method_name: String, method: FunctionDefinitionStatement) {
        self.methods.insert(method_name, method);
    }
}
