use crate::{
    AssertStatement, Block, BreakStatement, ContinueStatement, DoWhileStatement, Expression,
    ForEachStatement, ForStatement, Identifier, IfStatement, LocalVariableDeclaration,
    ReturnStatement, SwitchStatement, SynchronizedStatement, ThrowStatement, TryStatement,
    WhileStatement,
};

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
