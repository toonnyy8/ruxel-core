use io::BufRead;
use regex;
use rhai::RegisterFn;
use rhai::{Engine, Module, Scope};
use serde_json;
use std::io::{self, Write};
use std::process;

mod ruxel;
use ruxel::*;

fn default_render(canvas: tui::Canvas, color: tui::Rgba, cursor: unit::Position, up: i64) -> i64 {
    let mut view = tui::Canvas::new(canvas.size);
    let white = tui::Rgba::new(255, 255, 255, 63);
    let black = tui::Rgba::new(0, 0, 0, 63);
    let s = "▀";
    assert_eq!(s, "\u{2580}");

    for y in 0..canvas.size.y {
        let y = y as usize;
        for x in 0..canvas.size.x {
            let x = x as usize;
            view.data[y][x] = if ((x as i64) < cursor.x && (y as i64) == cursor.y)
                || ((x as i64) == cursor.x && (y as i64) < cursor.y)
                || ((x as i64) == cursor.x && (y as i64) == cursor.y && canvas.data[y][x] != color)
            {
                if canvas.data[y][x].lightness() < 128 {
                    white.compositing(&canvas.data[y][x])
                } else {
                    black.compositing(&canvas.data[y][x])
                }
            } else {
                canvas.data[y][x]
            }
        }
    }

    tui::clear_up(up);

    let mut up = 0;
    for y in (0..canvas.size.y).step_by(2) {
        let y = y as usize;
        if y != (canvas.size.y - 1) as usize {
            for x in 0..canvas.size.x {
                let x = x as usize;
                let pix_t = view.data[y][x];
                let pix_b = view.data[y + 1][x];
                print!("{}", tui::pixel_both(pix_t, pix_b, s));
            }
        } else {
            for x in 0..canvas.size.x {
                let x = x as usize;
                let pix_t = view.data[y][x];
                tui::pixel_fg(pix_t, s);
            }
        }
        print!("\n");
        up = up + 1;
    }

    print!(
        "{}|x:{},y:{}\n",
        tui::pixel_fg(color, s),
        cursor.x,
        cursor.y
    );
    return up + 1;
}

fn custom_render(
    canvas: tui::Canvas,
    color: tui::Rgba,
    cursor: unit::Position,
    up: i64,
    s: String,
    mode: String,
) -> i64 {
    let mut view = tui::Canvas::new(canvas.size);
    let white = tui::Rgba::new(255, 255, 255, 63);
    let black = tui::Rgba::new(0, 0, 0, 63);

    for y in 0..canvas.size.y {
        let y = y as usize;
        for x in 0..canvas.size.x {
            let x = x as usize;
            view.data[y][x] = if ((x as i64) < cursor.x && (y as i64) == cursor.y)
                || ((x as i64) == cursor.x && (y as i64) < cursor.y)
                || ((x as i64) == cursor.x && (y as i64) == cursor.y && canvas.data[y][x] != color)
            {
                if canvas.data[y][x].lightness() < 128 {
                    white.compositing(&canvas.data[y][x])
                } else {
                    black.compositing(&canvas.data[y][x])
                }
            } else {
                canvas.data[y][x]
            }
        }
    }

    tui::clear_up(up);

    let mut up = 0;
    for y in 0..canvas.size.y {
        let y = y as usize;
        for x in 0..canvas.size.x {
            let x = x as usize;
            let pix = view.data[y][x];

            print!(
                "{}",
                if mode == "fg" {
                    tui::pixel_fg(pix, &s)
                } else {
                    tui::pixel_bg(pix, &s)
                }
            );
        }
        print!("\n");
        up = up + 1;
    }
    print!(
        "{}|x:{},y:{}\n",
        if mode == "fg" {
            tui::pixel_fg(color, &s)
        } else {
            tui::pixel_bg(color, &s)
        },
        cursor.x,
        cursor.y
    );
    return up + 1;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let __canvas = Canvas_::new(unit::Position::new(10, 10));
    let __canvas = __canvas.update((1, 1), tui::Rgba::new(255, 55, 200, 255));

    let mut scope = Scope::new();
    scope
        .push("color", tui::Rgba::new(255, 255, 255, 255))
        .push("cursor", unit::Position::default())
        .push("cmd", "".to_string())
        .push(
            "canvas",
            tui::Canvas::new(command::move_to(
                unit::Position::default(),
                match args.get(1) {
                    Some(cmd) => cmd.to_string(),
                    None => "11x11y".to_string(),
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
        .register_type::<tui::Rgba>()
        .register_fn("color_new", tui::Rgba::new)
        .register_type::<unit::Position>()
        .register_fn("pos_new", unit::Position::new)
        .register_fn("as_pos", command::as_pos)
        .register_fn("+", |a: unit::Position, b: unit::Position| a + b)
        .register_fn("move_to", command::move_to)
        .register_fn("default_render", default_render)
        .register_fn("custom_render", custom_render)
        .register_type::<tui::Canvas>()
        .register_fn("draw", command::draw)
        .register_fn("msg_line_num", command::msg_line_num)
        .register_fn("clear_up", tui::clear_up)
        .register_fn("clear_down", tui::clear_down)
        .register_fn("save", file::save)
        .register_type::<position::Position>()
        .register_fn("app_road", position::Position::app_road)
        .register_fn("draw_", command::draw_)
        .register_fn("as_road", command::as_road);

    let config = command::load_json(".ruxel/config.json");

    for module in config["modules"].as_array().unwrap() {
        let name = module["name"].as_str().unwrap();
        let path = module["path"].as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let ast = engine.compile_file(path.into()).unwrap();
        let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine).unwrap();
        engine.register_static_module(name, module.into());
    }

    for path in config["init"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let _ = engine.eval_file_with_scope::<()>(&mut scope, path.into());
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
