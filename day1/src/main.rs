use std::{fs, process};

fn main() {
    if let Ok(file) = fs::read_to_string("data/input.txt") {
        let mut part_1_sum = 0;
        let mut part_2_sum = 0;
        for line in file.lines() {
            part_1_sum += calibration_part_1(line);
            part_2_sum += calibration_part_2(line);
        }
        println!("Part 1 sum is: {}", part_1_sum);
        println!("Part 2 sum is: {}", part_2_sum);
        process::exit(0);
    } else {
        eprintln!("Could not load file data/input.txt");
        process::exit(1);
    }
}

fn calibration_part_1(s: &str) -> u32 {
    let (mut first_digit, mut second_digit) = (0, 0);
    for l in s.chars() {
        if let Some(d) = l.to_digit(10) {
            first_digit = d;
            break;
        }
    }
    for l in s.chars().rev() {
        if let Some(d) = l.to_digit(10) {
            second_digit = d;
            break;
        }
    }
    first_digit * 10 + second_digit
}

fn calibration_part_2(s: &str) -> u32 {
    let (mut first_digit, mut second_digit) = (0, 0);
    let mut curr_word: &str;
    let mut index = 0;
    for l in s.chars() {
        if l.is_digit(10) {
            first_digit = l.to_digit(10).unwrap();
            break;
        } else {
            index += 1;
            curr_word = &s[..index];
            if let Some(digit) = check_number_word(curr_word) {
                first_digit = digit;
                break;
            }
        }
    }
    index = 0;
    for l in s.chars().rev() {
        if l.is_digit(10) {
            second_digit = l.to_digit(10).unwrap().try_into().unwrap();
            break;
        } else {
            index += 1;
            curr_word = &s[s.len() - index..];
            if let Some(digit) = check_number_word(curr_word) {
                second_digit = digit.try_into().unwrap();
                break;
            }
        }
    }
    first_digit * 10 + second_digit
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn check_number_word(s: &str) -> Option<u32> {
    for i in 0..NUMBERS.len() {
        if s.contains(NUMBERS[i]) {
            return Some((i + 1).try_into().unwrap());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::{calibration_part_1, calibration_part_2};

    #[test]
    fn part_1() {
        let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let outputs = [12, 38, 15, 77];
        for (i, o) in inputs.iter().zip(outputs) {
            assert_eq!(calibration_part_1(i), o);
        }
    }

    #[test]
    fn part_2() {
        let inputs = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "3nine4fourjclspd152rpv",
        ];
        let outputs = [29, 83, 13, 24, 42, 14, 76, 32];
        for (i, o) in inputs.iter().zip(outputs) {
            assert_eq!(calibration_part_2(i), o);
        }
    }
}
