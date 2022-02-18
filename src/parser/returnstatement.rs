use crate::parser::FunctionDefinitionStatement;
use crate::parser::*;

pub(crate) struct ReturnStatement {
    expr: Box<dyn Expression>,
    return_type: Type,
    //function: &'static FunctionDefinitionStatement,
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

    fn validate(&self, st: &SymbolTable) -> String {
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
    pub fn new(
        expr: Box<dyn Expression>,
        function: &FunctionDefinitionStatement,
    ) -> ReturnStatement {
        ReturnStatement {
            expr,
            return_type: Type::Void,
            //function
        }
    }
}
