use crate::{Block, Expression};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DoWhileStatement {
    condition: Expression,
    block: Block,
}
