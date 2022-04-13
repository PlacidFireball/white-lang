use std::any::Any;
use std::collections::HashMap;

pub struct Runtime {
    scopes: Vec<HashMap<String, Box<dyn Any>>>,
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            scopes: vec![HashMap::new()],
        }
    }

    // TODO : figure out how to make this not 'static :)
    pub fn get_value(&'static self, name: String) -> Option<Box<dyn Any + '_>> {
        for i in self.scopes.len()-1..0 {
            if self.scopes[i].contains_key(&name) {
                let val = self.scopes[i].get(&name).unwrap();
                return Option::Some(Box::new(val));
            }
        }
        Option::None
    }
    pub fn set_value(&mut self, name: String, value: Box<dyn Any>) {
        for i in 0..self.scopes.len() {
            if self.scopes[i].contains_key(&name) {
                self.scopes[i].insert(name.clone(), value);
                return;
            }
        }
        self.scopes.last_mut().unwrap().insert(name.clone(), value);
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
