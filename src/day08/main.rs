use std::{collections::HashMap, fs, process};

fn main() {
    let file = fs::read_to_string("data/day08.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
    let answer = solution(&file);
    println!("{}", answer.0);
    println!("{}", answer.1);
}

fn solution(s: &str) -> (u64, u64) {
    let mut map = HashMap::new();
    for line in s.lines() {
        let tokens: Vec<_> = line
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|&c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|word| word.len() != 0)
            .collect();
        if tokens.len() != 3 {
            continue;
        }
        map.insert(tokens[0].clone(), (tokens[1].clone(), tokens[2].clone()));
    }

    let mut curr = "AAA";
    let mut i = 0;
    let directions: Vec<char> = s.lines().nth(0).unwrap().chars().collect();
    loop {
        let dir = directions[i % directions.len()];
        println!("Looking for {}, going {}", curr, dir);
        match dir {
            'L' => {
                curr = &map.get(curr).unwrap().0;
            }
            'R' => {
                curr = &map.get(curr).unwrap().1;
            }
            _ => {
                println!("WWW")
            }
        }
        i += 1;
        if curr == "ZZZ" {
            break;
        }
    }
    (i as u64, 0)
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        let output = 288;
        assert_eq!(solution(input).0, output);
    }
}
