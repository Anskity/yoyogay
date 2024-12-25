use crate::{
    ast::{DeclarationType, Node, NodeData, VariableModificationType},
    parser::{expr::parse_expr, parse_tks, utils::parse_parameters, ParseErrorData},
    text_data::{BorrowedTextRange, TextRange},
    tokenizer::{Token, TokenData, TokensUtils},
    Boxxable,
};

use super::ParseError;

pub fn parse_stmt<'a>(tokens: &'a [Token]) -> Result<(Node<'a>, usize), ParseError> {
    if matches!(&tokens[0].data, TokenData::Const | TokenData::Var | TokenData::Let) {
        let semilicon = tokens.find_free(&TokenData::Semilicon).ok_or_else(|| {
            let text_data = BorrowedTextRange::from(tokens);
            ParseError::new(ParseErrorData::MissingSemilicon, text_data.into())
        })?;

        return Ok((
            parse_variable_declaration(&tokens[0..=semilicon])?,
            semilicon + 1,
        ));
    }

    if let TokenData::Fn = tokens[0].data {
        return parse_function_declaration(tokens);
    }

    if let TokenData::If = tokens[0].data {
        return parse_if(tokens);
    }

    if let TokenData::Identifier(_) = tokens[0].data {
        let semilicon = tokens.find_free(&TokenData::Semilicon).ok_or_else(|| {
            let text_data = BorrowedTextRange::from(tokens);
            ParseError::new(ParseErrorData::MissingSemilicon, text_data.into())
        })?;

        return Ok((
            parse_variable_modification(&tokens[..=semilicon])?,
            semilicon + 1,
        ));
    }

    panic!()
}

fn parse_variable_declaration<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    let declaration_type = DeclarationType::try_from(&tokens[0])
        .map_err(|tk| ParseError::new_unexpected_token(tk.clone()))?;
    assert!(matches!(
        &tokens.last().expect("Tokens slice was empty").data,
        TokenData::Semilicon
    ));

    let identifier: Node = if let TokenData::Identifier(id) = &tokens[1].data {
        Ok(Node {
            data: NodeData::Identifier(id).to_box(),
            text_range: BorrowedTextRange::from(&tokens[1].text_range),
        })
    } else {
        Err(ParseError::new_unexpected_token(tokens[1].clone()))
    }?;

    if !matches!(tokens.get(2).map(|tk| &tk.data), Some(TokenData::Equals)) {
        return Err(ParseError::new_unexpected_token(tokens[2].clone()));
    }

    let expr = parse_expr(&tokens[3..tokens.len() - 1])?;

    let text_range = BorrowedTextRange::from(tokens);
    let data = NodeData::VariableDeclaration(declaration_type, identifier, expr).to_box();
    let node = Node { text_range, data };

    Ok(node)
}

fn parse_function_declaration<'a>(tokens: &'a [Token]) -> Result<(Node<'a>, usize), ParseError> {
    assert!(matches!(tokens[0].data, TokenData::Fn));
    let identifier: Node = if let TokenData::Identifier(id) = &tokens[1].data {
        Ok(Node {
            data: NodeData::Identifier(id).to_box(),
            text_range: BorrowedTextRange::from(&tokens[1].text_range),
        })
    } else {
        Err(ParseError::new_unexpected_token(tokens[1].clone()))
    }?;

    if !matches!(tokens[2].data, TokenData::OpenParenthesis) {
        return Err(ParseError::new_unexpected_token(tokens[2].clone()));
    }

    let end_parenthesis = tokens.find_pair(2).ok_or(ParseError::new(
        ParseErrorData::UnclosedParenthesis,
        TextRange::from(tokens),
    ))?;

    let parameters = parse_parameters(&tokens[3..end_parenthesis])?;

    if end_parenthesis == tokens.len() - 1 {
        return Err(ParseError::new_unexpected_eof(tokens));
    }

    if !matches!(tokens[end_parenthesis + 1].data, TokenData::OpenCurly) {
        return Err(ParseError::new_unexpected_token(
            tokens[end_parenthesis + 1].clone(),
        ));
    }

    let end_curly = tokens
        .find_pair(end_parenthesis + 1)
        .ok_or(ParseError::new(
            ParseErrorData::UnclosedCurly,
            TextRange::from(tokens),
        ))?;

    let body = parse_tks(&tokens[end_parenthesis + 2..end_curly])?;

    let text_range = BorrowedTextRange::from(&tokens[0..=end_curly]);
    let data = NodeData::FunctionDeclaration(identifier, parameters, body).to_box();

    Ok((Node { data, text_range }, end_curly + 1))
}

