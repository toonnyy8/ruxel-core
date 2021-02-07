#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

pub fn trim_newline(s: String) -> String {
    let mut trim_s = s.clone();
    if trim_s.ends_with('\n') {
        trim_s.pop();
        if trim_s.ends_with('\r') {
            trim_s.pop();
        }
    }
    trim_s
}
