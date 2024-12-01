use adv_code_2024::*;
use anyhow::*;
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "25"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines = reader
            .lines()
            .filter(Result::is_ok)
            .filter_map(Result::ok)
            .collect_vec();
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        let chunks = lines.chunks(8).collect_vec();
        for chunk in chunks {
            let c = chunk
                .iter()
                .map(|s| {
                    s.chars()
                        .map(|c| if c == '#' { 1u8 } else { 0 })
                        .collect_vec()
                })
                .collect_vec();
            if chunk[0].eq("#####") {
                locks.push(Array2D::from_rows(&c[1..7])?);
            } else {
                keys.push(Array2D::from_rows(&c[..6])?);
            }
        }

        let keys = keys
            .iter()
            .map(|key| key.columns_iter().map(|c| c.sum::<u8>()).collect_vec())
            .collect_vec();

        let locks = locks
            .iter()
            .map(|lock| lock.columns_iter().map(|c| c.sum::<u8>()).collect_vec())
            .collect_vec();

        let mut sum = 0;
        for lock in &locks {
            for key in &keys {
                let overlap = lock.iter().zip(key.iter()).any(|(left, right)| left + right > 5);
                if !overlap {
                    sum += 1;
                }
            }
        }
        
        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
