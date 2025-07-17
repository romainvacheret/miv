use std::io::{Stdout, Write};


use crate::utils::Pos;

pub struct Renderer {
    info_pos_size: usize,
    info_bar_size: usize
}

impl Renderer {
    pub fn new() -> Self {
        return Renderer { info_pos_size: 1, info_bar_size: 1 };
    }

    fn get_display(&self, content: &Vec<Vec<char>>) -> String {
        // TODO: handle when text does not fit screen
        const COL_CHAR: char = '┃';
        // let full_content: String = content.iter().flatten().collect();
        let full_content: String = content.iter().enumerate().flat_map(|(idx, row)|{
            let mut row_content: Vec<char> = row.clone();
            let line_str: Vec<char> = idx.to_string().chars().collect();

            // We write the current line number padded with spaces to fit 
            // the number of digit of number of the last line.
            // Upper bound is exclusive because the `COL_CHAR` char is counted
            // in the `self.info_pos_size` 
            for i in 0..(self.info_pos_size - 1) {
                row_content.insert(0 + i, if i < line_str.len() { line_str[i] } else { ' ' })
            }
            // // The line number may have multiple digits
            // for (i, digit) in idx.to_string().chars().enumerate() {
            //     // Indexing starts at 0 and line numbers at 1
            //     row_content.insert(0 + i, digit)
            // }
            // Insert after the number of inserted digits
            row_content.insert(row_content.len() - row.len(), COL_CHAR);
            return row_content.into_iter();
        }).collect();

        return full_content;
    }

    pub fn render(&mut self, content: &Vec<Vec<char>>, stdout: &mut Stdout, current_pos: &Pos) {
        // Number of character of last line number plus `COL_CHAR`
        self.info_pos_size = content.len().to_string().len() + 1;

        let full_content = self.get_display(content);
        // write!(stdout, "{:?}", full_content);
        // stdout.flush().unwrap();
        // Clear entire screen, move cursor to top-left, 
        // print content then goes back to cursor position
        write!(stdout, "\x1B[2J\x1B[H{}\x1B[{};{}H", full_content, current_pos.row, current_pos.col + self.info_pos_size).unwrap();
        // write!(stdout, "\x1B[2J\x1B[H{}\x1B[{};{}H {:?}", full_content, current_pos.row, current_pos.col + self.info_pos_size, full_content).unwrap();
    }
}

