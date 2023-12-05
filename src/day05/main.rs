use std::{fs, process};

fn main() {
    if let Ok(file) = fs::read_to_string("data/day05.txt") {
        println!("{}", part_1(&file));
        println!("{}", part_2(&file));
    } else {
        eprintln!("could not open data/day05.txt");
        process::exit(1);
    }
}

enum Data {
    Seed(u32),
    Soil(u32),
    Fertilizer(u32),
    Water(u32),
    Light(u32),
    Temperature(u32),
    Humidity(u32),
    Location(u32),
}

struct Map {
    kind: (Data, Data),
    input_start: u32,
    output_start: u32,
    size: u32,
}

fn part_1(s: &str) -> u32 {
    // divide maps
    let data = s.split("\n\n").collect::<Vec<&str>>();
    let mut seeds: Vec<Data> = Vec::new();
    0
}

fn part_2(s: &str) -> u32 {
    1
}
