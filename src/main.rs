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
    let src = "
        // returns the nth fibonacci number
        fn fib(n : int) : int {
            if (n == 0) {
                return 0;
            }
            if (n == 1) {
                return 1;
            }
            return fib(n-1) + fib(n-2);
        }
        print(fib(2));";
    let mut program: Program = Program::from_src(String::from(src));
    program.execute();
    print!("{}", program.stdout);
}
