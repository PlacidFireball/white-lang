use std::char;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)]
enum TokenType {
    // Types
	Str, Int, Long, Char, List, Boolean, Float,
    // Syntax
	LeftParen, RightParen, LeftBracket,
	RightBracket, LeftBrace, RightBrace, Equal, Bang,
	Plus, PlusPlus, PlusEqual, Minus, MinusMinus, 
    MinusEqual, Star, Slash, Band, Land, Bor, Lor, Lnot, 
	Lxor, SemiColon, Colon, EqualEqual, Comma, Dot,
    Greater, Less, BangEqual, GreaterEqual, LessEqual,
    // Keywords
	And, Not, If, While, For, In, Else, Return,
    Eof, Error, Identifier, Var, Print, Function,
    True, False,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, ", self)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Token {
    typ: TokenType,
    string_value: String,
    start: usize,
    end: usize,
    line: usize,
    line_offset: usize
}
impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.typ)
    }
}
impl Token {
    fn init(typ: TokenType, string_value: String, start: usize, end: usize, line: usize, line_offset: usize) -> Token {
        Token {
            typ,
            string_value,
            start,
            end,
            line,
            line_offset
        }
    }
    fn get_type(&self) -> TokenType {
        self.typ
    }
    fn get_string_value(&self) -> String {
        self.string_value.clone()
    }
}

fn init_keywords() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("fn".to_string(), TokenType::Function);
    keywords.insert("return".to_string(), TokenType::Return);

    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("not".to_string(), TokenType::Not);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("true".to_string(), TokenType::True);

    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("while".to_string(), TokenType::While);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("var".to_string(), TokenType::Var);
    keywords.insert("print".to_string(), TokenType::Print);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("in".to_string(), TokenType::In);
    keywords
}

#[derive(Debug)]
pub struct Tokenizer {
    token_list: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    src: String, 
    char_vec: Vec<char>,
    curr_char: char,
    position: usize, 
    line: usize, 
    line_offset: usize
}
impl std::fmt::Display for Tokenizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token List: {:?}\nsrc: {}\n", self.token_list, self.src)
    }
}
impl Tokenizer {
    pub fn init(src: String) -> Tokenizer {
        let char_vec: Vec<char> = src.chars().collect();
        Tokenizer {
            token_list: vec![],
            keywords: init_keywords(),
            src,
            char_vec: char_vec,
            curr_char: 'a',
            position: 0,
            line: 1, 
            line_offset: 0
        }
    }

    fn get_token(&self, idx: usize) -> Token {
        self.token_list[idx].clone()
    }
    fn add_token(&mut self, typ: TokenType, strval: String) {
        let len = strval.len();
        self.token_list.push(Token::init(typ, strval, self.position, self.position+len, self.line, self.line_offset));
    }
    fn is_keyword(&self, kw: &String) -> bool {
        self.keywords.contains_key(kw)
    }
    fn tokenization_end(&self) -> bool {
        self.position >= self.src.len()
    }

    fn match_and_consume(&mut self, a: char) -> bool {
        if self.peek() == a {
            self.curr_char = self.consume_char();
            return true;
        }
        false
    }
    fn consume_char(&mut self) -> char {    
        if self.tokenization_end() {
            return '\0';
        }
        let chr = self.char_vec[self.position];
        self.curr_char = chr; 
        self.position += 1;
        chr
    }
    fn peek(&mut self) -> char {
        self.char_vec[self.position]
    }
    fn peek_next(&mut self) -> char {
        self.char_vec[self.position + 1]
    }

    fn scan_token(&mut self) { 
        if self.scan_number() {
            return
        }
        else if self.scan_string() {
            return
        }
        else if self.scan_identifier() {
            return
        }
        self.scan_syntax();
    }

