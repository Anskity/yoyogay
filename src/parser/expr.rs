use crate::{
    ast::{Node, NodeData, OperatorType},
    text_data::BorrowedTextRange,
    tokenizer::{Token, TokenData},
    Boxxable,
};

use super::ParseError;

pub fn parse_expr<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    dbg!(tokens);
    let mut tokens_to_parse: Vec<&[Token]> = Vec::new();
    let mut operators: Vec<OperatorType> = Vec::new();
    let mut prev_idx: usize = 0;

    for (i, tk) in tokens.iter().enumerate() {
        let is_operator = matches!(
            tk.data,
            TokenData::Add | TokenData::Sub | TokenData::Mul | TokenData::Div
        );
        let at_end = i == tokens.len() - 1;
        if is_operator || at_end {
            if is_operator {
                operators.push(tk.data.operator_type().expect("Token wasnt an operator"));
            }

            let range = if i == tokens.len() - 1 {
                tokens.len() - 1..tokens.len()
            } else {
                prev_idx..i
            };
            tokens_to_parse.push(&tokens[range]);
            prev_idx = i + 1;
        }
    }

    let nodes: Result<Vec<Node>, ParseError> = tokens_to_parse
        .into_iter()
        .map(|tks| parse_expr_component(tks))
        .collect();
    let mut nodes = nodes?;
    parse_operators(
        &mut nodes,
        &mut operators,
        &[OperatorType::Mul, OperatorType::Div],
    );
    parse_operators(
        &mut nodes,
        &mut operators,
        &[OperatorType::Add, OperatorType::Sub],
    );

    dbg!(nodes);

    todo!()
}

fn parse_operators<'a>(
    nodes: &mut Vec<Node<'a>>,
    operators: &mut Vec<OperatorType>,
    operators_to_parse: &'a [OperatorType],
) {
    assert_eq!(operators.len(), nodes.len() - 1);

    let mut ptr: usize = 0;

    while ptr < operators.len() {
        let operator = &operators[ptr];
        let current_operator = operators_to_parse
            .into_iter()
            .find(|optype| **optype == *operator);
        if let Some(op) = current_operator {
            let left = nodes.remove(ptr);
            let right = nodes.remove(ptr);
            let text_range = BorrowedTextRange::from((&left.text_range, &right.text_range));
            let node_data = NodeData::BinaryExpr(left, op, right);

            let node = Node {
                text_range,
                data: node_data.to_box(),
            };

            nodes.insert(ptr, node);
            operators.remove(ptr);
        } else {
            ptr += 1;
        }
    }
}

// TODO: Better Error Handling
fn parse_expr_component(tokens: &[Token]) -> Result<Node, ParseError> {
    if tokens.len() == 1 {
        let text_range = BorrowedTextRange::from(&tokens[0].text_range);
        let data = match &tokens[0].data {
            TokenData::Identifier(id) => NodeData::Identifier(&id),
            TokenData::NumericLiteral(num) => NodeData::NumericLiteral(&num),
            _ => todo!(),
        };

        return Ok(Node {text_range, data: data.to_box()});
    }
    todo!()
}
