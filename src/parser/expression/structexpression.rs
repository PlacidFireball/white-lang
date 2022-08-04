use crate::javascript::JavaScript;
use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::whitetypes::Type::Struct;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::UnknownName;
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
    typ: Type,
    fields: HashMap<String, Box<dyn Expression>>,
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

    fn transpile(&self, javascript: &mut JavaScript) {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        let mut struct_id = String::new();
        match self.typ.clone() {
            Struct(s) => struct_id = s,
            _ => add_parser_error(
                ParserErrorType::BadType(self.typ.clone()),
                format!("Expected a struct, got {:?}", self.typ),
            ),
        }
        let strct = match st.get_struct(struct_id.clone()) {
            Some(s) => s,
            None => {
                add_parser_error(
                    UnknownName(struct_id.clone()),
                    format!(
                        "Unable to retrieve struct type: {} from the symbol_table",
                        struct_id
                    ),
                );
                unreachable!()
            },
        };
        for (name, expr) in self.fields.iter() {
            let expected_typ = match strct.get_field_type(name.clone()) {
                Some(t) => t,
                None => {
                    add_parser_error(
                        UnknownName(name.clone()),
                        format!("No such field `{}` on struct `{}`", name, struct_id),
                    );
                    Type::Error // this will never be reached, but gotta appease the compiler
                },
            };
            if !expected_typ.is_assignable_to(expr.get_white_type()) {
                add_parser_error(
                    ParserErrorType::IncompatibleTypes(expected_typ.clone(), expr.get_white_type()),
                    format!(
                        "{:?} is not assignable to {:?}",
                        expr.get_white_type(),
                        expected_typ
                    ),
                )
            }
        }
        for (name, fds) in strct.methods.iter() {
            let method_name_pure = name.split(".").last().unwrap().to_string();
            st.register_function(format!("{}.{}", self.name, method_name_pure), fds.clone());
        }
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
    pub fn new(name: String, typ: Type) -> StructExpression {
        Self {
            name,
            typ,
            fields: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, field_name: String, expression: Box<dyn Expression>) {
        self.fields.insert(field_name, expression.clone());
    }
}
