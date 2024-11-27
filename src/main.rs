use libc::{self};
use std::io::{self, Read, Write};

fn main() -> Result<(), io::Error> {
    let _raw_mode = unsafe { EditorConfig::new() };

    println!("Press ctrl-q to quit");

    loop {
        editor_refresh_screen();
        editor_process_keypress()?;
    }
}

fn ctrl_key(ch: char) -> char {
    (ch as u8 & 0x1f) as char
}

fn editor_draw_rows() {
    for _ in 0..24 {
        print!("~\r\n");
    }
}

fn editor_refresh_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");

    editor_draw_rows();

    print!("\x1b[H");
}

fn editor_process_keypress() -> Result<(), std::io::Error> {
    let c = editor_read_key()?;

    if c == ctrl_key('q') {
        editor_refresh_screen();
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

struct EditorConfig {
    original_termios: libc::termios,
}

impl Drop for EditorConfig {
    fn drop(&mut self) {
        unsafe {
            set_termios(&self.original_termios);
        }
    }
}

impl EditorConfig {
    fn new() -> Self {
        let original_termios = unsafe { enable_raw_mode() };

        Self { original_termios }
    }
}

unsafe fn enable_raw_mode() -> libc::termios {
    let mut term_old: libc::termios = std::mem::zeroed();
    get_termios(&mut term_old);

    let mut term_new = term_old;

    term_new.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
    term_new.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
    term_new.c_cflag |= libc::CS8;

    // TODO: This kills terminal output after exiting the editor
    // term_new.c_oflag &= !(libc::OPOST);

    term_new.c_cc[libc::VMIN] = 0;
    term_new.c_cc[libc::VTIME] = 1;

    set_termios(&term_new);

    term_old
}

unsafe fn get_termios(termios: &mut libc::termios) {
    libc::tcgetattr(libc::STDIN_FILENO, termios);
}

unsafe fn set_termios(termios: &libc::termios) {
    libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, termios);
}
