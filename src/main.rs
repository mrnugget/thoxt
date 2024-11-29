use libc::{self};
use std::io::{self, Read, Write as _};

fn main() -> Result<(), io::Error> {
    let config = EditorConfig::new();
    let editor = Editor { _config: config };

    println!("Press ctrl-q to quit");

    loop {
        editor.refresh_screen();
        let quit = editor.process_keypress()?;
        if quit {
            break;
        };
    }

    editor.clear_screen();
    Ok(())
}

fn ctrl_key(ch: char) -> char {
    (ch as u8 & 0x1f) as char
}

struct Editor {
    _config: EditorConfig,
}

impl Editor {
    fn draw_rows(&self) {
        for _ in 0..24 {
            print!("~\r\n");
        }
    }

    fn clear_screen(&self) {
        print!("\x1b[2J");
        print!("\x1b[H");
        std::io::stdout().flush().unwrap();
    }

    fn refresh_screen(&self) {
        self.clear_screen();
        self.draw_rows();

        print!("\x1b[H");
    }

    fn process_keypress(&self) -> Result<bool, std::io::Error> {
        let c = self.read_key()?;

        if c == ctrl_key('q') {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn read_key(&self) -> Result<char, std::io::Error> {
        let mut stdin = std::io::stdin();
        let mut buffer = [0; 1];

        loop {
            if let Ok(_) = stdin.read_exact(&mut buffer) {
                break;
            }
        }
        Ok(buffer[0] as char)
    }
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
