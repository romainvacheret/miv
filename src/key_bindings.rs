use std::{io::{Read, Stdin, Stdout}, path::PathBuf};

use crate::{display::Renderer, utils::{io::{read_file, write_file}, Mode, Pos}};

pub struct InnerBehavior <'a> {
    mode: Mode,
    content_path: Option<PathBuf>,
    content: Vec<String>,
    pos: Pos,
    stdin: &'a mut Stdin,
    stdout: &'a mut Stdout,
    renderer: Renderer
}

impl <'a> InnerBehavior <'a> {
    pub fn new(stdin: &'a mut Stdin, stdout: &'a mut Stdout, content_path: Option<PathBuf>) -> Self {
        return InnerBehavior { 
            mode: Mode::NORMAL, 
            content: content_path.as_ref().map_or_else(
                || Vec::from([String::new()]),
                |path| read_file(path)),
            content_path,
            pos: Pos::new(1, 1), 
            stdin, 
            stdout, 
            renderer: Renderer::new()
        }
    }
    
    fn handle_char_normal(&mut self, last_chars: &str) -> bool {
        // TODO: need to check time if consecutive chars but not same moment
        // TODO: need to remove from page content if command (before executing the command)
        // TODO: how to do if there are several commands with same starts like j and jk?
        // TODO: j/k, keep col in mind when going up/down to go back to it
        // even while going through smaller rows
        if last_chars.ends_with("k") && self.pos.row > 1 {
            // `self.pos.row` starts at 1 but `Vec` indexing at 0
            let above_len = self.content[self.pos.row - 2].len();
            if above_len < self.content[self.pos.row - 1].len() {
                self.pos.col = above_len + 1;
            }
            self.pos.row -= 1;
        } else if last_chars.ends_with("j") && self.pos.row < self.content.len() {
            // `Vec` indexing always 1 less than pos.row: cannot reach
            // index `Vec::len`
            let below_len = self.content[self.pos.row].len();
            if below_len < self.content[self.pos.row - 1].len() {
                self.pos.col = below_len + 1;
            }
            self.pos.row += 1;
        } else if last_chars.ends_with("l") && 
        self.pos.col < self.content[self.pos.row - 1].len() {
            self.pos.col += 1;
        } else if last_chars.ends_with("h") && self.pos.col > 1 {
            self.pos.col -= 1;
        } else if last_chars.ends_with("0") {
            self.pos.col = 1;
        } else if last_chars.ends_with("gg") {
            self.pos.row = 1;
        } else if last_chars.ends_with("$") {
            self.pos.col = self.content[self.pos.row - 1].len();
        } else if last_chars.ends_with("G") {
            self.pos.row = self.content.len();
        } else if last_chars.ends_with("i") {
            self.mode = Mode::INSERT;
        } else if last_chars.ends_with("w") {
            self.content_path.as_ref().inspect(|path| { 
                let content = self.content.join("\n");
                let _ = write_file(path, content);
           });
        }

        false
    }

    fn handle_char_insert(&mut self, last_chars: &str) -> bool {
        // Escape 
        if last_chars.ends_with("\x1B") {
            self.mode = Mode::NORMAL;
            false
        // Backspace
        } else if last_chars.ends_with("\x7F") {
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
                next_row.push_str(&current_row);
                self.pos.row -= 1;
                self.pos.col = prev_len;
            }
            true
        } else if last_chars.ends_with("\r") {
            // If is the end of the line, create a new empty line below 
            // and move to it
            if self.pos.col == self.content[self.pos.row - 1].len() + 1 {
                self.content.insert(self.pos.row, String::new());
                self.pos.row += 1;
                self.pos.col = 1;
            // Else create a new line with the content after the cursor and
            // move to the start of the new line
            } else {
                let current_length = self.content[self.pos.row - 1].len();
                let end_content = self.content[self.pos.row - 1]
                    .drain((self.pos.col - 1)..current_length)
                    .collect();
                self.content.insert(self.pos.row, end_content);
                self.pos.row += 1;
                self.pos.col = 1;
            }
            true
        } else {
            self.content[self.pos.row - 1].insert(self.pos.col - 1, last_chars.chars().last().unwrap());
            self.pos.col += 1;
            true
        }
    }

    fn handle_char(&mut self, last_chars: &String) -> bool {
        return match self.mode {
            Mode::NORMAL => self.handle_char_normal(last_chars),
            Mode::INSERT => self.handle_char_insert(last_chars)
        }
    }

    fn render(&mut self, full_render: bool) {
        self.renderer.render(&self.content, &mut self.stdout, &self.pos, full_render, &self.mode);
    }

    pub fn launch_event_loop(&mut self) {
        // TODO: handle better
        let mut past_chars = String::from("");

        self.render(true);

        loop {
            let mut buffer = [0u8; 1];
            self.stdin.read_exact(&mut buffer).unwrap();
            let byte = buffer[0];

            past_chars.push(byte as char);
            let rerender = self.handle_char(&past_chars);

            self.render(rerender);

            if byte == b'q' {
                break;
            }
        }
    }
}
