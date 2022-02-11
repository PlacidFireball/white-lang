use crate::parser::Expression;


pub(crate) struct AdditiveExpression {

}
impl Expression for AdditiveExpression {
    fn evaluate(&self) -> f64 {
        todo!()
    }

    fn compile(&self) -> String {
        String::from("")
    }

    fn transpile(&self) -> String {
        String::from("")
    }
}