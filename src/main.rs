mod tokenizer;
use tokenizer::*;
mod parser;
use parser::Parser;

use std::cell::Cell;
use std::cell::RefCell;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

fn main() {
    let logger = Logger {
        enabled: Cell::new(true),
    };
    // open xxx.whl
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("[FATAL] must supply a valid path to a .whl file")
    }
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("[FATAL] couldn't read {}: {}", display, why),
    };
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Ok(_) => logger.info(format!("opened {} and got:\n{}", display, source.trim())),
        Err(why) => panic!("[FATAL] couldn't read {}: {}", display, why),
    };
    // run xxx.whl
    CORE_OBJECTS.with(|core| {
        core.borrow_mut().set_src(source.as_str());
        core.borrow_mut().get_program().execute();
        logger.info(format!(
            "output:\n{}",
            core.borrow_mut().get_program().stdout.clone()
        ));
    })
}
