use std::{any::Any, io::{Stdout, Write}};


use crate::{terminal::get_terminal_size, utils::{Mode, Pos}};

pub struct Renderer {
    info_pos_size: usize,
    info_bar_size: usize,
    win_size: (u16, u16)
}

impl Renderer {
    pub fn new() -> Self {
        let result = get_terminal_size();

        if result.is_err() {
            // TODO: handle properly
            panic!("Could not get the terminal size");
        }

        return Renderer { info_pos_size: 1, info_bar_size: 1, win_size: result.ok().unwrap() };
    }

    fn get_display(&self, content: &Vec<Vec<char>>) -> String {
        // TODO: handle when text does not fit screen
        const COL_CHAR: char = '┃';
        let full_content: String = content.iter().enumerate().flat_map(|(idx, row)|{
            let mut row_content: Vec<char> = row.clone();
            let line_str: Vec<char> = (idx + 1).to_string().chars().collect();

            // We write the current line number padded with spaces to fit 
            // the number of digit of number of the last line.
            // Upper bound is exclusive because the `COL_CHAR` char is counted
            // in the `self.info_pos_size` 
            for i in 0..(self.info_pos_size - 1) {
                row_content.insert(0 + i, if i < line_str.len() { line_str[i] } else { ' ' })
            }
            // Insert after the number of inserted digits
            row_content.insert(row_content.len() - row.len(), COL_CHAR);

            row_content.push('\n');
            row_content.push('\r');

            return row_content.into_iter();
        }).collect();

        return full_content;
    }

    fn render_cursor(&self, stdout: &mut Stdout, current_pos: &Pos) {
            write!(stdout, "\x1B[{};{}H", current_pos.row, current_pos.col + self.info_pos_size).unwrap();
    }

    fn render_info_bar(&mut self, stdout: &mut Stdout, mode: &Mode, current_pos: &Pos) {
        let mode_string = format!(" {}", mode.to_text());
        let pos_string = format!("{}:{} ", current_pos.row, current_pos.col);
        // TODO: handle properly
        let padding_size = (self.win_size.1 as usize - (mode_string.len() + pos_string.len())).try_into().expect("Should never be reached");
        let content = format!("{}{}{}", mode_string, " ".repeat(padding_size), pos_string);
        write!(stdout, "\x1B[{};0H{}", self.win_size.0, content).unwrap();
    }

    pub fn render(&mut self, content: &Vec<Vec<char>>, stdout: &mut Stdout, current_pos: &Pos, full_render: bool, mode: &Mode) {
        if full_render {
            // Number of character of last line number plus `COL_CHAR`
            self.info_pos_size = content.len().to_string().len() + 1;

            let full_content = self.get_display(content);
            // Clear entire screen, move cursor to top-left, 
            write!(stdout, "\x1B[2J\x1B[H{}", full_content).unwrap();
        } 
        self.render_info_bar(stdout, mode, current_pos);
        self.render_cursor(stdout, current_pos);
        stdout.flush().unwrap();
    }
}


