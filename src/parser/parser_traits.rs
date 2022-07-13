use crate::parser::expression::additiveexpression::AdditiveExpression;
use crate::parser::expression::booleanliteralexpression::BooleanLiteralExpression;
use crate::parser::expression::comparisonexpression::ComparisonExpression;
use crate::parser::expression::equalityexpression::EqualityExpression;
use crate::parser::expression::factorexpression::FactorExpression;
use crate::parser::expression::floatliteralexpression::FloatLiteralExpression;
use crate::parser::expression::functioncallexpression::FunctionCallExpression;
use crate::parser::expression::identifierexpression::IdentifierExpression;
use crate::parser::expression::integerliteralexpression::IntegerLiteralExpression;
use crate::parser::expression::listliteralexpression::ListLiteralExpression;
use crate::parser::expression::logicalexpression::LogicalExpression;
use crate::parser::expression::nullliteralexpression::NullLiteralExpression;
use crate::parser::expression::parenthesizedexpression::ParenthesizedExpression;
use crate::parser::expression::stringliteralexpression::StringLiteralExpression;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::expression::unaryexpression::UnaryExpression;
use crate::parser::statement::assignmentstatement::AssignmentStatement;
use crate::parser::statement::breakstatement::BreakStatement;
use crate::parser::statement::forstatement::ForStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use std::any::Any;
use std::fmt::Debug;
use crate::config::{WhiteLangBool, WhiteLangFloat, WhiteLangInt, WhiteLangList, WhiteLangString};

use crate::parser::statement::functioncallstatement::FunctionCallStatement;
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::statement::ifstatement::IfStatement;
use crate::parser::statement::printstatement::PrintStatement;
use crate::parser::statement::returnstatement::ReturnStatement;
use crate::parser::statement::syntaxerrorstatement::SyntaxErrorStatement;
use crate::parser::statement::variablestatement::VariableStatement;
use crate::parser::statement::whilestatement::WhileStatement;
use crate::runtime::Runtime;

use crate::CORE_OBJECTS;
use crate::parser::ParserErrorType;

pub trait ToAny: 'static {
    fn to_any(&self) -> &dyn Any;
}

pub fn default_expr() -> Box<dyn Expression> {
    Box::new(SyntaxErrorExpression::new())
}

pub fn add_parser_error(error: ParserErrorType) {
    CORE_OBJECTS.with(|core| {
        core.borrow_mut().get_parser().add_error(error)
    })
}

pub fn any_into_int_literal(any : &Box<dyn Any>) -> Option<IntegerLiteralExpression> {
    if let Some(integer) = any.downcast_ref::<WhiteLangInt>() {
        return Some(IntegerLiteralExpression::new(*integer));
    }
    None
}

pub fn any_into_f64_literal(any : &Box<dyn Any>) -> Option<FloatLiteralExpression> {
    if let Some(float) = any.downcast_ref::<WhiteLangFloat>() {
        return Some(FloatLiteralExpression::new(*float));
    }
    None
}

pub fn any_into_bool_literal(any : &Box<dyn Any>) -> Option<BooleanLiteralExpression> {
    if let Some(bool) = any.downcast_ref::<WhiteLangBool>() {
        return Some(BooleanLiteralExpression::new(*bool));
    }
    None
}

pub fn any_into_string_literal(any : &Box<dyn Any>) -> Option<StringLiteralExpression> {
    if let Some(string) = any.downcast_ref::<WhiteLangString>() {
        return Some(StringLiteralExpression::new(string.to_string()));
    }
    None
}

pub fn try_print_output(evaluated: &Box<dyn Any>) -> String {
    let mut output = String::new();
    if let Some(eval_f64) = evaluated.downcast_ref::<WhiteLangFloat>() {
        let push = eval_f64.to_string();
        output.push_str(push.as_str());
    } else if let Some(eval_isize) = evaluated.downcast_ref::<WhiteLangInt>() {
        let push = eval_isize.to_string();
        output.push_str(push.as_str());
    } else if let Some(eval_bool) = evaluated.downcast_ref::<WhiteLangBool>() {
        let push = eval_bool.to_string();
        output.push_str(push.as_str());
    } else if let Some(eval_str) = evaluated.downcast_ref::<&'static str>() {
        let push = eval_str.to_string();
        output.push_str(push.as_str());
    } else if let Some(eval_string) = evaluated.downcast_ref::<WhiteLangString>() {
        output.push_str(eval_string.as_str());
    } else if let Some(eval_list) = evaluated.downcast_ref::<WhiteLangList<Box<dyn Any>>>() {
        output.push_str("[");
        for (i, thing) in eval_list.iter().enumerate() {
            output.push_str(try_print_output(thing).as_str());
            if i < eval_list.len() - 1 {
                output.push_str(", ");
            }
        }
        output.push_str("]");
    }
    output
}

#[allow(dead_code)]
pub trait Expression: ToAny + Debug {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any>; // evaluate the expression
    fn compile(&self); // compile the expression to nasm
    fn transpile(&self); // transpile the expression to javascript
    fn validate(&mut self, st: &SymbolTable); // validate the expression via the symbol table
    fn debug(&self) -> String; // for retrieving information about the expression
    fn get_white_type(&self) -> Type; // getting the type of the expression
    fn has_errors(&self) -> bool; // check if the expression has errors
    fn get_expr_type(&self) -> String; // get the rust type of the expression
}

// using to any to downcast the dyn Expression to the concrete class
// and then cloning the concrete class and returning a box of it
impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        if let Some(expr) = self.to_any().downcast_ref::<AdditiveExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<BooleanLiteralExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<ComparisonExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<EqualityExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<FactorExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<FloatLiteralExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<FunctionCallExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<IdentifierExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<NullLiteralExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<ParenthesizedExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<ListLiteralExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<IntegerLiteralExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<StringLiteralExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<UnaryExpression>() {
            return Box::new(expr.clone());
        } else if let Some(expr) = self.to_any().downcast_ref::<LogicalExpression>() {
            return Box::new(expr.clone());
        }
        panic!("Didn't cover expressions exhaustively")
    }
}

#[allow(dead_code)]
pub trait Statement: ToAny + Debug {
    fn execute(&self, runtime: &mut Runtime); // execute the statement
    fn compile(&self); // compile the statement to nasm
    fn transpile(&self) -> String; // transpile the statement to Javascript
    fn validate(&mut self, st: &mut SymbolTable); // validate the statement via the symbol table
    fn get_expr(&self) -> &Box<dyn Expression>; // retrieve the expression if the statement has one
    fn get_statement_type(&self) -> String; // debug info of the class
    fn has_errors(&self) -> bool; // tells us if the statement has errors
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        if let Some(stmt) = self.to_any().downcast_ref::<FunctionDefinitionStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<ReturnStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<VariableStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<SyntaxErrorStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<AssignmentStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<ForStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<PrintStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<FunctionCallStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<IfStatement>() {
            return Box::new(stmt.clone());
        } else if let Some(stmt) = self.to_any().downcast_ref::<WhileStatement>() {
            return Box::new(stmt.clone());
        }
        else if let Some(stmt) = self.to_any().downcast_ref::<BreakStatement>() {
            return Box::new(stmt.clone());
        }        
        panic!("Didn't cover statements exhaustively");
    }
}
