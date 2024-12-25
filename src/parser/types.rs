use crate::{
    text_data::BorrowedTextRange,
    tokenizer::{Token, TokenData},
};

pub enum YoyogayType<'a> {
    Identifier(&'a String, Vec<YoyogayType<'a>>),
    Array(Box<YoyogayType<'a>>),
    Tuple(Vec<YoyogayType<'a>>),
}

pub struct TypeParsingError<'a> {
    data: TypeParsingErrorData<'a>,
    text_data: BorrowedTextRange<'a>,
}

impl TypeParsingError<'_> {
    pub fn new<'a>(
        data: TypeParsingErrorData<'a>,
        text_data: BorrowedTextRange<'a>,
    ) -> TypeParsingError<'a> {
        TypeParsingError { data, text_data }
    }
}

pub enum TypeParsingErrorData<'a> {
    UnexpectedToken(&'a Token),
    UnexpectedTokens(&'a [Token]),
    EmptyTokenStreamAfterBrackets,
}

impl YoyogayType<'_> {
    pub fn create_from_tokens<'a>(
        tokens: &'a [Token],
    ) -> Result<YoyogayType<'a>, TypeParsingError<'a>> {
        assert_ne!(tokens.len(), 0);

        match (
            tokens.get(0).map(|tk| &tk.data),
            tokens.get(1).map(|tk| &tk.data),
        ) {
            (Some(TokenData::OpenBracket), Some(TokenData::CloseBracket)) => {
                if tokens.len() < 3 {
                    Err(TypeParsingError::new(
                        TypeParsingErrorData::EmptyTokenStreamAfterBrackets,
                        BorrowedTextRange::from((&tokens[0], &tokens[1])),
                    ))
                } else {
                    YoyogayType::create_from_tokens(&tokens[2..])
                }
            }

            (Some(TokenData::Identifier(ref id)), Some(TokenData::GreaterThan)) => Ok(
                YoyogayType::Identifier(id, vec![YoyogayType::create_from_tokens(&tokens[2..])?]),
            ),

            _ => panic!(),
        }
    }
}
