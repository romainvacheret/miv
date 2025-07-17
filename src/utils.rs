pub enum Mode {
    NORMAL,
    INSERT
}

impl Mode {
    pub fn to_text(&self) -> &str {
        return match self {
            Mode::NORMAL => "NORMAL",
            Mode::INSERT => "INSERT"
        }
    }
}

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
