use std::{
    collections::{HashMap, HashSet},
    fs, process,
};
fn main() {
    if let Ok(file) = fs::read_to_string("data/day03.txt") {
        let res = part_1_and_2(&file);
        println!("Part 1 -> {}", res.0);
        println!("Part 2 -> {}", res.1);
        process::exit(0);
    } else {
        eprintln!("Could not load file data/input.txt");
        process::exit(1);
    }
}

fn part_1_and_2(s: &str) -> (u32, u32) {
    // convert lines into matrix of characters
    let matrix: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let h = matrix.len();
    let w = matrix[0].len();
    let mut sum = 0u32;
    // generate gears set for current number
    let mut gears = HashSet::new();
    // generate gears dictionary for whole map
    let mut gears_dict: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for row in 0..h {
        // reset current number and validity check
        let mut number = 0u32;
        let mut valid = false;
        for col in 0..w {
            let c = matrix[row][col];
            if c.is_digit(10) {
                // update current number value
                number = 10 * number + c.to_digit(10).unwrap();
                // check number neighbor for validity
                for subrow in row.saturating_sub(1)..=row + 1 {
                    for subcol in col.saturating_sub(1)..=col + 1 {
                        if subcol >= w || subrow >= h {
                            continue;
                        } else {
                            let sym = matrix[subrow][subcol];
                            if !sym.is_digit(10) && sym != '.' {
                                valid = true;
                                if sym == '*' {
                                    gears.insert((subrow, subcol));
                                }
                            }
                        }
                    }
                }
            }
            if !c.is_digit(10) || col == w - 1 {
                if valid {
                    sum += number
                }
                // update gears
                for gear in gears.clone() {
                    gears_dict.entry(gear).or_insert(Vec::new()).push(number)
                }
                // reset data
                gears.clear();
                number = 0;
                valid = false;
            }
        }
    }
    let mut ratio_sum = 0u32;
    for (_, l) in gears_dict {
        if l.len() == 2 {
            ratio_sum += l[0] * l[1];
        }
    }
    (sum, ratio_sum)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let output = 4361;
        assert_eq!(part_1_and_2(input).0, output);
    }
}
