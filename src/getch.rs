pub mod tui {
    fn getch() -> u8 {
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
}
