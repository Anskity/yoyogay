use crate::{
    ast::{Node, NodeData},
    text_data::BorrowedTextRange,
    tokenizer::{Token, TokenData, TokensUtils},
    Boxxable,
};

use super::ParseError;

pub mod delimiter_checker;

pub fn parse_parameters<'a>(tokens: &'a [Token]) -> Result<Vec<Node<'a>>, ParseError> {
    tokens.split_tks(TokenData::Comma).into_iter().map(|tks| {
        assert_eq!(tks.len(), 1);
        let identifier = (if let TokenData::Identifier(ref id) = tks[0].data {
            let text_range = BorrowedTextRange::from(&tks[0]);
            Ok(Node {data: NodeData::Identifier(id).to_box(), text_range})
        } else {
            Err(ParseError::new_unexpected_token(tks[0].clone()))
        })?;

        let text_range = BorrowedTextRange::from(tks);
        let data = NodeData::FunctionParemeter(identifier).to_box();
        
        Ok(Node {
            data,
            text_range,
        })
    }).collect()
}
