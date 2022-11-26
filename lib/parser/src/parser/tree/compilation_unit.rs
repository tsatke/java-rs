use crate::parser::tree::{
    AnnotationModifiers, Block, ClassModifiers, EnumModifiers, Expression, FieldModifiers,
    Identifier, InterfaceModifiers, MethodModifiers, ParameterModifiers, QualifiedName,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompilationUnit {
    package: Option<QualifiedName>,
    imports: Vec<ImportDeclaration>,
    types: Vec<TypeDeclaration>,
}

impl CompilationUnit {
    pub(in crate::parser) fn new() -> Self {
        Self {
            package: None,
            imports: vec![],
            types: vec![],
        }
    }

    pub(in crate::parser) fn set_package(&mut self, package: QualifiedName) {
        self.package = Some(package);
    }

    pub(in crate::parser) fn add_import(&mut self, import: ImportDeclaration) {
        self.imports.push(import);
    }

    pub(in crate::parser) fn add_type(&mut self, ty: TypeDeclaration) {
        self.types.push(ty);
    }

    pub fn package(&self) -> Option<&QualifiedName> {
        self.package.as_ref()
    }

    pub fn imports(&self) -> &[ImportDeclaration] {
        &self.imports
    }

    pub fn types(&self) -> &[TypeDeclaration] {
        &self.types
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImportDeclaration {
    SingleType(QualifiedName),
    OnDemand(QualifiedName),
    StaticSingleType(QualifiedName),
    StaticOnDemand(QualifiedName),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Annotation(AnnotationDeclaration),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassDeclaration {
    modifiers: ClassModifiers,
    name: Identifier,
    extends: Option<QualifiedName>,
    implements: Vec<QualifiedName>,
    members: Vec<ClassMember>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InterfaceDeclaration {
    visibility: InterfaceModifiers,
    name: Identifier,
    extends: Vec<QualifiedName>,
    members: Vec<InterfaceMember>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumDeclaration {
    modifiers: EnumModifiers,
    name: Identifier,
    implements: Vec<QualifiedName>,
    members: Vec<EnumMember>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnnotationDeclaration {
    modifiers: AnnotationModifiers,
    name: Identifier,
    members: Vec<AnnotationMember>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClassMember {
    Type(TypeDeclaration),
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Constructor(ConstructorDeclaration),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InterfaceMember {
    Type(TypeDeclaration),
    Method(MethodDeclaration),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EnumMember {
    EnumConstant(Identifier),
    Type(TypeDeclaration),
    Field(FieldDeclaration),
    Constructor(ConstructorDeclaration),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AnnotationMember {
    Type(TypeDeclaration),
    Field(FieldDeclaration),
    Method(MethodDeclaration),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDeclaration {
    modifiers: FieldModifiers,
    name: Identifier,
    field_type: QualifiedName,
    initializer: Option<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodDeclaration {
    modifiers: MethodModifiers,
    return_type: Option<QualifiedName>,
    parameters: Vec<Parameter>,
    throws: Vec<QualifiedName>,
    block: Option<Block>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameter {
    modifiers: ParameterModifiers,
    name: Identifier,
    parameter_type: QualifiedName,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstructorDeclaration {
    modifiers: MethodModifiers,
    parameters: Vec<Parameter>,
    throws: Vec<QualifiedName>,
    block: Block,
}
