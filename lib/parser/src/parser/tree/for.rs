use crate::parser::tree::local_var_decl::LocalVariableDeclaration;
use crate::{Block, Expression, Identifier, ParameterModifiers, QualifiedName};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForStatement {
    initializers: Vec<ForInitializer>,
    condition: Option<Expression>,
    updaters: Vec<Expression>,
    block: Block,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ForInitializer {
    LocalVariableDeclaration(LocalVariableDeclaration),
    Expression(Expression),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForEachStatement {
    variable: ForEachVariableDeclaration,
    expression: Expression,
    block: Block,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForEachVariableDeclaration {
    modifiers: ParameterModifiers,
    ty: QualifiedName,
    name: Identifier,
}
