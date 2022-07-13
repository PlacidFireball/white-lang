use crate::parser::parser_traits::*;
use crate::parser::statement::returnstatement::ReturnStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::*;
use crate::runtime::Runtime;
use log::info;
use crate::parser::ParserErrorType::MismatchedTypes;
use crate::IS_TESTING;

#[derive(Clone,Debug)]
pub struct FunctionDefinitionStatement {
    name: String,
    return_type: Type,
    args: Vec<Box<dyn Expression>>,
    arg_names: Vec<String>,
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
            arg_names: vec![],
            arg_types: vec![],
            statements: vec![],
            errors: vec![],
        }
    }
}

impl Statement for FunctionDefinitionStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.set_function(self.name.clone(), self.clone());
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
                    if !IS_TESTING.with(|test| test.get()) {
                        add_parser_error(MismatchedTypes);
                    }
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
            arg_names: vec![],
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
        self.arg_names.push(expr.debug()); // should be identifier expressions :)
        self.args.push(expr);
    }
    pub fn add_arg_type(&mut self, typ: Type) {
        self.arg_types.push(typ);
    }
    pub fn get_arg_names(&self) -> &Vec<String> {
        &self.arg_names
    }

    pub fn invoke(&self, runtime: &mut Runtime, args: Vec<Box<dyn Expression>>) -> Box<dyn Any> {
        runtime.push_scope(String::from("fn"));
        for (i, arg) in args.iter().enumerate() {
            /*println!(
                "Function Call: {} Argument: {} Value: {}",
                self.name,
                self.arg_names[i],
                try_print_output(&arg.evaluate(runtime))
            );*/
            runtime.set_value(self.arg_names[i].clone(), arg.clone());
        }
        for statement in &self.statements {
            statement.execute(runtime);
            if runtime.has_return() {
                return runtime.get_return();
            }
        }
        Box::new(())
    }
}
