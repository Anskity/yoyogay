#[derive(Debug)]
pub struct TextRange {
    pub start: TextPos,
    pub end: TextPos,
}

impl TextRange {
    pub fn new_empty() -> Self {
        TextRange {
            start: TextPos::new_empty(),
            end: TextPos::new_empty(),
        }
    }
}

#[derive(Debug)]
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
