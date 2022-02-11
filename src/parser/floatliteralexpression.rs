use crate::parser::Expression;

struct FloatLiteralExpression {

}
impl Expression for FloatLiteralExpression {
    fn evaluate<f64>(&self) -> f64 {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}