use std::{io::{Stdout, Write}, path::PathBuf};


use crate::{terminal::get_terminal_size, utils::{io::write_file, Mode, Pos}};

pub struct Renderer {
    info_pos_size: usize,
    info_bar_size: usize,
    // TODO: change ot usize?
    win_size: (u16, u16),
    display_window: (usize, usize)
}

impl Renderer {
    pub fn new() -> Self {
        let result = get_terminal_size();

        if result.is_err() {
            // TODO: handle properly
            panic!("Could not get the terminal size");
        }

        return Renderer { 
            info_pos_size: 1, 
            info_bar_size: 1, 
            win_size: result.ok().unwrap(),
            // TODO: set default
            display_window: (0, 0)
        };
    }

    fn get_display(&mut self, content: &Vec<String>) -> String {
        // TODO: handle when text does not fit screen
        const COL_CHAR: char = '┃';
        let full_content: String = content.iter().enumerate().map(|(idx, row)|{
            let mut row_content = row.clone();
            // Also count `COL_CHAR` character
            let line_str = (idx + 1).to_string().chars().collect::<Vec<char>>();

            // We write the current line number padded with spaces to fit 
            // the number of digit of number of the last line.
            // Upper bound is exclusive because the `COL_CHAR` char is counted
            // in the `self.info_pos_size` 
            // for i in 0..(self.info_pos_size - 1) {
            //     row_content.insert(0 + i, if i < line_str.len() { line_str[i] } else { ' ' })
            // }
            // row_content.insert(self.info_pos_size - 1, COL_CHAR);
            let line_nb = (0..(self.info_pos_size - 1)).map(|idx| {
                if idx < line_str.len() { line_str[idx] } else { ' ' }
            }).collect::<String>();
            let info = format!("{}{}", line_nb, COL_CHAR);

            // let start = std::cmp::max(0, self.display_window.0);
            // // `self.display_window` does not take self.`pos_size into account`
            // let end = std::cmp::min(row_content.len() - self.info_pos_size, self.display_window.1);
            //
            // // Info bar is part of the string but is not counted in `display_win`
            // row_content = row_content.chars()
            //     .take(self.info_pos_size)
            //     .skip(start)
            //     .take(end - start)
            //     .collect();
            
            // `self.display_window` does not take self.`pos_size into account`
            let start = std::cmp::max(0, self.display_window.0);
            // Don't substract - 1 to len of content even though index is outside of the 
            // string because would remove last element when cursor reaches the end
            let end = std::cmp::min(row_content.len(), self.display_window.1);


            // // WARNING: `replace_range` works on bytes, non ASCII characters 
            // // would cause a bug
            // row_content.replace_range(..start, "");
            // // the `start` first elements have already been removed
            // row_content.replace_range((end - start).., "");

            row_content = row_content.chars()
                .skip(start)
                .take(end - start)
                .collect::<String>();

            let x = format!("start: {} end: {} dis: {:?} win: {:?} content: {}|| len2: {}", 
                start, end, self.display_window, self.win_size, row_content, row_content.len());
            let file = PathBuf::from("log2.log");
            write_file(&file, x);

            row_content.push_str("\n\r");



            info + &row_content

        }).collect();

        return full_content;
    }

    fn render_cursor(&self, stdout: &mut Stdout, current_pos: &Pos) {
        let col = self.info_pos_size + 1 + current_pos.col.saturating_sub(1).saturating_sub(self.display_window.0);
        write!(stdout, "\x1B[{};{}H", current_pos.row, col).unwrap();
    }

    fn render_info_bar(&mut self, stdout: &mut Stdout, mode: &Mode, current_pos: &Pos) {
        let mode_string = format!(" {}", mode.to_text());
        let pos_string = format!("{}:{} ", current_pos.row, current_pos.col);
        // TODO: if screen too small breaks instantly
        // TODO: handle properly
        let padding_size = (self.win_size.1 as usize - (mode_string.len() + pos_string.len())).try_into().expect("Should never be reached");
        let content = format!("{}{}{}", mode_string, " ".repeat(padding_size), pos_string);
        // Position 0 is invalid
        write!(stdout, "\x1B[{};1H{}", std::cmp::max(0, self.win_size.0), content).unwrap();
    }

    pub fn render(&mut self, content: &Vec<String>, stdout: &mut Stdout, 
                  current_pos: &Pos, mut full_render: bool, mode: &Mode) {

        // Number of character of last line number plus `COL_CHAR`
        self.info_pos_size = content.len().to_string().len() + 1;

        // TODO: ensure never negative
        let width = self.win_size.1 as usize  - self.info_pos_size;
        if self.display_window == (0, 0) {
            self.display_window = (
                0,
                width
            )
        }

        let mut start = self.display_window.0;
        let mut end = self.display_window.1; // exclusive
        let col = current_pos.col.saturating_sub(1);

        // TODO:
        // if col - 1 < start {
        //     let diff_size = start - col;
        //     start -= diff_size;
        //     end -= diff_size;
        //     full_render = true;
        //     self.display_window = (start, end);
        // }

        // if col > end {
        //     let diff_size = col - end;
        //     start += diff_size;
        //     end += diff_size;
        //     full_render = true;
        //     self.display_window = (start, end);
        // }

        if col >= end {
            start = col + 1 - width;
            end = start + width;
            full_render = true;
             self.display_window = (start, end);
        }

        if col < start {
            start = col;
            end = start + width;
            full_render = true;
             self.display_window = (start, end);
        }

        let x = format!("{:?} start: {} end: {} len: {} dis: {:?} win: {:?}", current_pos, start, end, content.len(), self.display_window, self.win_size);
        let file = PathBuf::from("log.log");
        write_file(&file, x);

        if full_render {
            let full_content = self.get_display(content);
            // Clear entire screen, move cursor to top-left, 
            write!(stdout, "\x1B[2J\x1B[H{}", full_content).unwrap();
        } 
        self.render_info_bar(stdout, mode, current_pos);
        self.render_cursor(stdout, current_pos);
        stdout.flush().unwrap();
    }
}


