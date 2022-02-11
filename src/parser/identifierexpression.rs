use crate::parser::Expression;

struct IdentifierExpression {

}
impl Expression for IdentifierExpression {
    fn evaluate<String>(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}