mod tokenizer;
use tokenizer::*;
mod parser;
use crate::program::Program;
use parser::Parser;

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
    let mut program: Program = Program::from_src(String::from(src));
    program.execute();
    print!("{}", program.stdout);
}
