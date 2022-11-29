use crate::{Block, Expression};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfStatement {
    condition: Expression,
    then_statement: Block,
    else_ifs: Vec<ElseIfStatement>,
    else_statement: Option<Block>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ElseIfStatement {
    condition: Expression,
    statement: Block,
}
