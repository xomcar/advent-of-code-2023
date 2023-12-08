use std::{fs, process};

fn main() {
    let file = fs::read_to_string("data/day06.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
    let answer = solution(&file);
    println!("{}", answer.0);
    println!("{}", answer.1);
}

fn solution(s: &str) -> (u64, u64) {
    let times: Vec<_> = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|token| token.parse::<u64>().ok())
        .collect();
    let distances: Vec<_> = s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|token| token.parse::<u64>().ok())
        .collect();
    // each moment pressed increases the speed at the same amount
    // e.g. press for 1 s will move at 1 m/s
    let mut confidence_1 = 1;
    for (time, dist) in times.iter().zip(distances.iter()) {
        let won = ways_to_win(*time, *dist);
        confidence_1 = confidence_1 * won;
    }
    let time = remove_kerning(times);
    let distance = remove_kerning(distances);
    let confidence_2 = ways_to_win(time, distance);
    (confidence_1, confidence_2)
}

fn ways_to_win(time: u64, distance: u64) -> u64 {
    let mut won = 0;
    for i in 0..time {
        let speed = i;
        let time_left = time - i;
        if speed * time_left > distance {
            won += 1;
        }
    }
    won
}

fn remove_kerning(inputs: Vec<u64>) -> u64 {
    let mut number = String::new();
    for i in inputs {
        number.push_str(&i.to_string());
    }
    number.parse().unwrap()
}

#[cfg(test)]
mod tests {

    use crate::{remove_kerning, solution};

    #[test]
    fn test() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        let output = 288;
        assert_eq!(solution(input).0, output);
    }

    #[test]

    fn ker() {
        let n = remove_kerning(vec![1, 203]);
        assert_eq!(n, 1203);
    }
}
