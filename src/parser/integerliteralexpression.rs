use crate::parser::Expression;

struct IntegerLiteralExpression {

}
impl Expression for IntegerLiteralExpression {
    fn evaluate<isize>(&self) -> isize {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}