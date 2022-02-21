use std::any::Any;
use crate::parser::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::additiveexpression::AdditiveExpression;
use crate::parser::booleanliteralexpression::BooleanLiteralExpression;
use crate::parser::whitetypes::Type;
use crate::symbol_table::SymbolTable;
use crate::parser::comparisonexpression::ComparisonExpression;
use crate::parser::equalityexpression::EqualityExpression;
use crate::parser::factorexpression::FactorExpression;
use crate::parser::floatliteralexpression::FloatLiteralExpression;
use crate::parser::functioncallexpression::FunctionCallExpression;
use crate::parser::identifierexpression::IdentifierExpression;
use crate::parser::listliteralexpression::ListLiteralExpression;
use crate::parser::nullliteralexpression::NullLiteralExpression;
use crate::parser::parenthesizedexpression::ParenthesizedExpression;
use crate::parser::integerliteralexpression::IntegerLiteralExpression;
use crate::parser::stringliteralexpression::StringLiteralExpression;
use crate::parser::unaryexpression::UnaryExpression;

pub(crate) trait ToAny: 'static {
    fn to_any(&self) -> &dyn Any;
}

pub(crate) fn default_expr() -> Box<dyn Expression> {
    Box::new(SyntaxErrorExpression::new())
}

#[allow(dead_code)]
pub(crate) trait Expression: ToAny {
    fn evaluate(&self) -> Box<dyn Any>; // evaluate the expression
    fn compile(&self) -> String; // compile the expression to nasm
    fn transpile(&self) -> String; // transpile the expression to javascript
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
        if self.to_any().downcast_ref::<AdditiveExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<AdditiveExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<BooleanLiteralExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<BooleanLiteralExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<ComparisonExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<ComparisonExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<EqualityExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<EqualityExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<FactorExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<FactorExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<FloatLiteralExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<FloatLiteralExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<FunctionCallExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<FunctionCallExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<IdentifierExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<IdentifierExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<NullLiteralExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<NullLiteralExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<ParenthesizedExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<ParenthesizedExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<ListLiteralExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<ListLiteralExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<IntegerLiteralExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<IntegerLiteralExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<StringLiteralExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<StringLiteralExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<SyntaxErrorExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<SyntaxErrorExpression>().unwrap();
            return Box::new(expr.clone());
        } else if self.to_any().downcast_ref::<UnaryExpression>().is_some() {
            let expr = self.to_any().downcast_ref::<UnaryExpression>().unwrap();
            return Box::new(expr.clone());
        }
        panic!("Didn't cover expressions exhaustively")
    }
}

#[allow(dead_code)]
pub(crate) trait Statement: ToAny {
    fn execute(&self) -> String; // execute the statement
    fn compile(&self) -> String; // compile the statement to nasm
    fn transpile(&self) -> String; // transpile the statement to Javascript
    fn validate(&mut self, st: &mut SymbolTable) -> String; // validate the statement via the symbol table
    fn get_expr(&self) -> &Box<dyn Expression>; // retrieve the expression if the statement has one
    fn get_statement_type(&self) -> String;
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        panic!("Didn't cover statements exhaustively")
    }
}
