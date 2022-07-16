use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::Expression;
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use std::any::Any;
use std::collections::HashMap;

mod test;

pub struct Runtime {
    scopes: Vec<HashMap<String, Box<dyn Expression>>>,
    ids: Vec<String>,
    functions: HashMap<String, FunctionDefinitionStatement>,
    ret: Box<dyn Expression>,
    pub(crate) output: String,
    brk: bool,
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            scopes: vec![HashMap::new()],
            ids: vec![String::from("global")],
            functions: HashMap::new(),
            ret: Box::new(SyntaxErrorExpression::new()),
            output: String::new(),
            brk: false,
        }
    }

    pub fn get_value(&mut self, name: String) -> Option<Box<dyn Any + '_>> {
        for i in (0..self.scopes.len()).rev() {
            println!("[RUNTIME] ---- Scope [{}] ----", i);
            for (name, expr) in &self.scopes[i] {
                println!("[RUNTIME] Name: {}\tValue: {}", name, expr.debug());
            }
            if self.scopes[i].contains_key(&name) {
                let val = self.scopes[i].remove(&name).unwrap();
                self.scopes[i].insert(name.clone(), val.clone());
                println!(
                    "[RUNTIME]: Name: {}\tValue: {}\t Scope: {}",
                    name,
                    val.debug(),
                    i
                );
                return Some(val.evaluate(self));
            }
        }
        Option::None
    }

    pub fn set_value(&mut self, name: String, value: Box<dyn Expression>) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name) {
                self.scopes[i].insert(name.clone(), value.clone());
            }
        }
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
    // may not be necessary
    pub fn get_value_in_scope(&mut self, id: String, name: String) -> Option<Box<dyn Any + '_>> {
        let idx = match self.ids.iter().position(|id_| id_.clone() == id) {
            Some(idx) => idx,
            None => {
                crate::LOGGER.warn(format!("[RUNTIME] Did not find scope {}, sending 0", id));
                0
            }
        };
        if self.scopes[idx].contains_key(&id) {
            let val = self.scopes[idx].remove(&name).unwrap();
            self.scopes[idx].insert(name.clone(), val.clone());
            Some(val.evaluate(self))
        } else {
            None
        }
    }

    pub fn set_function(&mut self, name: String, fds: FunctionDefinitionStatement) {
        self.functions.insert(name, fds);
    }

    pub fn get_function(&mut self, name: String) -> FunctionDefinitionStatement {
        if !self.functions.contains_key(&name) {
            panic!("function: `{}` not in the functions map", name);
        }
        let func = self.functions.remove(&name).unwrap();
        self.functions.insert(name.clone(), func.clone());
        func
    }

    pub fn push_scope(&mut self, typ: String) {
        self.ids.push(typ);
        let last = self.scopes.last().unwrap().clone();
        self.scopes.push(last);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
        self.ids.pop();
    }

    pub fn pop_nearest_loop(&mut self) {
        for i in (0..self.ids.len()).rev() {
            if self.ids[i].eq("while") {
                // TODO: trying to implement inner break statement
            }
        }
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
}
