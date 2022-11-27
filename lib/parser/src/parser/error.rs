use crate::lexer::span::Span;
use crate::lexer::token::Token;
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("unexpected token: got {found:?} but want one of {expected:?}")]
    UnexpectedToken {
        found: Option<Token>,
        expected: &'static [&'static str],
    },
    #[error("unexpected end of input, expected one of {expected:?}")]
    UnexpectedEOF { expected: &'static [&'static str] },
    #[error("not implemented yet")]
    NotImplemented(Option<Span>),
}
