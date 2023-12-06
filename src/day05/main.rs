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

fn part_1(s: &str) -> u64 {
    // read seeds
    let mut blocks = s.split("\n\n");
    let seeds: Vec<u64> = blocks
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|t| t.parse().ok())
        .collect();
    // read maps
    let maps: Vec<Vec<Map>> = blocks
        .map(|block| {
            block
                .lines()
                .filter_map(|line| Map::from_str(line))
                .collect()
        })
        .collect();

    // solve maps for each seed and save minimum
    let mut min_loc = u64::MAX;
    for seed in seeds {
        // traverse each block
        let mut res = seed;
        for block in &maps {
            // check each map and update value
            for map in block {
                let new_res = map.convert(res);
                if new_res != res {
                    res = new_res;
                    break;
                } else {
                    res = new_res;
                }
            }
        }
        // update new mimimal location (if any)
        if res < min_loc {
            min_loc = res;
        }
    }

    min_loc
}

fn part_2(_: &str) -> u64 {
    0
}

struct Map {
    start_input: u64,
    start_output: u64,
    size: u64,
}

impl Map {
    fn from_str(s: &str) -> Option<Map> {
        let tokens: Vec<_> = s
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();
        if tokens.len() != 3 {
            return None;
        } else {
            Some(Map {
                start_output: tokens[0],
                start_input: tokens[1],
                size: tokens[2],
            })
        }
    }
    fn convert(&self, x: u64) -> u64 {
        // println!(
        //     "input {} - range [{},{}) -> [{},{})",
        //     x,
        //     self.start_input,
        //     self.start_input + self.size,
        //     self.start_output,
        //     self.start_output + self.size,
        // );
        if x >= self.start_input && x < self.start_input + self.size {
            // println!("output {}", self.start_output + (x - self.start_input));
            self.start_output + (x - self.start_input)
        } else {
            // println!("output due to out of bounds {}", x);
            x
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::part_1;

    #[test]
    fn check() {
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
