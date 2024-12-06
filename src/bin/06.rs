use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn find_start(terrain: &Array2D<char>) -> (usize, usize) {
        for row in 0..terrain.num_rows() {
            for column in 0..terrain.num_columns() {
                if terrain.get(row,column).filter(|&&c| c == '^').is_some() {
                    return (row, column)
                }
            }
        }

        panic!("start position not found");
    }

    fn track(terrain: &Array2D<char>, start_row: usize, start_column: usize) -> Array2D<char> {
        let mut trace = Array2D::filled_with(' ', terrain.num_rows(), terrain.num_columns());

        let mut row = start_row;
        let mut column = start_column;
        let mut direction = '^';

        loop {
            trace.set(row, column, direction);

            let (next_row, next_column) = match direction {
                '^' => (row.wrapping_sub(1), column),
                '>' => (row, column + 1),
                'v' => (row + 1, column),
                '<' => (row, column.wrapping_sub(1)),
                _ => (row, column),
            };

            if next_row > terrain.num_rows() - 1 || next_column > terrain.num_columns() - 1 {
                break;
            }

            if terrain.get(next_row, next_column).filter(|&&c| c == '#').is_some() {
                direction = match direction {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => direction,
                };
                continue;
            }

            row = next_row;
            column = next_column;
        }

        trace
    }

    fn is_loop(terrain: Array2D<char>, start_row: usize, start_column: usize) -> bool {
        let mut trace : HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        let mut row = start_row;
        let mut column = start_column;
        let mut direction = '^';

        loop {
            if trace.get(&(row, column)).filter(|set| set.contains(&direction)).is_some() {
                return true;
            }
            trace.entry((row, column)).or_default().insert(direction);

            let (next_row, next_column) = match direction {
                '^' => (row.wrapping_sub(1), column),
                '>' => (row, column + 1),
                'v' => (row + 1, column),
                '<' => (row, column.wrapping_sub(1)),
                _ => (row, column),
            };

            if next_row > terrain.num_rows() - 1 || next_column > terrain.num_columns() - 1 {
                return false;
            }

            if terrain.get(next_row, next_column).filter(|&&c| c == '#').is_some() {
                direction = match direction {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => direction,
                };
                continue;
            }

            row = next_row;
            column = next_column;
        }
    }

    fn count(trace: &Array2D<char>) -> u32 {
        let mut sum = 0;
        for row in 0..trace.num_rows() {
            for column in 0..trace.num_columns() {
                if trace.get(row, column).filter(|&&c| !c.is_ascii_whitespace()).is_some() {
                    sum += 1;
                }
            }
        }

        sum
    }

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let terrain = Array2D::from_rows(&rows)?;

        let (start_row , start_column) = find_start(&terrain);

        let trace = track(&terrain, start_row, start_column);

        let answer = count(&trace);
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let terrain = Array2D::from_rows(&rows)?;

        let (start_row , start_column) = find_start(&terrain);

        let trace = track(&terrain, start_row, start_column);

        let mut sum = 0;

        for row in 0..trace.num_rows() {
            println!("{} / {}", row, trace.num_rows());
            for column in 0..trace.num_columns() {
                if row == start_row && column == start_column {
                    continue; // ignore start point
                }

                if trace.get(row, column).filter(|&&c| c.is_ascii_whitespace()).is_some() {
                    continue;
                }

                let mut new_terrain = terrain.clone();
                new_terrain.set(row, column, '#');

                if (is_loop(new_terrain, start_row, start_column)) {
                    sum += 1;
                }
            }
        }

        Ok(sum)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
