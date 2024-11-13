use crate::tokenizer::{Token, TokenData};

use super::delimiter_checker::DelimiterChecker;

pub fn find_free_token(tokens: &[Token], search_tk: &Token) -> Option<usize> {
    assert!(!matches!(search_tk.data, TokenData::OpenParenthesis 
                                    | TokenData::OpenCurly
                                    | TokenData::OpenBracket
                                    | TokenData::CloseParenthesis
                                    | TokenData::CloseCurly
                                    | TokenData::CloseBracket));

    let mut delimiter_checker = DelimiterChecker::new();
    for (i, tk) in tokens.iter().enumerate() {
        delimiter_checker.check(&tk).ok()?;
        if tk.data == search_tk.data {
            return Some(i);
        }
    }

    None
}
