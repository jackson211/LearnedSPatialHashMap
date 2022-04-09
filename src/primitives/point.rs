#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub id: usize,
    pub x: T,
    pub y: T,
}
