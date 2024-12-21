use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn check(design: &str, towels: &[String]) -> bool {
        if design.is_empty() {
            return true;
        }

        for towel in towels {
            if design.starts_with(towel) {
                if check(&design[towel.len()..], towels) {
                    return true;
                }
            }
        }

        false
    }

    fn check2(design: &str, towels: &[String], cache: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(variants) = cache.get(design) {
            return *variants;
        }

        let mut sum = 0;
        for towel in towels {
            if design.starts_with(towel) {
                sum += check2(&design[towel.len()..], towels, cache)
            }
        }

        cache.insert(design.to_string(), sum);
        sum
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (designs, towels) = read_input(reader)?;

        let mut sum = 0;
        for design in designs {
            if check(&design, &towels) {
                sum += 1;
            }
        }

        Ok(sum)
    }
    fn read_input<R: BufRead>(reader: R) -> Result<(Vec<String>, Vec<String>), Error> {
        let mut towels_flag = true;
        let mut designs = Vec::new();
        let mut towels = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                towels_flag = false;
                continue;
            }

            if towels_flag {
                towels = line.split(", ").map(|s| s.to_owned()).collect();
            } else {
                designs.push(line);
            }
        }
        Ok((designs, towels))
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (designs, towels) = read_input(reader)?;
        let mut cache = HashMap::new();
            
        let mut sum = 0;
        for design in designs {
            sum += check2(&design, &towels, &mut cache);
        }

        Ok(sum)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
