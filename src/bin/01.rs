use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut ls : Vec<i32> = Vec::new();
        let mut rs : Vec<i32> = Vec::new();
        for line in reader.lines().filter(Result::is_ok).map(Result::unwrap).filter(|l| l.len() > 0) {
            let split = line.split("   ").collect_vec();
            let left = split[0].parse::<i32>()?;
            let right = split[1].parse::<i32>()?;

            ls.push(left);
            rs.push(right);
        }

        ls.sort();
        rs.sort();

        let mut sum = 0;
        for m in ls.iter().zip(rs.iter()) {
            sum = sum + m.0.abs_diff(*m.1);
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");


    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let mut ls : Vec<u32> = Vec::new();
        let mut frequencies = HashMap::new();

        for line in reader.lines().filter(Result::is_ok).map(Result::unwrap).filter(|l| l.len() > 0) {
            let split = line.split("   ").collect_vec();
            let left = split[0].parse::<u32>()?;
            let right = split[1].parse::<u32>()?;

            ls.push(left);
            *frequencies.entry(right).or_insert(0u32) += 1;
        }

        let mut sum = 0;
        for m in ls.iter() {
            sum = sum + m * frequencies.get(m).unwrap_or(&0u32);
        }

        Ok(sum)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
