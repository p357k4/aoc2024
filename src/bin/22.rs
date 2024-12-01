use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;

const DAY: &str = "22"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
"; // TODO: Add the test input

const TEST2: &str = "\
1
2
3
2024
"; // TODO: Add the test input

fn mix(number: i64, secret: i64) -> i64 {
    number ^ secret
}

fn prune(secret: i64) -> i64 {
    secret & 0xffffff
}

fn evolve(secret: i64) -> i64 {
    let secret1 = prune(mix(secret << 6, secret));
    let secret2 = prune(mix(secret1 >> 5, secret1));
    let secret3 = prune(mix(secret2 << 11, secret2));
    secret3
}

fn find(sequence: &[i64], changes: &[i64; 4]) -> Option<i64> {
    for i in 3..sequence.len() {
        if sequence[i] != changes[3] {
            continue;
        }
        if sequence[i - 1] != changes[2] {
            continue;
        }
        if sequence[i - 2] != changes[1] {
            continue;
        }
        if sequence[i - 3] != changes[0] {
            continue;
        }
        return Some(sequence[i]);
    }
    None
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let secrets = reader
            .lines()
            .filter_map(Result::ok)
            .map(|s| s.parse::<i64>())
            .filter_map(Result::ok)
            .collect_vec();

        let mut sum = 0;
        for secret in secrets {
            let mut next_secret = secret;
            for i in 0..2000 {
                next_secret = evolve(next_secret);
            }
            sum += next_secret;
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let secrets = reader
            .lines()
            .filter_map(Result::ok)
            .map(|s| s.parse::<i64>())
            .filter_map(Result::ok)
            .collect_vec();

        let mut accumulated = [0; 65536];
        
        for secret in secrets {
            let mut sequence = Vec::with_capacity(2000);
            let mut digits = Vec::with_capacity(2000);
            let mut next_secret = secret;
            for i in 0..2000 {
                let prev_secret = next_secret;
                next_secret = evolve(next_secret);
                let digit = next_secret % 10;
                let diff = digit - (prev_secret % 10);
                let delta = diff;
                sequence.push(delta);
                digits.push(digit);
            }

            let mut bits = [0u8; 65536];
            for i in 3..sequence.len() {
                let k = index(&sequence, i);
                if bits[k as usize] != 0 {
                    continue;
                }
                accumulated[k as usize] = accumulated[k as usize] + digits[i];
                bits[k as usize] = 1;
            }
        }
        
        let bananas = *accumulated.iter().max().unwrap();

        Ok(bananas)
    }
    
    fn index(sequence: &[i64], i: usize) -> i64 {
        (sequence[i - 3] & 0xf)
            + ((sequence[i - 2] << 4) & 0xf0)
            + ((sequence[i - 1] << 8) & 0xf00)
            + ((sequence[i] << 12) & 0xf000)
    }

    assert_eq!(23, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
