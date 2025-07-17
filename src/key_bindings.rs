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
        return InnerBehavior { mode: Mode::NORMAL, content: Vec::from([Vec::new()]),
            pos: Pos::new(1, 1), stdin: stdin, stdout: stdout }
    }
    
    fn handle_char_normal(&mut self, last_chars: &String) -> bool {
        let last: &str = last_chars;
        // TODO: need to check time if consecutive chars but not same moment
        // TODO: need to remove from page content if command (before executing the command)
        // TODO: how to do if there are several commands with same starts like j and jk?
        return match last {
            // TODO: j/k, keep col in mind when going up/down to go back to it
            // even while going through smaller rows
            name if name.ends_with("k") && self.pos.row > 1 => {
                // `self.pos.row` starts at 1 but `Vec` indexing at 0
                let above_len = self.content[self.pos.row - 2].len();
                if above_len < self.content[self.pos.row - 1].len() {
                    self.pos.col = above_len + 1;
                }

                self.pos.row -= 1;
                false
            },
            name if name.ends_with("j") && self.pos.row < self.content.len() => {
                // `Vec` indexing always 1 less than pos.row: cannot reach
                // index `Vec::len`
                let below_len = self.content[self.pos.row].len();
                if below_len < self.content[self.pos.row - 1].len() {
                    self.pos.col = below_len + 1;
                }
                self.pos.row += 1;
                false
            },
            name if name.ends_with("l") && self.pos.col < self.content[self.pos.row - 1].len() => {
                self.pos.col += 1;
                false
            },
            name if name.ends_with("h") && self.pos.col > 1 => {
                self.pos.col -= 1;
                false
            },
            name if name.ends_with("0") => {
                self.pos.col = 1;
                false
            }
            name if name.ends_with("gg") => {
                self.pos.row = 1;
                false
            },
            name if name.ends_with("$") => {
                self.pos.col = self.content[self.pos.row - 1].len();
                false
            },
            name if name.ends_with("G") => {
                self.pos.row = self.content.len();
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
            // Escape 
            name if name.ends_with("\x1B") => {
                self.mode = Mode::NORMAL;
                false
            },
            // Backspace
            name if name.ends_with("\x7F") => {
                // If is not on the first colomn, remove the character before 
                if self.pos.col > 1 {
                    self.content[self.pos.row - 1].remove(self.pos.col - 2);
                    self.pos.col -= 1;
                    return true
                // Else if is but also not the first row, concatenate content 
                // of current line to the one above. Place cursor a the previous
                // lenght of the line above
                } else if self.pos.row > 1 {
                    self.pos.col = self.content[self.pos.row - 2].len();
                    let current_row = self.content.remove(self.pos.row - 1);
                    let next_row = &mut self.content[self.pos.row - 2];
                    let prev_len = next_row.len();
                    next_row.extend(current_row);
                    self.pos.row -= 1;
                    self.pos.col = prev_len;
                }
                true
            }
            name if name.ends_with("\r") => {
                // If is the end of the line, create a new empty line below 
                // and move to it
                if self.pos.col == self.content[self.pos.row - 1].len() + 1 {
                    self.content.insert(self.pos.row, Vec::new());
                    self.pos.row += 1;
                    self.pos.col = 1;
                // Else create a new line with the content after the cursor and
                // move to the start of the new line
                } else {
                    let current_length = self.content[self.pos.row - 1].len();
                    let end_content: Vec<char> = self.content[self.pos.row - 1]
                        .drain((self.pos.col - 1)..current_length)
                        .collect();
                    self.content.insert(self.pos.row, end_content);
                    self.pos.row += 1;
                    self.pos.col = 1;
                }
                true
            },
            _ => {
                self.content[self.pos.row - 1].insert(self.pos.col - 1, last.chars().last().unwrap());
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

        renderer.render(&self.content, &mut self.stdout, &self.pos, true);

        loop {
            let mut buffer = [0u8; 1];
            self.stdin.read_exact(&mut buffer).unwrap();
            let byte = buffer[0];

            past_chars.push(byte as char);
            let rerender = self.handle_char(&past_chars);

            renderer.render(&self.content, &mut self.stdout, &self.pos, rerender);

            if byte == b'q' {
                break;
            }
        }
    }
}
