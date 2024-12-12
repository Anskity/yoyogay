use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct TextRange {
    pub start: TextPos,
    pub end: TextPos,
}

#[derive(Debug, Clone)]
pub struct BorrowedTextRange<'a> {
    pub start: &'a TextPos,
    pub end: &'a TextPos,
}

impl TextRange {
    pub fn new_empty() -> Self {
        TextRange {
            start: TextPos::new_empty(),
            end: TextPos::new_empty(),
        }
    }
}

impl<'a> From<(&BorrowedTextRange<'a>, &BorrowedTextRange<'a>)> for BorrowedTextRange<'a> {
    fn from(value: (&BorrowedTextRange<'a>, &BorrowedTextRange<'a>)) -> BorrowedTextRange<'a> {
        let (start, end) = value;
        BorrowedTextRange {
            start: &start.start,
            end: &end.end,
        }
    }
}

impl<'a> From<&'a TextRange> for BorrowedTextRange<'a> {
    fn from(value: &'a TextRange) -> Self {
        BorrowedTextRange {
            start: &value.start,
            end: &value.end,
        }
    }
}

impl<'a> From<&'a [Token]> for BorrowedTextRange<'a> {
    fn from(value: &'a [Token]) -> Self {
        BorrowedTextRange {
            start: &value[0].text_range.start,
            end: &value.last().unwrap().text_range.end,
        }
    }
}

impl<'a> From<&'a Token> for BorrowedTextRange<'a> {
    fn from(value: &'a Token) -> Self {
        BorrowedTextRange {
            start: &value.text_range.start,
            end: &value.text_range.end,
        }
    }
}

impl<'a, T:Into<BorrowedTextRange<'a>>> From<T> for TextRange {
    fn from(value: T) -> Self {
        let borrowed: BorrowedTextRange<'a> = value.into();

        TextRange {
            start: borrowed.start.clone(),
            end: borrowed.end.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextPos {
    pub line: usize,
    pub pos: usize,
}

impl TextPos {
    pub fn new_empty() -> Self {
        TextPos {
            line: 1,
            pos: 0,
        }
    }
}
