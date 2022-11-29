use crate::Expression;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssertStatement {
    condition: Expression,
    detail: Option<Expression>,
}