    fn scan_syntax(&mut self) {
        if !self.tokenization_end() {
            if self.match_and_consume('{') { self.add_token(TokenType::LeftBrace, String::from("{")); }
            else if self.match_and_consume('}') { self.add_token(TokenType::RightBrace, String::from("}")); }
            else if self.match_and_consume('[') { self.add_token(TokenType::LeftBracket, String::from("[")); }
            else if self.match_and_consume(']') { self.add_token(TokenType::RightBracket, String::from("]")); }
            else if self.match_and_consume('(') { self.add_token(TokenType::LeftParen, String::from("(")); }
            else if self.match_and_consume(')') { self.add_token(TokenType::RightParen, String::from(")")) }
            else if self.match_and_consume(';') { self.add_token(TokenType::SemiColon, String::from(";")); }
            else if self.match_and_consume(':') { self.add_token(TokenType::Colon, String::from(":")); }
            else if self.match_and_consume(',') { self.add_token(TokenType::Comma, String::from(",")); }
            else if self.match_and_consume('.') { self.add_token(TokenType::Dot, String::from(".")); }
            else if self.match_and_consume('>') { 
                if self.match_and_consume('=') { self.add_token(TokenType::GreaterEqual, String::from(">=")) }
                else { self.add_token(TokenType::Greater, String::from(">")); }
            }
            else if self.match_and_consume('<') {
                if self.match_and_consume('=') { self.add_token(TokenType::LessEqual, String::from("<=")) }
                else { self.add_token(TokenType::Less, String::from("<")); }
            }
            else if self.match_and_consume('/') {
                if self.match_and_consume('/') { 
                    while !self.match_and_consume('\n') && !self.tokenization_end() {
                        self.consume_char();
                        if self.tokenization_end() { break; }
                    }
                }
                else { self.add_token(TokenType::Slash, String::from("/")); }
            }
            else if self.match_and_consume('+') {
                if self.match_and_consume('+') { self.add_token(TokenType::PlusPlus, String::from("++")); }
                else if self.match_and_consume('=') { self.add_token(TokenType::PlusEqual, String::from("+=")); }
                else { self.add_token(TokenType::Plus, String::from("+")); }
            }
            else if self.match_and_consume('-') {
                if self.match_and_consume('-') { self.add_token(TokenType::MinusMinus, String::from("--")); }
                else if self.match_and_consume('=') { self.add_token(TokenType::MinusEqual, String::from("-=")); }
                else { self.add_token(TokenType::Minus, String::from("-")); }
            }
            else if self.match_and_consume('*') {
                self.add_token(TokenType::Star, String::from("*"));
            }
            else if self.match_and_consume('&') {
                if self.match_and_consume('&') { self.add_token(TokenType::Land, String::from("&&")); }
                else { self.add_token(TokenType::Band, String::from("&")); }
            }
            else if self.match_and_consume('|') {
                if self.match_and_consume('|') { self.add_token(TokenType::Lor, String::from("||")); }
                else { self.add_token(TokenType::Bor, String::from("|")); }
            }
            else if self.match_and_consume('=') {
                if self.match_and_consume('=') { self.add_token(TokenType::EqualEqual, String::from("==")); }
                else { self.add_token(TokenType::Equal, String::from("=")); }
            }
            else if self.match_and_consume('!') {
                if self.match_and_consume('=') { self.add_token(TokenType::BangEqual, String::from("!=")); }
                else { self.add_token(TokenType::Bang, String::from("!")); }
            }
            else if self.match_and_consume('~') { self.add_token(TokenType::Lnot, String::from("~")); }
            else if self.match_and_consume('^') { self.add_token(TokenType::Lxor, String::from("^")); }
            else {
                let error = self.consume_char();
                self.add_token(TokenType::Error, String::from(error));
            }
        }
    }
    fn scan_string(&mut self) -> bool {
        if !self.tokenization_end() {
            if self.match_and_consume('"') {
                let start = self.position;
                while !self.match_and_consume('"') {
                    if self.tokenization_end() {
                        // TODO: add error (Unterminated String)
                        return true;
                    }
                    else if self.match_and_consume('\\') {
                        if self.match_and_consume('"') {}
                    }
                    else if self.match_and_consume('"') {
                        break;
                    }    
                    else if !self.tokenization_end() {
                        self.consume_char();
                    }            
                }
                let substr = self.src.as_str()[start..self.position-1].to_string();
                let tok = Token::init(TokenType::Str, substr, start, self.position, self.line, self.line_offset);
                self.token_list.push(tok);
                return true;
            }
        }
        false
    }
    fn scan_number(&mut self) -> bool {
        if !self.tokenization_end() { // regex: [0-9]+\.[0-9]
            let start = self.position;
            let mut float_flag: bool = false;
            if self.peek().is_numeric() {
                while self.peek().is_numeric() {
                    self.consume_char();
                    if self.tokenization_end() {
                        break;
                    }
                    if self.peek() == '.' && self.peek_next().is_numeric() {
                        self.consume_char();
                        float_flag = true;
                    }
                }
                let substr: String = self.src.as_mut_str()[start..self.position].to_string();
                if float_flag {
                    let tok = Token::init(TokenType::Float, substr, start, self.position, self.line, self.line_offset);
                    self.token_list.push(tok);
                    return true;
                }
                else {
                    let tok = Token::init(TokenType::Int, substr, start, self.position, self.line, self.line_offset);
                    self.token_list.push(tok);
                    return true;
                }
            }
            else {
                return false;
            }
        }
        false
    }

    fn scan_identifier(&mut self) -> bool {
        if self.peek().is_alphabetic() { // regex: [a-zA-Z_][a-zA-Z0-9]*
            let start = self.position;
            self.consume_char();
            while self.peek().is_alphanumeric() {
                self.consume_char();
                if self.tokenization_end() {
                    break;
                }
            }
            let mut substr: String = self.src.as_mut_str()[start..self.position].to_string(); // this breaks down if the compiler encounters a non-ascii char :/
            let substr_clone = substr.clone();
            if self.is_keyword(&substr) {
                let typ = *self.keywords.get(&substr.clone()).unwrap_or(&TokenType::Error);
                let tok = Token::init(typ, substr, start, self.position, self.line, self.line_offset);
                self.token_list.push(tok);
            }
            else {
                let tok = Token::init(TokenType::Identifier, substr_clone, start, self.position, self.line, self.line_offset);
                self.token_list.push(tok);
            }
            return true;
        }
        false
    }
    fn consume_whitespace(&mut self) {
        while !self.tokenization_end() {
            if self.peek() == '\r' || self.peek() == '\t' || self.peek() == ' ' {
                self.consume_char();
                self.line_offset += 1;
                continue;
            }
            if self.peek() == '\n' {
                self.consume_char();
                self.line += 1;
                self.line_offset = 0;
                continue;
            }
            break;
        }
    }

