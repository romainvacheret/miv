use std::os::unix::io::AsRawFd;
use libc::{cfmakeraw, tcgetattr, tcsetattr, termios, TCSANOW};
use std::io::{self, Write, Stdout};

pub fn start_tui(stdout: &mut Stdout) {
    // Switch to alternate screen
    write!(stdout, "\x1b[?1049h").unwrap();
    // Clear screen and move to top
    write!(stdout, "\x1b[2J\x1b[H").unwrap();
    stdout.flush().unwrap()
}

/// Generated
pub fn enable_raw_mode() -> termios {
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

pub fn quit_restore_mode(config: termios, stdout: &mut Stdout) {
    let fd = io::stdin().as_raw_fd();
    unsafe {
        tcsetattr(fd, TCSANOW, &config);
    }
    // Disable alternate screen
    write!(stdout, "\x1b[?1049l").unwrap();
}

// Generated
#[repr(C)]
#[derive(Debug)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

// Generated
pub fn terminal_size() -> io::Result<(u16, u16)> {
    let fd = io::stdout().as_raw_fd();

    let mut size = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let result = unsafe {
        libc::ioctl(fd, libc::TIOCGWINSZ, &mut size)
    };

    return if result == 0 {
        Ok((size.ws_row, size.ws_col))
    } else {
        Err(io::Error::last_os_error())
    };
}
