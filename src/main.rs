mod tokenizer;
use tokenizer::*;
mod parser;
use crate::program::Program;
use parser::Parser;

use std::cell::RefCell;
use std::ops::DerefMut;

mod config;
mod core;
mod program;
mod runtime;
use crate::core::CoreObjects;

thread_local! {
    static CORE_OBJECTS: RefCell<CoreObjects> = RefCell::new(
        CoreObjects::new_uninit()
    );

    pub static IS_TESTING: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

#[allow(unused_variables)]
fn main() {
    CORE_OBJECTS.with(|core| {
        core.borrow_mut().set_src(
            "\
            print(\"Hello World!\");\
            ",
        );
        core.borrow_mut().get_program().execute();
        println!("{}", core.borrow_mut().get_program().stdout);
    })
}
