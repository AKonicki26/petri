use crate::parser::statement::Statement;
use crate::tokenizer::tokens::Token;

mod expression;
mod statement;

pub struct Parser {
    #[allow(unused)]
    tokens: Vec<Token>,
    current_token_index: usize,
}

type StatementList = Vec<Box<dyn Statement>>;
impl Parser {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            current_token_index: 0,
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token_index: 0,
        }
    }
    pub fn parse(&self) -> Result<StatementList, String> {
        Ok(vec![])
    }
}
