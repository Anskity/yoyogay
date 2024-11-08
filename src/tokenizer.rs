use crate::text_data::TextRange;

pub struct Token {
    pub data: TokenData,
    pub text_range: TextRange,
}

pub enum TokenData {
    Identifier(String),
    NumericLiteral(usize),
    Equals,
    Add,
    Sub,
    Mul,
    Div,
}
