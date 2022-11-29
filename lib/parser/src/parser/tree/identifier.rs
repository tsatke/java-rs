use crate::lexer::span::{Span, Spanned};
use crate::lexer::token::Ident;
use crate::lexer::GraphemeIndex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier {
    span: Span,
}

impl From<Ident> for Identifier {
    fn from(ident: Ident) -> Self {
        ident.span().into()
    }
}

impl<I> From<(I, I)> for Identifier
where
    I: Into<GraphemeIndex>,
{
    fn from((start, end): (I, I)) -> Self {
        Self {
            span: Span::new(start, end),
        }
    }
}

impl From<Span> for Identifier {
    fn from(span: Span) -> Self {
        Self { span }
    }
}

impl Spanned for Identifier {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

impl Identifier {
    pub fn span(&self) -> &Span {
        &self.span
    }
}
