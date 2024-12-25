use crate::{
    text_data::BorrowedTextRange,
    tokenizer::{Token, TokenData},
};

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
    FunctionDeclaration(Node<'a>, Vec<Node<'a>>, Node<'a>),
    FunctionCall(Node<'a>, Vec<Node<'a>>),
    FunctionParemeter(Node<'a>),
    If(Node<'a>, Node<'a>, Option<Node<'a>>),
    Else(Node<'a>),
    VariableDeclaration(DeclarationType, Node<'a>, Node<'a>),
    VariableModification(Node<'a>, VariableModificationType, Node<'a>),
    StructAccess(Node<'a>, Node<'a>),
    ModAccess(Node<'a>, Node<'a>),
    ArrayAccess(Node<'a>, Node<'a>),
    Neg(Node<'a>),
    Type
}

#[derive(Debug)]
pub enum DeclarationType {
    Const,
    Var,
    Let,
}

impl<'a> TryFrom<&'a Token> for DeclarationType {
    type Error = &'a Token;
    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value.data {
            TokenData::Const => Ok(DeclarationType::Const),
            TokenData::Var => Ok(DeclarationType::Var),
            _ => Err(value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,

    NotEquals,
    IsEquals,
    Or,
}

impl ToString for OperatorType {
    fn to_string(&self) -> String {
        match self {
            OperatorType::Add => "+".to_string(),
            OperatorType::Sub => "-".to_string(),
            OperatorType::Mul => "*".to_string(),
            OperatorType::Div => "/".to_string(),
            OperatorType::NotEquals => "!=".to_string(),
            OperatorType::IsEquals => "==".to_string(),
            OperatorType::Or => "||".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VariableModificationType {
    IncreaseBy,
    DecreaseBy,
    MultiplyBy,
    DivideBy,
    Set,
}

impl ToString for VariableModificationType {
    fn to_string(&self) -> String {
        match self {
            VariableModificationType::IncreaseBy => "+=",
            VariableModificationType::DecreaseBy => "-=",
            VariableModificationType::MultiplyBy => "*=",
            VariableModificationType::DivideBy => "/=",
            VariableModificationType::Set => "=",
        }.to_string()
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertyAccessType {
    Struct,
    Mod,
}

impl<'a> ToString for Node<'a> {
    fn to_string(&self) -> String {
        match &*self.data {
            NodeData::Identifier(id) => id.to_string(),
            NodeData::NumericLiteral(num) => num.to_string(),
            NodeData::FunctionCall(id, params) => {
                let mut params_txt = "".to_string();
                for (i, param) in params.iter().enumerate() {
                    params_txt.push_str(&param.to_string());
                    if i < params.len() - 1 {
                        params_txt.push_str(", ");
                    }
                }
                format!("{}({})", id.to_string(), params_txt)
            }
            NodeData::StructAccess(struct_node, prop) => {
                format!("{}.{}", struct_node.to_string(), prop.to_string())
            }
            NodeData::ModAccess(mod_node, prop) => {
                format!("{}.{}", mod_node.to_string(), prop.to_string())
            }
            NodeData::ArrayAccess(arr_node, idx_node) => {
                format!("{}[{}]", arr_node.to_string(), idx_node.to_string())
            }
            NodeData::BinaryExpr(a, op, b) => {
                format!("({}) {} ({})", a.to_string(), op.to_string(), b.to_string())
            }
            NodeData::VariableDeclaration(declaration_type, id, expr) => format!(
                "{} {} = {};",
                declaration_type.to_string(),
                id.to_string(),
                expr.to_string()
            ),
            NodeData::Program(statements) => {
                let mut txt = String::new();

                for (i, stmt) in statements.iter().enumerate() {
                    txt.push_str(&stmt.to_string());

                    if i < statements.len()-1 {
                        txt.push_str("\n\n");
                    }
                }

                txt
            }
            NodeData::Tuple(values) => {
                let mut txt = "(".to_string();

                for (i, value) in values.iter().enumerate() {
                    txt.push_str(&value.to_string());

                    if i < values.len()-1 {
                        txt.push_str(", ");
                    }
                }

                txt.push_str(")");

                txt
            }
            NodeData::FunctionDeclaration(name, args, body) => {
                let mut txt = "fn ".to_string();
                txt.push_str(&name.to_string());
                txt.push_str("(");

                for (i, arg) in args.iter().enumerate() {
                    txt.push_str(&arg.to_string());
                    if i < args.len() - 1 {
                        txt.push_str(", ");
                    }
                }
                txt.push_str(") {\n    ");
                txt.push_str(&body.to_string().replace("\n", "\n    "));
                txt.push_str("\n}");

                txt
            }
            NodeData::FunctionParemeter(param) => param.to_string(),
            NodeData::Neg(expr) => format!("(-{})", expr.to_string()),
            NodeData::If(condition, body, else_node) => {
                let mut txt = format!("if {} {{\n    ", condition.to_string());
                txt.push_str(&body.to_string().replace("\n", "\n    "));
                txt.push_str("\n}");

                if let Some(node) = else_node {
                    match *node.data {
                        NodeData::Else(_) => txt.push_str(&format!(" {}", node.to_string())),
                        _ => panic!("Else node wasnt an else node"),
                    }
                }

                txt
            }
            NodeData::Else(body) => {
                let mut txt = "else {\n    ".to_string();
                txt.push_str(&body.to_string().replace("\n", "\n    "));
                txt.push_str("\n}");

                txt
            }
            NodeData::VariableModification(id, mod_type, value) => format!(
                "{} {} {};",
                id.to_string(),
                mod_type.to_string(),
                value.to_string()
            ),

            _ => panic!("unhandled: {:?}", self.data),
        }
    }
}

impl ToString for DeclarationType {
    fn to_string(&self) -> String {
        match self {
            DeclarationType::Var => "var".to_string(),
            DeclarationType::Const => "const".to_string(),
            DeclarationType::Let => "let".to_string(),
        }
    }
}
