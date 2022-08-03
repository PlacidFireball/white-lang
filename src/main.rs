mod tokenizer;

use std::borrow::BorrowMut;
use tokenizer::*;
mod parser;
use crate::parser::Parser;

use clap::{App, Arg, Command, Parser as ClapParser, SubCommand};
use std::cell::Cell;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use std::panic;

mod config;
mod core;
mod javascript;
mod logger;
mod program;
mod runtime;
use crate::logger::Logger;

use crate::core::CoreObjects;

thread_local! {
    pub static CORE_OBJECTS: RefCell<CoreObjects> = RefCell::new(
        CoreObjects::new_uninit()
    );

    pub static IS_TESTING: Cell<bool> = Cell::new(false); // legacy trying to move away from this

    pub static DEBUG_INFO_LOGGING_ENABLED: Cell<bool> = Cell::new(false);

    pub static RUNTIME_DEBUG_LOGGING_ENABLED: Cell<bool> = Cell::new(false);
}

const LOGGER: Logger = Logger {};

fn main() {
    let matches = App::new("white-lang")
        .version("1.0.0")
        .about("The white-lang compiler")
        .arg(
            Arg::new("src")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to a file containing your source code"),
        )
        .arg(
            Arg::new("interpret")
                .short('i')
                .long("interpret")
                .takes_value(false)
                .help("Whether or not to use rust as a runtime for white-lang"),
        )
        .arg(
            Arg::new("transpile")
                .short('t')
                .long("transpile")
                .takes_value(false)
                .help("Whether or not to transpile the source file to javascript"),
        )
        .arg(
            Arg::new("compile")
                .short('c')
                .long("compile")
                .takes_value(false)
                .help("Whether or not to compile the source code to x86 assembly"),
        )
        .arg(
            Arg::new("parse-info")
                .short('p')
                .long("parse-info")
                .takes_value(false)
                .help("Log debug information for the parser to the console"),
        )
        .arg(
            Arg::new("runtime-info")
                .short('r')
                .long("runtime-info")
                .takes_value(false)
                .help("Show runtime debug information on execute"),
        )
        .subcommand(
            Command::new("test")
                .about("Run tests")
                .subcommand_required(false), // TODO: run when we do `white-lang test`
        )
        .get_matches();
    let src_path = match matches.get_one::<String>("src") {
        None => panic!("You must provide a source path"),
        Some(path) => path.clone(),
    };
    let should_transpile = matches.is_present("transpile");
    let should_compile = matches.is_present("compile");
    let should_interpret = matches.is_present("interpret");

    if matches.is_present("parse-info") {
        DEBUG_INFO_LOGGING_ENABLED.with(|c| c.set(true));
        LOGGER.info("verbose output is enabled".to_string());
    }

    if matches.is_present("runtime-info") {
        RUNTIME_DEBUG_LOGGING_ENABLED.with(|c| c.set(true));
        LOGGER.info("runtime-info is enabled".to_string());
    }

    env::set_var("RUST_BACKTRACE", "1");
    // open xxx.whl
    let path = Path::new(&src_path);
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("[FATAL] couldn't read {}: {}", display, why),
    };
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Ok(_) => LOGGER.debug(
            format!("opened {} and got:\n{}", display, source.trim()),
            true,
        ),
        Err(why) => panic!("[FATAL] couldn't read {}: {}", display, why),
    };
    // run xxx.whl
    CORE_OBJECTS.with(|core| {
        core.borrow_mut().set_src(source.as_str());
        if should_compile {
            LOGGER.warn(format!("compilation is unimplemented."));
        }
        if should_transpile {
            let js = core.borrow_mut().get_program_mut().transpile_to_js();
            LOGGER.info(format!("javascript:\n{}", js));
        }
        if should_interpret {
            core.borrow_mut().get_program_mut().execute();
            LOGGER.info(format!(
                "output:\n{}",
                core.borrow_mut().get_program_mut().stdout.clone()
            ));
        }
    })
}
