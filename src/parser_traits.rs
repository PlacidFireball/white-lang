use crate::parser::additiveexpression::AdditiveExpression;
use crate::parser::assignmentstatement::AssignmentStatement;
use crate::parser::booleanliteralexpression::BooleanLiteralExpression;
use crate::parser::comparisonexpression::ComparisonExpression;
use crate::parser::equalityexpression::EqualityExpression;
use crate::parser::factorexpression::FactorExpression;
use crate::parser::floatliteralexpression::FloatLiteralExpression;
use crate::parser::forstatement::ForStatement;
use crate::parser::functioncallexpression::FunctionCallExpression;
use crate::parser::identifierexpression::IdentifierExpression;
use crate::parser::integerliteralexpression::IntegerLiteralExpression;
use crate::parser::listliteralexpression::ListLiteralExpression;
use crate::parser::nullliteralexpression::NullLiteralExpression;
use crate::parser::parenthesizedexpression::ParenthesizedExpression;
use crate::parser::stringliteralexpression::StringLiteralExpression;
use crate::parser::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::unaryexpression::UnaryExpression;
use crate::parser::whitetypes::Type;
use crate::symbol_table::SymbolTable;
use std::any::Any;

use crate::parser::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::printstatement::PrintStatement;
use crate::parser::returnstatement::ReturnStatement;
use crate::parser::syntaxerrorstatement::SyntaxErrorStatement;
use crate::parser::variablestatement::VariableStatement;
use crate::parser::functioncallstatement::FunctionCallStatement;

pub(crate) trait ToAny: 'static {
    fn to_any(&self) -> &dyn Any;
}

pub(crate) fn default_expr() -> Box<dyn Expression> {
    Box::new(SyntaxErrorExpression::new())
}

#[allow(dead_code)]
pub(crate) trait Expression: ToAny {
    fn evaluate(&self) -> Box<dyn Any>;         // evaluate the expression
    fn compile(&self) -> String;                // compile the expression to nasm
    fn transpile(&self) -> String;              // transpile the expression to javascript
    fn validate(&mut self, st: &SymbolTable);   // validate the expression via the symbol table
    fn debug(&self) -> String;                  // for retrieving information about the expression
    fn get_white_type(&self) -> Type;           // getting the type of the expression
    fn has_errors(&self) -> bool;               // check if the expression has errors
    fn get_expr_type(&self) -> String;          // get the rust type of the expression
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
        }
        panic!("Didn't cover expressions exhaustively")
    }
}

#[allow(dead_code)]
pub(crate) trait Statement: ToAny {
    fn execute(&self) -> String;                        // execute the statement
    fn compile(&self) -> String;                        // compile the statement to nasm
    fn transpile(&self) -> String;                      // transpile the statement to Javascript
    fn validate(&mut self, st: &mut SymbolTable);       // validate the statement via the symbol table
    fn get_expr(&self) -> &Box<dyn Expression>;         // retrieve the expression if the statement has one
    fn get_statement_type(&self) -> String;             // debug info of the class
    fn has_errors(&self) -> bool;                       // tells us if the statement has errors
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
        } /*else if let Some(stmt) = self.to_any().downcast_ref::<FunctionCallStatement>() {
            return Box::new(stmt.clone());
        }*/
        panic!("Didn't cover statements exhaustively");
    }
}
