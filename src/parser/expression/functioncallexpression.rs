use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::{ArgMismatch, UnknownName};
use std::any::Any;

#[derive(Clone)]
pub(crate) struct FunctionCallExpression {
    name: String,
    args: Vec<Box<dyn Expression>>,
    typ: Type,
    errors: Vec<ParserErrorType>,
}

impl ToAny for FunctionCallExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for FunctionCallExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        let fds_opt = st.get_function(self.name.clone());
        if fds_opt.is_none() {
            self.errors.push(UnknownName);
            self.typ = Type::Null; // TODO: default typing (maybe Object)
        } else {
            let mut fds = fds_opt.unwrap();
            let args = fds.get_args();
            if self.args.len() != args.len() {
                self.errors.push(ArgMismatch);
            } else {
                for i in 0..args.len() {
                    let arg = &mut args[i];
                    arg.validate(st);
                    let param_type = self.args[i].get_white_type();
                    if !param_type.is_assignable_from(arg.get_white_type()) {
                        self.errors.push(ParserErrorType::IncompatibleTypes);
                    }
                }
            }
        }
    }

    fn debug(&self) -> String {
        let mut builder: String = String::new();
        builder.push_str(&*self.name);
        builder.push_str(": ");
        for arg in &self.args {
            builder.push_str(arg.debug().as_str());
            builder.push_str(" ")
        }
        builder
    }

    fn get_white_type(&self) -> Type {
        todo!()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        String::from("FunctionCallExpression")
    }
}
impl FunctionCallExpression {
    pub fn new(name: String) -> FunctionCallExpression {
        FunctionCallExpression {
            name,
            args: vec![],
            typ: Type::Initialized,
            errors: vec![],
        }
    }

    pub fn add_arg(&mut self, arg: Box<dyn Expression>) {
        self.args.push(arg);
    }
    pub fn get_args(&self) -> &Vec<Box<dyn Expression>> {
        &self.args
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
