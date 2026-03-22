pub mod expression;

pub struct Span {
    pub line: usize, 
    pub col: usize, 
}

pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}
