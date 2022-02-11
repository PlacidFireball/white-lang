use crate::parser::Expression;

struct ComparisonExpression {

}
impl Expression for ComparisonExpression {
    fn evaluate<bool>(&self) -> bool {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}