use crate::parser::Expression;

pub(crate) struct SyntaxErrorExpression {

}
impl Expression for SyntaxErrorExpression {
    fn evaluate(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}
impl SyntaxErrorExpression {
    pub fn new() -> SyntaxErrorExpression {
        SyntaxErrorExpression { }
    }
}