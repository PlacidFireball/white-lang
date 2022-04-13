use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::*;
use crate::runtime::Runtime;

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

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
impl ReturnStatement {
    pub fn new(expr: Box<dyn Expression>, function: String) -> ReturnStatement {
        let return_type = expr.get_white_type();
        ReturnStatement {
            expr,
            return_type,
            function,
            errors: vec![],
        }
    }
}
