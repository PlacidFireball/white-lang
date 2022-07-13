mod tokenizer;
use tokenizer::*;
mod parser;
use crate::program::Program;
use parser::Parser;

use std::cell::RefCell;
use std::ops::DerefMut;

mod config;
mod program;
mod runtime;
mod core;
use crate::core::CoreObjects;

thread_local! {
    static CORE_OBJECTS: RefCell<CoreObjects> = RefCell::new(
        CoreObjects::new(
            "\
            fn foo(x : int) { \
                print(x);\
            } \
            foo(1);"
        )
    );

    pub static IS_TESTING: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

#[allow(unused_variables)]
fn main() {
    CORE_OBJECTS.with(|core| {
        core.borrow_mut()
            .get_program()
            .execute();
    })
}
