use crate::parser::Expression;

pub(crate) struct ComparisonExpression {

}
impl Expression for ComparisonExpression {
    fn evaluate(&self) -> bool {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}