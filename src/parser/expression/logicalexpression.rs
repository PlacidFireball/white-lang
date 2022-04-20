use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct LogicalExpression {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
    operator: String,
    errors: Vec<ParserErrorType>,
}
#[allow(unused_variables)]
impl ToAny for LogicalExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for LogicalExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let lhs_eval = self.lhs.evaluate(runtime);
        let rhs_eval = self.rhs.evaluate(runtime);
        let lhs_to_bool = *lhs_eval.downcast_ref::<bool>().unwrap();
        let rhs_to_bool = *rhs_eval.downcast_ref::<bool>().unwrap();
        if self.is_and() {
            return Box::new(lhs_to_bool && rhs_to_bool);
        } else {
            return Box::new(lhs_to_bool || rhs_to_bool);
        }
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, _st: &SymbolTable) {
        if self.operator.ne("&&") && self.operator.ne("||") {
            self.errors.push(ParserErrorType::BadOperator);
        }
        if self.lhs.get_white_type() != self.rhs.get_white_type()
            && self.lhs.get_white_type() != Type::Boolean
        {
            self.errors.push(ParserErrorType::MismatchedTypes)
        }
    }

    fn debug(&self) -> String {
        String::from("lol debug :)")
    }

    fn get_white_type(&self) -> Type {
        Type::Boolean
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        String::from("LogicalExpression")
    }
}
impl LogicalExpression {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        LogicalExpression {
            lhs,
            rhs,
            operator: String::new(),
            errors: Vec::new(),
        }
    }

    pub fn set_operator(&mut self, operator: String) {
        if operator.eq("&&") || operator.eq("||") {
            self.operator = operator;
        }
    }

    fn is_and(&self) -> bool {
        self.operator.contains("&&")
    }
}