fn parse_if<'a>(tokens: &'a [Token]) -> Result<(Node<'a>, usize), ParseError> {
    assert!(matches!(tokens[0].data, TokenData::If));

    let start_curly = tokens
        .find_free(TokenData::OpenCurly)
        .ok_or(ParseError::new(
            ParseErrorData::UnexpectedEOF,
            TextRange::from(tokens),
        ))?;
    let end_curly = tokens.find_pair(start_curly).ok_or(ParseError::new(
        ParseErrorData::UnclosedCurly,
        TextRange::from(tokens),
    ))?;
    let condition = parse_expr(&tokens[1..start_curly])?;
    let body = parse_tks(&tokens[start_curly + 1..end_curly])?;

    let (else_node, used) =
        if let Some(TokenData::Else) = tokens.get(end_curly + 1).map(|tk| &tk.data) {
            if end_curly + 1 == tokens.len() - 1 {
                return Err(ParseError::new(
                    ParseErrorData::UnexpectedEOF,
                    TextRange::from(tokens),
                ));
            }

            if !matches!(tokens[end_curly + 2].data, TokenData::OpenCurly) {
                return Err(ParseError::new_unexpected_token(
                    tokens[end_curly + 2].clone(),
                ));
            }

            let else_start_curly = end_curly + 2;
            let else_end_curly = tokens.find_pair(else_start_curly).ok_or(ParseError::new(
                ParseErrorData::UnclosedCurly,
                TextRange::from(tokens),
            ))?;

            let else_body = parse_tks(&tokens[else_start_curly + 1..else_end_curly])?;
            let text_range = BorrowedTextRange::from(&tokens[end_curly + 1..=else_end_curly]);
            let data = NodeData::Else(else_body).to_box();

            (Some(Node { text_range, data }), else_end_curly + 1)
        } else {
            (None, end_curly + 1)
        };

    let text_range = BorrowedTextRange::from(&tokens[0..]);
    let data = NodeData::If(condition, body, else_node).to_box();

    Ok((Node { data, text_range }, used))
}

fn parse_variable_modification<'a>(tokens: &'a [Token]) -> Result<Node<'a>, ParseError> {
    assert!(matches!(
        tokens.last().expect("Tokens slice is empty").data,
        TokenData::Semilicon
    ));

    let mut variable_mod: Option<(usize, VariableModificationType)> = None;

    let mut switch = false;
    for (i, tk) in tokens.iter().enumerate() {
        let variable_mod_type_maybe = tk.data.variable_modification_type();
        if let Some(variable_mod_type) = variable_mod_type_maybe {
            variable_mod = Some((i, variable_mod_type));
            break;
        }

        if (!switch && !matches!(tk.data, TokenData::Identifier(_)))
            || (switch && !matches!(tk.data, TokenData::Comma))
        {
            return Err(ParseError::new_unexpected_token(tk.clone()));
        }
        switch = !switch;
    }

    let (mod_idx, variable_mod) =
        variable_mod.ok_or_else(|| ParseError::new_unexpected_eof(tokens))?;

    let id = parse_expr(&tokens[0..mod_idx])?;
    let value = parse_expr(&tokens[mod_idx + 1..tokens.len() - 1])?;

    let text_range = BorrowedTextRange::from(tokens);
    let data = NodeData::VariableModification(id, variable_mod, value).to_box();

    Ok(Node { text_range, data })
}
