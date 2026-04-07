#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub line: usize, 
    pub col: usize, 
}
