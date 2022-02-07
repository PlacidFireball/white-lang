
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum TokenType {
    Identifier, Var,
	Print, Function,
	EqualEqual, Greater, Less, BangEqual,
	GreaterEqual, LessEqual,
	Str, Int, Long, Char, List, Boolean,
    True, False,
	LeftParen, RightParen, LeftBracket,
	RightBracket, LeftBrace, RightBrace,
	Plus, Minus, Star, Slash, Land, Lor, Lnot, 
	Lxor, 
	And, Not, If, While, For, In, Else, Return
}

#[derive(PartialEq, Debug)]
struct Token {
    typ: TokenType,
    string_value: String,
    position: usize,
    line: usize,
    line_offset: usize
}
impl Token {
    fn init(typ: TokenType, string_value: String, position: usize, line: usize, line_offset: usize) -> Token {
        Token {
            typ,
            string_value,
            position,
            line,
            line_offset
        }
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

struct Tokenizer {
    token_list: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    src: String, 
    char_vec: Vec<char>,
    position: usize, 
    line: usize, 
    line_offset: usize
}
impl Tokenizer {
    fn init(src: String) -> Tokenizer {
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
}