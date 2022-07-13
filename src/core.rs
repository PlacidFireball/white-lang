use crate::parser::Parser;
use crate::program::Program;
use crate::tokenizer::Tokenizer;

pub struct CoreObjects {
    pub parser: Parser,
    pub tokenizer: Tokenizer,
    pub program: Program,
}
impl CoreObjects {
    #[allow(dead_code)]
    pub(crate) fn new(src: &str) -> Self {
        let mut tokenizer = Tokenizer::new(src.to_string());
        tokenizer.tokenize();
        let mut parser = Parser::new(&mut tokenizer);
        parser.parse();
        let program = Program::from_parser(&mut parser);
        Self {
            parser,
            tokenizer,
            program,
        }
    }

    pub(crate) fn new_uninit() -> Self {
        let tokenizer = Tokenizer::new_uninit();
        let parser = Parser::new_uninit();
        let program = Program::new_uninit();
        Self {
            parser,
            tokenizer,
            program,
        }
    }

    pub(crate) fn set_src(&mut self, src: &str) {
        // init tokenizer
        self.tokenizer.set_source(src.to_string());
        self.tokenizer.tokenize();
        // init parser
        self.parser.set_token_list(self.tokenizer.get_token_list());
        self.parser.parse();
        // init program
        self.program.set_statements_or_expr(
            self.parser.get_statements().get_or_insert(&Vec::new()),
            self.parser.get_expr(),
        );
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
