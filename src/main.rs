use libc::{self};
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let _raw_mode = unsafe { TerminalRawMode::enable() };

    println!("Press ctrl-q to quit");

    loop {
        editor_refresh_screen();
        editor_process_keypress()?;
    }
}

fn ctrl_key(ch: char) -> char {
    (ch as u8 & 0x1f) as char
}

fn editor_refresh_screen() {
    print!("\x1b[2J");
}

fn editor_process_keypress() -> Result<(), std::io::Error> {
    let c = editor_read_key()?;

    if c == ctrl_key('q') {
        std::process::exit(0);
    } else {
        println!("\n\rkey: {}", c);
    }
    Ok(())
}

fn editor_read_key() -> Result<char, std::io::Error> {
    let mut stdin = std::io::stdin();
    let mut buffer = [0; 1];

    loop {
        if let Ok(_) = stdin.read_exact(&mut buffer) {
            break;
        }
    }
    Ok(buffer[0] as char)
}

struct TerminalRawMode {
    original_state: libc::termios,
}

impl Drop for TerminalRawMode {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &self.original_state);
        }
    }
}

impl TerminalRawMode {
    unsafe fn enable() -> Self {
        let mut term_old: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut term_old);

        let mut term_new = term_old;

        term_new.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        term_new.c_oflag &= !(libc::OPOST);
        term_new.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        term_new.c_cflag |= libc::CS8;

        term_new.c_cc[libc::VMIN] = 0;
        term_new.c_cc[libc::VTIME] = 1;

        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term_new);

        Self {
            original_state: term_old,
        }
    }
}
