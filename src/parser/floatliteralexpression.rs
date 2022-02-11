use crate::parser::Expression;

pub(crate) struct FloatLiteralExpression {

}
impl Expression for FloatLiteralExpression {
    fn evaluate(&self) -> f64 {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}