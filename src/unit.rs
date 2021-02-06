use super::tui;

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

pub fn as_rgb(cmd: String) -> tui::Color {
    let red = u8::from_str_radix(&cmd[1..3], 16).unwrap();
    let green = u8::from_str_radix(&cmd[3..5], 16).unwrap();
    let blue = u8::from_str_radix(&cmd[5..], 16).unwrap();
    let color = tui::Color::new(red, green, blue, 0);
    color
}

pub fn as_pos(cmd: String) -> Position {
    let s = cmd.as_bytes();
    let mut x = 0;
    let mut y = 0;
    let mut move_x = 0;
    let mut move_y = 0;
    let mut step = 0;
    for i in 0..s.len() {
        match s[i] as char {
            'h' | 'j' | 'k' | 'l' | '#' => {
                if step == 0 {
                    x += move_x;
                    y += move_y;
                } else {
                    x += move_x * step;
                    y += move_y * step;
                    step = 0;
                }
            }
            _ => {}
        }

        match s[i] as char {
            'h' => {
                move_x = -1;
                move_y = 0;
            }
            'j' => {
                move_x = 0;
                move_y = 1;
            }
            'k' => {
                move_x = 0;
                move_y = -1;
            }
            'l' => {
                move_x = 1;
                move_y = 0;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                step = step * 10 + (s[i] - '0' as u8) as i64
            }
            _ => {}
        }
    }
    Position::new(x, y)
}

pub fn move_to(cmd: String) -> Position {
    let s = cmd.as_bytes();
    let mut x = 0;
    let mut y = 0;
    enum Setting {
        X,
        Y,
        N,
    }
    let mut setting = Setting::N;
    for i in 0..s.len() {
        match s[i] as char {
            'x' => {
                setting = Setting::X;
            }
            'y' => {
                setting = Setting::Y;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => match setting {
                Setting::X => x = x * 10 + (s[i] - '0' as u8) as i64,
                Setting::Y => y = y * 10 + (s[i] - '0' as u8) as i64,
                Setting::N => {}
            },
            _ => {}
        }
    }
    Position::new(x, y)
}
