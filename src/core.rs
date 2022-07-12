
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::program::Program;

pub struct CoreObjects {
    pub parser: Parser,
    pub tokenizer: Tokenizer,
    pub program: Program
}
impl CoreObjects {
    pub(crate) fn new(src: &str) -> Self {
        let mut tokenizer = Tokenizer::init(src.to_string());
        tokenizer.tokenize();
        let mut parser = Parser::new(&mut tokenizer);
        parser.parse();
        let mut program = Program::from_parser(&mut parser);
        Self {
            parser,
            tokenizer,
            program
        }
    }

    pub fn get_tokenizer(&mut self) -> &mut Tokenizer {
        &mut self.tokenizer
    }

    pub fn get_parser(&mut self) -> &mut Parser {
        &mut self.parser
    }

    pub fn get_program(&mut self) -> &mut Program {
        &mut self.program
    }
}