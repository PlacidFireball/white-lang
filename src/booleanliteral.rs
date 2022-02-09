use expression::Expression;

struct BooleanLiteralExpression {
    expr: Expression,
    value: bool
}
impl BooleanLiteralExpression {
    pub fn new() -> BooleanLiteralExpression {
        BooleanLiteralExpression { }
    }
    pub fn set_expr(&mut self, expression: Expression) {
        self.expr = expression;
    }
}
impl Expression for BooleanLiteralExpression {
    fn evaluate() -> bool {
        false
    }
    fn compile() -> String {
        String::from("")
    }
    fn transpile() -> String {
        String::from("")
    }
}


#[#[cfg(test)]]
mod test {

}
