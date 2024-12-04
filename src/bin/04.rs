use adv_code_2024::*;
use anyhow::*;
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let array2d = Array2D::from_rows(&rows)?;

        let rows = array2d.num_rows();
        let columns = array2d.num_columns();

        let check = |p: (i32, i32), c: char| -> bool {
            if p.0 < 0 || (rows as i32) <= p.0 {
                return false;
            }

            if p.1 < 0 || (columns as i32) <= p.1 {
                return false;
            }

            return array2d.get(p.0 as usize, p.1 as usize).unwrap() == &c;
        };

        let check_xmas = |p: (i32, i32), next: fn((i32, i32)) -> (i32, i32)| -> bool {
            if !check(p, 'X') {
                return false;
            }

            let p = next(p);
            if !check(p, 'M') {
                return false;
            }

            let p = next(p);
            if !check(p, 'A') {
                return false;
            }

            let p = next(p);
            if !check(p, 'S') {
                return false;
            }

            true
        };

        let mut sum = 0;
        for row in 0..rows as i32 {
            for column in 0..columns as i32 {
                if check_xmas((row, column), |p| (p.0, p.1 + 1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0, p.1 - 1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0 + 1, p.1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0 - 1, p.1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0 + 1, p.1 + 1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0 - 1, p.1 - 1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0 + 1, p.1 - 1)) {
                    sum += 1
                }
                if check_xmas((row, column), |p| (p.0 - 1, p.1 + 1)) {
                    sum += 1
                }
            }
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let array2d = Array2D::from_rows(&rows)?;

        let rows = array2d.num_rows();
        let columns = array2d.num_columns();

        let check = |p: (i32, i32), c: char| -> bool {
            if p.0 < 0 || (rows as i32) <= p.0 {
                return false;
            }

            if p.1 < 0 || (columns as i32) <= p.1 {
                return false;
            }

            return array2d.get(p.0 as usize, p.1 as usize).unwrap() == &c;
        };

        let check_xmas0 = |p: (i32, i32)| -> bool {
            if !check(p, 'M') {
                return false;
            }

            if !check((p.0+2, p.1), 'M') {
                return false;
            }

            if !check((p.0+1, p.1+1), 'A') {
                return false;
            }

            if !check((p.0, p.1+2), 'S') {
                return false;
            }

            if !check((p.0+2, p.1+2), 'S') {
                return false;
            }

            true
        };

        let check_xmas1 = |p: (i32, i32)| -> bool {
            if !check(p, 'S') {
                return false;
            }

            if !check((p.0+2, p.1), 'S') {
                return false;
            }

            if !check((p.0+1, p.1+1), 'A') {
                return false;
            }

            if !check((p.0, p.1+2), 'M') {
                return false;
            }

            if !check((p.0+2, p.1+2), 'M') {
                return false;
            }

            true
        };

        let check_xmas2 = |p: (i32, i32)| -> bool {
            if !check(p, 'S') {
                return false;
            }

            if !check((p.0+2, p.1), 'M') {
                return false;
            }

            if !check((p.0+1, p.1+1), 'A') {
                return false;
            }

            if !check((p.0, p.1+2), 'S') {
                return false;
            }

            if !check((p.0+2, p.1+2), 'M') {
                return false;
            }

            true
        };

        let check_xmas3 = |p: (i32, i32)| -> bool {
            if !check(p, 'M') {
                return false;
            }

            if !check((p.0+2, p.1), 'S') {
                return false;
            }

            if !check((p.0+1, p.1+1), 'A') {
                return false;
            }

            if !check((p.0, p.1+2), 'M') {
                return false;
            }

            if !check((p.0+2, p.1+2), 'S') {
                return false;
            }

            true
        };

        let mut sum = 0;
        for row in 0..rows as i32 {
            for column in 0..columns as i32 {
                if check_xmas0((row, column)) {
                    sum += 1
                }
                if check_xmas1((row, column)) {
                    sum += 1
                }
                if check_xmas2((row, column)) {
                    sum += 1
                }
                if check_xmas3((row, column)) {
                    sum += 1
                }
            }
        }

        Ok(sum)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
