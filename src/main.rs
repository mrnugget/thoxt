use libc;
use std::io::{stdin, Read};

unsafe fn enable_raw_mode() {
    let mut old_attributes: libc::termios = std::mem::zeroed();
    libc::tcgetattr(libc::STDIN_FILENO, &mut old_attributes);

    let mut new_attributes = old_attributes;
    new_attributes.c_lflag &= libc::ECHO;
    libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &new_attributes);
}

fn main() {
    unsafe { enable_raw_mode() };
    let mut buf = [0; 1];
    while let Ok(n) = stdin().read(&mut buf) {
        println!("buf[0]: {:?}", buf[0]);
        if n == 1 && buf[0] == 'q' as u8 {
            return;
        }
    }
}
