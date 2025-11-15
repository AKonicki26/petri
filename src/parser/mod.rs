use crate::parser;
use crate::parser::statement::Statement;
use crate::tokenizer::Token;

mod expression;
mod statement;

pub struct Parser {
    tokens: Vec<Token>
}

type StatementList = Vec<Box<dyn Statement>>;
impl Parser {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
    pub fn parse(&self) -> Result<StatementList, String> {
        Ok(vec![])
    }
}