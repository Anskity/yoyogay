use crate::{tokenizer::{Token, TokenData}};

pub struct DelimiterChecker {
    pub paren_level: usize,
    pub curly_level: usize,
    pub brack_level: usize,
}

pub enum DelimiterCheckerError<'a> {
    UnexpectedClosingParenthesis(&'a Token),
    UnexpectedClosingCurlyBrace(&'a Token),
    UnexpectedClosingBracket(&'a Token),
}

impl DelimiterChecker {
    pub fn new() -> Self {
        DelimiterChecker {
            paren_level: 0,
            curly_level: 0,
            brack_level: 0,
        }
    }

    pub fn is_free(&self) -> bool {
        self.paren_level == 0 && self.curly_level == 0 && self.brack_level == 0
    }

    pub fn check<'a>(&mut self, token: &'a Token) -> Result<(), DelimiterCheckerError<'a>> {
        match token.data {
            TokenData::OpenParenthesis => self.paren_level += 1,
            TokenData::OpenCurly => self.curly_level += 1,
            TokenData::OpenBracket => self.brack_level += 1,

            TokenData::CloseParenthesis => {
                if self.paren_level > 0 {
                    self.paren_level -= 1;
                } else {
                    return Err(DelimiterCheckerError::UnexpectedClosingParenthesis(&token));
                }
            }
            TokenData::CloseBracket => {
                if self.brack_level > 0 {
                    self.brack_level -= 1;
                } else {
                    return Err(DelimiterCheckerError::UnexpectedClosingBracket(&token));
                }
            }
            TokenData::CloseCurly => {
                if self.curly_level > 0 {
                    self.curly_level -= 1;
                } else {
                    return Err(DelimiterCheckerError::UnexpectedClosingCurlyBrace(&token));
                }
            }

            _ => {}
        }
        
        Ok(())
    }

    pub fn check_reverse<'a>(&mut self, token: &'a Token) -> Result<(), DelimiterCheckerError<'a>> {
        match token.data {
            TokenData::CloseParenthesis => self.paren_level += 1,
            TokenData::CloseCurly => self.curly_level += 1,
            TokenData::CloseBracket => self.brack_level += 1,

            TokenData::OpenParenthesis => {
                if self.paren_level > 0 {
                    self.paren_level -= 1;
                } else {
                    return Err(DelimiterCheckerError::UnexpectedClosingParenthesis(&token));
                }
            }
            TokenData::OpenBracket => {
                if self.brack_level > 0 {
                    self.brack_level -= 1;
                } else {
                    return Err(DelimiterCheckerError::UnexpectedClosingBracket(&token));
                }
            }
            TokenData::OpenCurly => {
                if self.curly_level > 0 {
                    self.curly_level -= 1;
                } else {
                    return Err(DelimiterCheckerError::UnexpectedClosingCurlyBrace(&token));
                }
            }

            _ => {}
        }
        
        Ok(())
    }
}
