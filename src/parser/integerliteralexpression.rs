use crate::parser::Expression;

pub(crate) struct IntegerLiteralExpression {
    value: isize
}
impl Expression for IntegerLiteralExpression {
    fn evaluate(&self) -> isize {
        self.value
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}