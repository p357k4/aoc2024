use adv_code_2024::*;
use anyhow::*;
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn hiking(
        expected: u32,
        map: &Array2D<u32>,
        p: (usize, usize),
        nines: &mut HashSet<(usize, usize)>,
    ) {
        if let Some(&actual) = map.get(p.0, p.1) {
            if actual != expected {
                return;
            }
            if actual == 9 {
                nines.insert((p.0, p.1));
                return;
            }

            hiking(expected + 1, map, (p.0.wrapping_sub(1), p.1), nines);
            hiking(expected + 1, map, (p.0, p.1.wrapping_sub(1)), nines);
            hiking(expected + 1, map, (p.0 + 1, p.1), nines);
            hiking(expected + 1, map, (p.0, p.1 + 1), nines);
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let rows: Vec<Vec<u32>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        let mut sum = 0;

        let map = Array2D::from_rows(&rows)?;
        for row in 0..map.num_rows() {
            for column in 0..map.num_columns() {
                if let Some(&value) = map.get(row, column) {
                    if value == 0 {
                        let mut nines: HashSet<(usize, usize)> = HashSet::new();
                        hiking(0, &map, (row, column), &mut nines);
                        let n = nines.len();
                        sum += n;
                    }
                }
            }
        }

        Ok(sum)
    }


    // TODO: Set the expected answer for the test input
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");


    fn hiking2(expected: u32, map: &Array2D<u32>, p: (usize, usize)) -> usize {
        if let Some(&actual) = map.get(p.0, p.1) {
            if actual != expected {
                return 0;
            }
            if actual == 9 {
                return 1;
            }

            return hiking2(expected + 1, map, (p.0.wrapping_sub(1), p.1))
                + hiking2(expected + 1, map, (p.0, p.1.wrapping_sub(1)))
                + hiking2(expected + 1, map, (p.0 + 1, p.1))
                + hiking2(expected + 1, map, (p.0, p.1 + 1));
        }

        0
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let rows: Vec<Vec<u32>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        let mut sum = 0;

        let map = Array2D::from_rows(&rows)?;
        for row in 0..map.num_rows() {
            for column in 0..map.num_columns() {
                if let Some(&value) = map.get(row, column) {
                    if value == 0 {
                        let n = hiking2(0, &map, (row, column));
                        sum += n;
                    }
                }
            }
        }

        Ok(sum)
    }
    
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
