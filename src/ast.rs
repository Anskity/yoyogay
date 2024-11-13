use crate::text_data::{BorrowedTextRange, TextRange};

#[derive(Debug)]
pub struct Node<'a> {
    pub data: Box<NodeData<'a>>,
    pub text_range: BorrowedTextRange<'a>,
}

#[derive(Debug)]
pub enum NodeData<'a> {
    Program(Vec<Node<'a>>),
    Identifier(&'a String),
    NumericLiteral(&'a usize),
    String(&'a String),
    BinaryExpr(Node<'a>, &'a OperatorType, Node<'a>),
    Tuple(Vec<Node<'a>>),
    ModAccess(Node<'a>, Node<'a>),
    FunctionDeclaration(Node<'a>, Vec<Node<'a>>, Node<'a>),
    FunctionCall(Node<'a>, Vec<Node<'a>>),
    FunctionParemeter(Node<'a>),
    FunctionArgument(Node<'a>),
    If(Node<'a>, Node<'a>, Option<Node<'a>>),
    Else(Node<'a>),
    VariableDeclaration(DeclarationType, Node<'a>, Node<'a>),
    VariableModification(Node<'a>, VariableModificationType, Node<'a>),
}

#[derive(Debug)]
pub enum DeclarationType {
    Const,
    Var,
}

#[derive(Debug, PartialEq)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum VariableModificationType {
    IncreaseBy,
    DecreaseBy,
    MultiplyBy,
    DivideBy,
    Set,
} 
