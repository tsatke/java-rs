use crate::{Block, Expression};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SynchronizedStatement {
    expression: Expression,
    block: Block,
}
