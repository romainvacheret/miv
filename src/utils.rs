use std::path::{Path, PathBuf};

use crate::utils::cli::get_arguments;

pub enum Mode {
    NORMAL,
    INSERT
}

impl Mode {
    pub fn to_text(&self) -> &str {
        return match self {
            Mode::NORMAL => "NORMAL",
            Mode::INSERT => "INSERT"
        }
    }
}

#[derive(Debug)]
pub struct Pos {
    pub row: usize,
    pub col: usize
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        return Pos { row: row, col: col };
    }
}


pub fn get_content_path() -> Option<PathBuf> {
    let args = get_arguments();
    args.iter()
        // Index 0 is the name of the file
        .nth(1)
        .map(PathBuf::from)
        .filter(|path| path.exists())
}

pub mod io {
    use std::{fs, io};
    use std::path::PathBuf;

    pub fn read_file(path: &PathBuf) -> Vec<String> {
        println!("Path: {:?}", path);
        fs::read_to_string(path).unwrap()
            .lines()
            .map(String::from)
            .collect::<Vec<String>>()
    }

    pub fn write_file(path: &PathBuf, content: String) -> io::Result<()> {
        fs::write(path, content)?;
        Ok(())
    }
}

pub mod cli {
    pub fn get_arguments() -> Vec<String> {
        std::env::args().collect()
    }
}
