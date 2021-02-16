#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    UpperLeft,
    UpperRight,
    Down,
    LowerLeft,
    LowerRight,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy)]
pub struct Position {
    upper_bound: (u32, u32), // x, y
    lower_bound: (u32, u32), // x, y
    x: u32,
    y: u32,
}
impl Position {
    pub fn new(
        upper_bound: (u32, u32), // x, y
        lower_bound: (u32, u32), // x, y
        x: u32,
        y: u32,
    ) -> Self {
        Self {
            upper_bound,
            lower_bound,
            x,
            y,
        }
    }
    pub fn move_xy(&self, x: u32, y: u32) -> Self {
        Position::new(self.upper_bound, self.lower_bound, x, y)
    }
    pub fn move_x(&self, x: u32) -> Self {
        self.move_xy(x, self.y)
    }
    pub fn move_y(&self, y: u32) -> Self {
        self.move_xy(self.x, y)
    }
    pub fn left(&self, step: u32) -> Self {
        let y = self.y;
        let x = (self.x as i64 - step as i64);
        let x = if x < self.lower_bound.0 as i64 {
            self.lower_bound.0
        } else {
            x as u32
        };
        self.move_xy(x, y)
    }
    pub fn right(&self, step: u32) -> Self {
        let y = self.y;
        let x = (self.x as i64 + step as i64);
        let x = if x > self.upper_bound.0 as i64 {
            self.upper_bound.0
        } else {
            x as u32
        };
        self.move_xy(x, y)
    }
    pub fn up(&self, step: u32) -> Self {
        let x = self.x;
        let y = (self.y as i64 - step as i64);
        let y = if y < self.lower_bound.1 as i64 {
            self.lower_bound.1
        } else {
            y as u32
        };
        self.move_xy(x, y)
    }
    pub fn down(&self, step: u32) -> Self {
        let x = self.x;
        let y = (self.y as i64 + step as i64);
        let y = if y > self.upper_bound.1 as i64 {
            self.upper_bound.1
        } else {
            y as u32
        };
        self.move_xy(x, y)
    }
    pub fn app_roads(&self, road: Vec<Direction>) -> Position {
        road.iter().fold(self.clone(), |pos, dir| -> Position {
            match dir {
                Direction::Up => pos.up(1),
                Direction::UpperLeft => pos.up(1).left(1),
                Direction::UpperRight => pos.up(1).right(1),
                Direction::Down => pos.down(1),
                Direction::LowerLeft => pos.down(1).left(1),
                Direction::LowerRight => pos.down(1).right(1),
                Direction::Left => pos.left(1),
                Direction::Right => pos.right(1),
            }
        })
    }
}

impl From<Position> for (u32, u32) {
    fn from(item: Position) -> (u32, u32) {
        (item.x, item.y)
    }
}
