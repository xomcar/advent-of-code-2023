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

#[derive(Debug)]
struct Map {
    input_start: u64,
    output_start: u64,
    size: u64,
}

impl Map {
    fn from_vec(i: Vec<u64>) -> Map {
        Map {
            output_start: i[0],
            input_start: i[1],
            size: i[2],
        }
    }
}

#[derive(Debug)]
struct Converter {
    maps: Vec<Map>,
}

impl Converter {
    fn convert_forward(&self, n: u64) -> u64 {
        println!("Converting input {}", n);
        for map in &self.maps {
            let end = map.input_start + map.size;
            println!(
                "Map: [{},{}) -> [{},{})",
                map.input_start,
                map.input_start + map.size,
                map.output_start,
                map.output_start + map.size
            );
            if n >= map.input_start && n < end {
                // conversion is allowed
                let offset = map.output_start as i64 - map.input_start as i64;
                println!(
                    "Input is within range, output will be {} = {} + {}",
                    n as i64 + offset,
                    n,
                    offset,
                );
                return (n as i64 + offset) as u64;
            } else {
                println!("Input is not within range")
            }
        }
        // if no valid range is found, return original number
        n
    }

    fn from_maps(maps: Vec<Map>) -> Converter {
        Converter { maps: maps }
    }
}

fn part_1(s: &str) -> u64 {
    // get initial seeds
    let inputs: Vec<u64> = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|c| c.parse::<u64>().ok())
        .collect();
    // build chain of converters
    let blocks: Vec<_> = s
        .split("\n\n")
        .filter(|block| block.contains("map"))
        .collect();
    let mut converters = Vec::new();
    for block in blocks {
        let mut maps: Vec<Map> = Vec::new();
        for line in block.lines() {
            let values: Vec<_> = line
                .split_whitespace()
                .filter_map(|token| token.parse::<u64>().ok())
                .collect();
            if values.len() != 3 {
                continue;
            }
            println!("Loaded map {} {} {}", values[0], values[1], values[2]);
            maps.push(Map::from_vec(values));
        }
        converters.push(Converter::from_maps(maps))
    }
    let mut min = u64::MAX;
    for seed in &inputs {
        let mut res = *seed;
        for converter in &converters {
            res = converter.convert_forward(res);
        }
        if res < min {
            min = res;
        }
    }
    min
}

fn part_2(_: &str) -> u64 {
    2
}

#[cfg(test)]
mod tests {

    use crate::part_1;

    #[test]
    fn converter() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = 35;
        assert_eq!(part_1(input), output);
    }
}
