use std::mem::Discriminant;
use crate::parser::expression::*;
use crate::parser::statement::*;
use crate::tokenizer::tokens::*;

mod expression;
mod statement;

pub struct Parser {
    #[allow(unused)]
    tokens: Vec<Token>,
    current_token_index: usize,
}

type StatementList = Vec<Box<dyn Statement>>;

#[allow(unused)]
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
    pub fn parse(&mut self) -> Result<StatementList, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let statement = self.parse_statement();
            statements.push(statement?);
        }

        Ok(statements)
    }


    fn peek(&self) -> &Token {
        &self.tokens[self.current_token_index]
    }

    fn peek_next(&self) -> Option<&Token> {
        if self.current_token_index + 1 >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.current_token_index + 1])
        }
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().clone().discriminant() == token_type.clone().discriminant()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_token_index += 1;
        }
        &self.tokens[self.current_token_index - 1]
    }

    fn expect_token_type(&mut self, expected: Discriminant<Token>) -> Result<(), String> {
        let token = self.peek();

        if token.clone().discriminant() == expected {
            self.advance();
            Ok(())
        } else {
            // TODO: Change this formatting to be more readable
            Err(format!("Expected {:?}, got {:?}", expected, token))
        }
    }

    fn parse_type(&mut self) -> Result<EvaluationResult, String> {
        match self.advance() {
            Token::IntKeyword { .. } => Ok(EvaluationResult::Number),
            Token::StringKeyword { .. } => Ok(EvaluationResult::String),
            Token::BooleanKeyword { .. } => Ok(EvaluationResult::Boolean),
            Token::DecimalKeyword { .. } => Ok(EvaluationResult::Decimal),
            other => Err(format!("Expected type keyword, got {:?}", other)),
        }
    }

    // STATEMENT PARSERS

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        match self.peek() {
            Token::Let { .. } => self.parse_let_statement(),
            Token::Const { .. } => todo!("Parse const statement"),
            Token::Identifier(_) => {
                // Could be an assignment or function call
                // You'll need to look ahead to decide
                todo!("Parse assignment or function call")
            }
            Token::If { .. } => todo!("Parse if statement"),
            Token::While { .. } => todo!("Parse while statement"),
            Token::Return { .. } => todo!("Parse return statement"),
            Token::Eof => Err("Unexpected end of file".to_string()),
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        self.expect_token_type(Token::Let{index: 0}.discriminant())?;

        let identifier = match self.advance() {
            Token::Identifier(identifier) => identifier.clone(),
            other => return Err(format!("Expected identifier, got {:?}", other)),
        };

        let eval_type = if self.check(&Token::Colon{index: 0})
        {
            self.advance();
            self.parse_type()?
        } else {
            return Err("Expected type after colon".to_string());
        };

        self.expect_token_type(Token::Eq{index: 0}.discriminant())?;

        let expression = self.parse_expression()?;

        self.expect_token_type(Token::Semicolon{index: 0}.discriminant())?;

        Ok(Box::new(LetStatement {
            identifier,
            evaluation_type: eval_type,
            expression,
        }))
    }
    // EXPRESSION PARSERS
    fn parse_expression(&mut self) -> Result<Box<dyn Expression>, String> {
        // For now, just handle literals
        // You'll want to expand this to handle:
        // - Binary operations (5 + 3)
        // - Function calls
        // - Identifiers
        // - etc.

        match self.peek() {
            Token::NumberLiteral(data) => {
                let data = data.clone();
                self.advance();
                Ok(Box::new(NumberLiteralExpr { value: data }))
            }
            Token::StringLiteral(data) => {
                let data = data.clone();
                self.advance();
                Ok(Box::new(StringLiteralExpr { value: data }))
            }
            Token::True { .. } => {
                self.advance();
                Ok(Box::new(BooleanLiteralExpr { value: true }))
            }
            Token::False { .. } => {
                self.advance();
                Ok(Box::new(BooleanLiteralExpr { value: false }))
            }
            other => Err(format!("Cannot parse expression starting with {:?}", other)),
        }
    }
}