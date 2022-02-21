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
    function: String,
    errors: Vec<ParserErrorType>,
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

    fn validate(&mut self, st: &mut SymbolTable) {
        let fds = st.get_function(self.function.clone()).unwrap();
        if self.return_type != fds.get_return_type() {
            self.errors.push(ParserErrorType::BadReturnType);
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("ReturnStatement")
    }
}
impl ReturnStatement {
    pub fn new(expr: Box<dyn Expression>, function: String) -> ReturnStatement {
        let return_type = expr.get_white_type();
        ReturnStatement {
            expr,
            return_type,
            function,
            errors: vec![]
        }
    }
    pub fn new_no_fn(expr: Box<dyn Expression>) -> ReturnStatement {
        let return_type = expr.get_white_type();
        ReturnStatement {
            expr,
            return_type,
            function: String::new(),
            errors: vec![]
        }
    }

    pub fn set_fds(&mut self, func: String) {
        self.function = func;
    }
}
