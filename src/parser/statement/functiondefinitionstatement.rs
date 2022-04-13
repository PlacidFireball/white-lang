use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::statement::returnstatement::ReturnStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::*;
use crate::runtime::Runtime;

#[derive(Clone)]
pub struct FunctionDefinitionStatement {
    name: String,
    return_type: Type,
    args: Vec<Box<dyn Expression>>,
    arg_types: Vec<Type>,
    statements: Vec<Box<dyn Statement>>,
    errors: Vec<ParserErrorType>,
}

impl ToAny for FunctionDefinitionStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Default for FunctionDefinitionStatement {
    fn default() -> Self {
        FunctionDefinitionStatement {
            name: String::from(""),
            return_type: Type::Void,
            args: vec![],
            arg_types: vec![],
            statements: vec![],
            errors: vec![],
        }
    }
}

impl Statement for FunctionDefinitionStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.set_value(self.name.clone(), Box::new(Self {
            name: self.name.clone(),
            return_type: self.return_type,
            args: self.args.clone(),
            arg_types: self.arg_types.clone(),
            statements: self.statements.clone(),
            errors: self.errors.clone()
        }));
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        st.register_function(self.name.clone(), self.clone());
        for statement in &mut self.statements {
            statement.validate(st);
            let opt_rs = statement.to_any().downcast_ref::<ReturnStatement>();
            if opt_rs.is_some() {
                let rs = opt_rs.unwrap();
                if rs.get_expr().get_white_type() != self.return_type {
                    self.errors.push(ParserErrorType::MismatchedTypes);
                }
            }
        }
        for arg in &mut self.args {
            arg.validate(st);
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        unimplemented!();
    }

    fn get_statement_type(&self) -> String {
        String::from("FunctionDefinitionStatement")
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
impl FunctionDefinitionStatement {
    pub fn new(name: String) -> FunctionDefinitionStatement {
        FunctionDefinitionStatement {
            name,
            return_type: Type::Void,
            args: vec![],
            arg_types: vec![],
            statements: vec![],
            errors: vec![],
        }
    }

    pub fn get_return_type(&self) -> Type {
        self.return_type
    }
    pub fn set_return_type(&mut self, return_type: Type) {
        self.return_type = return_type;
    }

    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
    pub fn get_args(&mut self) -> &mut Vec<Box<dyn Expression>> {
        &mut self.args
    }
    pub fn add_arg(&mut self, expr: Box<dyn Expression>) {
        self.args.push(expr);
    }
    pub fn add_arg_type(&mut self, typ: Type) {
        self.arg_types.push(typ);
    }
}
