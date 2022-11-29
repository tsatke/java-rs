use crate::parser::tree::statement::Statement;
use crate::Expression;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SwitchStatement {
    selector: Expression,
    cases: Vec<SwitchCase>,
    default: Option<Vec<Statement>>, // not technically a block
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SwitchCase {
    label: Option<Expression>,
    statements: Vec<Statement>, // not technically a block
}
