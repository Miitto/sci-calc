use crate::{operation::Operation, operator::Operator};

pub enum Token {
    Used,
    OpenParenthhesis,
    CloseParenthesis,
    Operator(Operator),
    Operation(Box<dyn Operation>),
}

impl Token {
    pub fn is_used(&self) -> bool {
        match self {
            Token::Used => true,
            _ => false,
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Operator(_) => true,
            _ => false,
        }
    }

    pub fn is_operation(&self) -> bool {
        match self {
            Token::Operation(_) => true,
            _ => false,
        }
    }

    pub fn is_parenthesis(&self) -> bool {
        match self {
            Token::OpenParenthhesis | Token::CloseParenthesis => true,
            _ => false,
        }
    }

    pub fn is_open_parenthesis(&self) -> bool {
        match self {
            Token::OpenParenthhesis => true,
            _ => false,
        }
    }

    pub fn is_close_parentesis(&self) -> bool {
        match self {
            Token::CloseParenthesis => true,
            _ => false,
        }
    }
}
