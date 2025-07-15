use std::fs::File;
use std::os::unix::io::AsRawFd;
use libc::{cfmakeraw, tcgetattr, tcsetattr, termios, TCSANOW};
use std::io::{self, Read, Write, Stdout};

enum Mode {
    NORMAL,
    INSERT
}

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

struct Pos {
    row: usize,
    col: usize
}

struct InnerBehavior <'a> {
    mode: Mode,
    content: Vec<Vec<char>>,
    pos: Pos,
    stdin: &'a mut File,
    stdout: &'a mut Stdout,
}

impl <'a> InnerBehavior <'a> {

    pub fn new(stdin: &'a mut File, stdout: &'a mut Stdout) -> Self {
        return InnerBehavior { mode: Mode::NORMAL, content: Vec::from([Vec::new()]),
            pos: Pos {row: 1, col: 1 }, stdin: stdin, stdout: stdout }
    }
    
    fn handle_char_normal(&mut self, last_chars: &String) -> bool {
        let last: &str = last_chars;
        // TODO: need to check time if consecutive chars but not same moment
        // TODO: need to remove from page content if command (before executing the command)
        // TODO: how to do if there are several commands with same starts like j and jk?
        // TODO handle when text does not fit screen
        return match last {
            name if name.ends_with("k") && self.pos.row > 1 => {
                write!(self.stdout, "\x1B[A").unwrap();
                self.pos.row -= 1;
                false
            },
            name if name.ends_with("j") && self.pos.row < self.content.len() => {
                write!(self.stdout, "\x1B[B").unwrap();
                self.pos.row += 1;
                false
            },
            name if name.ends_with("l") && self.pos.col < self.content.get(self.pos.row - 1).unwrap().len() => {
                write!(self.stdout, "\x1B[C").unwrap();
                self.pos.col += 1;
                false
            },
            name if name.ends_with("h") && self.pos.col > 1 => {
                write!(self.stdout, "\x1B[D").unwrap();
                self.pos.col -= 1;
                false
            },
            name if name.ends_with("0") => {
                write!(self.stdout, "\x1B[G").unwrap();
                self.pos.col = 0;
                false
            }
            name if name.ends_with("gg") => {
                write!(self.stdout, "\x1B[1;1H").unwrap();
                self.pos.row = 1;
                false
            },
            name if name.ends_with("$") => {
                self.pos.col = self.content.get(self.pos.row - 1).unwrap().len();
                write!(self.stdout, "\x1B[{}G", self.pos.col).unwrap();
                false
            },
            name if name.ends_with("G") => {
                self.pos.row = self.content.len();
                write!(self.stdout, "\x1B[{};1H", self.pos.row).unwrap();
                false
            },
            name if name.ends_with("i") => {
                self.mode = Mode::INSERT;
                false
            },
            _ => {
                false
            }
        }
    }

    fn handle_char_insert(&mut self, last_chars: &String) -> bool {
        let last: &str = last_chars;
        return match last {
            // Escape character
            name if name.ends_with("\x1B") => {
                self.mode = Mode::NORMAL;
                false
            },
            // Backspace
            // name if name.ends_with("\x1B[D\x1B[P") && self.pos.col > 1 => {
            name if name.ends_with("\x7F") => {
                // Condition MUST be inside the match.
                // If this condition is not met, will enter the default case
                // which should not happen
                if self.pos.col > 1 {
                    self.content.get_mut(self.pos.row - 1).unwrap().remove(self.pos.col - 2);
                    self.pos.col -= 1;
                    return true
                }
                // write!(self.stdout, "ouiiiiiiii");
                // write!(self.stdout, "{:?}", self.content).unwrap();
                false
            }
            // name if name.ends_with("\n") => {
            name if name.ends_with("\0x44") || name.ends_with("\r") || name.ends_with("\n") => {
                self.content.get_mut(self.pos.row - 1).unwrap().insert(self.pos.col - 1, '\n');
                // self.content.insert(self.pos.row, Vec::from([Vec::new()]));
                self.content.insert(self.pos.row, Vec::new());
                self.pos.row += 1;
                self.pos.col = 1;
                // write!(self.stdout, "\x1B[E").unwrap();
                write!(self.stdout, "ici").unwrap();
                true
            },
            _ => {
                // write!(self.stdout, "ici {:X}", last_chars.chars().last().unwrap() as i32).unwrap();
                self.content.get_mut(self.pos.row - 1).unwrap().insert(self.pos.col - 1, last.chars().last().unwrap());
                self.pos.col += 1;
                true
            }
        }
    }

    fn handle_char(&mut self, last_chars: &String) -> bool {
        // write!(self.stdout, "{:?}", self.content).unwrap();
        // write!(self.stdout, "{:X}", last_chars.chars().last().unwrap() as u32).unwrap();
        return match self.mode {
            Mode::NORMAL => self.handle_char_normal(last_chars),
            Mode::INSERT => self.handle_char_insert(last_chars)
        }
    }

    fn launch_event_loop(&mut self) {
        let mut past_chars = String::from("");
        loop {
            let mut buffer = [0u8; 1];
            self.stdin.read_exact(&mut buffer).unwrap();
            let byte = buffer[0];

            past_chars.push(byte as char);
            let rerender = self.handle_char(&past_chars);
            let full_content: String = self.content.iter().flatten().collect();
            // Clear screen
            // write!(self.stdout, "\x1B[2J").unwrap();
            // write!(self.stdout, "\x1B[s\x1B[2J\x1B[H{}\x1B[u", full_content).unwrap();
            if rerender {
                // write!(self.stdout, "\x1B[s\x1B[2J{}\x1B[u", full_content).unwrap();
                // write!(self.stdout, "\x1B[2J{}\x1B[{};{}H", full_content, self.pos.row, self.pos.col).unwrap();
                write!(self.stdout, "\x1B[2J\x1B[H{}\x1B[{};{}H", full_content, self.pos.row, self.pos.col).unwrap();
                // write!(self.stdout, "{}\x1B[{};{}H", full_content, self.pos.row, self.pos.col).unwrap();
                // write!(self.stdout, "\n\n{:?}", self.content).unwrap();
            }
            self.stdout.flush().unwrap();
            // write!(self.stdout, "{:X}", past_chars.chars().last().unwrap() as i32).unwrap();

            if byte == b'q' {
                break;
            }
        }
    }
}


fn main() {
let original_mode = enable_raw_mode();
    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut stdout = io::stdout();

    start_tui(&mut stdout);

    InnerBehavior::new(&mut stdin, &mut stdout).launch_event_loop();

    quit_restore_mode(original_mode, &mut stdout);
    writeln!(stdout, "Exited.").unwrap();
}
