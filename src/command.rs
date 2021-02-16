use super::tui;
use super::unit;
use serde_json;
use std::fs;

pub fn load_json(path: &str) -> serde_json::Value {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    serde_json::from_str(&contents).unwrap()
}

pub fn as_rgb(cmd: String) -> tui::Rgba {
    let red = u8::from_str_radix(&cmd[1..3], 16).unwrap();
    let green = u8::from_str_radix(&cmd[3..5], 16).unwrap();
    let blue = u8::from_str_radix(&cmd[5..], 16).unwrap();
    let color = tui::Rgba::new(red, green, blue, 255);
    color
}

pub fn as_pos(cmd: String) -> unit::Position {
    let s = cmd.as_bytes();
    let mut x = 0;
    let mut y = 0;
    let mut step = 0;
    for i in 0..s.len() {
        match s[i] as char {
            'h' => {
                x -= if step == 0 { 1 } else { step };
            }
            'j' => {
                y += if step == 0 { 1 } else { step };
            }
            'k' => {
                y -= if step == 0 { 1 } else { step };
            }
            'l' => {
                x += if step == 0 { 1 } else { step };
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                step = step * 10 + (s[i] - '0' as u8) as i64
            }
            _ => {}
        }
    }
    unit::Position::new(x, y)
}

pub fn move_to(pos: unit::Position, cmd: String) -> unit::Position {
    let s = cmd.as_bytes();
    let mut x = pos.x;
    let mut y = pos.y;
    let mut step = 0;
    for i in 0..s.len() {
        match s[i] as char {
            'x' => {
                x = step;
                step = 0;
            }
            'y' => {
                y = step;
                step = 0;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                step = step * 10 + (s[i] - '0' as u8) as i64;
            }
            _ => {}
        }
    }
    unit::Position::new(x, y)
}

pub fn draw(
    canvas: &mut tui::Canvas,
    cursor: unit::Position,
    color: tui::Rgba,
    cmd: String,
) -> unit::Position {
    let s = cmd.as_bytes();
    let mut x = cursor.x;
    let mut y = cursor.y;
    let mut move_x = 0;
    let mut move_y = 0;
    let mut step = 0;

    for i in 0..s.len() {
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

        match s[i] as char {
            'h' | 'j' | 'k' | 'l' => {
                if step == 0 {
                    x += move_x;
                    y += move_y;

                    if (x < canvas.size.x && 0 <= x) && (y < canvas.size.y && 0 <= y) {
                        canvas.data[y as usize][x as usize] = color;
                    }
                } else {
                    for _ in 0..step {
                        x += move_x;
                        y += move_y;
                        if (x < canvas.size.x && 0 <= x) && (y < canvas.size.y && 0 <= y) {
                            canvas.data[y as usize][x as usize] = color;
                        }
                    }
                    step = 0;
                }
            }
            'd' => {
                if x < canvas.size.x && 0 <= x && y < canvas.size.y && 0 <= y {
                    canvas.data[y as usize][x as usize] = color;
                }
            }
            _ => {}
        }
    }

    unit::Position::new(x, y)
}

pub fn msg_line_num(msg: String) -> i64 {
    let mut line_num = 1;
    for &b in msg.as_bytes() {
        if b as char == '\n' {
            line_num += 1;
        }
    }
    line_num
}
