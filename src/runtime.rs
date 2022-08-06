use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::Expression;
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::statement::structdefinitionstatement::StructDefinitionStatement;
use std::any::Any;
use std::collections::HashMap;

mod test;

pub struct Runtime {
    scopes: Vec<HashMap<String, Box<dyn Expression>>>,
    ids: Vec<String>,
    functions: HashMap<String, FunctionDefinitionStatement>,
    structs: HashMap<String, StructDefinitionStatement>,
    ret: Box<dyn Expression>,
    pub(crate) output: String,
    brk: bool,
    __self: String,
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            scopes: vec![HashMap::new()],
            ids: vec![String::from("global")],
            functions: HashMap::new(),
            structs: HashMap::new(),
            ret: Box::new(SyntaxErrorExpression::new()),
            output: String::new(),
            brk: false,
            __self: String::new(),
        }
    }

    pub fn get_value(&mut self, name: String) -> Option<Box<dyn Any + '_>> {
        let log_runtime_debug = crate::RUNTIME_DEBUG_LOGGING_ENABLED.with(|cell| !cell.get());
        crate::LOGGER.debug(
            format!("[RUNTIME] Searching for value: {}", name),
            log_runtime_debug,
        );
        for (i, scope) in self.scopes.iter_mut().rev().enumerate() {
            crate::LOGGER.debug(
                format!("[RUNTIME] scope: sid.{}", self.ids[i]),
                log_runtime_debug,
            );
            for (name, expr) in scope.iter() {
                crate::LOGGER.debug(
                    format!("-- {} \t\t=> {}", name, expr.debug()),
                    log_runtime_debug,
                );
            }
            if scope.contains_key(&name) {
                let val = scope.remove(&name).unwrap();
                crate::LOGGER.debug(
                    format!("[RUNTIME] got {} value: {:?}", name, val),
                    log_runtime_debug,
                );
                scope.insert(name.clone(), val.clone());
                return Some(val.evaluate(self));
            }
        }
        None
    }

    pub fn set_value(&mut self, name: String, value: Box<dyn Expression>) {
        crate::LOGGER.debug(
            format!("[RUNTIME] Setting {} -> {:?}", name, value),
            crate::RUNTIME_DEBUG_LOGGING_ENABLED.with(|cell| !cell.get()),
        );
        for (i, scope) in self.scopes.iter_mut().enumerate() {
            if scope.contains_key(&name) {
                crate::LOGGER.debug(
                    format!("[RUNTIME] Setting {} in {}", name, self.ids[i]),
                    crate::RUNTIME_DEBUG_LOGGING_ENABLED.with(|cell| !cell.get()),
                );
                scope.insert(name.clone(), value.clone());
            }
        }
        crate::LOGGER.debug(
            format!("[RUNTIME] Setting {} in {}", name, self.ids.last().unwrap()),
            crate::RUNTIME_DEBUG_LOGGING_ENABLED.with(|cell| !cell.get()),
        );
        self.scopes.last_mut().unwrap().insert(name.clone(), value);
    }

    pub fn set_value_in_scope(&mut self, id: String, name: String, value: Box<dyn Expression>) {
        let idx = match self.ids.iter().position(|id_| id_.clone() == id) {
            Some(idx) => idx,
            None => {
                crate::LOGGER.warn(format!("[RUNTIME] Did not find scope {}, sending 0", id));
                0
            }
        };
        self.scopes[idx].insert(name.clone(), value.clone());
    }

    pub fn add_function(&mut self, name: String, fds: FunctionDefinitionStatement) {
        self.functions.insert(name, fds);
    }

    pub fn get_function(&mut self, name: String) -> FunctionDefinitionStatement {
        if !self.functions.contains_key(&name) {
            crate::LOGGER.error(format!("function: `{}` not in the functions map", name));
            unreachable!();
        }
        self.functions.get(&name).unwrap().clone()
    }

    pub fn add_struct(&mut self, name: String, sds: StructDefinitionStatement) {
        self.structs.insert(name.clone(), sds.clone());
    }

    pub fn get_struct(&self, name: String) -> StructDefinitionStatement {
        if !self.structs.contains_key(&name) {
            crate::LOGGER.error(format!("struct: `{}` not in the structs map", name));
            unreachable!(); // need this here to get compiler to heck off
        }
        self.structs.get(&name).unwrap().clone()
    }

    pub fn push_scope(&mut self, typ: String) {
        self.ids.push(typ);
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
        self.ids.pop();
    }

    pub fn set_return(&mut self, ret: Box<dyn Expression>) {
        self.ret = ret;
    }

    pub fn has_return(&mut self) -> bool {
        if let Some(_) = self.ret.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return false;
        }
        return true;
    }

    pub fn get_return(&mut self) -> Box<dyn Any> {
        let ret = self.ret.clone(); // get the return value
        self.ret = Box::new(SyntaxErrorExpression::new()); // clear the return slot
        ret.evaluate(self)
    }

    pub fn push_output(&mut self, str: String) {
        self.output.push_str(str.as_str());
    }

    pub fn get_output(&self) -> String {
        return self.output.clone();
    }

    pub fn has_symbol(&self, name: String) -> bool {
        for i in self.scopes.len() - 1..0 {
            if self.scopes[i].contains_key(&name) {
                return true;
            }
        }
        false
    }

    pub fn set_break(&mut self, brk: bool) {
        self.brk = brk;
    }
    pub fn get_break(&self) -> bool {
        self.brk
    }

    pub fn set_self(&mut self, name: String) {
        self.__self = name;
    }
    pub fn get_self(&self) -> String {
        self.__self.clone()
    }
}
