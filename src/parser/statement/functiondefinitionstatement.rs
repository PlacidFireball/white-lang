use crate::parser::parser_traits::*;
use crate::parser::statement::returnstatement::ReturnStatement;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::ParserErrorType::MismatchedTypes;
use crate::parser::*;
use crate::runtime::Runtime;

use crate::javascript::JavaScript;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct FunctionDefinitionStatement {
    pub name: String,
    pub(crate) return_type: Type,
    pub(crate) args: Vec<Box<dyn Expression>>,
    pub(crate) arg_names: Vec<String>,
    pub(crate) arg_types: Vec<Type>,
    pub(crate) statements: Vec<Box<dyn Statement>>,
}

impl ToAny for FunctionDefinitionStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
    fn to_any_mut(&mut self) -> &mut dyn Any {
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
        }
    }
}

impl Statement for FunctionDefinitionStatement {
    fn execute(&mut self, runtime: &mut Runtime) {
        runtime.add_function(self.name.clone(), self.clone());
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append(format!("function {}(", self.name));
        for (i, arg) in self.args.iter().enumerate() {
            arg.transpile(javascript);
            if i != self.args.len() - 1 {
                javascript.append(String::from(", "));
            }
        }
        javascript.append(String::from(") {")).newline().indent();
        for stmt in self.statements.iter() {
            stmt.transpile(javascript);
        }
        javascript
            .newline()
            .outdent()
            .append(String::from("}"))
            .newline();
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        let mut i = 0;
        for arg in &mut self.args {
            st.register_symbol(arg.debug(), self.arg_types[i].clone());
            i += 1;
            arg.validate(st);
        }
        for statement in &mut self.statements {
            statement.validate(st);
            let opt_rs = statement.to_any().downcast_ref::<ReturnStatement>();
            if opt_rs.is_some() {
                let rs = opt_rs.unwrap();
                if rs.get_expr().get_white_type() != self.return_type {
                    add_parser_error(
                        MismatchedTypes(rs.get_expr().get_white_type(), self.return_type.clone()),
                        format!(
                            "You cannot return {:?} from [{}], it is defined to return: {:?}",
                            rs.get_expr().get_white_type(),
                            self.name,
                            self.return_type
                        ),
                    );
                }
            }
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        unimplemented!();
    }

    fn get_statement_type(&self) -> String {
        String::from("FunctionDefinitionStatement")
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
        }
    }

    pub fn get_return_type(&self) -> Type {
        self.return_type.clone()
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

    pub fn invoke(
        &mut self,
        runtime: &mut Runtime,
        args: Vec<Box<dyn Expression>>,
    ) -> Box<dyn Any> {
        let id = Uuid::new_v4();
        runtime.push_scope(id.to_string());
        for (i, arg) in args.iter().enumerate() {
            runtime.set_value_in_scope(id.to_string(), self.arg_names[i].clone(), arg.clone());
        }
        for statement in &mut self.statements {
            statement.execute(runtime);
            if runtime.has_return() {
                return runtime.get_return();
            }
        }
        Box::new(())
    }
}
