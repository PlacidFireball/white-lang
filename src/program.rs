use crate::parser;
use crate::parser::parser_traits::{Expression, Statement};
use crate::parser::ParserErrorType;

struct Program {
    statements: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
    errors: Vec<ParserErrorType>
}
impl Program {
    pub fn from_parser(parser: &mut parser::Parser) -> Self {
        todo!()
    }
}
