use crate::{Expression, Identifier};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BreakStatement {
    label: Option<Identifier>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContinueStatement {
    label: Option<Identifier>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReturnStatement {
    expression: Option<Expression>,
}
