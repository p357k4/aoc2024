use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn search(target: u64, value: u64, values: &[u64], i: usize) -> bool {
        if i == values.len() {
            return target == value;
        }

        let first = value + values[i];
        if search(target, first, values, i + 1) {
            return true;
        }

        let second = value * values[i];
        if search(target, second, values, i + 1) {
            return true;
        }

        false
    }

    fn search3(target: u64, value: u64, values: &[u64], i: usize) -> bool {
        if i == values.len() {
            return target == value;
        }

        let first = value + values[i];
        if search3(target, first, values, i + 1) {
            return true;
        }

        let second = value * values[i];
        if search3(target, second, values, i + 1) {
            return true;
        }

        let mut multiplication = 1;
        while multiplication <= values[i] {
            multiplication *= 10;
        }
        
        let third = value * multiplication + values[i];
        if search3(target, third, values, i + 1) {
            return true;
        }

        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let input = read_input(reader)?;

        let mut sum = 0;
        for (target, values) in input.iter() {
            if search(*target, values[0], values, 1) {
                sum += target;
            }
        }

        Ok(sum)
    }
    
    fn read_input<R: BufRead>(reader: R) -> Result<Vec<(u64, Vec<u64>)>, Error> {
        let mut input = Vec::new();

        for line in reader.lines() {
            let line = line?;
            // Split each line into key and values
            let mut parts = line.split(": ");
            let key_str = parts.next().unwrap();
            let value_str = parts.next().unwrap();
            let key = key_str.parse::<u64>()?;
            // Parse the values into a vector
            let values = value_str
                .split(" ")
                .filter_map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>();
            input.push((key, values));
        }
        Ok(input)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let input = read_input(reader)?;
        
        let mut sum = 0;
        for (target, values) in input.iter() {
            if search3(*target, values[0], values, 1) {
                sum += target;
            }
        }

        Ok(sum)
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
