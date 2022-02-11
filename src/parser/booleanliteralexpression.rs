
use crate::parser::Expression;

struct BooleanLiteralExpression {
    boolean: bool
}
impl Expression for BooleanLiteralExpression {
    fn evaluate<bool>(&self) -> bool {
        self.boolean
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}