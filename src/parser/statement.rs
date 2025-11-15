use crate::parser::expression::{EvaluationResult, Expression};
use crate::tokenizer::tokens::*;
use std::fmt::Debug;

pub trait Statement: Debug {}

#[allow(unused)]
pub struct LetStatement {
    pub identifier: IdentifierData,
    // optional because we can also have type inference
    pub evaluation_type: EvaluationResult,
    pub expression: dyn Expression,
}

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
