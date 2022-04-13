use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use crate::config::*;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::Expression;

pub struct Runtime {
    scopes: Vec<HashMap<String, Rc<RefCell<dyn Any>>>>,
    ret: Box<dyn Expression>,
    pub(crate) output: String
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            scopes: vec![HashMap::new()],
            ret: Box::new(SyntaxErrorExpression::new()),
            output: String::new()
        }
    }

    pub fn get_value(&mut self, name: String) -> Option<Box<dyn Any + '_>> {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name) {
                let val = self.scopes[i].remove(&name).unwrap();
                if let Some(integer) = val.borrow().downcast_ref::<isize>() {
                    println!("Got an integer!");
                }
                let cloned = val.clone();
                self.scopes[i].insert(name.clone(), cloned);
                return Some(Box::new(val));
            }
        }
        Option::None
    }
    pub fn set_value(&mut self, name: String, value: Box<dyn Any>) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name) {
                self.scopes[i].insert(name.clone(), Rc::new(RefCell::new(value)));
                return;
            }
        }
        self.scopes.last_mut().unwrap().insert(name.clone(), Rc::new(RefCell::new(value)));
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn set_return(&mut self, ret: Box<dyn Expression>) { self.ret = ret; }

    pub fn has_return(&mut self) -> bool {
        if let Some(expr) = self.ret.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return false
        }
        return true;
    }
    pub fn push_output(&mut self, str: String) {
        self.output.push_str(str.as_str());
    }

    pub fn get_output(&self) -> String { return self.output.clone(); }

    pub fn has_symbol(&self, name : String) -> bool {
        for i in self.scopes.len()-1..0 {
            if self.scopes[i].contains_key(&name) {
                return true
            }
        }
        false
    }
}
