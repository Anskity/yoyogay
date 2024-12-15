use crate::{
    ast::{Node, NodeData, OperatorType, PropertyAccessType},
    parser::{utils::delimiter_checker::DelimiterChecker, BorrowedTextRange, ParseErrorData},
    text_data::TextRange,
    tokenizer::{Token, TokenData, TokensUtils},
    Boxxable,
};

use super::ParseError;

pub fn parse_expr<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    let mut tokens_to_parse: Vec<&[Token]> = Vec::new();
    let mut operators: Vec<OperatorType> = Vec::new();
    let mut prev_idx: usize = 0;
    let mut delimiter_checker = DelimiterChecker::new();

    if tokens.find_free(TokenData::Comma).is_some() {
        return parse_tuple(tokens);
    }

    for (i, tk) in tokens.iter().enumerate() {
        let is_operator = tk.data.operator_type().is_some() && i != 0;
        delimiter_checker.check(tk)?;
        
        let at_end = i == tokens.len() - 1;
        if (is_operator && delimiter_checker.is_free()) || at_end {
            if is_operator {
                operators.push(tk.data.operator_type().expect("Token wasnt an operator"));
            }

            let range = if at_end {
                prev_idx..tokens.len()
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
    parse_operators(
        &mut nodes,
        &mut operators,
        &[OperatorType::NotEquals, OperatorType::IsEquals],
    );
    parse_operators(&mut nodes, &mut operators, &[OperatorType::Or]);

    assert_eq!(nodes.len(), 1);

    Ok(nodes.remove(0))
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
    assert_ne!(tokens.len(), 0);

    if tokens.len() == 1 {
        return parse_primary(&tokens[0]);
    }

    if let TokenData::Sub = tokens[0].data {
        let text_range = BorrowedTextRange::from(tokens);
        let data = NodeData::Neg(parse_expr(&tokens[1..])?).to_box();
        return Ok(Node { text_range, data });
    }

    if let TokenData::OpenParenthesis = tokens[0].data {
        if tokens
            .find_pair(0)
            .is_some_and(|idx| idx == tokens.len() - 1)
        {
            return parse_expr(&tokens[1..tokens.len() - 1]);
        }
    }

    if tokens.find_free(&TokenData::Comma).is_some() {
        return parse_tuple(tokens);
    }

    if let TokenData::Identifier(_) = &tokens[0].data {
        let node = parse_primary(&tokens[0])?;
        return parse_chain(node, &tokens[1..]);
    }

    todo!()
}

fn parse_primary(tk: &Token) -> Result<Node, ParseError> {
    let text_range = BorrowedTextRange::from(&tk.text_range);
    let data = match &tk.data {
        TokenData::Identifier(id) => NodeData::Identifier(&id),
        TokenData::NumericLiteral(num) => NodeData::NumericLiteral(&num),
        _ => panic!("UNEXPECTED: {:?}", tk),
    };

    Ok(Node {
        text_range,
        data: data.to_box(),
    })
}

fn parse_function_call<'a>(
    func_node: Node<'a>,
    tokens: &'a [Token],
) -> Result<Node<'a>, ParseError> {
    assert!(tokens.len() > 1);
    assert!(matches!(tokens[0].data, TokenData::OpenParenthesis));

    let close_paren = tokens.find_pair(0).unwrap();

    let args_tks = &tokens[1..close_paren].split_tks(&TokenData::Comma);
    let args_nodes: Result<Vec<Node>, ParseError> =
        args_tks.into_iter().map(|tks| parse_expr(tks)).collect();

    let text_range = BorrowedTextRange::from(tokens);
    let data = NodeData::FunctionCall(func_node, args_nodes?);
    let node = Node {
        text_range,
        data: data.to_box(),
    };

    if close_paren < tokens.len() - 1 {
        parse_chain(node, &tokens[close_paren + 1..])
    } else {
        Ok(node)
    }
}

fn parse_property_access<'a>(
    struct_node: Node<'a>,
    tokens: &'a [Token],
    property_access_type: PropertyAccessType,
) -> Result<Node<'a>, ParseError> {
    assert!(tokens[0].data.property_access_type().is_some());
    if tokens.get(1).is_none() {
        return Err(ParseError::new_unexpected_token(tokens[0].clone()));
    }

    if let TokenData::Identifier(_) = &tokens[1].data {
        let text_range = BorrowedTextRange::from((
            &struct_node.text_range,
            &BorrowedTextRange::from(&tokens[0..2]),
        ));
        let prop_node = parse_expr(&tokens[1..=1])?;
        let data = match property_access_type {
            PropertyAccessType::Struct => NodeData::StructAccess(struct_node, prop_node),
            PropertyAccessType::Mod => NodeData::ModAccess(struct_node, prop_node),
        };
        let node = Node {
            data: data.to_box(),
            text_range,
        };
        if tokens.len() == 2 {
            Ok(node)
        } else {
            parse_chain(node, &tokens[2..])
        }
    } else {
        Err(ParseError::new_unexpected_token(tokens[1].clone()))
    }
}

fn parse_array_access<'a>(arr_node: Node<'a>, tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    assert_eq!(tokens[0].data, TokenData::OpenBracket);

    let end_brack = tokens.find_pair(0).ok_or_else(|| {
        let text_range: TextRange = BorrowedTextRange::from(tokens).into();
        ParseError::new(ParseErrorData::UnclosedBracket, text_range)
    })?;

    let idx_node = parse_expr(&tokens[1..end_brack])?;

    let text_range = BorrowedTextRange::from((
        &arr_node.text_range,
        &BorrowedTextRange::from(&tokens[0..=end_brack]),
    ));
    let data = NodeData::ArrayAccess(arr_node, idx_node).to_box();
    let node = Node { data, text_range };

    if end_brack == tokens.len() - 1 {
        Ok(node)
    } else {
        parse_chain(node, &tokens[end_brack + 1..])
    }
}

fn parse_chain<'a>(node: Node<'a>, tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    assert!(tokens.len() > 1);
    match tokens[0].data {
        TokenData::OpenParenthesis => parse_function_call(node, tokens),
        TokenData::Dot => parse_property_access(node, tokens, PropertyAccessType::Struct),
        TokenData::ModAccess => parse_property_access(node, tokens, PropertyAccessType::Mod),
        TokenData::OpenBracket => parse_array_access(node, tokens),
        _ => todo!(),
    }
}

fn parse_tuple<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    let nodes: Result<Vec<Node<'a>>, ParseError> = tokens
        .split_tks(&TokenData::Comma)
        .iter()
        .map(|tks| parse_expr(tks))
        .collect();
    let text_range = BorrowedTextRange::from(tokens);
    let data = NodeData::Tuple(nodes?).to_box();

    Ok(Node { data, text_range })
}
