mod config;
mod core;
mod javascript;
mod logger;
mod parser;
mod program;
mod runtime;
mod tokenizer;

use clap::{App, AppSettings, Arg, ArgMatches, Command};
use std::cell::Cell;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::path::Path;

use crate::core::CoreObjects;
use crate::logger::Logger;
use crate::parser::Parser;
use crate::tokenizer::*;

thread_local! {
    pub static CORE_OBJECTS: RefCell<CoreObjects> = RefCell::new(
        CoreObjects::new_uninit()
    );

    pub static IS_TESTING: Cell<bool> = Cell::new(false); // legacy trying to move away from this

    pub static DEBUG_INFO_LOGGING_ENABLED: Cell<bool> = Cell::new(false);

    pub static RUNTIME_DEBUG_LOGGING_ENABLED: Cell<bool> = Cell::new(false);
}

const LOGGER: Logger = Logger {};

fn cli_builder() -> ArgMatches {
    App::new("white-lang")
        .version("1.0.0")
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DisableVersionFlag)
        .about("The white-lang compiler")
        .arg(
            Arg::new("src")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to a file containing your source code"),
        )
        .subcommands(vec![
            Command::new("interpret")
                .subcommand_required(false)
                .about("Interpret the source file using rust as a runtime for white-lang")
                .arg(
                    Arg::new("runtime-info")
                        .short('r')
                        .long("runtime-info")
                        .takes_value(false)
                        .help("Show runtime debug information on execute")
                ),
            Command::new("transpile")
                .subcommand_required(false)
                .about("Transpile the source file into javascript")
                .args(vec![
                    Arg::new("evaluate")
                        .short('e')
                        .long("evaluate")
                        .takes_value(false)
                        .help("Use node to evaluate the js file generated by white-lang\n[NOTE] requires that node is installed and in your PATH on your system"),
                    Arg::new("do-cleanup")
                        .short('d')
                        .long("do-cleanup")
                        .takes_value(false)
                        .help("Remove the javascript file after the transpile process, only does something if used with -e"),
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("What to name the output file generated by white-lang ex: FILE = myfile, output file will be myfile.js"),
                    ]
                ),
            Command::new("compile")
                .subcommand_required(false)
                .about("Compile the source code into x86 assembly")
        ])
        .arg(
            Arg::new("parse-info")
                .short('p')
                .long("parse-info")
                .takes_value(false)
                .help("Log debug information for the parser to the console"),
        )
        .get_matches()
}

fn get_filename_no_extension(path: &String) -> String {
    let path_string = path.to_string();
    let parts = path_string.split("/").collect::<Vec<&str>>();
    let white_lang_file_with_extension = parts
        .last()
        .expect("unexpected error getting name of the file");
    let ext_string = white_lang_file_with_extension.to_string();
    let no_ext_opt = ext_string.strip_suffix(config::WHITE_LANG_FILE_EXTENSION);
    match no_ext_opt {
        Some(no_ext) => no_ext.to_string(),
        None => "a".to_string(),
    }
}

fn main() {
    let matches = cli_builder();

    let mut should_transpile: bool = false;
    let mut should_compile: bool = false;
    let mut should_interpret: bool = false;
    // argument options
    {
        if let Some(_) = matches.subcommand_matches("interpret") {
            LOGGER.info("Interpreting the source file using rust as a runtime".to_string());
            should_interpret = true;
        }

        if let Some(_) = matches.subcommand_matches("compile") {
            LOGGER.info("Interpreting the source file using rust as a runtime".to_string());
            should_compile = true;
        }

        if let Some(_) = matches.subcommand_matches("transpile") {
            LOGGER.info("Interpreting the source file using rust as a runtime".to_string());
            should_transpile = true;
        }

        if !should_transpile && !should_interpret && !should_compile {
            LOGGER.warn("Please provide a command to specify what you want to do with the source file\n[HELP] -- interpret, compile or transpile".to_string());
            return;
        }



        if matches.is_present("parse-info") {
            DEBUG_INFO_LOGGING_ENABLED.with(|c| c.set(true));
            LOGGER.info("parse-info is enabled".to_string());
        }

        if matches.subcommand_matches("interpret").is_some() {
            let subcommand = matches.subcommand_matches("interpret").unwrap();
            if subcommand.is_present("runtime-info") {
                RUNTIME_DEBUG_LOGGING_ENABLED.with(|c| c.set(true));
                LOGGER.info("runtime-info is enabled".to_string());
            }
        }
    }

    env::set_var("RUST_BACKTRACE", "1");
    // open xxx.whl

    let src_path = match matches.get_one::<String>("src") {
        None => panic!("You must provide a source path"),
        Some(path) => path,
    };
    let path = Path::new(src_path);
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
    // do stuff with xxx.whl
    CORE_OBJECTS.with(|core| {
        core.borrow_mut().set_src(source.as_str());
        // compile the program
        if should_compile {
            LOGGER.warn(format!("compilation is unimplemented."));
        }
        // transpile the program
        if should_transpile {
            let js = core.borrow_mut().get_program_mut().transpile_to_js();
            let mut javascript_file_path = String::new();
            let mut should_evaluate = false;
            let mut should_cleanup = false;
            if let Some(subcommand) = matches.subcommand_matches("transpile") {
                if let Some(output) = subcommand.get_one::<String>("output") {
                    javascript_file_path = format!("{}.js", output)
                } else {
                    javascript_file_path = format!("{}.js", get_filename_no_extension(src_path));
                }
                if subcommand.is_present("evaluate") {
                    should_evaluate = true;
                }
                if subcommand.is_present("do-cleanup") {
                    should_cleanup = true;
                }
            }
            std::fs::write(format!("{}", javascript_file_path), js)
                .expect(format!("transpile: failed to write to {}", javascript_file_path).as_str());
            if should_evaluate {
                let node_exec = std::process::Command::new("node")
                    .arg(javascript_file_path.clone())
                    .output()
                    .expect(format!("Unable to execute `node {}`", javascript_file_path).as_str());

                LOGGER.info(format!(
                    "Ran: `node {}\nstdout:\n{}\nstderr:\n{}",
                    javascript_file_path,
                    String::from_utf8_lossy(&*node_exec.stdout),
                    String::from_utf8_lossy(&*node_exec.stderr)
                ));

                if should_cleanup {
                    std::process::Command::new("rm")
                        .arg(javascript_file_path.clone())
                        .spawn()
                        .expect(format!("Failed to execute `rm {}", javascript_file_path).as_str());
                }
            }
        }
        // interpret the program
        if should_interpret {
            core.borrow_mut().get_program_mut().execute();
            LOGGER.info(format!(
                "output:\n{}",
                core.borrow_mut().get_program_mut().stdout.clone()
            ));
        }
    })
}
