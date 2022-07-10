#[cfg(test)]
mod test {
    use crate::*;

    fn init_parser(src: String) -> Parser {
        let tokenizer: Tokenizer = Tokenizer::init(src);
        Parser::new(&mut tokenizer.clone())
    }

    fn test_execute(src: &str, expected: &str) {
        let mut parser = init_parser(src.to_string());
        parser.parse();
        let mut program = Program::from_parser(&mut parser);
        program.execute();
        assert_eq!(program.stdout.as_str(), expected);
    }

    #[test]
    fn test_basic_expression_eval() {
        test_execute("1", "1\n");
        test_execute("false", "false\n");
        test_execute("null", "null\n");
        test_execute("\"Hello World!\"", "Hello World!\n");
        test_execute("[1, 2, 3, 4]", "[1, 2, 3, 4]\n");
    }

    #[test]
    fn test_additive_expression_eval_integers() {
        test_execute("1 + 1", "2\n");
        test_execute("2 + 3", "5\n");
        test_execute("1 + 0", "1\n");
        test_execute("1 + -1", "0\n");
        test_execute("1 - 1", "0\n");
        test_execute("1 - -1", "2\n");
        test_execute("1 - 0", "1\n");
        test_execute("2 - 3", "-1\n");
    }

    #[test]
    fn test_additive_expression_eval_float() {
        test_execute("1.1 - 0.2", "0.9000000000000001\n");
        test_execute("1.1 - 2.1", "-1\n");
        test_execute("0.33 + 0.33", "0.66\n");
        test_execute("21.54 + 0.46", "22\n");
    }

    #[test]
    fn test_factor_expression_eval_integers() {
        test_execute("9 * 3", "27\n");
        test_execute("8 * -1", "-8\n");
        test_execute("-1 * -1", "1\n");
        test_execute("9 / -3", "-3\n");
        test_execute("0 * 8", "0\n");
        test_execute("0 / 456.24", "0\n");
    }

    #[test]
    fn test_factor_expression_eval_floats() {
        test_execute("9 * 0.33", "2.97\n");
        test_execute("9 * 0.1", "0.9\n");
        test_execute("9 / 3.0", "3\n");
    }

    #[test]
    fn test_comparison_expression_eval() {
        test_execute("2 < 1", "false\n");
        test_execute("2 <= 2", "true\n");
        test_execute("1 < 2", "true\n");
        test_execute("1 >= 1", "true\n");
        test_execute("2 > 1", "true\n");
        test_execute("-1 < 0", "true\n");
    }

    #[test]
    fn test_equality_expression_eval() {
        test_execute("1 == 1", "true\n");
        test_execute("1 != 2", "true\n");
        test_execute("1 != 1", "false\n");
        test_execute("1 == null", "false\n");
        test_execute("\"\" == null", "false\n");
        test_execute("\"\" == \"\"", "true\n");
        test_execute("null == null", "true\n");
        test_execute("1 == 1.0", "true\n");
        test_execute("1 != 1.1", "true\n");
        test_execute("1.1 != null", "true\n");
    }

    #[test]
    fn test_basic_assignment() {
        test_execute("let x : int = 0; x = 1; print(x);", "1\n");
    }

    #[test]
    fn assignment_statement_executes() {
        let src = "\
        let x : int = 0;\
        x = x + 1;\
        print(x);";
        test_execute(src, "1\n");
    }

    #[test]
    fn test_if_statement_execute() {
        test_execute("if(false) { print(1); } else { print(2); }", "2\n");
        test_execute("if(true) { print(1); } else { print(2); }", "1\n");
        let src = "\
        let x = 10;\
        if (x > 9) { \
            print(1); \
        } else { \
            print(2); \
        }";
        test_execute(src, "1\n");
    }

    #[test]
    fn test_if_statements_statically_scoped() {
        let mut src = "\
        let x = 10; \
        if (true) { \
            let x = 20; \
            print(x); \
        }";
        test_execute(src, "20\n");
        src = "\
        let x = 10;\
        if (false) {}\
        else {\
            print(x); \
        }";
        test_execute(src, "10\n");
    }

    #[test]
    fn test_function_call_no_return() {
        let src = "\
        fn foo(x : int) { \
            print(x);\
        } \
        foo(1);";
        test_execute(src, "1\n");
    }

    #[test]
    fn test_function_call_return() {
        let src = "
        fn foo(x: int) : int { 
            return x + 1;
        }
        print(foo(1));";
        test_execute(src, "2\n");
    }

    #[test]
    fn test_logical_expression_evaluates() {
        let mut src = "print(true && false);";
        test_execute(src, "false\n");
        src = "let x = 10; let y = 20; let z = 30; print(x < y && x < z);";
        test_execute(src, "true\n");
        src = "let x = 10; let y = 20; let z = 30; print(x > y || x > z);";
        test_execute(src, "false\n");
    }

    #[test]
    fn test_while_statement_executes() {
        let src = "
        let x : int = 0;
        while (x < 5) {
            print(x);
            x = x + 1; // this test is not passing because of this statement, causes infinite recursion
        }";
        test_execute(src, "0\n1\n2\n3\n4\n");
    }

    
    #[test]
    fn test_function_recursion_executes() {
        let src = "\
        fn foo(x : int) : int {\
            if (x == 0) {\
                return 1;\
            }\
            else {\
               return foo(x-1);\
            }\
        }\
        print(foo(1));";
        test_execute(src, "1\n");
    }

    #[test]
    fn test_basic_program() {
        let src = "\
        fn finiteSum(n : int) : int {\
            return n*(n+1)/2;\
        }\
        let x = 10;
        while (x > 0) {
            print(finiteSum(x));
            x = x - 1;
        }";
        test_execute(src, "55\n45\n36\n28\n21\n15\n10\n6\n3\n1\n");
    }

    #[test]
    fn test_classic_fibonacci() {
        // TODO: multiple function calls in the return statement cause stack overflow. basic recursion works.
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
        print(fib(6));";
        test_execute(src, "5\n");
    }

    #[test]
    fn test_top_level_break_while() {
        let src = "
        while(true) { 
            print(1); 
            break; 
        }";
        test_execute(src, "1\n");
    }

    #[test]
    fn test_inner_break_works() {
        let src = "
        while(true) {
            print(1);
            if (true) {
                break;
            }
        }";
        test_execute(src, "1\n");
    }

    #[test]
    fn test_for_statement_works() {
        let src = "
        for (x in [1, 2, 3]) { 
            print(x);
        }";
        test_execute(src, "1\n2\n3\n");
    }

    #[test]
    fn test_for_statement_string_list() {
        let src = "
        for (x in [\"Hello\", \"World\", \"!\"]) { 
            print(x); 
        }";
        test_execute(src, "Hello\nWorld\n!\n")
    }

    #[test]
    fn test_for_statement_with_extra_var() {
        let src = "
        let x = [1, 2, 3];
        for (y in x) {
            print(y);
        }";
        test_execute(src, "1\n2\n3\n");
    }
    
    #[test]
    fn test_while_statement_break() {
        let src = "
        while(true) { break; }";
        test_execute(src, "");
    }

    #[test]
    fn test_for_statement_break() {
        let src = "for(x in [1,2,3]) { print(x); break; }";
        test_execute(src, "1\n");
    }
}