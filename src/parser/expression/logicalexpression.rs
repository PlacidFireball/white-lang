use crate::javascript::JavaScript;
use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::MismatchedTypes;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct LogicalExpression {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
    operator: String,
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

    fn transpile(&self, javascript: &mut JavaScript) {
        self.lhs.transpile(javascript);
        javascript.append_no_tabs(format!(" {} ", self.operator));
        self.rhs.transpile(javascript);
    }

    fn validate(&mut self, _st: &mut SymbolTable) {
        if self.operator.ne("&&") && self.operator.ne("||") {
            add_parser_error(
                ParserErrorType::BadOperator(self.operator.clone()),
                format!("Operator: {} is not valid here", self.operator),
            );
        }
        if self.lhs.get_white_type() != self.rhs.get_white_type()
            && self.lhs.get_white_type() != Type::Boolean
        {
            add_parser_error(MismatchedTypes(self.lhs.get_white_type(), self.rhs.get_white_type()), format!("You cannot and/or two expressions that do not evaluate to booleans. lhs: {:?} rhs: {:?}", self.lhs, self.rhs));
        }
    }

    fn debug(&self) -> String {
        String::from("lol debug :)")
    }

    fn get_white_type(&self) -> Type {
        Type::Boolean
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
