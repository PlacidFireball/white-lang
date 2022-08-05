use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::whitetypes::Type;
use std::any::Any;
use std::collections::HashMap;

use super::statement::structdefinitionstatement::StructDefinitionStatement;

pub struct SymbolTable {
    symbol_stack: Vec<HashMap<String, Box<dyn Any>>>,
    __self: String
}

#[allow(dead_code)]
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbol_stack: vec![HashMap::<String, Box<dyn Any>>::new()], // <- the global scope
            __self: String::new()
        }
    }

    pub fn has_symbol(&self, name: String) -> bool {
        self.get_symbol(name).is_some()
    }

    pub fn get_symbol(&self, name: String) -> Option<&Box<dyn Any>> {
        for next in &self.symbol_stack {
            match next.get(&name) {
                Some(s) => {
                    return Option::Some(s);
                }
                None => {
                    continue;
                }
            }
        }
        Option::None
    }

    pub fn get_symbol_as<T: 'static>(&self, name: String) -> Option<T>
    where
        T: Clone,
    {
        let retrieve = self.get_symbol(name);
        if retrieve.is_some() {
            if retrieve.unwrap().downcast_ref::<T>().is_some() {
                return Option::Some(retrieve.unwrap().downcast_ref::<T>().unwrap().clone());
            }
        }
        Option::None
    }

    pub fn register_symbol(&mut self, name: String, typ: Type) {
        self.symbol_stack[0].insert(name, Box::new(typ));
    }

    pub fn register_function(&mut self, name: String, def: FunctionDefinitionStatement) {
        self.symbol_stack[0].insert(name, Box::new(def));
    }

    pub fn register_struct(&mut self, name: String, def: StructDefinitionStatement) {
        self.symbol_stack[0].insert(name, Box::new(def));
    }

    pub fn get_symbol_type(&self, name: String) -> Option<Type> {
        return match self.get_symbol(name) {
            Some(t) => {
                if t.downcast_ref::<Type>().is_some() {
                    return Option::Some(t.downcast_ref::<Type>().unwrap().clone());
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        };
    }

    pub fn get_function(&self, name: String) -> Option<FunctionDefinitionStatement> {
        return match self.get_symbol(name) {
            Some(t) => {
                if t.downcast_ref::<FunctionDefinitionStatement>().is_some() {
                    Option::Some(
                        t.downcast_ref::<FunctionDefinitionStatement>()
                            .unwrap()
                            .clone(),
                    )
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        };
    }

    pub fn get_struct(&self, name: String) -> Option<StructDefinitionStatement> {
        return match self.get_symbol(name) {
            Some(t) => {
                if t.downcast_ref::<StructDefinitionStatement>().is_some() {
                    Option::Some(
                        t.downcast_ref::<StructDefinitionStatement>()
                            .unwrap()
                            .clone(),
                    )
                } else {
                    Option::None
                }
            }
            _ => Option::None,
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
