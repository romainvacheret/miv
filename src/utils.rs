
#[derive(Debug)]
pub struct Pos {
    pub row: usize,
    pub col: usize
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        return Pos { row: row, col: col };
    }
}
