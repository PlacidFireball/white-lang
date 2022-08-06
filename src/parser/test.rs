///
/// WhiteLang Parser Tests
///

#[cfg(test)]
mod test {
    use crate::parser::expression::additiveexpression::AdditiveExpression;
    use crate::parser::expression::booleanliteralexpression::BooleanLiteralExpression;
    use crate::parser::expression::comparisonexpression::ComparisonExpression;
    use crate::parser::expression::equalityexpression::EqualityExpression;
    use crate::parser::expression::factorexpression::FactorExpression;
    use crate::parser::expression::floatliteralexpression::FloatLiteralExpression;
    use crate::parser::expression::functioncallexpression::FunctionCallExpression;
    use crate::parser::expression::identifierexpression::IdentifierExpression;
    use crate::parser::expression::integerliteralexpression::IntegerLiteralExpression;
    use crate::parser::expression::listliteralexpression::ListLiteralExpression;
    use crate::parser::expression::logicalexpression::LogicalExpression;
    use crate::parser::expression::nullliteralexpression::NullLiteralExpression;
    use crate::parser::expression::parenthesizedexpression::ParenthesizedExpression;
    use crate::parser::expression::stringliteralexpression::StringLiteralExpression;
    use crate::parser::expression::structexpression::StructExpression;
    use crate::parser::expression::unaryexpression::UnaryExpression;
    use crate::parser::parser_traits::Statement;
    use crate::parser::statement::assignmentstatement::AssignmentStatement;
    use crate::parser::statement::forstatement::ForStatement;
    use crate::parser::statement::functioncallstatement::FunctionCallStatement;
    use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
    use crate::parser::statement::ifstatement::IfStatement;
    use crate::parser::statement::printstatement::PrintStatement;
    use crate::parser::statement::structdefinitionstatement::StructDefinitionStatement;
    use crate::parser::statement::variablestatement::VariableStatement;
    use crate::parser::statement::whilestatement::WhileStatement;
    use crate::parser::symbol_table::SymbolTable;
    use crate::parser::whitetypes::Type;
    use crate::TokenType::*;
    use crate::Tokenizer;
    use crate::IS_TESTING;
    use crate::{CoreObjects, Parser};

    fn init_parser(src: String) -> Parser {
        let mut core: CoreObjects = CoreObjects::new(src.as_str());
        println!("Start test...");
        print!("Tokens: [");
        let tokenizer = core.get_tokenizer_mut();
        for token in tokenizer.get_token_list() {
            print!("{} ", token.get_type());
        }
        println!("]");
        core.parser
    }

    #[test]
    /// Make sure that token consuming is working properly
    fn test_match_and_consume() {
        let mut parser = Parser::new(&mut Tokenizer::new(String::from("1")));
        assert_eq!(parser.curr_idx, 0);
        assert!(parser.match_and_consume(Int));
        assert_eq!(parser.curr_idx, 1);
    }

    #[test]
    #[should_panic]
    /// Assert that a string is not an integer
    fn test_require() {
        let mut parser = init_parser("\"\"".to_string());
        parser.require_token(Int);
    }

    #[test]
    /// Test parsing an integer literal
    fn test_parse_integer_expression() {
        let parser = init_parser("1".to_string());
        assert!(parser
            .expr
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>()
            .is_some());
    }

    #[test]
    /// Test parsing a string literal
    fn test_parse_string_expression() {
        let parser = init_parser("\"Hello World\"".to_string());
        if let None = parser
            .expr
            .to_any()
            .downcast_ref::<StringLiteralExpression>()
        {
            panic!()
        }
    }

    #[test]
    /// Test parsing a float literal
    fn test_parse_float_expression() {
        let parser = init_parser("1.1".to_string());
        assert!(parser
            .expr
            .to_any()
            .downcast_ref::<FloatLiteralExpression>()
            .is_some());
        assert_eq!(parser.expr.debug(), "1.1");
    }

    #[test]
    /// Test parsing a null literal expression
    fn test_null_literal_expression() {
        let parser = init_parser("null".to_string());
        assert!(parser
            .expr
            .to_any()
            .downcast_ref::<NullLiteralExpression>()
            .is_some());
    }

