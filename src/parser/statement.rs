use crate::tokenizer::tokens::Token;
use std::fmt::Debug;

pub trait Statement: Debug {}

pub struct LetStatement {
    pub identifier: Token,
}
