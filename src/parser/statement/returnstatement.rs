use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::*;
use crate::runtime::Runtime;

#[derive(Clone, Debug)]
pub(crate) struct ReturnStatement {
    expr: Box<dyn Expression>,
    return_type: Type,
    function: String,
}

impl ToAny for ReturnStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn execute(&self, runtime: &mut Runtime) {
        runtime.set_return(self.expr.clone());
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        let fds = st.get_function(self.function.clone()).unwrap();
        self.expr.validate(st);
        self.return_type = self.expr.get_white_type();
        LOGGER.info(format!("Got {:?}", fds));
        if self.return_type != fds.get_return_type() {
            add_parser_error(
                ParserErrorType::MismatchedTypes,
                format!(
                    "You cannot return {:?} from [{}], it is defined to return: {:?}",
                    self.return_type,
                    fds.name,
                    fds.get_return_type()
                ),
            );
        }
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("ReturnStatement")
    }
}
impl ReturnStatement {
    pub fn new(expr: Box<dyn Expression>, function: String) -> ReturnStatement {
        ReturnStatement {
            expr,
            return_type: Type::Initialized,
            function,
        }
    }
}
