use crate::parser::FunctionDefinitionStatement;
use crate::parser::*;
use crate::parser_traits::ToAny;
use crate::symbol_table::SymbolTable;
use std::mem::{uninitialized, MaybeUninit};

// TODO: The FunctionDefinitionStatement lifetime really needs to not be static

#[derive(Clone)]
pub(crate) struct ReturnStatement {
    expr: Box<dyn Expression>,
    return_type: Type,
    function: &'static FunctionDefinitionStatement,
}

impl ToAny for ReturnStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) -> String {
        todo!()
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        String::from("ReturnStatement")
    }
}
impl ReturnStatement {
    pub fn new(expr: Box<dyn Expression>) -> ReturnStatement {
        ReturnStatement {
            expr,
            return_type: Type::Void,
            function: todo!(),
        }
    }

    pub fn set_fds(&mut self, func: &'static FunctionDefinitionStatement) {
        self.function = func;
    }
}
