use crate::parser::Expression;

struct AdditiveExpression {

}
impl Expression for AdditiveExpression {
    fn evaluate<f64>(&self) -> f64 {
        todo!()
    }

    fn compile(&self) -> String {
        String::from("")
    }

    fn transpile(&self) -> String {
        String::from("")
    }
}