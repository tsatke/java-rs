use crate::lexer::span::{Span, Spanned};
use crate::parser::error::Error;
use crate::parser::tree::identifier::Identifier;
use crate::parser::tree::qualified_name::QualifiedName;
use crate::parser::tree::{
    AnnotationModifiers, Block, ClassModifiers, EnumModifiers, Expression, FieldModifiers,
    InterfaceModifiers, MethodModifiers, ParameterModifiers,
};
use crate::Visibility;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompilationUnit {
    errors: Vec<Error>,
    package: Option<QualifiedName>,
    imports: Vec<ImportDeclaration>,
    types: Vec<TypeDeclaration>,
}

impl CompilationUnit {
    pub(in crate::parser) fn new() -> Self {
        Self {
            errors: vec![],
            package: None,
            imports: vec![],
            types: vec![],
        }
    }

    pub(in crate::parser) fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
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

impl Spanned for ImportDeclaration {
    fn span(&self) -> Option<Span> {
        match self {
            ImportDeclaration::SingleType(v) => v.span(),
            ImportDeclaration::OnDemand(v) => v.span(),
            ImportDeclaration::StaticSingleType(v) => v.span(),
            ImportDeclaration::StaticOnDemand(v) => v.span(),
        }
    }
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
    visibility: Visibility,
    modifiers: ClassModifiers,
    name: Identifier,
    extends: Option<QualifiedName>,
    implements: Vec<QualifiedName>,
    members: Vec<ClassMember>,
}

impl ClassDeclaration {
    pub(in crate::parser) fn new(
        visibility: Visibility,
        modifiers: ClassModifiers,
        name: Identifier,
    ) -> Self {
        Self {
            visibility,
            modifiers,
            name,
            extends: None,
            implements: vec![],
            members: vec![],
        }
    }

    pub(in crate::parser) fn add_member(&mut self, member: ClassMember) {
        self.members.push(member);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InterfaceDeclaration {
    visibility: Visibility,
    modifiers: InterfaceModifiers,
    name: Identifier,
    extends: Vec<QualifiedName>,
    members: Vec<InterfaceMember>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumDeclaration {
    visibility: Visibility,
    modifiers: EnumModifiers,
    name: Identifier,
    implements: Vec<QualifiedName>,
    members: Vec<EnumMember>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnnotationDeclaration {
    visibility: Visibility,
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
    visibility: Visibility,
    modifiers: FieldModifiers,
    name: Identifier,
    field_type: QualifiedName,
    initializer: Option<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodDeclaration {
    visibility: Visibility,
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
    visibility: Visibility,
    modifiers: MethodModifiers,
    parameters: Vec<Parameter>,
    throws: Vec<QualifiedName>,
    block: Block,
}
