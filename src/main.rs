use std::fs::File;
use std::io::{self, Write};

use crate::key_bindings::InnerBehavior;
use crate::terminal::{enable_raw_mode, quit_restore_mode, start_tui};

mod terminal;
mod key_bindings;
mod display;
mod utils;


fn main() {
    let original_mode = enable_raw_mode();
    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut stdout = io::stdout();

    start_tui(&mut stdout);

    InnerBehavior::new(&mut stdin, &mut stdout).launch_event_loop();

    quit_restore_mode(original_mode, &mut stdout);
    writeln!(stdout, "Exited.").unwrap();
}
