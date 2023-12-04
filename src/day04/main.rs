use std::time::Duration;
use std::{process,fs};
use std::collections::{HashSet, HashMap, VecDeque};

fn main() {
    if let Ok(file) = fs::read_to_string("data/day04.txt") {
        println!("Part 1 -> {}", part1(&file));
        println!("Part 2 -> {}", part2(&file));
    } else {
        eprintln!("Could not load file data/input.txt");
        process::exit(1);
    }
}

fn part1(s : &str) -> u32 {
    let mut sum = 0u32;
    for line in s.lines() {
        let mut same = 0u32;
        let res = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let mut scratched = HashSet::<u32>::new();
        let mut winning = HashSet::<u32>::new();
        for number in res[0].split_whitespace() {
            scratched.insert(number.parse().unwrap());
        }
        for number in res[1].split_whitespace() {
            winning.insert(number.parse().unwrap());
        }
        for number in winning {
            if scratched.contains(&number) {
                same+=1;
            }
        }
        if same != 0 {
            sum += 1 << (same-1);
        }
        }
    sum
}

fn part2(s : &str) -> u32 {
    // parse scratch cards
    let mut cards_values = Vec::<usize>::new();
    for line in s.lines(){
        let mut same = 0;
        let res = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let mut scratched = HashSet::<u32>::new();
        let mut winning = HashSet::<u32>::new();
        for number in res[0].split_whitespace() {
            scratched.insert(number.parse().unwrap());
        }
        for number in res[1].split_whitespace() {
            winning.insert(number.parse().unwrap());
        }
        for number in winning {
            if scratched.contains(&number) {
                same+=1;
            }
        }
        cards_values.push(same);
    }
    dbg!(&cards_values);
    let mut cards = VecDeque::new();
    for i in 0..s.lines().count() {
        cards.push_back(i);
    }
    let mut total = s.lines().count();
    while cards.len() != 0 {
        let c  = cards.pop_front().unwrap();
        let winning_cards = cards_values[c];
        for i in c+1..c+1+winning_cards {
            cards.push_back(i);
            total += 1;
        }
    }
    total.try_into().unwrap()
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = [8, 2, 2, 1, 0 ,0];
        for (i, o) in input.lines().zip(output) {
            assert_eq!(part1(&i), o);
        }
    }
}