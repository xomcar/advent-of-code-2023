use std::{fs, process};
const PART_1_RED: u32 = 12;
const PART_1_GREEN: u32 = 13;
const PART_1_BLUE: u32 = 14;

fn main() {
    if let Ok(file) = fs::read_to_string("data/input.txt") {
        let (mut part_1_sum, mut part_2_sum) = (0u32, 0u32);
        for line in file.lines() {
            let game = get_game(line);
            if game.is_possible(PART_1_RED, PART_1_GREEN, PART_1_BLUE) {
                part_1_sum += game.id;
            }
            part_2_sum += game.minimum_power()
        }
        println!("Part 1 ➡️ {}", part_1_sum);
        println!("Part 2 ➡️ {}", part_2_sum);
        process::exit(0);
    } else {
        eprintln!("Could not load file data/input.txt");
        process::exit(1);
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn new() -> Hand {
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Game {
    fn new() -> Game {
        Game {
            id: 0,
            hands: vec![],
        }
    }

    fn is_possible(&self, r: u32, g: u32, b: u32) -> bool {
        for hand in &self.hands {
            if hand.blue > b || hand.green > g || hand.red > r {
                return false;
            }
        }
        true
    }

    fn minimum_power(&self) -> u32 {
        let (mut max_g, mut max_r, mut max_b) = (0u32, 0u32, 0u32);
        for hand in &self.hands {
            if hand.blue > max_b {
                max_b = hand.blue
            }
            if hand.red > max_r {
                max_r = hand.red
            }
            if hand.green > max_g {
                max_g = hand.green
            }
        }
        max_b * max_g * max_r
    }
}

fn get_game(s: &str) -> Game {
    let v: Vec<&str> = s.split(":").collect();
    let id_string = v[0];
    let mut g: Game = Game::new();
    g.id = id_string.split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let game = v[1];
    for hand in game.split("; ") {
        g.hands.push(get_hand(hand));
    }
    g
}

fn get_hand(s: &str) -> Hand {
    let mut h = Hand::new();
    for values in s.split(", ") {
        let value: Vec<&str> = values.trim().split(" ").collect();
        if value[1].contains("green") {
            h.green = value[0].parse().unwrap();
        } else if value[1].contains("blue") {
            h.blue = value[0].parse().unwrap();
        } else if value[1].contains("red") {
            h.red = value[0].parse().unwrap();
        }
    }
    h
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_possible() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let output = [true, true, false, false, true];
        for (i, o) in input.into_iter().zip(output) {
            let game = get_game(i);
            assert_eq!(game.is_possible(PART_1_RED, PART_1_GREEN, PART_1_BLUE), o);
        }
    }

    #[test]
    fn check_power() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let output = [48, 12, 1560, 630, 36];
        for (i, o) in input.into_iter().zip(output) {
            let game = get_game(i);
            assert_eq!(game.minimum_power(), o);
        }
    }
}
