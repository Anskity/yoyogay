use crate::text_data::TextRange;

pub struct Node {
    pub data: Box<NodeData>,
    pub text_range: TextRange,
}

pub enum NodeData {
    Identifier(String),
    NumericLiteral(usize),
    String(String),
    BinaryExpr(Node, OperatorType, Node),
    Tuple(Vec<Node>),
    ModAccess(Node, Node),
    FunctionDeclaration(Node, Vec<Node>, Node),
    FunctionCall(Node, Vec<Node>),
    FunctionParemeter(Node),
    FunctionArgument(Node),
    If(Node, Node, Option<Node>),
    Else(Node),
    VariableDeclaration(DeclarationType, Node, Node),
    VariableModification(Node, VariableModificationType, Node),
}

pub enum DeclarationType {
    Const,
    Var,
}

pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum VariableModificationType {
    IncreaseBy,
    DecreaseBy,
    MultiplyBy,
    DivideBy,
    Set,
} 
