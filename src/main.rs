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
use crate::core::Core;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(
        Core::new(
            "\
            fn foo(x : int) { \
                print(x);\
            } \
            foo(1);"
        )
    );
}

#[allow(unused_variables)]
fn main() {
    CORE.with(|core| {
        core.borrow_mut()
            .deref_mut()
            .get_program()
            .execute();
    })
}
