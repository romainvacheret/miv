use std::os::unix::io::AsRawFd;
use libc::{cfmakeraw, tcgetattr, tcsetattr, termios, TCSANOW};
use log::{debug, error};
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
pub struct WinSize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

// Generated
pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    let fd = io::stdout().as_raw_fd();

    let mut size = WinSize {
        ws_row: 0, // nubmer of rows
        ws_col: 0, // number of columns
        ws_xpixel: 0, // width in pixels
        ws_ypixel: 0, // height in pixels
    };

    let result = unsafe {
        libc::ioctl(fd, libc::TIOCGWINSZ, &mut size)
    };


    return if result == 0 {
        debug!("Terminal size is: {:?}", size);
        Ok((size.ws_row, size.ws_col))
    } else {
        error!("Enable to get the terminal size");
        Err(io::Error::last_os_error())
    };
}
