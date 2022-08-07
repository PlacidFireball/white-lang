use crate::parser::expression::identifierexpression::IdentifierExpression;
use crate::parser::expression::stringliteralexpression::StringLiteralExpression;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::Expression;
use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser::statement::structdefinitionstatement::StructDefinitionStatement;
use crate::parser::whitetypes::Type;
use std::any::Any;
use std::collections::HashMap;

mod test;

struct Intrinsic {
    name: Name,
    _return_type: Type, // these fields are mostly for symbolic posturing, just so we know what happens here
    _args: HashMap<Name, Type>, // typing will be inforced by the handler
}
impl Intrinsic {
    pub(crate) fn new(name: String, return_type: Type, args: HashMap<String, Type>) -> Self {
        Intrinsic {
            name,
            _return_type: return_type,
            _args: args,
        }
    }
}

type Name = String;

pub struct Runtime {
    scopes: Vec<HashMap<String, Box<dyn Expression>>>,
    ids: Vec<String>,
    functions: HashMap<Name, FunctionDefinitionStatement>,
    structs: HashMap<Name, StructDefinitionStatement>,
    intrinsics: HashMap<Name, Intrinsic>,
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
            intrinsics: HashMap::new(),
            ret: Box::new(SyntaxErrorExpression::new()),
            output: String::new(),
            brk: false,
            __self: String::new(),
        }
    }

    /// Register some hard-coded intrinsics for basic whitetypes
    #[allow(unused_variables)]
    pub fn register_intrinsics(&mut self) {
        // strings
        let string_to_list = Intrinsic::new(
            "String.to_list".to_string(),
            Type::ListString,
            HashMap::from([("s".to_string(), Type::String)]),
        );

        self.intrinsics.insert(string_to_list.name.clone(), string_to_list);
    }

    pub fn has_intrisic(&self, name: Name) -> bool {
        self.intrinsics.contains_key(&name)
    }

    pub fn handle_intrinsic(&mut self, name: Name, args: HashMap<Name, Box<dyn Expression>>) -> Box<dyn Expression> {
        match name.as_str() {
            "String.to_list" => {
                let mut expr = args.get(&"s".to_string()).unwrap().clone();
                let to_list = if let Some(string) = expr.to_any().downcast_ref::<StringLiteralExpression>() {
                    string.to_list_literal()
                } else if let Some(identifier) = expr.to_any_mut().downcast_mut::<IdentifierExpression>() {
                    if identifier.get_white_type() != Type::String {
                        panic!("Identifier: {:?} is not a string!", identifier);
                    }
                    let eval = identifier.evaluate(self);
                    let the_string = eval.downcast_ref::<String>().expect("Evaluated an identifier that I thought was a string but turns out it's not. What gives?");
                    StringLiteralExpression::new(the_string.clone()).to_list_literal()
                } else {
                    panic!("Called intrinsic String.to_list without a string argument")
                };
                Box::new(to_list)
            }

            _ => panic!("No such intrinsic: {}", name),
        }
    }

    pub fn get_value(&mut self, name: Name) -> Option<Box<dyn Any + '_>> {
        if self.has_intrisic(name.clone()) {
            // TODO: arg passing?
        }
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
