mod tokenizer;
use tokenizer::*;
mod parser;
use parser::Parser;
use crate::program::Program;

mod config;
mod program;
mod runtime;

#[allow(unused_variables)]
fn main() {
    let mut program : Program = Program::from_src(
        String::from("print(\"Hello World!\");")
    );
    program.execute();
    print!("{}", program.output);
}
