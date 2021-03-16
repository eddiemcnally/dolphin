use std::io;
use std::io::prelude::*;

pub fn uci_poll() -> String {
    let stdin = io::stdin();
    // Read one line of input iterator-style
    let input = stdin.lock().lines().next();
    input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string()
}
