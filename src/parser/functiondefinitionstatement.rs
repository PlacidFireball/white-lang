use crate::parser::*;

pub(crate) struct FunctionDefinitionStatement {
    name: String,
    return_type: Type,
    statements: Vec<Box<dyn Statement>>
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
        todo!()
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
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
