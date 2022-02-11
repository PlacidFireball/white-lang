use crate::parser::Expression;

pub(crate) struct UnaryExpression {

}
impl Expression for UnaryExpression {
    fn evaluate(&self) -> Self::Object {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}