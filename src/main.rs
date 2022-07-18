mod tokenizer;
use tokenizer::*;
mod parser;
use parser::Parser;

use std::cell::Cell;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::panic;

mod config;
mod core;
mod logger;
mod program;
mod runtime;
use crate::logger::Logger;

use crate::core::CoreObjects;

thread_local! {
    pub static CORE_OBJECTS: RefCell<CoreObjects> = RefCell::new(
        CoreObjects::new_uninit()
    );

    pub static IS_TESTING: Cell<bool> = Cell::new(false);
}

const LOGGER: Logger = Logger {};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // open xxx.whl
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args = vec!["white-lang".to_string(), "./scratch.whl".to_string()];
        LOGGER.warn("Didn't supply a path to a .whl file, defaulting to ./scratch.whl".to_string());
    }
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("[FATAL] couldn't read {}: {}", display, why),
    };
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Ok(_) => LOGGER.info(format!("opened {} and got:\n{}", display, source.trim())),
        Err(why) => panic!("[FATAL] couldn't read {}: {}", display, why),
    };
    // run xxx.whl
    CORE_OBJECTS.with(|core| {
        core.borrow_mut().set_src(source.as_str());
        core.borrow_mut().get_program_mut().execute();
        LOGGER.info(format!(
            "output:\n{}",
            core.borrow_mut().get_program_mut().stdout.clone()
        ));
    })
}
