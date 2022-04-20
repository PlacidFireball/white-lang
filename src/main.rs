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
        fn calculateSum(n : int) : int {\
            return (n*(n+1))/2;\
        }\
        let x : int = 1;\
        while (x < 10) {\
            print(calculateSum(x));\
        }";
    let mut program: Program = Program::from_src(String::from(src));
    program.execute();
    print!("{}", program.stdout);
}
