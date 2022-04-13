use crate::parser::expression::functioncallexpression::FunctionCallExpression;
use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::symbol_table::SymbolTable;
use std::any::Any;
use crate::runtime::Runtime;

#[derive(Clone)]
pub struct FunctionCallStatement {
    name: String,
    expr: Box<dyn Expression>,
    fds: FunctionDefinitionStatement,
    args: Vec<Box<dyn Expression>>,
}

impl ToAny for FunctionCallStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for FunctionCallStatement {
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
        self.expr.validate(st);
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("FunctionCallStatement")
    }

    fn has_errors(&self) -> bool {
        !self.expr.has_errors()
    }
}

impl FunctionCallStatement {
    pub(crate) fn new(expr: Box<dyn Expression>, fds: FunctionDefinitionStatement) -> Self {
        let fce = expr
            .to_any()
            .downcast_ref::<FunctionCallExpression>()
            .unwrap();
        FunctionCallStatement {
            name: fce.get_name(),
            expr: expr.clone(),
            fds,
            args: fce.get_args().clone(),
        }
    }
}
