mod tui {
    pub fn getch() -> u8 {
        use std::io::Read;
        let org_opts = termios::Termios::from_fd(libc::STDIN_FILENO).unwrap();
        let mut new_opts = org_opts.clone(); // make a mutable copy of termios
                                             // that we will modify
        new_opts.c_lflag &= !(termios::ICANON | termios::ECHO); // no echo and canonical mode
        termios::tcsetattr(libc::STDIN_FILENO, termios::TCSANOW, &mut new_opts).unwrap();

        let mut buffer = [0; 1]; // read exactly one byte
        std::io::stdin().read_exact(&mut buffer).unwrap();
        termios::tcsetattr(libc::STDIN_FILENO, termios::TCSANOW, &org_opts).unwrap(); // reset the stdin to
                                                                                      // original termios data
        buffer[0]
    }

    pub struct RGBA {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    pub fn pixel(upper: RGBA, lower: RGBA) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2580}\x1b[0m",
            upper.r, upper.g, upper.b, lower.r, lower.g, lower.b
        )
    }
    pub fn pixel_bottom(upper: RGBA) -> String {
        format!(
            "\x1B[38;2;{};{};{}m\u{2580}\x1B[0m",
            upper.r, upper.g, upper.b
        )
    }
}

fn main() {
    loop {
        let c = tui::getch();
        print!(
            "{}",
            tui::pixel(
                tui::RGBA {
                    r: c,
                    g: 0,
                    b: 0,
                    a: 0
                },
                tui::RGBA {
                    r: 0,
                    g: c,
                    b: 0,
                    a: 0
                }
            )
        );
        println!("{}", c as char);
        if c == 113 {
            break;
        }
    }
}
