use crate::config::WhiteLangList;
use crate::{parser, Parser, Tokenizer};
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::{Expression, Statement};
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::config::*;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
    expr: Box<dyn Expression>,
    runtime: Runtime,
    pub output: String,
    errors: Vec<ParserErrorType>,
}
impl Program {
    pub fn from_src(src: String) -> Self {
        let mut tokenizer : Tokenizer = Tokenizer::init(src);
        let mut parser : Parser = Parser::new(&mut tokenizer);
        parser.parse();
        Program::from_parser(&mut parser)
    }
    pub fn from_parser(parser: &mut parser::Parser) -> Self {
        if let Some(statements) = parser.get_statements() {
            return Program {
                statements: statements.clone(),
                expr: Box::new(SyntaxErrorExpression::new()),
                runtime: Runtime::new(),
                output: String::new(),
                errors: vec![],
            };
        }
        if let Some(expr) = parser.get_expr() {
            return Program {
                statements: vec![],
                expr: expr.clone(),
                runtime: Runtime::new(),
                output: String::new(),
                errors: vec![],
            };
        }
        panic!("Uncaught parser error...")
    }

    pub fn execute(&mut self) {
        if self.statements.is_empty() {
            let eval = self.expr.evaluate(&mut self.runtime);
            self.output += &Program::try_print_output(&eval);
            self.output.push_str("\n");
        } else {
            for statement in &self.statements {
                statement.execute(&mut self.runtime);
                self.output = self.runtime.get_output();
            }
        }
    }

    pub fn try_print_output(evaluated: &Box<dyn Any>) -> String  {
        type WlFloat = Rc<RefCell<WhiteLangFloat>>;
        type WlInt = Rc<RefCell<WhiteLangInt>>;
        type WlBool = Rc<RefCell<WhiteLangBool>>;
        type WlString = Rc<RefCell<WhiteLangString>>;
        type WlStr = Rc<RefCell<&'static str>>;
        let mut output = String::new();
        if let Some(eval_f64) = evaluated.downcast_ref::<WlFloat>() {
            let push = eval_f64.borrow().to_string();
            output.push_str(push.as_str());
        } else if let Some(eval_isize) = evaluated.downcast_ref::<WlInt>() {
            let push = eval_isize.borrow().to_string();
            output.push_str(push.as_str());
        } else if let Some(eval_bool) = evaluated.downcast_ref::<WlBool>() {
            let push = eval_bool.borrow().to_string();
            output.push_str(push.as_str());
        } else if let Some(eval_str) = evaluated.downcast_ref::<WlStr>() {
            let push = eval_str.borrow().to_string();
            output.push_str(push.as_str());
        } else if let Some(eval_string) = evaluated.downcast_ref::<WlString>() {
            output.push_str(eval_string.borrow().as_str());
        } else if let Some(eval_list) = evaluated.downcast_ref::<WhiteLangList<Box<dyn Any>>>() {
            output.push_str("[");
            for (i, thing) in eval_list.iter().enumerate() {
                output.push_str(Program::try_print_output(thing).as_str());
                if i < eval_list.len() - 1 {
                    output.push_str(", ");
                }
            }
            output.push_str("]");
        }
        output
    }
}
