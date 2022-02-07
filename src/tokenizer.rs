use std::char;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)]
enum TokenType {
    Identifier, Var,
	Print, Function,
	EqualEqual, Greater, Less, BangEqual,
	GreaterEqual, LessEqual,
	Str, Int, Long, Char, List, Boolean, Float,
    True, False,
	LeftParen, RightParen, LeftBracket,
	RightBracket, LeftBrace, RightBrace,
	Plus, Minus, Star, Slash, Land, Lor, Lnot, 
	Lxor, 
	And, Not, If, While, For, In, Else, Return
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

fn initKeyWords() -> HashMap<String, TokenType> {
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
        Tokenizer {
            token_list: vec![],
            keywords: initKeyWords(),
            src,
            char_vec: vec![],
            position: 0,
            line: 1, 
            line_offset: 0
        }
    }

    fn get_token(&self, idx: usize) -> Token {
        self.token_list[idx].clone()
    }

    
    fn isKeyword(&self, kw: String) -> bool {
        self.keywords.contains_key(&kw)
    }

    fn tokenization_end(&self) -> bool {
        self.position >= self.src.len()
    }

    fn consume_char(&mut self) -> char {
        if self.char_vec.is_empty() {
            self.char_vec = self.src.chars().collect();
        }        
        if self.tokenization_end() {
            return '\0';
        }
        let chr = self.char_vec[self.position];
        self.position += 1;
        chr
    }
    fn peek(&mut self) -> char {
        if self.char_vec.is_empty() {
            self.char_vec = self.src.chars().collect();
        }
        self.char_vec[self.position]
    }
    fn peek_next(&mut self) -> char {
        if self.char_vec.is_empty() {
            self.char_vec = self.src.chars().collect();
        }
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

    fn scan_syntax(&mut self) -> bool {
        while !self.tokenization_end() {
            break;
        }
        false
    }
    fn scan_string(&mut self) -> bool {
        while !self.tokenization_end() {
            break;
        }
        false
    }
    fn scan_number(&mut self) -> bool {
        while !self.tokenization_end() {
            let start = self.position;
            let mut float_flag: bool = false;
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
        false
    }

    fn scan_identifier(&mut self) -> bool {
        while !self.tokenization_end() {
            if self.peek().is_alphabetic() {
                let start = self.position;
                self.consume_char();
                while self.peek().is_alphanumeric() {
                    self.consume_char();
                }
            }
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
        while !self.tokenization_end() {
            self.consume_whitespace();
            self.scan_token();
        }  
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_keywords() {
        let tokenizer = Tokenizer::init("".to_string());
        assert_eq!(false, tokenizer.isKeyword("not_a_keyword".to_string()));
        assert_eq!(true, tokenizer.isKeyword("for".to_string()));
        println!("Test keywords:\t OK");
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
        let mut tokenizer = Tokenizer::init("1".to_string());
        tokenizer.tokenize();
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Int);
        assert_eq!(token.get_string_value(), String::from("1"));
    }

    #[test]
    fn test_tokenize_float() {
        let mut tokenizer = Tokenizer::init("1.1".to_string());
        tokenizer.tokenize();
        let token = tokenizer.get_token(0);
        assert_eq!(token.get_type(), TokenType::Float);
        assert_eq!(token.get_string_value(), String::from("1.1"));
    }
}