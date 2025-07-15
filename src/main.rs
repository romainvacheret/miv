use std::fs::File;
use std::os::unix::io::AsRawFd;
use libc::{termios, tcgetattr, cfmakeraw, tcsetattr, TCSANOW};
use std::io::{self, Read, Write, Stdout};


fn start_tui(stdout: &mut Stdout) {
    // Switch to alternate screen
    write!(stdout, "\x1b[?1049h").unwrap();
    // Clear screen and move to top
    write!(stdout, "\x1b[2J\x1b[H").unwrap();
    stdout.flush().unwrap()
}

/// Generated
fn enable_raw_mode() -> termios {
    let fd = io::stdin().as_raw_fd();
    let mut termios = unsafe {
        let mut t = std::mem::MaybeUninit::<termios>::uninit();
        tcgetattr(fd, t.as_mut_ptr());
        t.assume_init()
    };

    let original = termios;
    unsafe {
        cfmakeraw(&mut termios);
        tcsetattr(fd, TCSANOW, &termios);
    }

    return original;
}

fn quit_restore_mode(config: termios, stdout: &mut Stdout) {
    let fd = io::stdin().as_raw_fd();
    unsafe {
        tcsetattr(fd, TCSANOW, &config);
    }
    // Disable alternate screen
    write!(stdout, "\x1b[?1049l").unwrap();
}

fn handle_char_normal(last_chars: &String, stdout: &mut Stdout, mode: &mut Mode) {
    let last: &str = last_chars;
    // TODO: need to check time if consecutive chars but not same moment
    // TODO: need to remove from page content if command (before executing the command)
    // TODO: how to do if there are several commands with same starts like j and jk?
    match last {
        name if name.ends_with("k") => write!(stdout, "\x1B[A").unwrap(),
        name if name.ends_with("j") => write!(stdout, "\x1B[B").unwrap(),
        name if name.ends_with("l") => write!(stdout, "\x1B[C").unwrap(),
        name if name.ends_with("h") => write!(stdout, "\x1B[D").unwrap(),
        name if name.ends_with("0") => write!(stdout, "\x1B[G").unwrap(),
        name if name.ends_with("gg") => write!(stdout, "\x1B[1;1H").unwrap(),
        name if name.ends_with("i") => *mode = Mode::INSERT,
        _ => ()
    }
}

fn handle_char_insert(last_chars: &String, stdout: &mut Stdout, mode: &mut Mode) {
    let last: &str = last_chars;
    match last {
        // Escape character
        name if name.ends_with("\x1B") => *mode = Mode::NORMAL,
        _ => {
            write!(stdout, "Just typed: {}\r\n", last_chars.chars().last().unwrap()).unwrap();
        }
    }
}

fn handle_char(last_chars: &String, stdout: &mut Stdout, mode: &mut Mode) {
    match mode {
        Mode::NORMAL => handle_char_normal(last_chars, stdout, mode),
        Mode::INSERT => handle_char_insert(last_chars, stdout, mode)
    }
    stdout.flush().unwrap();
}

enum Mode {
    NORMAL,
    INSERT
}

fn main() {
let original_mode = enable_raw_mode();
    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut stdout = io::stdout();
    let mut past_chars = String::from("");
    let mut mode: Mode = Mode::NORMAL;

    start_tui(&mut stdout);
    writeln!(stdout, "Start typing (press 'q' to quit):").unwrap();
    stdout.flush().unwrap();

    loop {
        let mut buffer = [0u8; 1];
        stdin.read_exact(&mut buffer).unwrap();
        let byte = buffer[0];


        past_chars.push(byte as char);
        handle_char(&past_chars, &mut stdout, &mut mode);

        if byte == b'q' {
            break;
        }
    }

    quit_restore_mode(original_mode, &mut stdout);
    writeln!(stdout, "Exited.").unwrap();
}
