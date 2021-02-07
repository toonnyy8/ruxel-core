use io::BufRead;
use regex;
use rhai::{Engine, Module, Scope};
use rhai::{RegisterFn, RegisterResultFn};
use serde_json;
use std::io::{self, Write};
use std::process;

mod command;
mod tui;
mod unit;

fn default_render(canvas: tui::Canvas, color: tui::Color, cursor: unit::Position) {
    for y in (0..canvas.size.y).step_by(2) {
        let y = y as usize;
        if y != (canvas.size.y - 1) as usize {
            for x in 0..canvas.size.x {
                let x = x as usize;
                let pix_t = if cursor.x == x as i64 || cursor.y == y as i64 {
                    if cursor.x > x as i64 || cursor.y > y as i64 {
                        tui::Color::mix(
                            tui::Color::mix(color, canvas.data[y][x]),
                            canvas.data[y][x],
                        )
                    } else if cursor.x == x as i64 && cursor.y == y as i64 {
                        tui::Color::mix(
                            tui::Color::mix(color, canvas.data[y][x]),
                            canvas.data[y][x],
                        )
                    } else {
                        canvas.data[y][x]
                    }
                } else {
                    canvas.data[y][x]
                };
                let pix_b = if cursor.x == x as i64 || cursor.y == (y + 1) as i64 {
                    if cursor.x > x as i64 || cursor.y > (y + 1) as i64 {
                        tui::Color::mix(
                            tui::Color::mix(color, canvas.data[y + 1][x]),
                            canvas.data[y + 1][x],
                        )
                    } else if cursor.x == x as i64 && cursor.y == (y + 1) as i64 {
                        tui::Color::mix(
                            tui::Color::mix(color, canvas.data[y + 1][x]),
                            canvas.data[y + 1][x],
                        )
                    } else {
                        canvas.data[y + 1][x]
                    }
                } else {
                    canvas.data[y + 1][x]
                };
                print!("{}", tui::pixel(pix_t, pix_b));
            }
        } else {
            for x in 0..canvas.size.x {
                let x = x as usize;
                // let pix_t = canvas.data[y][x];

                let pix_t = if cursor.x == x as i64 || cursor.y == y as i64 {
                    if cursor.x > x as i64 || cursor.y > y as i64 {
                        tui::Color::mix(
                            tui::Color::mix(color, canvas.data[y][x]),
                            canvas.data[y][x],
                        )
                    } else if cursor.x == x as i64 && cursor.y == y as i64 {
                        tui::Color::mix(
                            tui::Color::mix(color, canvas.data[y][x]),
                            canvas.data[y][x],
                        )
                    } else {
                        canvas.data[y][x]
                    }
                } else {
                    canvas.data[y][x]
                };
                print!("{}", tui::pixel_bottom(pix_t));
            }
        }
        println!("");
    }
    println!("{}", tui::pixel_bottom(color));
    println!("x:{},y:{}", cursor.x, cursor.y);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut scope = Scope::new();
    scope
        .push("color", tui::Color::new(255, 255, 255, 255))
        .push("cursor", unit::Position::default())
        .push("cmd", "".to_string())
        .push(
            "canvas",
            tui::Canvas::new(command::move_to(
                unit::Position::default(),
                match args.get(1) {
                    Some(cmd) => cmd.to_string(),
                    None => "x11y11".to_string(),
                },
            )),
        );

    let mut engine = Engine::new();
    engine
        .on_print(|x| print!("{}", x))
        .register_fn("exit_fn", || {
            process::exit(0);
        })
        .register_fn("as_rgb", command::as_rgb)
        .register_type::<tui::Color>()
        .register_fn("color_new", tui::Color::new)
        .register_type::<unit::Position>()
        .register_fn("pos_new", unit::Position::new)
        .register_fn("as_pos", command::as_pos)
        .register_fn("+", |a: unit::Position, b: unit::Position| a + b)
        .register_fn("move_to", command::move_to)
        .register_fn("default_render", default_render)
        .register_type::<tui::Canvas>()
        .register_fn("draw", command::draw);

    let config = command::load_json(".ruxel/config.json");

    for path in config["init"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let ast = engine.compile_file(path.into()).unwrap();
        let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine).unwrap();
        engine.register_global_module(module.into());
    }

    let render_ast_vec = config["render"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| {
            let path = path.as_str().unwrap();
            let path = &format!(".ruxel/{}", path);
            engine.compile_file(path.into()).unwrap()
        })
        .collect::<Vec<_>>();

    let mut cmd_content_vec: Vec<serde_json::Value> = Vec::new();
    for path in config["cmds_content"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let mut _cmd_content_vec = command::load_json(path);
        let mut _cmd_content_vec = _cmd_content_vec.as_array_mut().unwrap();
        cmd_content_vec.append(_cmd_content_vec);
    }

    let cmd_reg_vec = cmd_content_vec
        .iter()
        .map(|cmd_content| {
            let reg_str = cmd_content["regex"].as_str().unwrap();
            let reg = regex::Regex::new(reg_str).unwrap();
            return reg;
        })
        .collect::<Vec<_>>();

    let cmd_ast_vec = cmd_content_vec
        .iter()
        .map(|cmd_content| {
            engine
                .compile(cmd_content["script"].as_str().unwrap())
                .unwrap()
        })
        .collect::<Vec<_>>();

    for render_ast in &render_ast_vec {
        let _ = engine.eval_ast_with_scope::<()>(&mut scope, render_ast);
    }
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let cmd = unit::trim_newline(line.unwrap());

        for idx in 0..cmd_reg_vec.len() {
            if cmd_reg_vec[idx].is_match(&cmd) {
                scope.set_value("cmd", cmd);
                let _ = engine.eval_ast_with_scope::<()>(&mut scope, &cmd_ast_vec[idx]);
                break;
            }
        }

        for render_ast in &render_ast_vec {
            let _ = engine.eval_ast_with_scope::<()>(&mut scope, render_ast);
        }
        io::stdout().flush().unwrap();
    }
}
