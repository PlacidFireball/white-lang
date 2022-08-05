use std::char;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    // all token types that White-Lang implements
    // Types
    Str,   // "string"
    Int,   // 123
    Float, // 123.456
    // Syntax
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Equal,        // =
    Bang,         // !
    Plus,         // +
    PlusPlus,     // ++
    PlusEqual,    // +=
    Minus,        // -
    MinusMinus,   // --
    MinusEqual,   // -=
    Star,         // *
    Slash,        // /
    Band,         // & (bitwise and)
    Land,         // &&
    Bor,          // | (bitwise or)
    Lor,          // ||
    Lnot,         // ~
    Lxor,         // ^
    SemiColon,    // ;
    Colon,        // :
    EqualEqual,   // ==
    Comma,        // ,
    Dot,          // .
    Greater,      // >
    Less,         // <
    BangEqual,    // !=
    GreaterEqual, // >=
    LessEqual,    // <=
    // Keywords
    Null,       // null
    And,        // and
    Not,        // not
    If,         // if
    While,      // while
    For,        // for
    In,         // in
    Else,       // else
    Return,     // return
    Eof,        // not technically a keyword
    Error,      // error reporting
    Identifier, // a123b456
    Let,        // let
    Print,      // print
    Function,   // fn
    True,       // true
    False,      // false
    Break,      // break
    Struct,     // struct
    Implement,  // implement
    _Self,      // self
    /* Future Tokens */
    As,    // as
    Arrow, // ->
    GoTo,
    Extends,
    Implements,
}
#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
enum ErrorType {
    // Errors, will probably add more
    UnterminatedString,
    UnexpectedToken,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = match self {
            ErrorType::UnterminatedString => String::from("Unterminated String"),
            ErrorType::UnexpectedToken => String::from("Unexpected Token"),
        };
        write!(f, "{}", s)
    }
}
impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, ", self)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    // the Token
    typ: TokenType,       // has a type
    string_value: String, // stores its string value
    start: usize,         // and maintains location data for future error reporting TODO
    end: usize,
    line: usize,
    line_offset: usize,
}
impl Display for Token {
    // for debug info
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line: {}, offset: {}, type: {}",
            self.line, self.line_offset, self.typ
        )
    }
}
#[allow(dead_code)]
impl Token {
    // the constructor
    fn init(
        typ: TokenType,
        string_value: String,
        start: usize,
        end: usize,
        line: usize,
        line_offset: usize,
    ) -> Token {
        Token {
            typ,
            string_value,
            start,
            end,
            line,
            line_offset,
        }
    }
    pub fn get_type(&self) -> TokenType {
        self.typ
    }
    pub fn get_string_value(&self) -> String {
        self.string_value.clone()
    }
}

fn init_keywords() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    // intrinsics
    keywords.insert("print".to_string(), TokenType::Print);
    // function related
    keywords.insert("fn".to_string(), TokenType::Function);
    keywords.insert("return".to_string(), TokenType::Return);
    // boolean operators
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("not".to_string(), TokenType::Not);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("true".to_string(), TokenType::True);
    // variables
    keywords.insert("null".to_string(), TokenType::Null);
    keywords.insert("let".to_string(), TokenType::Let);
    // loops and branching
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("while".to_string(), TokenType::While);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("break".to_string(), TokenType::Break);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("in".to_string(), TokenType::In);
    keywords.insert("goto".to_string(), TokenType::GoTo);
    // objects
    keywords.insert("struct".to_string(), TokenType::Struct);
    keywords.insert("implement".to_string(), TokenType::Implement);
    keywords.insert("extends".to_string(), TokenType::Extends);
    keywords.insert("implements".to_string(), TokenType::Implements);
    keywords.insert("self".to_string(), TokenType::_Self);
    keywords
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    // the tokenizer
    token_list: Vec<Token>,               // maintains a list of tokens
    errors: Vec<ErrorType>,               // associated errors
    keywords: HashMap<String, TokenType>, // all the keywords in White-Lang
    src: String,                          // the source code
    char_vec: Vec<char>,                  // the source code, but characters
    curr_char: char,                      // the current char we are reading
    position: usize,                      // position data in the string
    line: usize,                          // what line we are on
    line_offset: usize,                   // what the line offset is on the line
}
impl std::fmt::Display for Tokenizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for printing off the tokenizer data
        write!(f, "Token List: {:?}\nsrc: {}\n", self.token_list, self.src)
    }
}
impl Tokenizer {
    pub fn new(src: String) -> Tokenizer {
        // the constructor
        let char_vec: Vec<char> = src.chars().collect();
        Tokenizer {
            token_list: vec![],
            errors: vec![],
            keywords: init_keywords(),
            src,
            char_vec: char_vec,
            curr_char: '\0',
            position: 0,
            line: 1,
            line_offset: 0,
        }
    }

