use std::{fs, process};

fn main() {
    let file = fs::read_to_string("data/day09.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file {}", err);
        process::exit(1);
    });
    let answer1 = solution1(&file);
    let answer2 = solution2(&file);
    println!("{}\n{}", answer1, answer2);
}

fn solution1(s: &str) -> i64 {
    let mut total = 0;
    for line in s.lines() {
        // get numerical values from string
        let mut prev: Vec<i64> = line
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();
        let mut sum = *prev.last().unwrap();
        // continue to reduce rows
        loop {
            println!("{:?}", &prev);
            let v: Vec<_> = prev.windows(2).map(|vals| vals[1] - vals[0]).collect();
            if v.len() == v.iter().filter(|&c| *c == 0).count() {
                break;
            } else {
                sum += *v.last().unwrap();
                prev = v;
            }
            //std::thread::sleep(std::time::Duration::from_secs(1));
        }
        // sum predicted value to sum
        total += sum;
    }
    total
}

fn solution2(s: &str) -> i64 {
    let mut total = 0;
    for line in s.lines() {
        // get numerical values from string
        let mut prev: Vec<i64> = line
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();
        // continue to reduce rows
        let mut history = vec![];
        history.push(prev.clone());
        loop {
            println!("{:?}", &prev);
            prev.reverse();
            let v: Vec<_> = prev
                .windows(2)
                .rev()
                .map(|vals| {
                    let i = vals[0] - vals[1];
                    i
                })
                .collect();
            if v.len() == v.iter().filter(|&c| *c == 0).count() {
                break;
            } else {
                prev = v;
                history.push(prev.clone());
            }
            //std::thread::sleep(std::time::Duration::from_secs(1));
        }
        let mut value = 0;
        for row in history.iter().rev() {
            value = *row.first().unwrap() - value;
        }
        // sum predicted value to sum
        total += value;
    }
    total
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn predict1() {
        let inputs = [
            "0   3   6   9  12  15",
            "10  13  16  21  30  45",
            "1 3 6 10 15 21",
        ];
        let outputs = [18, 68, 28];
        let mut sum = 0;
        for (i, o) in inputs.iter().zip(outputs) {
            let s = solution1(i);
            assert_eq!(s, o);
            sum += s
        }
        assert_eq!(sum, 114);
    }

    #[test]
    fn predict2() {
        let inputs = [
            "0   3   6   9  12  15",
            "10  13  16  21  30  45",
            "1 3 6 10 15 21",
        ];
        let outputs = [-3, 5, 0];
        for (i, o) in inputs.iter().zip(outputs) {
            assert_eq!(solution2(i), o);
        }
    }
}
