use super::View;

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
    Middle,
}
#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: u32,
    y: u32,
}
impl Position {
    fn _new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn new((x, y): (u32, u32)) -> Self {
        Self::_new(x, y)
    }
    pub fn default() -> Self {
        Self::_new(0, 0)
    }

    pub fn move_xy(&self, (x, y): (u32, u32)) -> Self {
        Self::_new(x, y)
    }
    pub fn move_x(&self, x: u32) -> Self {
        let (_, y) = self.into();
        Self::_new(x, y)
    }
    pub fn move_y(&self, y: u32) -> Self {
        let (x, _) = self.into();
        Self::_new(x, y)
    }

    pub fn left(&self, step: u32) -> Self {
        let (x, y) = self.into();
        let step = std::cmp::min(u32::MIN + x, step);
        Self::_new(x - step, y)
    }
    pub fn right(&self, step: u32) -> Self {
        let (x, y) = self.into();
        let step = std::cmp::min(u32::MAX - x, step);
        Self::_new(x + step, y)
    }
    pub fn up(&self, step: u32) -> Self {
        let (x, y) = self.into();
        let step = std::cmp::min(u32::MIN + y, step);
        Self::_new(x, y - step)
    }
    pub fn down(&self, step: u32) -> Self {
        let (x, y) = self.into();
        let step = std::cmp::min(u32::MAX - y, step);
        Self::_new(x, y + step)
    }

    pub fn put_in(&self, view: &View) -> Self {
        let (x, y) = self.into();
        let (org_x, org_y, w, h) = view.into();
        let x = std::cmp::max(x, org_x);
        let x = std::cmp::min(x, org_x + (w - 1));
        let y = std::cmp::max(y, org_y);
        let y = std::cmp::min(y, org_y + (h - 1));
        Self::_new(x, y)
    }

    pub fn app_road_with_fold<F, T>(&self, road: &Vec<Direction>, init: T, f: F) -> (Position, T)
    where
        F: Fn((u32, u32), T) -> T,
    {
        road.iter()
            .fold((self.clone(), init), |(pos, item), dir| -> (Position, T) {
                let pos = match dir {
                    Direction::Up => pos.up(1),
                    Direction::UpperLeft => pos.up(1).left(1),
                    Direction::UpperRight => pos.up(1).right(1),
                    Direction::Down => pos.down(1),
                    Direction::LowerLeft => pos.down(1).left(1),
                    Direction::LowerRight => pos.down(1).right(1),
                    Direction::Left => pos.left(1),
                    Direction::Right => pos.right(1),
                    Direction::Middle => pos,
                };
                (pos, f(pos.into(), item))
            })
    }
    pub fn app_road(&self, road: &Vec<Direction>) -> Position {
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
                Direction::Middle => pos,
            }
        })
    }

    pub fn get_direction(&self, target: &Position) -> Direction {
        let (src_x, src_y) = self.into();
        let (dst_x, dst_y) = target.into();
        if src_x > dst_x && src_y < dst_y {
            Direction::LowerLeft
        } else if src_x > dst_x && src_y > dst_y {
            Direction::UpperLeft
        } else if src_x > dst_x && src_y == dst_y {
            Direction::Left
        } else if src_x < dst_x && src_y < dst_y {
            Direction::LowerRight
        } else if src_x < dst_x && src_y > dst_y {
            Direction::UpperRight
        } else if src_x < dst_x && src_y == dst_y {
            Direction::Right
        } else if src_x == dst_x && src_y < dst_y {
            Direction::Down
        } else if src_x == dst_x && src_y > dst_y {
            Direction::Up
        } else {
            Direction::Middle
        }
    }
}

impl From<Position> for (u32, u32) {
    fn from(item: Position) -> (u32, u32) {
        (item.x, item.y)
    }
}
impl From<&Position> for (u32, u32) {
    fn from(item: &Position) -> (u32, u32) {
        (item.x, item.y)
    }
}
