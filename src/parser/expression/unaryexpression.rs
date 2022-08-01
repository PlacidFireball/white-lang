use crate::config::{WhiteLangFloat, WhiteLangInt};
use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;
use crate::javascript::JavaScript;

#[derive(Clone, Debug)]
pub(crate) struct UnaryExpression {
    operator: String,
    expr: Box<dyn Expression>,
    is_not: bool,
}

impl ToAny for UnaryExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        println!("[UNARY EXPR] eval: {:?}", self);
        if self.is_not {
            let eval = self.expr.evaluate(runtime);
            if let Some(eval_bool) = eval.downcast_ref::<bool>() {
                return Box::new(!eval_bool);
            }
        } else {
            let eval = self.expr.evaluate(runtime);
            if let Some(eval_isize) = eval.downcast_ref::<WhiteLangInt>() {
                return Box::new(-1 * eval_isize);
            }
            if let Some(eval_f64) = eval.downcast_ref::<WhiteLangFloat>() {
                return Box::new(-1 as WhiteLangFloat * eval_f64);
            }
        }
        unimplemented!()
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        if self.is_not {
            javascript.append(String::from("!"));
            self.expr.transpile(javascript);
        } else {
            javascript.append(String::from("-"));
            self.expr.transpile(javascript);
        }
    }

    fn validate(&mut self, _st: &SymbolTable) {
        if self.operator == "not"
            && (self.expr.get_white_type() == Type::Integer
                || self.expr.get_white_type() == Type::Float)
        {
            add_parser_error(
                ParserErrorType::BadOperator(self.operator.clone()),
                format!("You cannot use `not` on numerical types."),
            );
        }
        if self.operator == "-" && self.expr.get_white_type() == Type::Boolean {
            add_parser_error(
                ParserErrorType::BadOperator(self.operator.clone()),
                format!("You cannot use `-` on boolean types."),
            );
        }
    }

    fn debug(&self) -> String {
        let mut builder = self.operator.clone();
        builder += &*self.expr.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        self.expr.get_white_type()
    }

    fn get_expr_type(&self) -> String {
        todo!()
    }
}
impl UnaryExpression {
    pub(crate) fn new(operator: String, expr: Box<dyn Expression>) -> UnaryExpression {
        UnaryExpression {
            operator: operator.clone(),
            expr,
            is_not: operator.contains("not"),
        }
    }
}
