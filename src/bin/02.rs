use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn check_increased(v : &Vec<i32>, i : usize) -> bool {
        if i == v.len() {
            return true;
        }

        let diff = v[i] - v[i-1];
        if 1 <= diff && diff <= 3 {
            return check_increased(v, i + 1);
        }

        false
    }

    fn check_decreased(v : &Vec<i32>, i : usize) -> bool {
        if i == v.len() {
            return true;
        }

        let diff = v[i] - v[i-1];
        if -3 <= diff && diff <= -1 {
            return check_decreased(v, i + 1);
        }

        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut sum = 0;

        for line in reader.lines().filter(Result::is_ok).map(Result::unwrap) {
            let v = line.split(" ").map(| token| token.parse::<i32>().unwrap()).collect_vec();

            if check_increased(&v, 1) {
                sum += 1;
                continue;
            }

            if check_decreased(&v, 1) {
                sum += 1;
                continue
            }
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let mut sum = 0;

        for line in reader.lines().filter(Result::is_ok).map(Result::unwrap) {
            let mut v = line.split(" ").map(| token| token.parse::<i32>().unwrap()).collect_vec();

            if check_increased(&v, 1) {
                sum += 1;
                continue;
            }

            if check_decreased(&v, 1) {
                sum += 1;
                continue
            }

            for i in 0..v.len() {
                let mut u = v.clone();
                u.remove(i);
                if check_increased(&u, 1) {
                    sum += 1;
                    break;
                }

                if check_decreased(&u, 1) {
                    sum += 1;
                    break;
                }

            }
        }

        Ok(sum)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
