use crate::parser::*;
use crate::parser::returnstatement::ReturnStatement;
use crate::parser_traits::ToAny;
use crate::symbol_table::SymbolTable;

pub(crate) struct FunctionDefinitionStatement {
    name: String,
    return_type: Type,
    statements: Vec<Box<dyn Statement>>,
    errors: Vec<ParserErrorType>
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
            statements: vec![],
            errors: vec![]
        }
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

    fn validate(&mut self, st: &SymbolTable) -> String {
        for statement in &mut self.statements {
            statement.validate(st);
            let ret_statement = statement.to_any().downcast_ref::<ReturnStatement>();
            if ret_statement.is_some() {
                let unwrapped = ret_statement.unwrap();
                if unwrapped.get_expr().get_white_type() != self.return_type {
                    self.errors.push(ParserErrorType::BadReturnType);
                }
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
            errors: vec![]
        }
    }

    pub fn set_return_type(&mut self, return_type: Type) {
        self.return_type = return_type;
    }

    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}
