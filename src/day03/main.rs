use std::{fs, process};

fn main() {
    if let Ok(file) = fs::read_to_string("data/day03.txt") {
        part1(&file);
        process::exit(0);
    } else {
        eprintln!("Could not load file data/input.txt");
        process::exit(1);
    }
}


fn part1(s : &str) {
   
}