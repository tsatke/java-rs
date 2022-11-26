use crate::parser::tree::{Expression, Identifier, ParameterModifiers, QualifiedName};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Statement {
    label: Identifier,
    statement: StatementKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatementKind {
    Empty,
    Block(Block),
    Expression(Expression),
    If(IfStatement),
    Switch(SwitchStatement),
    While(WhileStatement),
    DoWhile(DoWhileStatement),
    For(ForStatement),
    ForEach(ForEachStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Return(ReturnStatement),
    Synchronized(SynchronizedStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
    Assert(AssertStatement),
    LocalVariableDeclaration(LocalVariableDeclaration),
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SwitchStatement {
    selector: Expression,
    cases: Vec<SwitchCase>,
    default: Option<Vec<Statement>>, // not technically a block
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SwitchCase {
    label: Option<Expression>,
    statements: Vec<Statement>, // not technically a block
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WhileStatement {
    condition: Expression,
    block: Block,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DoWhileStatement {
    condition: Expression,
    block: Block,
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BreakStatement {
    label: Option<Identifier>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContinueStatement {
    label: Option<Identifier>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReturnStatement {
    expression: Option<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SynchronizedStatement {
    expression: Expression,
    block: Block,
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssertStatement {
    condition: Expression,
    detail: Option<Expression>,
}

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
