use stmt::parse_stmt;

use crate::{
    ast::{Node, NodeData},
    text_data::{BorrowedTextRange, TextRange},
    tokenizer::{Token, TokenData},
    Boxxable,
};

pub mod expr;
pub mod stmt;
pub mod utils;

#[derive(Debug)]
pub struct ParseError {
    pub data: ParseErrorData,
    pub text_range: TextRange,
}

impl ParseError {
    fn new(data: ParseErrorData, text_range: TextRange) -> Self {
        ParseError {
            data,
            text_range,
        }
    }
    fn new_unexpected_token(token: Token) -> ParseError {
        ParseError {
            data: ParseErrorData::UnexpectedToken(token.data),
            text_range: token.text_range,
        }
    }
    fn new_unexpected_eof<'a, T: Into<TextRange>>(range: T) -> ParseError {
        ParseError {
            data: ParseErrorData::UnexpectedEOF,
            text_range: range.into(),
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorData {
    UnexpectedToken(TokenData),
    MissingSemilicon,
    UnclosedBracket,
    UnclosedParenthesis,
    UnclosedCurly,
    UnexpectedEOF,
}

pub fn parse<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    let mut statements: Vec<Node<'a>> = Vec::new();
    let mut ptr: usize = 0;

    while ptr < tokens.len() {
        let (node, used) = parse_stmt(&tokens[ptr..])?;

        assert_ne!(used, 0);
        ptr += used;

        statements.push(node);
    }

    let text_range = BorrowedTextRange::from((
        &statements[0].text_range,
        &statements.last().unwrap().text_range,
    ));

    Ok(Node {
        data: NodeData::Program(statements).to_box(),
        text_range,
    })
}