    pub fn tokenize(&mut self) {
        self.consume_whitespace();
        while !self.tokenization_end() {
            self.consume_whitespace();
            self.scan_token();
        } 
        self.add_token(TokenType::Eof, String::new());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_test(test_src: String) -> Tokenizer {
        let mut tokenizer = Tokenizer::init(test_src);
        tokenizer.tokenize();
        tokenizer
    }

    #[test]
    fn test_keywords() {
        let tokenizer = init_test(String::from(""));
        assert_eq!(false, tokenizer.is_keyword(&"not_a_keyword".to_string()));
        assert_eq!(true, tokenizer.is_keyword(&"for".to_string()));
    }
    #[test]
    fn test_consume_char() {
        let mut tokenizer = Tokenizer::init("abc".to_string());
        assert_eq!('a', tokenizer.consume_char());
        assert_eq!('b', tokenizer.consume_char());
        assert_eq!('c', tokenizer.consume_char());
        assert_eq!('\0', tokenizer.consume_char());
    }

    #[test]
    fn test_tokenize_int() {
        let tokenizer = init_test(String::from("1"));
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Int);
        assert_eq!(token.get_string_value(), String::from("1"));
    }

    #[test]
    fn test_tokenize_float() {
        let mut tokenizer = init_test(String::from("1.1"));
        tokenizer.tokenize();
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Float);
        assert_eq!(token.get_string_value(), String::from("1.1"));
    }

    #[test]
    fn test_tokenize_identifier() {
        let mut tokenizer = init_test(String::from("a123b567"));
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Identifier);
        assert_eq!(token.get_string_value(), String::from("a123b567"));
    }

    #[test]
    fn test_tokenize_keyword() {
        let mut tokenizer = init_test(String::from("for"));
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::For);
        assert_eq!(token.get_string_value(), String::from("for"));
    }

    #[test]
    fn test_tokenize_string() {
        let mut tokenizer = Tokenizer::init("\"abc\" \"\"".to_string());
        tokenizer.tokenize();
        let abc = tokenizer.get_token(0);
        let empty = tokenizer.get_token(1);
        assert_eq!(abc.get_type(), TokenType::Str);
        assert_eq!(abc.get_string_value(), String::from("abc"));
        assert_eq!(empty.get_type(), TokenType::Str);
        assert_eq!(empty.get_string_value(), String::from(""));
    }

    #[test]
    fn test_escaped_strings() {
        let tokenizer = init_test(String::from("\"abc\\\"\"")); // 'abc\"'
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Str);
        assert_eq!(token.get_string_value(), String::from("abc\\\""));
    }

    #[test]
    fn test_line_comments() {
        let tokenizer = init_test(String::from("1 \"abc\"// this is a comment/ ** :) with all kinds of random junk"));
        let int_tok = tokenizer.get_token(0);
        let str_tok = tokenizer.get_token(1);
        let eof_tok = tokenizer.get_token(2);
        assert_eq!(int_tok.get_type(), TokenType::Int);
        assert_eq!(str_tok.get_type(), TokenType::Str);
        assert_eq!(eof_tok.get_type(), TokenType::Eof);
    }

    #[test]
    fn test_syntax_tokenization() {
        let mut tokenizer = Tokenizer::init(String::from("{ } [ ] ( ) , . ; : + ++ += - -- -= = == != ! > >= < <= & && | || ~ ^"));
        tokenizer.tokenize();
        use TokenType::*;
        let tok_l = tokenizer.token_list.clone();
        let type_vec: Vec<TokenType> = vec![LeftBrace, RightBrace, LeftBracket, RightBracket, LeftParen, RightParen, Comma, Dot, SemiColon, Colon, Plus, PlusPlus, 
        PlusEqual, Minus, MinusMinus, MinusEqual, Equal, EqualEqual, BangEqual, Bang, Greater, GreaterEqual, Less, LessEqual, Band, Land,Bor, Lor, Lnot, Lxor];
        for i in 0..tok_l.len()-1 {
            assert_eq!(tok_l[i].get_type(), type_vec[i]);
        }
    }

    #[test]
    fn test_list_literal_tokenization() {
        let mut tokenizer = Tokenizer::init("[1, 2, 3]".to_string());
        tokenizer.tokenize();
        use TokenType::*;
        let tok_l = tokenizer.token_list.clone();
        let type_vec: Vec<TokenType> = vec![LeftBracket, Int, Comma, Int, Comma, Int, RightBracket];
        for i in 0..tok_l.len()-1 {
            assert_eq!(tok_l[i].get_type(), type_vec[i]);
        }
    }
}