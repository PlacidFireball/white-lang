use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType::MismatchedTypes;
use crate::runtime::Runtime;
use crate::LOGGER;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct ListLiteralExpression {
    exprs: Vec<Box<dyn Expression>>,
    typ: Type,
}

impl ToAny for ListLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for ListLiteralExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let mut evals: Vec<Box<dyn Any>> = vec![];
        for expr in &self.exprs {
            evals.push(expr.evaluate(runtime));
        }
        Box::new(evals)
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        if self.exprs.is_empty() {
            return;
        }
        self.typ = self.exprs[0].get_white_type().get_list_type();
        for expr in &mut self.exprs {
            expr.validate(st);
            if expr.get_white_type() != self.typ.get_type_from_list() {
                add_parser_error(
                    MismatchedTypes,
                    format!("All items in the list must be of the same type."),
                );
            }
        }
        LOGGER.info(format!("Validated a list literal expression: {:?}", self))
    }

    fn debug(&self) -> String {
        let mut builder = String::from("[");
        for x in 0..self.exprs.len() {
            builder += &*self.exprs[x].debug();
            if x != self.exprs.len() - 1 {
                builder += ", "
            }
        }
        builder += "]";
        builder
    }

    fn get_white_type(&self) -> Type {
        self.typ
    }

    fn get_expr_type(&self) -> String {
        String::from("ListLiteralExpression")
    }
}
impl ListLiteralExpression {
    pub fn new() -> ListLiteralExpression {
        ListLiteralExpression {
            exprs: vec![],
            typ: Type::Initialized,
        }
    }
    pub fn add_expr(&mut self, expr: Box<dyn Expression>) {
        self.exprs.push(expr);
    }
}
