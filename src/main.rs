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
    let src = "\
    let x : int = 10;\
    print(x);\
    let y = 30;\
    print(x+y);\
    ";
    let mut program : Program = Program::from_src(
        String::from(src)
    );
    program.execute();
    print!("{}", program.output);
}
