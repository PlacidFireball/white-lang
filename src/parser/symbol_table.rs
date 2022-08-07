use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::whitetypes::Type;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use super::statement::structdefinitionstatement::StructDefinitionStatement;

pub struct SymbolTable {
    symbol_stack: Vec<HashMap<String, Box<dyn Any>>>,
    __self: String,
}

impl Debug for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display_str = String::new();
        for stack in self.symbol_stack.iter() {
            for key in stack.keys(){
                if let Some(fds) = self.get_function(key.clone()) {
                    display_str.push_str(format!("-- {} -> {:?}\n", key, fds).as_str());
                } else if let Some(sds) = self.get_struct(key.clone()) {
                    display_str.push_str(format!("-- {} -> {:?}\n", key, sds).as_str());
                } else if let Some(typ) = self.get_symbol_type(key.clone()) {
                    display_str.push_str(format!("-- {} -> {:?}\n", key, typ).as_str());
                }
            }
            display_str.push_str("\n");
        }
        write!(f, "self: `{}`\nstacks:\n{}", self.__self, display_str)
    }
}

#[allow(dead_code)]
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbol_stack: vec![HashMap::<String, Box<dyn Any>>::new()], // <- the global scope
            __self: String::new(),
        }
    }

    pub fn has_symbol(&self, name: String) -> bool {
        self.get_symbol(name).is_some()
    }

    pub fn get_symbol(&self, name: String) -> Option<&Box<dyn Any>> {
        for next in &self.symbol_stack {
            match next.get(&name) {
                Some(s) => {
                    return Some(s);
                }
                None => {
                    continue;
                }
            }
        }
        None
    }

    pub fn get_symbol_as<T>(&self, name: String) -> Option<T>
    where
        T: Clone + 'static,
    {
        let retrieve = self.get_symbol(name);
        if retrieve.is_some() {
            if retrieve.unwrap().downcast_ref::<T>().is_some() {
                return Some(retrieve.unwrap().downcast_ref::<T>().unwrap().clone());
            }
        }
        None
    }

    pub fn register_symbol(&mut self, name: String, typ: Type) {
        self.symbol_stack
            .last_mut()
            .unwrap()
            .insert(name, Box::new(typ));
    }

    pub fn register_function(&mut self, name: String, def: FunctionDefinitionStatement) {
        self.symbol_stack
            .last_mut()
            .unwrap()
            .insert(name, Box::new(def));
    }

    pub fn register_struct(&mut self, name: String, def: StructDefinitionStatement) {
        self.symbol_stack
            .last_mut()
            .unwrap()
            .insert(name, Box::new(def));
    }

    pub fn get_symbol_type(&self, name: String) -> Option<Type> {
        return match self.get_symbol(name) {
            Some(t) => {
                if t.downcast_ref::<Type>().is_some() {
                    return Some(t.downcast_ref::<Type>().unwrap().clone());
                } else {
                    None
                }
            }
            _ => None,
        };
    }

    pub fn get_function(&self, name: String) -> Option<FunctionDefinitionStatement> {
        return match self.get_symbol(name) {
            Some(t) => {
                if t.downcast_ref::<FunctionDefinitionStatement>().is_some() {
                    Some(
                        t.downcast_ref::<FunctionDefinitionStatement>()
                            .unwrap()
                            .clone(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        };
    }

    pub fn get_struct(&self, name: String) -> Option<StructDefinitionStatement> {
        return match self.get_symbol(name) {
            Some(t) => {
                if t.downcast_ref::<StructDefinitionStatement>().is_some() {
                    Some(
                        t.downcast_ref::<StructDefinitionStatement>()
                            .unwrap()
                            .clone(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        };
    }

    pub fn push_scope(&mut self) {
        self.symbol_stack
            .push(HashMap::<String, Box<dyn Any>>::new());
    }
    pub fn pop_scope(&mut self) {
        self.symbol_stack.pop();
    }

    pub fn set_self(&mut self, name: String) {
        self.__self = name;
    }
    pub fn get_self(&self) -> String {
        self.__self.clone()
    }
}
