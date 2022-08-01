use crate::parser::expression::identifierexpression::IdentifierExpression;
use crate::parser::expression::listliteralexpression::ListLiteralExpression;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::ParserErrorType::IncompatibleTypes;
use crate::runtime::Runtime;
use std::any::Any;
use uuid::Uuid;
use crate::javascript::JavaScript;

#[derive(Clone, Debug)]
pub(crate) struct ForStatement {
    statements: Vec<Box<dyn Statement>>,
    variable: Box<dyn Expression>, // list literal expression
    iterator: Box<dyn Expression>, // identifier expression
}

impl ToAny for ForStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ForStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.push_scope(String::from(Uuid::new_v4().to_string()));
        let any = self.iterator.evaluate(runtime);
        let list = any.downcast_ref::<Vec<Box<dyn Any>>>().unwrap();
        for item in list.iter() {
            let mut is_broken = false;
            if let Some(integer) = any_into_int_literal(item) {
                runtime.set_value(self.variable.debug(), Box::new(integer));
            } else if let Some(float) = any_into_f64_literal(item) {
                runtime.set_value(self.variable.debug(), Box::new(float));
            } else if let Some(boolean) = any_into_bool_literal(item) {
                runtime.set_value(self.variable.debug(), Box::new(boolean));
            } else if let Some(string) = any_into_string_literal(item) {
                runtime.set_value(self.variable.debug(), Box::new(string));
            } else {
                panic!("Some type in the list variable not covered");
            }
            for statement in self.statements.iter() {
                statement.execute(runtime);
                if runtime.get_break() {
                    runtime.set_break(false);
                    is_broken = true;
                    break;
                }
            }
            if is_broken {
                break;
            }
        }
        runtime.pop_scope();
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        st.push_scope();
        if let Some(id_expr) = self
            .variable
            .to_any()
            .downcast_ref::<IdentifierExpression>()
        {
            let name = id_expr.debug();
            if st.has_symbol(name.clone()) {
                add_parser_error(
                    ParserErrorType::DuplicateName(
                        name.clone(),
                        st.get_symbol_type(name.clone()).unwrap(),
                    ),
                    format!(
                        "Duplicate name: [{}] has already been defined.",
                        id_expr.debug()
                    ),
                );
            }
        }

        self.iterator.validate(st);
        let typ = self.iterator.get_white_type().get_type_from_list();
        if typ != Type::Error && typ != Type::Initialized {
            st.register_symbol(self.variable.debug(), typ);
        } else {
            add_parser_error(
                IncompatibleTypes(typ, self.variable.clone().get_white_type()),
                format!("Unexpected typing error."),
            );
        }
        for stmt in &mut self.statements {
            stmt.validate(st);
        }
        st.pop_scope();
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.variable
    }

    fn get_statement_type(&self) -> String {
        String::from("ForStatement")
    }
}

impl ForStatement {
    pub fn new() -> Self {
        ForStatement {
            statements: vec![],
            variable: Box::new(SyntaxErrorExpression::new()),
            iterator: Box::new(SyntaxErrorExpression::new()),
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
            add_parser_error(
                ParserErrorType::UnexpectedExpression(iter_var),
                format!("You must use an identifier as your iteration variable."),
            );
        } else {
            self.variable = iter_var.clone();
        }
    }
    pub fn set_iter(&mut self, iter: Box<dyn Expression>) {
        if iter
            .to_any()
            .downcast_ref::<ListLiteralExpression>()
            .is_some()
        {
            self.iterator = iter.clone();
        } else if iter
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .is_some()
        {
            self.iterator = iter.clone();
        } else {
            add_parser_error(
                ParserErrorType::UnexpectedExpression(iter),
                format!("Unexpected token, make sure your iterator is a list type."),
            );
        }
    }
}
