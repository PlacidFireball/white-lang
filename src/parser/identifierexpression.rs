use crate::parser::Expression;

pub(crate) struct IdentifierExpression {
    name: String
}
impl Expression for IdentifierExpression {
    fn evaluate(&self) -> String {
        self.name.clone()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}