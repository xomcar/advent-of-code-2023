use std::{fs, process};

fn main() {
    let file = fs::read_to_string("data/day06.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
}