    #[test]
    /// Test parsing a boolean literal expression
    fn test_boolean_literal_expression() {
        let mut parser = init_parser("true".to_string());
        let mut expr = parser.expr;
        assert!(expr
            .to_any()
            .downcast_ref::<BooleanLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "true");
        parser = init_parser("false".to_string());
        expr = parser.expr;
        assert!(expr
            .to_any()
            .downcast_ref::<BooleanLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "false");
    }

    #[test]
    /// test parsing a function call expression
    fn test_function_call_expression() {
        let parser = init_parser("fn x(){} x();".to_string()); // had to modify this test with the core changes
        let expr = parser.statement_list[1].clone();
        assert!(expr
            .to_any()
            .downcast_ref::<FunctionCallStatement>()
            .is_some());
        //assert_eq!(expr.debug(), "x: ");
    }

    #[test]
    /// test parsing a function call expression, this time with args
    fn test_function_call_args_expression() {
        let mut parser = init_parser("fn foo(y : int, z : int) {} foo(1 ,2);".to_string());
        let uncertain_fcs = parser.statement_list[1].clone();
        let uncertain_fds = parser.statement_list[0].clone();
        let fcs = uncertain_fcs
            .to_any()
            .downcast_ref::<FunctionCallStatement>()
            .unwrap();
        let fds = uncertain_fds
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
        assert_eq!("This looks right 8/4/22", "This looks right 8/4/22");
        println!("{:?}", fds);
        println!("{:?}", fcs);
    }

    #[test]
    #[should_panic]
    /// test for errors when the function call doesn't have a closed paren
    fn test_fn_unterminated_args() {
        let mut parser = Parser::new(&mut Tokenizer::new("x(".to_string()));
        let expr = parser.parse_function_call_expression();
    }

    #[test]
    /// test parsing a comparison expression
    fn test_parse_comparison_expression() {
        let parser = init_parser("2 > 1".to_string());
        let expr = parser.expr.clone();
        assert!(expr
            .to_any()
            .downcast_ref::<ComparisonExpression>()
            .is_some());
        assert_eq!(expr.debug(), "2 > 1");
    }

    #[test]
    /// test parsing an additive expression
    fn test_parse_additive_expression() {
        let mut parser = init_parser("1 + 1".to_string());
        let mut expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(expr.debug(), "1 + 1");
        parser = init_parser("1 - 1".to_string());
        expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(expr.debug(), "1 - 1");
    }

    #[test]
    /// test parsing associativity of additive expressions
    fn additive_expressions_are_associative() {
        let parser = init_parser("1 + 1 - 1".to_string());
        let expr = parser.expr.clone();
        let additive_expression = expr.to_any().downcast_ref::<AdditiveExpression>().unwrap();
        let lhs = additive_expression
            .get_lhs()
            .to_any()
            .downcast_ref::<AdditiveExpression>();
        let rhs = additive_expression
            .get_rhs()
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>();
        assert!(lhs.is_some());
        assert!(rhs.is_some());
    }

