///
/// WhiteLang Parser Tests
///
#[cfg(test)]
mod test {
    use crate::{Parser, Tokenizer};
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
    use crate::parser::expression::nullliteralexpression::NullLiteralExpression;
    use crate::parser::expression::parenthesizedexpression::ParenthesizedExpression;
    use crate::parser::expression::stringliteralexpression::StringLiteralExpression;
    use crate::parser::expression::unaryexpression::UnaryExpression;
    use super::*;
    use crate::parser::parser_traits::{Expression, Statement, ToAny};
    use crate::parser::statement::assignmentstatement::AssignmentStatement;
    use crate::parser::statement::forstatement::ForStatement;
    use crate::parser::statement::functioncallstatement::FunctionCallStatement;
    use crate::parser::statement::functiondefinitionstatement::FunctionDefinitionStatement;
    use crate::parser::statement::ifstatement::IfStatement;
    use crate::parser::statement::variablestatement::VariableStatement;
    use crate::parser::symbol_table::SymbolTable;
    use crate::parser::whitetypes::Type;
    use crate::TokenType::*;


    fn init_parser(src: String) -> Parser {
        let tokenizer: Tokenizer = Tokenizer::init(src);
        Parser::init(&mut tokenizer.clone())
    }

    #[test]
    /// Make sure that token consuming is working properly
    fn test_match_and_consume() {
        let mut parser = init_parser(String::from("1"));
        assert_eq!(parser.curr_idx, 0);
        assert_eq!(parser.match_and_consume(Int), true);
        assert_eq!(parser.curr_idx, 1);
    }

    #[test]
    /// Assert that a string is not an integer
    fn test_require() {
        let mut parser = init_parser("\"\"".to_string());
        parser.require_token(Int);
        assert_eq!(parser.has_errors(), true);
    }

    #[test]
    /// Test parsing an integer literal
    fn test_parse_integer_expression() {
        let mut parser = init_parser("1".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>()
            .is_some());
    }

    #[test]
    /// Test parsing a string literal
    fn test_parse_string_expression() {
        let mut parser = init_parser("\"Hello World\"".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<StringLiteralExpression>()
            .is_some());
    }

    #[test]
    /// Test parsing a float literal
    fn test_parse_float_expression() {
        let mut parser = init_parser("1.1".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<FloatLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "1.1");
    }

    #[test]
    /// Test parsing a null literal expression
    fn test_null_literal_expression() {
        let mut parser = init_parser("null".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<NullLiteralExpression>()
            .is_some());
    }

