use std::fmt::{Debug, Formatter};
use crate::parser::expression::EvaluationResult::{Boolean, Number, String};
use crate::tokenizer::tokens::{NumberLiteralData, StringLiteralData};

#[allow(unused)]
pub enum EvaluationResult {
    Number,
    String,
    Decimal,
    Boolean,
    // I might introduce support for these types later on
    /*
    Array,
    Object,
    Function,
    Null,
     */
}

#[allow(unused)]
pub trait Expression: Debug {
    fn get_evaluation_type(&self) -> EvaluationResult;
}
#[allow(unused)]
pub struct NumberLiteralExpr {
    pub(crate) value: NumberLiteralData
}
#[allow(unused)]
impl Debug for NumberLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[allow(unused)]
impl Expression for NumberLiteralExpr {
    fn get_evaluation_type(&self) -> EvaluationResult {
        Number
    }
}
#[allow(unused)]
pub struct BooleanLiteralExpr {
    pub(crate) value: bool
}
#[allow(unused)]
impl Debug for BooleanLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[allow(unused)]
impl Expression for BooleanLiteralExpr {
    fn get_evaluation_type(&self) -> EvaluationResult {
        Boolean
    }
}
#[allow(unused)]
pub struct StringLiteralExpr {
    pub(crate) value: StringLiteralData
}
#[allow(unused)]
impl Debug for StringLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[allow(unused)]
impl Expression for StringLiteralExpr {
    fn get_evaluation_type(&self) -> EvaluationResult {
        String
    }
}
