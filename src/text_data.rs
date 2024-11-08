pub struct TextRange {
    pub start: TextPos,
    pub end: TextPos,
}

pub struct TextPos {
    pub line: usize,
    pub pos: usize,
}
