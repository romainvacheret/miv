use std::io::{self, Write};

use crate::key_bindings::InnerBehavior;
use crate::logger::setup_logger;
use crate::terminal::{enable_raw_mode, quit_restore_mode, start_tui};
use crate::utils::get_content_path;

mod logger;
mod terminal;
mod key_bindings;
mod display;
mod utils;

fn main() {
    setup_logger().unwrap();

    let original_mode = enable_raw_mode();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    start_tui(&mut stdout);

    InnerBehavior::new(
        &mut stdin, 
        &mut stdout,
        get_content_path().filter(|path| path.exists())
    ).launch_event_loop();

    quit_restore_mode(original_mode, &mut stdout);
    writeln!(stdout, "Exited.").unwrap();
}
