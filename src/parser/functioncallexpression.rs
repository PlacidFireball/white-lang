use crate::parser::Expression;
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct FunctionCallExpression {
    name: String,
    args: Vec<Box<dyn Expression>>,
}
impl Expression for FunctionCallExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self) {
        todo!()
    }

    fn debug(&self) -> String {
        let mut builder: String = String::new();
        builder.push_str(&*self.name);
        builder.push_str(": ");
        for arg in &self.args {
            builder.push_str(arg.debug().as_str());
            builder.push_str(" ")
        }
        builder
    }

    fn get_white_type(&self) -> Type {
        todo!()
    }

    fn has_errors(&self) -> bool {
        todo!()
    }

    fn get_expr_type(&self) -> String {
        String::from("FunctionCallExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl FunctionCallExpression {
    pub fn new(name: String) -> FunctionCallExpression {
        FunctionCallExpression { name, args: vec![] }
    }
    pub fn add_arg(&mut self, arg: Box<dyn Expression>) {
        self.args.push(arg);
    }
}
