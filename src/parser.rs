use stmt::parse_stmt;
use utils::delimiter_checker::DelimiterCheckerError;

use crate::{
    ast::{Node, NodeData},
    text_data::{BorrowedTextRange, TextRange},
    tokenizer::{tokenize, Token, TokenData, TokenizeError},
    Boxxable,
};

pub mod expr;
pub mod stmt;
pub mod types;
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

impl From<DelimiterCheckerError<'_>> for ParseError {
    fn from(checker: DelimiterCheckerError) -> ParseError {
        match checker {
            DelimiterCheckerError::UnexpectedClosingParenthesis(token) => ParseError::new_unexpected_token(token.clone()),
            DelimiterCheckerError::UnexpectedClosingCurlyBrace(token) => ParseError::new_unexpected_token(token.clone()),
            DelimiterCheckerError::UnexpectedClosingBracket(token) => ParseError::new_unexpected_token(token.clone()),
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
    TokenizeError(TokenizeError),
}

impl From<TokenizeError> for ParseError {
    fn from(value: TokenizeError) -> Self {
        let text_range = value.text_range.clone();
        ParseError::new(ParseErrorData::TokenizeError(value), text_range)
    }
}

pub fn parse_tks<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
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
