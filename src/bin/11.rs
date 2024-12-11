use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // TODO: Solve Part 1 of the puzzle
    fn part1<R: BufRead>(reader: R, n: u64) -> Result<u64> {
        let mut cache = HashMap::<(u64, u64), u64>::new();
        let stones = reader
            .lines()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .flat_map(|line| {
                line.split(" ")
                    .map(|token| token.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut sum = 0;
        for stone in stones {
            let b = blink(&mut cache, stone, n);
            sum += b
        }

        Ok(sum)
    }

    fn digits(number: u64) -> u32 {
        let mut n = number;
        let mut d = 0;
        loop {
            n /= 10;
            d += 1;
            if n == 0 {
                break;
            }
        }
        d
    }

    fn blink(cache: &mut HashMap<(u64, u64), u64>, stone: u64, n: u64) -> u64 {
        if n == 0 {
            return 1;
        }

        if let Some(&c) = cache.get(&(stone, n)) {
            return c;
        }

        if stone == 0 {
            let value = blink(cache, 1, n - 1);
            cache.insert((1, n - 1), value);
            return value;
        }

        let d = digits(stone);
        if d % 2 == 0 {
            let divider: u64 = 10_u64.pow(d / 2);
            let stone1 = stone / divider;
            let value1 = blink(cache, stone1, n - 1);
            cache.insert((stone1, n - 1), value1);
            
            let stone2 = stone % divider;
            let value2 = blink(cache, stone2, n - 1);
            cache.insert((stone2, n - 1), value2);

            return value1 + value2;
        }

        let value = blink(cache, stone * 2024, n - 1);
        cache.insert((stone * 2024, n - 1), value);

        value
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()), 25)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 25)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 75)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
