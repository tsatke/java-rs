use bitflags::bitflags;

use crate::lexer::span::{Span, Spanned};
use crate::lexer::token::Ident;
use crate::lexer::GraphemeIndex;
pub use block::*;
pub use compilation_unit::*;
pub use expression::*;

mod block;
mod compilation_unit;
mod expression;

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MethodModifiers : u16 {
        const Public =       0b0000_0000_0000_0001;
        const Protected =    0b0000_0000_0000_0010;
        const Private =      0b0000_0000_0000_0100;
        const Static =       0b0000_0000_0000_1000;
        const Final =        0b0000_0000_0001_0000;
        const Transient =    0b0000_0000_0010_0000;
        const Volatile =     0b0000_0000_0100_0000;
        const Strictfp =     0b0000_0000_1000_0000;
        const Abstract =     0b0000_0001_0000_0000;
        const Native =       0b0000_0010_0000_0000;
        const Synchronized = 0b0000_0100_0000_0000;
        const Default =      0b0000_1000_0000_0000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct FieldModifiers : u8 {
        const Public =    0b00000001;
        const Protected = 0b00000010;
        const Private =   0b00000100;
        const Static =    0b00001000;
        const Final =     0b00010000;
        const Transient = 0b00100000;
        const Volatile =  0b01000000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ClassModifiers : u8 {
        const Public =    0b00000001;
        const Protected = 0b00000010;
        const Private =   0b00000100;
        const Static =    0b00001000;
        const Final =     0b00010000;
        const Abstract =  0b00100000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct AnnotationModifiers : u8 {
        const Public =    0b00000001;
        const Protected = 0b00000010;
        const Private =   0b00000100;
        const Static =    0b00001000;
        const Final =     0b00010000;
        const Abstract =  0b00100000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct InterfaceModifiers : u8 {
        const Public =    0b00000001;
        const Protected = 0b00000010;
        const Private =   0b00000100;
        const Static =    0b00001000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct EnumModifiers : u8 {
        const Public =    0b00000001;
        const Protected = 0b00000010;
        const Private =   0b00000100;
        const Static =    0b00001000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ParameterModifiers : u8 {
        const Final =     0b00000001;
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QualifiedName {
    segments: Vec<Identifier>,
}

impl Spanned for QualifiedName {
    fn span(&self) -> Option<Span> {
        match (self.segments.first(), self.segments.last()) {
            (Some(first), Some(last)) => Some(Span::new(first.span.start(), last.span.end())),
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