    pub fn new_uninit() -> Tokenizer {
        Tokenizer {
            token_list: vec![],
            errors: vec![],
            keywords: init_keywords(),
            src: "".to_string(),
            char_vec: vec![],
            curr_char: '\0',
            position: 0,
            line: 1,
            line_offset: 0,
        }
    }

    pub fn set_source(&mut self, src: String) {
        if self.src.eq("") {
            self.src = src.clone();
            self.char_vec = src.chars().collect()
        } else {
            panic!("Do not set the source if you have already initialized it!")
        }
    }

    // returns a reference to the token list
    pub fn get_token_list(&self) -> &Vec<Token> {
        &self.token_list
    }
    // gets a specific token at a specific index
    #[allow(dead_code)]
    pub fn get_token(&self, idx: usize) -> Token {
        self.token_list[idx].clone()
    }
    // a quick little function for putting tokens into the list instead of typing out the whole function every time
    fn add_token(&mut self, typ: TokenType, strval: String) {
        let len = strval.len();
        self.token_list.push(Token::init(
            typ,
            strval,
            self.position,
            self.position + len,
            self.line,
            self.line_offset,
        ));
    }
    // tells you if an identifier is a keyword or not
    fn is_keyword(&self, kw: &String) -> bool {
        self.keywords.contains_key(kw)
    }
    // tells you if you are done tokenizing src
    fn tokenization_end(&self) -> bool {
        self.position >= self.src.len()
    }
    // if `a` matches whatever is at src[position], consume it, otherwise move on
    fn match_and_consume(&mut self, a: char) -> bool {
        if self.peek() == a {
            self.curr_char = self.consume_char();
            return true;
        }
        false
    }
    // consumes the character at src[position]
    fn consume_char(&mut self) -> char {
        if self.tokenization_end() {
            return '\0';
        }
        let chr = self.char_vec[self.position];
        self.curr_char = chr;
        self.position += 1;
        chr
    }
    // returns the character at src[position] without consuming it
    fn peek(&self) -> char {
        if self.tokenization_end() {
            return '\0';
        }
        self.char_vec[self.position]
    }
    // for cases when you want to look at the character at src[position + 1]
    fn peek_next(&self) -> char {
        if self.tokenization_end() {
            return '\0';
        }
        self.char_vec[self.position + 1]
    }
    // the crux of token scanning
    fn scan_token(&mut self) {
        if self.scan_number() {
            // scan number, then string if that fails, then identifier, then syntax if all those fail
            return;
        } else if self.scan_string() {
            return;
        } else if self.scan_identifier() {
            return;
        }
        self.scan_syntax();
    }
    // scans all the syntax supported by White-Lang, code pretty self explanatory
    fn scan_syntax(&mut self) {
        if !self.tokenization_end() {
            if self.match_and_consume('{') {
                self.add_token(TokenType::LeftBrace, String::from("{"));
            } else if self.match_and_consume('}') {
                self.add_token(TokenType::RightBrace, String::from("}"));
            } else if self.match_and_consume('[') {
                self.add_token(TokenType::LeftBracket, String::from("["));
            } else if self.match_and_consume(']') {
                self.add_token(TokenType::RightBracket, String::from("]"));
            } else if self.match_and_consume('(') {
                self.add_token(TokenType::LeftParen, String::from("("));
            } else if self.match_and_consume(')') {
                self.add_token(TokenType::RightParen, String::from(")"))
            } else if self.match_and_consume(';') {
                self.add_token(TokenType::SemiColon, String::from(";"));
            } else if self.match_and_consume(':') {
                self.add_token(TokenType::Colon, String::from(":"));
            } else if self.match_and_consume(',') {
                self.add_token(TokenType::Comma, String::from(","));
            } else if self.match_and_consume('.') {
                self.add_token(TokenType::Dot, String::from("."));
            } else if self.match_and_consume('>') {
                if self.match_and_consume('=') {
                    self.add_token(TokenType::GreaterEqual, String::from(">="))
                } else {
                    self.add_token(TokenType::Greater, String::from(">"));
                }
            } else if self.match_and_consume('<') {
                if self.match_and_consume('=') {
                    self.add_token(TokenType::LessEqual, String::from("<="))
                } else {
                    self.add_token(TokenType::Less, String::from("<"));
                }
            } else if self.match_and_consume('/') {
                if self.match_and_consume('/') {
                    // line comment begin
                    while !self.match_and_consume('\n') && !self.tokenization_end() {
                        self.consume_char();
                        if self.tokenization_end() {
                            break;
                        }
                    }
                } else if self.match_and_consume('*') {
                    // multiline comment begin
                    while !(self.peek() == '*' && self.peek_next() == '/')
                        && !self.tokenization_end()
                    {
                        self.consume_char();
                        if self.tokenization_end() {
                            break;
                        }
                    }
                    self.consume_char(); // consume * and / at the end of the comment block
                    self.consume_char();
                } else {
                    self.add_token(TokenType::Slash, String::from("/"));
                }
            } else if self.match_and_consume('+') {
                if self.match_and_consume('+') {
                    self.add_token(TokenType::PlusPlus, String::from("++"));
                } else if self.match_and_consume('=') {
                    self.add_token(TokenType::PlusEqual, String::from("+="));
                } else {
                    self.add_token(TokenType::Plus, String::from("+"));
                }
            } else if self.match_and_consume('-') {
                if self.match_and_consume('-') {
                    self.add_token(TokenType::MinusMinus, String::from("--"));
                } else if self.match_and_consume('=') {
                    self.add_token(TokenType::MinusEqual, String::from("-="));
                } else {
                    self.add_token(TokenType::Minus, String::from("-"));
                }
            } else if self.match_and_consume('*') {
                self.add_token(TokenType::Star, String::from("*"));
            } else if self.match_and_consume('&') {
                if self.match_and_consume('&') {
                    self.add_token(TokenType::Land, String::from("&&"));
                } else {
                    self.add_token(TokenType::Band, String::from("&"));
                }
            } else if self.match_and_consume('|') {
                if self.match_and_consume('|') {
                    self.add_token(TokenType::Lor, String::from("||"));
                } else {
                    self.add_token(TokenType::Bor, String::from("|"));
                }
            } else if self.match_and_consume('=') {
                if self.match_and_consume('=') {
                    self.add_token(TokenType::EqualEqual, String::from("=="));
                } else {
                    self.add_token(TokenType::Equal, String::from("="));
                }
            } else if self.match_and_consume('!') {
                if self.match_and_consume('=') {
                    self.add_token(TokenType::BangEqual, String::from("!="));
                } else {
                    self.add_token(TokenType::Bang, String::from("!"));
                }
            } else if self.match_and_consume('~') {
                self.add_token(TokenType::Lnot, String::from("~"));
            } else if self.match_and_consume('^') {
                self.add_token(TokenType::Lxor, String::from("^"));
            } else {
                let error = self.consume_char();
                self.add_token(TokenType::Error, String::from(error));
            }
        }
    }
    // String scanning [regex: "[literally anything in unicode]"]
    fn scan_string(&mut self) -> bool {
        if !self.tokenization_end() {
            if self.match_and_consume('"') {
                let start = self.position; // set the start
                while !self.match_and_consume('"') {
                    // while we haven't terminated the string
                    if self.tokenization_end() {
                        // if we got to the end of tokenization, we have an unterminated string
                        self.errors.push(ErrorType::UnterminatedString); // push the error and exit
                        return true;
                    } else if self.match_and_consume('\\') {
                        // deal with escaped quotes or just backslashes
                        if self.match_and_consume('"') {}
                    } else if self.match_and_consume('"') {
                        // checks for the end of the string
                        break;
                    } else if !self.tokenization_end() {
                        // otherwise consume whatever is there
                        self.consume_char();
                    }
                }
                let substr = self.src.as_str()[start..self.position - 1].to_string(); // retrieve the substring, kinda wonky, will break with unicode chars
                let tok = Token::init(
                    TokenType::Str,
                    substr,
                    start,
                    self.position,
                    self.line,
                    self.line_offset,
                ); // create the token
                self.token_list.push(tok); // push it
                return true;
            }
        }
        false
    }
    fn scan_number(&mut self) -> bool {
        if !self.tokenization_end() {
            // regex: [0-9]+\.[0-9]
            let start = self.position;
            let mut float_flag: bool = false; // handle decimal numbers
            if self.peek().is_numeric() {
                // rust has nice built int char functions that tell you if stuff is alphabetic or numeric, in this case
                while self.peek().is_numeric() {
                    // while we are dealing with numbers
                    self.consume_char(); // consume the character
                    if self.tokenization_end() {
                        // check for the end
                        break;
                    }
                    if self.peek() == '.' {
                        if self.peek_next().is_numeric() {
                            float_flag = true;
                            self.consume_char();
                        } else {
                            // TODO: make test_tokenize_bad_float pass
                            self.src = self.src.replacen(".", "", 1);
                        }
                    }
                }
                let substr: String = self.src.as_mut_str()[start..self.position].to_string(); // retrieve the substring
                if float_flag {
                    // create either int or float token based on whether or not we encountered "."
                    let tok = Token::init(
                        TokenType::Float,
                        substr,
                        start,
                        self.position,
                        self.line,
                        self.line_offset,
                    );
                    self.token_list.push(tok);
                    return true;
                } else {
                    let tok = Token::init(
                        TokenType::Int,
                        substr,
                        start,
                        self.position,
                        self.line,
                        self.line_offset,
                    );
                    self.token_list.push(tok);
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }

    fn scan_identifier(&mut self) -> bool {
        if self.peek().is_alphabetic() {
            // regex: [a-zA-Z_][a-zA-Z_0-9]*
            let start = self.position; // set start
            self.consume_char(); // consume the first char
            while self.peek().is_alphanumeric() || self.peek().eq(&'_') {
                // while we have anything [a-zA-Z_0-9]
                self.consume_char();
                if self.tokenization_end() {
                    break;
                }
            }
            let substr: String = self.src.as_mut_str()[start..self.position].to_string(); // this breaks down if the compiler encounters a non-ascii char :/
            let substr_clone = substr.clone();
            if self.is_keyword(&substr) {
                // this is weird rust spaghetti, because the memory thing is pissy otherwise, feels dumb
                let typ = *self
                    .keywords
                    .get(&substr.clone())
                    .unwrap_or(&TokenType::Error); // retrieve the type from the keywords hashmap
                let tok = Token::init(
                    typ,
                    substr,
                    start,
                    self.position,
                    self.line,
                    self.line_offset,
                ); // init the token
                self.token_list.push(tok);
            } else {
                // if we don't have a keyword, we just have an identifier, which is pretty cash money
                let tok = Token::init(
                    TokenType::Identifier,
                    substr_clone,
                    start,
                    self.position,
                    self.line,
                    self.line_offset,
                );
                self.token_list.push(tok);
            }
            return true;
        }
        false
    }
    fn consume_whitespace(&mut self) {
        while !self.tokenization_end() {
            // consume whitespace
            if self.peek() == '\r' || self.peek() == '\t' || self.peek() == ' ' {
                self.consume_char();
                self.line_offset += 1; // updates line_offset and line as needed
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

    // our main loop, drives all the above code
    pub fn tokenize(&mut self) {
        self.consume_whitespace(); // get rid of whitespace at the beginning of src
        while !self.tokenization_end() {
            // while we've still got chars to read
            self.consume_whitespace(); // consume that whitespace
            self.scan_token(); // scan them tokens
        }
        self.add_token(TokenType::Eof, String::new()); // add eof at the end of token_list
    }
}

// Tests!
#[cfg(test)]
mod test {
    use super::*;
    use crate::TokenType::{Eof, Identifier, LeftBrace, LeftParen, RightBrace, RightParen, Struct};

    fn init_test(test_src: String) -> Tokenizer {
        let mut tokenizer = Tokenizer::new(test_src);
        tokenizer.tokenize();
        tokenizer
    }

    #[test]
    fn test_keywords() {
        let tokenizer = init_test(String::from(""));
        assert!(!tokenizer.is_keyword(&"not_a_keyword".to_string()));
        assert!(tokenizer.is_keyword(&"for".to_string()));
    }
    #[test]
    fn test_consume_char() {
        let mut tokenizer = Tokenizer::new("abc".to_string());
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

    /*
    #[test]
    fn test_tokenize_bad_float() {
        let mut tokenizer = init_test("1.".to_string());
        tokenizer.tokenize();
        let token = tokenizer.get_token(0);
        assert_eq!(tokenizer.get_token_list().len(), 2);
        assert_eq!(token.get_type(), TokenType::Int);
        assert_eq!(token.get_string_value(), String::from("1"));
    }
    */

    #[test]
    fn test_tokenize_identifier() {
        let tokenizer = init_test(String::from("a123b567"));
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Identifier);
        assert_eq!(token.get_string_value(), String::from("a123b567"));
    }

    #[test]
    fn test_tokenize_keyword() {
        let tokenizer = init_test(String::from("for"));
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::For);
        assert_eq!(token.get_string_value(), String::from("for"));
    }

    #[test]
    fn test_tokenize_string() {
        let mut tokenizer = Tokenizer::new("\"abc\" \"\"".to_string());
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
        let tokenizer = init_test(String::from(
            "1 \"abc\"// this is a comment/ ** :) with all kinds of random junk",
        ));
        let int_tok = tokenizer.get_token(0);
        let str_tok = tokenizer.get_token(1);
        let eof_tok = tokenizer.get_token(2);
        assert_eq!(int_tok.get_type(), TokenType::Int);
        assert_eq!(str_tok.get_type(), TokenType::Str);
        assert_eq!(eof_tok.get_type(), TokenType::Eof);
    }

    #[test]
    fn test_syntax_tokenization() {
        let mut tokenizer = Tokenizer::new(String::from(
            "{ } [ ] ( ) , . ; : + ++ += - -- -= = == != ! > >= < <= & && | || ~ ^",
        ));
        tokenizer.tokenize();
        use TokenType::*;
        let tok_l = tokenizer.token_list.clone();
        let type_vec: Vec<TokenType> = vec![
            LeftBrace,
            RightBrace,
            LeftBracket,
            RightBracket,
            LeftParen,
            RightParen,
            Comma,
            Dot,
            SemiColon,
            Colon,
            Plus,
            PlusPlus,
            PlusEqual,
            Minus,
            MinusMinus,
            MinusEqual,
            Equal,
            EqualEqual,
            BangEqual,
            Bang,
            Greater,
            GreaterEqual,
            Less,
            LessEqual,
            Band,
            Land,
            Bor,
            Lor,
            Lnot,
            Lxor,
        ];
        for i in 0..tok_l.len() - 1 {
            assert_eq!(tok_l[i].get_type(), type_vec[i]);
        }
    }

    #[test]
    fn test_list_literal_tokenization() {
        let mut tokenizer = Tokenizer::new("[1, 2, 3]".to_string());
        tokenizer.tokenize();
        use TokenType::*;
        let tok_l = tokenizer.token_list.clone();
        let type_vec: Vec<TokenType> = vec![LeftBracket, Int, Comma, Int, Comma, Int, RightBracket];
        for i in 0..tok_l.len() - 1 {
            assert_eq!(tok_l[i].get_type(), type_vec[i]);
        }
    }

    #[test]
    fn test_multiline_comment() {
        let mut tokenizer = Tokenizer::new(
            "/* fasdfkjas;ldfkjas;ldkgfjas;lhkgjnas;lfgkjasjmfl;askjesmf;laskjmfe;asjf







            */"
            .to_string(),
        );
        tokenizer.tokenize();
        assert_eq!(tokenizer.token_list.len(), 1);
    }

    #[test]
    fn tokenize_struct() {
        let mut tokenizer = Tokenizer::new(String::from("struct X () {}"));
        tokenizer.tokenize();
        assert_eq!(tokenizer.token_list.len(), 7);
        let token_types = vec![
            Struct, Identifier, LeftParen, RightParen, LeftBrace, RightBrace, Eof,
        ];
        for i in 0..tokenizer.token_list.len() - 1 {
            assert_eq!(tokenizer.token_list[i].get_type(), token_types[i]);
        }
    }
}
