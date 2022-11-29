use crate::parser::tree::statement::Statement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
    statements: Vec<Statement>,
}
