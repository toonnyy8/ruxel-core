use io::BufRead;
use regex;
use rhai::RegisterFn;
use rhai::{Engine, Module, Scope};
use serde_json;
use std::io::{self, Write};
use std::process;

mod command;
mod unit;
use unit::*;
mod tui;

fn default_render(color: tui::Color, cursor: Position) {
    println!("{}", tui::pixel_bottom(color));
    println!("x:{},y:{}", cursor.x, cursor.y);
}

fn main() {
    let config = command::load_json(".ruxel/config.json");

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

    let mut scope = Scope::new();
    scope
        .push("color", tui::Color::default())
        .push("cursor", Position::default())
        .push("cmd", "".to_string());

    let mut engine = Engine::new();
    engine
        .on_print(|x| print!("{}", x))
        .register_fn("exit_fn", || {
            process::exit(0);
        })
        .register_fn("as_rgb", as_rgb)
        .register_type::<tui::Color>()
        .register_fn("color_new", tui::Color::new)
        .register_type::<Position>()
        .register_fn("pos_new", Position::new)
        .register_fn("as_pos", as_pos)
        .register_fn("+", |a: Position, b: Position| a + b)
        .register_fn("move_to", move_to)
        .register_fn("default_render", default_render);

    let cmd_ast_vec = cmd_content_vec
        .iter()
        .map(|cmd_content| {
            engine
                .compile(cmd_content["script"].as_str().unwrap())
                .unwrap()
        })
        .collect::<Vec<_>>();

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

    for path in config["init"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let ast = engine.compile_file(path.into()).unwrap();
        let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine).unwrap();
        engine.register_global_module(module.into());
    }

    for render_ast in &render_ast_vec {
        let _ = engine.eval_ast_with_scope::<()>(&mut scope, render_ast);
    }
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let cmd = trim_newline(line.unwrap());

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
