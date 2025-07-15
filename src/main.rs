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

fn main() {
let original_mode = enable_raw_mode();
    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut stdout = io::stdout();

    start_tui(&mut stdout);
    writeln!(stdout, "Start typing (press 'q' to quit):").unwrap();
    stdout.flush().unwrap();

    loop {
        let mut buffer = [0u8; 1];
        stdin.read_exact(&mut buffer).unwrap();
        let byte = buffer[0];

        write!(stdout, "Just typed: {}\r\n", byte as char).unwrap();
        stdout.flush().unwrap();

        if byte == b'q' {
            break;
        }
    }

    quit_restore_mode(original_mode, &mut stdout);
    writeln!(stdout, "Exited.").unwrap();
}
