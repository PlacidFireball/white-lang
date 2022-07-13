use crate::parser::parser_traits::{
    add_parser_error, any_into_bool_literal, any_into_f64_literal, any_into_int_literal,
    try_print_output, Expression, ToAny,
};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::{ArgMismatch, IncompatibleTypes, UnknownName};
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
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
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let fds = runtime.get_function(self.name.clone());
        let mut evaluated_args: Vec<Box<dyn Expression>> = vec![];
        for expr in &self.args {
            let eval = expr.evaluate(runtime);
            let mut tmp = expr.clone();
            if let Some(integer) = any_into_int_literal(&eval) {
                tmp = Box::new(integer);
            }
            if let Some(float) = any_into_f64_literal(&eval) {
                tmp = Box::new(float);
            }
            if let Some(boolean) = any_into_bool_literal(&eval) {
                tmp = Box::new(boolean);
            }
            evaluated_args.push(tmp);
        }
        assert_eq!(self.args.len(), evaluated_args.len());
        /*for (i, arg) in evaluated_args.iter().enumerate() {
            println!(
                "[FNCALL: {}]: Arg Name: {}\tValue: {}",
                self.name,
                fds.get_arg_names()[i],
                try_print_output(&arg.evaluate(runtime))
            );
        }*/
        println!(
            "[FUNCTION CALL] invoking {}...\n| args: {:?}",
            self.name, self.args
        );
        let value = fds.invoke(runtime, evaluated_args);
        runtime.pop_scope();
        value
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        let fds_opt = st.get_function(self.name.clone());
        if fds_opt.is_none() {
            add_parser_error(UnknownName);
            self.typ = Type::Null; // TODO: default typing (maybe Object)
        } else {
            let mut fds = fds_opt.unwrap();
            let args = fds.get_args();
            if self.args.len() != args.len() {
                add_parser_error(ArgMismatch);
            } else {
                for i in 0..args.len() {
                    let arg = &mut args[i];
                    arg.validate(st);
                    let param_type = self.args[i].get_white_type();
                    if !param_type.is_assignable_from(arg.get_white_type()) {
                        add_parser_error(IncompatibleTypes);
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
        self.typ
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
    #[allow(dead_code)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
