use io::BufRead;
use regex;
use rhai::RegisterFn; // use 'RegisterFn' trait for 'register_fn'
use rhai::RegisterResultFn;
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Module, Scope};
use serde_json;
use std::i64;
use std::io::{self, Write};
use std::process;

mod tui {
    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    impl Color {
        pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self { r, g, b, a }
        }
        pub fn default() -> Self {
            Self {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }
        }
    }

    pub fn pixel(upper: Color, lower: Color) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2580}\x1b[0m",
            upper.r, upper.g, upper.b, lower.r, lower.g, lower.b
        )
    }
    pub fn pixel_bottom(upper: Color) -> String {
        format!(
            "\x1B[38;2;{};{};{}m\u{2580}\x1B[0m",
            upper.r, upper.g, upper.b
        )
    }
}

mod command;

fn trim_newline(s: String) -> String {
    let mut trim_s = s.clone();
    if trim_s.ends_with('\n') {
        trim_s.pop();
        if trim_s.ends_with('\r') {
            trim_s.pop();
        }
    }
    trim_s
}

fn cmd_to_script(cmd: &str, script: &str) -> String {
    format!(r#"let cmd = "{}";{};"#, cmd, script,)
}

fn rgb(cmd: String) -> tui::Color {
    let red = u8::from_str_radix(&cmd[1..3], 16).unwrap();
    let green = u8::from_str_radix(&cmd[3..5], 16).unwrap();
    let blue = u8::from_str_radix(&cmd[5..], 16).unwrap();
    let color = tui::Color::new(red, green, blue, 0);
    // println!("{}", tui::pixel_bottom(color));
    color
}

fn main() {
    let config = command::load_json(".ruxel/config.json");
    let mut cmds_content: Vec<serde_json::Value> = Vec::new();
    for path in config["cmds_content"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let mut _cmds_content = command::load_json(path);
        let mut _cmds_content = _cmds_content.as_array_mut().unwrap();
        cmds_content.append(_cmds_content);
    }

    let reg_vec = cmds_content
        .iter()
        .map(|cmd_content| {
            let reg_str = cmd_content["regex"].as_str().unwrap();
            let reg = regex::Regex::new(reg_str).unwrap();
            return reg;
        })
        .collect::<Vec<_>>();

    let mut scope = Scope::new();
    scope.push("color", tui::Color::default());

    let mut engine = Engine::new();
    engine
        .register_fn("exit_fn", || {
            process::exit(0);
        })
        .register_fn("rgb_fn", rgb)
        .register_type::<tui::Color>()
        .register_fn("color_new", tui::Color::new);

    for path in config["init"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let path = &format!(".ruxel/{}", path);
        let ast = engine.compile_file(path.into()).unwrap();
        let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine).unwrap();
        engine.register_global_module(module.into());
    }

    let mut color = tui::Color::default();

    let stdin = io::stdin();

    print!(":");
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        let cmd = trim_newline(line.unwrap());

        for idx in 0..reg_vec.len() {
            if reg_vec[idx].is_match(&cmd) {
                engine
                    .eval_with_scope::<()>(
                        &mut scope,
                        &cmd_to_script(&cmd, cmds_content[idx]["script"].as_str().unwrap()),
                    )
                    .unwrap();
                break;
            }
        }
        color = engine
            .eval_with_scope::<tui::Color>(&mut scope, "color")
            .unwrap();
        println!("{}", tui::pixel_bottom(color));
        print!(":");
        io::stdout().flush().unwrap();
    }
}
