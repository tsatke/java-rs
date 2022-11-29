use crate::parser::tree::local_var_decl::LocalVariableDeclaration;
use crate::{Block, Expression, Identifier, ParameterModifiers, QualifiedName};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThrowStatement {
    expression: Expression,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryStatement {
    block: Block,
    resources: Vec<TryResource>,
    catches: Vec<CatchClause>,
    finally: Option<Block>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryResource {
    variable: LocalVariableDeclaration,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CatchClause {
    parameter: CatchParameter,
    block: Block,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CatchParameter {
    modifiers: ParameterModifiers,
    name: Identifier,
    ty: Vec<QualifiedName>,
}
