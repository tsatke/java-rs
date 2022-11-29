use crate::lexer::span::{Span, Spanned};
use crate::Identifier;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QualifiedName {
    segments: Vec<Identifier>,
}

impl Spanned for QualifiedName {
    fn span(&self) -> Option<Span> {
        match (self.segments.first(), self.segments.last()) {
            (Some(first), Some(last)) => Some(Span::new(first.span().start(), last.span().end())),
            _ => None,
        }
    }
}

impl<T, I> From<T> for QualifiedName
where
    T: IntoIterator<Item = I>,
    I: Into<Identifier>,
{
    fn from(segments: T) -> Self {
        Self {
            segments: segments.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl Default for QualifiedName {
    fn default() -> Self {
        Self::new()
    }
}

impl QualifiedName {
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    pub fn segments(&self) -> &[Identifier] {
        &self.segments
    }

    pub(in crate::parser) fn push(&mut self, segment: Identifier) {
        self.segments.push(segment);
    }
}
