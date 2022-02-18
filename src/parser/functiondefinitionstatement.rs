use crate::parser::*;
use crate::parser::returnstatement::ReturnStatement;

pub(crate) struct FunctionDefinitionStatement {
    name: String,
    return_type: Type,
    statements: Vec<Box<dyn Statement>>,
}

impl ToAny for FunctionDefinitionStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for FunctionDefinitionStatement {
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&self, st: &SymbolTable) -> String {
        for statement in &self.statements {
            statement.validate(st);
            if statement.to_any().downcast_ref::<ReturnStatement>().is_some() {

            }
        }
        String::from("")
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
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
            statements: vec![],
        }
    }

    pub fn set_return_type(&mut self, return_type: Type) {
        self.return_type = return_type;
    }

    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}