    #[test]
    /// test parsing a factor expression
    fn test_parse_factor_expression() {
        let parser = init_parser("1 * 1".to_string());
        let expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<FactorExpression>().is_some());
        assert_eq!(expr.debug(), "1 * 1");
    }

    #[test]
    /// test parsing equality expressions
    fn test_parse_equality_expression() {
        let parser = init_parser("1 == 1".to_string());
        let expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<EqualityExpression>().is_some());
        assert_eq!(expr.debug(), "1 == 1");
    }

    #[test]
    /// test parsing list literal expressions
    fn test_parse_list_expression() {
        let parser = init_parser("[1, 2, 3, 4]".to_string());
        let expr = parser.expr.clone();
        assert!(expr
            .to_any()
            .downcast_ref::<ListLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "[1, 2, 3, 4]");
    }

    #[test]
    /// test parsing identifiers
    fn test_parse_identifier_expression() {
        let mut parser = Parser::new(&mut Tokenizer::new("x".to_string()));
        let expr = parser.parse_identifier_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .is_some());
        assert_eq!(expr.debug(), "x");
    }

    #[test]
    /// test parsing parenthesized expressions
    fn test_parse_parenthesized_expression() {
        let parser = init_parser("(1+1)".to_string());
        let expr = parser.expr.clone();
        assert!(expr
            .to_any()
            .downcast_ref::<ParenthesizedExpression>()
            .is_some());
        let typed_expr = expr
            .to_any()
            .downcast_ref::<ParenthesizedExpression>()
            .unwrap();
        let interior = typed_expr.get_expr();
        assert!(interior
            .to_any()
            .downcast_ref::<AdditiveExpression>()
            .is_some());
        assert_eq!(interior.debug(), "1 + 1");
    }

    #[test]
    /// beefy test for parsing unary expressions
    fn test_unary_expressions() {
        let mut parser = init_parser("not true".to_string());
        let mut expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<UnaryExpression>().is_some());
        parser = init_parser("-1".to_string());
        expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<UnaryExpression>().is_some());
        parser = init_parser("-1.1314".to_string());
        expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<UnaryExpression>().is_some());
        // TODO: maybe support this later?
        //parser = init_parser("not (2 > 3)".to_string()); // negation of larger expressions
        //expr = parser.expr.clone();
        //assert!(expr.to_any().downcast_ref::<UnaryExpression>().is_some());
        /*
        // TODO: potentially make not do bitwise negation instead of throwing an error
        // commented this part of the test out because the program will panic on parse errors now
        parser = init_parser("not 1".to_string()); // not 1 doesn't make any sense because an integer is not a boolean
        expr = parser.expr.clone();
        assert!(parser.has_errors());
        parser = init_parser("-true".to_string());
        expr = parser.expr.clone();
        assert!(parser.has_errors());
        */
    }

    #[test]
    /// test retrieving values from SymbolTable
    fn test_symbol_table() {
        let mut st: SymbolTable = SymbolTable::new();
        st.register_symbol(String::from("x"), Type::Integer);
        assert!(st.has_symbol(String::from("x")));
        assert_eq!(
            st.get_symbol_type(String::from("x")).unwrap(),
            Type::Integer
        );
    }

    #[test]
    fn test_parse_variable_statement() {
        let parser = init_parser("let x = 10;".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(variable_statement
            .get_expr()
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>()
            .is_some());
        assert_eq!(variable_statement.get_type(), Type::Integer);
    }

    #[test]
    fn test_parse_variable_statement_explicit_type() {
        let parser = init_parser("let x : string = \"Hello World\";".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(variable_statement
            .get_expr()
            .to_any()
            .downcast_ref::<StringLiteralExpression>()
            .is_some());
        assert_eq!(variable_statement.get_type(), Type::String);
    }

    #[test]
    #[should_panic]
    fn test_parse_variable_statement_bad_assignment_type() {
        let mut parser = init_parser(String::from("let x : string = 10;"));
        println!("{:?}", parser.statement_list);
    }

    #[test]
    fn test_parse_function_definition() {
        let parser = init_parser("fn foo() {}".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let _ = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
    }

    #[test]
    fn test_parse_function_definition_with_args() {
        let parser = init_parser("fn foo(x : int) {}".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let _ = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
    }
    #[test]
    fn test_parse_function_definition_with_stmts() {
        let parser = init_parser("fn foo() : int { let x = 10; return x; }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let _ = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_function_definition_mismatched_return() {
        let parser = init_parser("fn foo() : string { let x = 10; return x; }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors()); // parser will actually have errors and panic for reals
        let _ = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
    }

    #[test]
    fn test_fn_returns_list() {
        let parser = init_parser("fn foo() : list<int> { return [1, 2, 3]; }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let _ = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
    }

    #[test]
    fn test_for_statement_parses() {
        let parser = init_parser("for (x in [1, 2, 3]) { let y = x; }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(!parser.has_errors());
        let _ = stmt.to_any().downcast_ref::<ForStatement>().unwrap();
    }

    #[test]
    fn test_assign_statement_parses() {
        let parser = init_parser("let x : int = 10; x = 5;".to_string());
        let _ = parser.statement_list[0]
            .to_any()
            .downcast_ref::<VariableStatement>()
            .unwrap();
        let _ = parser.statement_list[1]
            .to_any()
            .downcast_ref::<AssignmentStatement>()
            .unwrap();
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_print_statement_parse() {
        let parser = init_parser("print(1);".to_string());
        let _ = parser
            .statement_list
            .first()
            .unwrap()
            .to_any()
            .downcast_ref::<PrintStatement>()
            .unwrap();
    }

    #[test]
    fn test_if_statement_with_else_parses() {
        let parser = init_parser(
            "if (1 < 2) { print(\"Hello World\"); } else { print(\"Goodbye!\"); }".to_string(),
        );
        let stmt = parser.statement_list.first().unwrap();
        assert!(stmt.to_any().downcast_ref::<IfStatement>().is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_if_statement_no_else_parses() {
        let parser = init_parser("if (1 < 2) { print(\"Hello World!\\n\"); }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(stmt.to_any().downcast_ref::<IfStatement>().is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_if_statement_empty_parses() {
        let parser = init_parser("if (1 < 2) { }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(stmt.to_any().downcast_ref::<IfStatement>().is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_function_call_statement_parses() {
        let parser =
            init_parser("fn foo() : string { return \"Hello World!\\n\"; } foo();".to_string());
        let stmt = parser.statement_list[1].clone();
        assert!(stmt
            .to_any()
            .downcast_ref::<FunctionCallStatement>()
            .is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_logical_expression_parses() {
        let parser = init_parser("true && false".to_string());
        let expr = parser.expr.clone();
        assert!(expr.to_any().downcast_ref::<LogicalExpression>().is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_while_statement_parses() {
        let parser = init_parser("while (true) { print(1); }".to_string());
        let stmt = parser.statement_list.first().unwrap();
        assert!(stmt.to_any().downcast_ref::<WhileStatement>().is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_for_statement_with_extra_var_parses() {
        let src = "
        let x = [1, 2, 3];
        for (y in x) {
            print(y);
        }"
        .to_string();
        let parser = init_parser(src);
        let variable = parser.statement_list[0].clone();
        let for_stmt = parser.statement_list[1].clone();
        assert!(variable
            .to_any()
            .downcast_ref::<VariableStatement>()
            .is_some());
        assert!(for_stmt.to_any().downcast_ref::<ForStatement>().is_some());
        assert!(!parser.has_errors());
    }

    #[test]
    /*
    This test comes from recursion adventures with the fibonacci sequence
    fib(x-1) -> fib(x, -1) somehow
    */
    fn test_subtractive_expression_parses_correctly() {
        let parser = init_parser("x-1".to_string());
        assert!(parser
            .expr
            .clone()
            .to_any()
            .downcast_ref::<AdditiveExpression>()
            .is_some());
        let expr = parser.expr.clone();
        let additive_expr = expr.to_any().downcast_ref::<AdditiveExpression>().unwrap();
        assert!(additive_expr
            .get_lhs()
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .is_some());
        assert!(additive_expr
            .get_rhs()
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>()
            .is_some());
    }

    #[test]
    fn test_struct_definition_no_methods_parses() {
        let parser = init_parser(
            "
        struct X { y: string, z: int };
        "
            .to_string(),
        );
        //crate::LOGGER.debug(format!("{:?}", parser.statement_list[0]));
        assert!(parser.statement_list[0]
            .clone()
            .to_any()
            .downcast_ref::<StructDefinitionStatement>()
            .is_some());
    }

    #[test]
    fn test_struct_definition_methods_parses() {
        let parser = init_parser("
        struct X { y: string, z: int } implement X { fn foo(xx: string) {} fn bar(xy: int) {} fn baz(xz: float) {} };
        ".to_string());
        //crate::LOGGER.debug(format!("{:?}", parser.statement_list[0]));
        assert!(parser.statement_list[0]
            .clone()
            .to_any()
            .downcast_ref::<StructDefinitionStatement>()
            .is_some());
    }

    #[test]
    fn test_struct_expression_parses() {
        let parser = init_parser(
            "
        struct X { x: int };
        let myStruct = X(x = 1);
        struct Y { y: string, x: int } implement Y { fn foo(z: string) { print(z); } };
        let myStruct2 = Y(x = 2, y = \"Hello World\");
        print(myStruct.x);
        myStruct2.foo(\"Hello World\");
        "
            .to_string(),
        );
        assert!(parser.statement_list[1]
            .clone()
            .to_any()
            .downcast_ref::<VariableStatement>()
            .is_some());
        let v_s = parser.statement_list[1].clone();
        let strct_expr = v_s.get_expr();
        println!("{:?}", strct_expr);
        assert_eq!("This looks right", "This looks right");
    }
}
