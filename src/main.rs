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
            print(\"Returned 0\");
            return 0;
        }
        if (n == 1) {
            print(\"Returned 0\");
            return 0;
        }
        if (n == 2) {
            print(\"Returned 1\");
            return 1;
        }
        else {
            let a : int = fib(n-1);
            let b : int = fib(n-2);
            return a + b;
        }
    }
    print(fib(5));";
    let mut program: Program = Program::from_src(String::from(src));
    program.execute();
    print!("{}", program.stdout);
}
