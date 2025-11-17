use crate::parser::expression::{EvaluationResult, Expression};
use crate::tokenizer::tokens::*;
use std::fmt::{Debug, Formatter};

pub trait Statement: Debug {}

#[allow(unused)]
pub struct LetStatement {
    pub identifier: IdentifierData,
    pub evaluation_type: EvaluationResult,
    pub expression: Box<dyn Expression>,
}

#[allow(unused)]
impl Debug for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Statement for LetStatement {}

#[allow(unused)]
pub struct AssignmentStatement {
    pub identifier: IdentifierData,
    pub evaluation_type: EvaluationResult,
    pub expression: dyn Expression,
}

#[allow(unused)]
pub struct ConstStatement {
    pub identifier: IdentifierData,
    pub evaluation_type: EvaluationResult,
    pub expression: dyn Expression,
}
