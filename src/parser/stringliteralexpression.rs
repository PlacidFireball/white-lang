
use crate::parser::Expression;

pub(crate) struct StringLiteralExpression {
    string_value: String
}
impl Expression for StringLiteralExpression {
    fn evaluate(&self) -> String {
        self.string_value.clone()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}