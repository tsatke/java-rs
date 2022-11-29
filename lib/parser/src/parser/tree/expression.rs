use crate::lexer::span::Span;
use crate::parser::tree::qualified_name::QualifiedName;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    StringLiteral(StringLiteral),
    MethodCall(MethodCall),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringLiteral {
    span: Span,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodCall {
    name: QualifiedName,
    arguments: Vec<Expression>,
}
