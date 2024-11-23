use libc::{self, OLD_TIME};
use std::{
    io::{stdin, Read},
    sync::Once,
};

struct TerminalRawMode {
    original_state: libc::termios,
}

impl TerminalRawMode {
    unsafe fn enable() -> Self {
        let mut old: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut old);

        let mut new = old;

        new.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | IXON);
        new.c_oflag &= !(libc::OPOST);
        new.c_cflag |= libc::CS8;
        new.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);

        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &new);

        Self {
            original_state: old,
        }
    }
}

fn main() {
    let _raw_mode = unsafe { TerminalRawMode::enable() };
    let mut stdin = std::io::stdin();
    let mut buffer = [0; 1];

    while stdin.read_exact(&mut buffer).is_ok() {
        let c = buffer[0] as char;
        if c == 'q' {
            break;
        }

        if c.is_control() {
            // For control characters, just print the numeric value
            println!("{}\r", buffer[0]);
        } else {
            // For regular characters, print both value and character
            println!("{} ('{}')\r", buffer[0], c);
        }
    }
}
