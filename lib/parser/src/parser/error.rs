use crate::lexer::token::Token;
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("unexpected token: got {found:?} but want one of {expected:?}")]
    UnexpectedToken {
        found: Option<Token>,
        expected: Vec<&'static str>,
    },
}
