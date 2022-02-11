
use crate::parser::Expression;

pub(crate) struct BooleanLiteralExpression {
    boolean: bool
}
impl Expression for BooleanLiteralExpression {

    fn evaluate(&self) -> bool {
        self.boolean
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}