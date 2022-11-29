use crate::{Expression, Identifier, ParameterModifiers, QualifiedName};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalVariableDeclaration {
    modifiers: ParameterModifiers,
    ty: QualifiedName,
    variables: Vec<LocalVariableDeclarationPart>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalVariableDeclarationPart {
    name: Identifier,
    value: Option<Expression>,
}
