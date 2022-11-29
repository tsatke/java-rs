use crate::{Block, Expression};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WhileStatement {
    condition: Expression,
    block: Block,
}
