use std::{fs::File, io::{Read, Stdout, Write}};

use crate::{display::{Renderer}, utils::Pos};

enum Mode {
    NORMAL,
    INSERT
}

pub struct InnerBehavior <'a> {
    mode: Mode,
    content: Vec<Vec<char>>,
    pos: Pos,
    stdin: &'a mut File,
    stdout: &'a mut Stdout,
}

impl <'a> InnerBehavior <'a> {

    pub fn new(stdin: &'a mut File, stdout: &'a mut Stdout) -> Self {
        // return InnerBehavior { mode: Mode::NORMAL, content: Vec::from([vec!['\n', '\r']]),
        return InnerBehavior { mode: Mode::NORMAL, content: Vec::from([Vec::new()]),
            pos: Pos::new(1, 1), stdin: stdin, stdout: stdout }
    }
    
    fn handle_char_normal(&mut self, last_chars: &String) -> bool {
        let last: &str = last_chars;
        // TODO: need to check time if consecutive chars but not same moment
        // TODO: need to remove from page content if command (before executing the command)
        // TODO: how to do if there are several commands with same starts like j and jk?
        return match last {
            name if name.ends_with("k") && self.pos.row > 1 => {
                write!(self.stdout, "\x1B[A").unwrap();
                self.pos.row -= 1;

                // TODO: 
                // let above_len = self.content.get(self.pos.row - 1).unwrap().len();
                // if above_len < self.content.get(self.pos.row).unwrap().len() {
                //     // TODO: change 2 by len of the pos bar
                //     self.pos.col = above_len + 2;
                // }
                false
            },
            name if name.ends_with("j") && self.pos.row < self.content.len() => {
                write!(self.stdout, "\x1B[B").unwrap();
                self.pos.row += 1;
                false
            },
            name if name.ends_with("l") && self.pos.col < self.content.get(self.pos.row - 1).map(|row| {
                // When opening blank file, does not contain a return line contrary to any other
                // line. Handle that by checking if line contains `\n` and `\r`
                let len = row.len();
                return if len >= 2 { len - 2 } else { 0 }
            }).unwrap() => {
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
            name if name.ends_with("\x7F") => {
                // Condition MUST be inside the match.
                // If this condition is not met, will enter the default case
                // which should not happen
                if self.pos.col > 1 {
                    self.content.get_mut(self.pos.row - 1).unwrap().remove(self.pos.col - 2);
                    self.pos.col -= 1;
                    return true
                }
                false
            }
            // TODO: handle ENTER in the middle of a line
            name if name.ends_with("\r") => {
                // Need to be at the START of a NEW line
                self.content.get_mut(self.pos.row - 1).unwrap().insert(self.pos.col - 1, '\n');
                self.content.get_mut(self.pos.row - 1).unwrap().insert(self.pos.col - 1, '\r');
                self.content.insert(self.pos.row, Vec::new());
                self.pos.row += 1;
                self.pos.col = 1;
                true
            },
            _ => {
                self.content.get_mut(self.pos.row - 1).unwrap().insert(self.pos.col - 1, last.chars().last().unwrap());
                self.pos.col += 1;
                true
            }
        }
    }

    fn handle_char(&mut self, last_chars: &String) -> bool {
        return match self.mode {
            Mode::NORMAL => self.handle_char_normal(last_chars),
            Mode::INSERT => self.handle_char_insert(last_chars)
        }
    }

    pub fn launch_event_loop(&mut self) {
        let mut past_chars = String::from("");
        let mut renderer = Renderer::new();

        renderer.render(&self.content, &mut self.stdout, &self.pos);
        // At the moment the flush is not done inside the renderer
        self.stdout.flush().unwrap();

        loop {
            let mut buffer = [0u8; 1];
            self.stdin.read_exact(&mut buffer).unwrap();
            let byte = buffer[0];

            past_chars.push(byte as char);
            let rerender = self.handle_char(&past_chars);

            if rerender {
                renderer.render(&self.content, &mut self.stdout, &self.pos);
            }
            // Should be done in the rendering in case it is not needed but 
            // the cursor has changed position
            self.stdout.flush().unwrap();

            if byte == b'q' {
                break;
            }
        }
    }
}
