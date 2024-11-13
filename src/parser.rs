use stmt::parse_stmt;

use crate::{
    ast::{Node, NodeData},
    text_data::BorrowedTextRange,
    tokenizer::Token,
    Boxxable,
};

pub mod expr;
mod stmt;
mod utils;

#[derive(Debug)]
pub enum ParseError {

}

pub fn parse<'a>(tokens: &'a [Token]) -> Node<'a> {
    let mut statements: Vec<Node<'a>> = Vec::new();
    let mut ptr: usize = 0;

    while ptr < tokens.len() {
        let (node, used) = parse_stmt(tokens);

        assert_ne!(used, 0);
        ptr += used;

        statements.push(node);
    }

    let text_range = BorrowedTextRange::from((
        &statements[0].text_range,
        &statements.last().unwrap().text_range,
    ));

    Node {
        data: NodeData::Program(statements).to_box(),
        text_range,
    }
}