    #[test]
    /// Test parsing a boolean literal expression
    fn test_boolean_literal_expression() {
        let mut parser = init_parser("true false".to_string());
        let mut expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<BooleanLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "true");
        expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<BooleanLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "false");
    }

    #[test]
    /// test parsing a function call expression
    fn test_function_call_expression() {
        let mut parser = init_parser("x()".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<FunctionCallExpression>()
            .is_some());
        assert_eq!(expr.debug(), "x: ");
    }

    #[test]
    /// test parsing a function call expression, this time with args
    fn test_function_call_args_expression() {
        let mut parser = init_parser("x(1, 2)".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<FunctionCallExpression>()
            .is_some());
        assert_eq!(expr.debug(), "x: 1 2 ");
    }

    #[test]
    /// test for errors when the function call doesn't have a closed paren
    fn test_fn_unterminated_args() {
        let mut parser = init_parser("x(".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<FunctionCallExpression>()
            .is_some());
        assert!(parser.has_errors()); // TODO: FunctionCallExpression has errors instead of the parser
    }

    #[test]
    /// test parsing a comparison expression
    fn test_parse_comparison_expression() {
        let mut parser = init_parser("2 > 1".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<ComparisonExpression>()
            .is_some());
        assert_eq!(expr.debug(), "2 > 1");
    }

    #[test]
    /// test parsing an additive expression
    fn test_parse_additive_expression() {
        let mut parser = init_parser("1 + 1 1 - 1".to_string());
        let mut expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(expr.debug(), "1 + 1");
        expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(expr.debug(), "1 - 1");
    }

    #[test]
    /// test parsing associativity of additive expressions
    fn additive_expressions_are_associative() {
        let mut parser = init_parser("1 + 1 - 1".to_string());
        let expr = parser.parse_expression();
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
        let mut parser = init_parser("1 * 1".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<FactorExpression>().is_some());
        assert_eq!(expr.debug(), "1 * 1");
    }

    #[test]
    /// test parsing equality expressions
    fn test_parse_equality_expression() {
        let mut parser = init_parser("1 == 1".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<EqualityExpression>().is_some());
        assert_eq!(expr.debug(), "1 == 1");
    }

    #[test]
    /// test parsing list literal expressions
    fn test_parse_list_expression() {
        let mut parser = init_parser("[1, 2, 3, 4]".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<ListLiteralExpression>()
            .is_some());
        assert_eq!(expr.debug(), "[1, 2, 3, 4]");
    }

    #[test]
    /// test parsing identifiers
    fn test_parse_identifier_expression() {
        let mut parser = init_parser("x".to_string());
        let expr = parser.parse_expression();
        assert!(expr
            .to_any()
            .downcast_ref::<IdentifierExpression>()
            .is_some());
        assert_eq!(expr.debug(), "x");
    }

    #[test]
    /// test parsing parenthesized expressions
    fn test_parse_parenthesized_expression() {
        let mut parser = init_parser("(1+1)".to_string());
        let expr = parser.parse_expression();
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
        let mut parser = init_parser("not not true".to_string()); // not not true is valid WhiteLang :)
        let mut expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<UnaryExpression>().is_some());
        parser = init_parser("-(-1)".to_string());
        expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<UnaryExpression>().is_some());
        // TODO: potentially make not do bitwise negation instead of throwing an error
        parser = init_parser("not 1".to_string()); // not 1 doesn't make any sense because an integer is not a boolean
        expr = parser.parse_expression();
        assert!(expr.has_errors());
        parser = init_parser("-true".to_string());
        expr = parser.parse_expression();
        assert!(expr.has_errors());

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
        let mut parser = init_parser("let x = 10;".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(!variable_statement.has_errors());
        assert!(variable_statement
            .get_expr()
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>()
            .is_some());
        assert_eq!(variable_statement.get_type(), Type::Integer);
    }

    #[test]
    fn test_parse_variable_statement_explicit_type() {
        let mut parser = init_parser("let x : string = \"Hello World\";".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(!variable_statement.has_errors());
        assert!(variable_statement
            .get_expr()
            .to_any()
            .downcast_ref::<StringLiteralExpression>()
            .is_some());
        assert_eq!(variable_statement.get_type(), Type::String);
    }

    #[test]
    fn test_parse_variable_statement_bad_assignment_type() {
        let mut parser = init_parser("let x : string = 10;".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(variable_statement.has_errors());
    }

    #[test]
    fn test_parse_function_definition() {
        let mut parser = init_parser("fn foo() {}".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let fds = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
        assert!(!fds.has_errors());
    }

    #[test]
    fn test_parse_function_definition_with_args() {
        let mut parser = init_parser("fn foo(x : int) {}".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let fds = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
        assert!(!fds.has_errors());
    }
    #[test]
    fn test_parse_function_definition_with_stmts() {
        let mut parser = init_parser("fn foo() : int { let x = 10; return x; }".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let fds = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
        assert!(!fds.has_errors());
    }

    #[test]
    fn test_parse_function_definition_mismatched_return() {
        let mut parser = init_parser("fn foo() : string { let x = 10; return x; }".to_string());
        let mut stmt = parser.parse_statement();
        stmt.validate(&mut SymbolTable::new());
        assert!(!parser.has_errors());
        let fds = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
        assert!(fds.has_errors());
    }

    #[test]
    fn test_fn_returns_list() {
        let mut parser = init_parser("fn foo() : list<int> { return [1, 2, 3]; }".to_string());
        let mut stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        stmt.validate(&mut SymbolTable::new());
        let fds = stmt
            .to_any()
            .downcast_ref::<FunctionDefinitionStatement>()
            .unwrap();
        assert!(!fds.has_errors());
    }

    #[test]
    fn test_for_statement_parses() {
        let mut parser = init_parser("for (x in [1, 2, 3]) { let y = x; }".to_string());
        let mut stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        stmt.validate(&mut SymbolTable::new());
        let for_stmt = stmt.to_any().downcast_ref::<ForStatement>().unwrap();
        assert!(!for_stmt.has_errors());
    }

    #[test]
    fn test_assign_statement_parses() {
        let mut parser = init_parser("let x : int = 10; x = 5;".to_string());
        let mut st = SymbolTable::new();
        let mut var_stmt = parser.parse_statement();
        var_stmt.validate(&mut st);
        let mut stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        stmt.validate(&mut st);
        let a_stmt = stmt.to_any().downcast_ref::<AssignmentStatement>().unwrap();
        assert!(!a_stmt.has_errors());
    }

    #[test]
    fn test_print_statement_parse() {
        let mut parser = init_parser("print(1);".to_string());
        let mut st = SymbolTable::new();
        let mut print_stmt = parser.parse_statement();
        print_stmt.validate(&mut st);
        assert!(!print_stmt.has_errors());
    }

    #[test]
    fn test_if_statement_with_else_parses() {
        let mut parser = init_parser("if (1 < 2) { print(\"Hello World\"); } else { print(\"Goodbye!\"); }".to_string());
        let mut st = SymbolTable::new();
        let mut stmt = parser.parse_statement();
        stmt.validate(&mut st);
        assert!(stmt.to_any().downcast_ref::<IfStatement>().is_some());
        assert!(!stmt.has_errors());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_if_statement_no_else_parses() {
        let mut parser = init_parser("if (1 < 2) { print(\"Hello World!\\n\"); }".to_string());
        let mut st = SymbolTable::new();
        let mut stmt = parser.parse_statement();
        stmt.validate(&mut st);
        assert!(stmt.to_any().downcast_ref::<IfStatement>().is_some());
        assert!(!stmt.has_errors());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_if_statement_empty_parses() {
        let mut parser = init_parser("if (1 < 2) { }".to_string());
        let mut st = SymbolTable::new();
        let mut stmt = parser.parse_statement();
        stmt.validate(&mut st);
        assert!(stmt.to_any().downcast_ref::<IfStatement>().is_some());
        assert!(!stmt.has_errors());
        assert!(!parser.has_errors());
    }

    #[test]
    fn test_function_call_statement_parses() {
        let mut parser = init_parser("fn foo() : string { return \"Hello World!\\n\"; } foo();".to_string());
        let mut st = SymbolTable::new();
        parser.parse_statement();
        let mut stmt = parser.parse_statement();
        stmt.validate(&mut st);
        assert!(stmt.to_any().downcast_ref::<FunctionCallStatement>().is_some());
        assert!(!stmt.has_errors());
        assert!(!parser.has_errors());
    }

}
