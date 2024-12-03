use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"; // TODO: Add the test input
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn extract_numbers(input: &str) -> Vec<(u64, u64)> {
    // Define the regex pattern to capture numbers within `mul(number,number)`
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    // Iterate over matches, extract and parse the captured numbers
    re.captures_iter(input)
        .filter_map(|cap| {
            let num1 = cap.get(1)?.as_str().parse::<u64>().ok()?;
            let num2 = cap.get(2)?.as_str().parse::<u64>().ok()?;
            Some((num1, num2))
        })
        .collect()
}

fn extract_multiplications_and_strings(input: &str) -> Vec<String> {
    // Define the regex pattern for `mul(number,number)` and `don't()` or `do()`
    let pattern = r"mul\(\d+,\d+\)|don't\(\)|do\(\)";
    let re = Regex::new(pattern).unwrap();

    // Collect all matches into a vector
    re.find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let mut sum = 0;
        for line in reader.lines() {
            let mes = extract_numbers(line?.as_str());

            for me in mes {
                sum += me.0*me.1;
            }
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut sum = 0;
        let mut enable = true;
        for line in reader.lines() {
            let exts = extract_multiplications_and_strings(line?.as_str());

            for ext in exts {
                if ext.eq("don't()") {
                    enable = false;
                } else if ext.eq("do()") {
                    enable = true;
                } else if enable {
                    let mes = extract_numbers(ext.as_str());
                    for me in mes {
                        sum += me.0* me.1;
                    }
                }
            }
        }

        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
