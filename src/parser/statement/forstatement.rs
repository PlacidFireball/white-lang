use crate::parser::expression::identifierexpression::IdentifierExpression;
use crate::parser::expression::listliteralexpression::ListLiteralExpression;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::symbol_table::SymbolTable;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct ForStatement {
    errors: Vec<ParserErrorType>,
    statements: Vec<Box<dyn Statement>>,
    iter_var: Box<dyn Expression>, // list literal expression
    iter: Box<dyn Expression>,     // identifier expression
}

impl ToAny for ForStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ForStatement {
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        st.push_scope();
        if let Some(id_expr) = self
            .iter_var
            .to_any()
            .downcast_ref::<IdentifierExpression>()
        {
            let name = id_expr.debug();
            if st.has_symbol(name.clone()) {
                self.errors.push(ParserErrorType::DuplicateName);
            } else if let Some(lle) = self.iter.to_any().downcast_ref::<ListLiteralExpression>() {
                let mut lle_cln = lle.clone();
                lle_cln.validate(st);
                let typ = lle_cln.get_white_type().get_type_from_list();
                self.iter = Box::new(lle_cln);
                if typ != Type::Error {
                    st.register_symbol(name.clone(), typ);
                } else {
                    self.errors.push(ParserErrorType::IncompatibleTypes);
                    st.register_symbol(name.clone(), Type::Void); // TODO: Make this Object
                }
            }
        }
        for stmt in &mut self.statements {
            stmt.validate(st);
        }
        st.pop_scope();
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        String::from("ForStatement")
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl ForStatement {
    pub fn new() -> Self {
        ForStatement {
            errors: vec![],
            statements: vec![],
            iter_var: Box::new(SyntaxErrorExpression::new()),
            iter: Box::new(SyntaxErrorExpression::new()),
        }
    }

    pub fn add_statement(&mut self, stmt: Box<dyn Statement>) {
        self.statements.push(stmt);
    }
    pub fn set_iter_var(&mut self, iter_var: Box<dyn Expression>) {
        if iter_var
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .is_none()
        {
            self.errors.push(ParserErrorType::UnexpectedToken);
        } else {
            self.iter_var = iter_var.clone();
        }
    }
    pub fn set_iter(&mut self, iter: Box<dyn Expression>) {
        if iter
            .to_any()
            .downcast_ref::<ListLiteralExpression>()
            .is_none()
        {
            self.errors.push(ParserErrorType::UnexpectedToken);
        } else {
            self.iter = iter.clone();
        }
    }
}
