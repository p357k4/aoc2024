use adv_code_2024::*;
use anyhow::*;
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn go2(
        maze: &Array2D<char>,
        costs: &mut Array2D<u32>,
        p: (usize, usize),
        cost: u32,
        cheat_start: Option<(usize, usize)>,
        cheat_end: Option<(usize, usize)>,
        cheat_steps: u32,
        cheated: bool,
        savings: &mut Vec<((usize, usize), (usize, usize), u32)>,
        cheat_max: u32,
    ) {
        if p.0 == 0 || p.1 == 0 || p.0 == maze.num_rows() - 1 || p.1 == maze.num_columns() - 1 {
            return;
        }

        if cheat_steps == 1 && maze.get(p.0, p.1).filter(|&&c| c == '#').is_some() {
            return;
        }

        let nce = if cheat_steps == 1 {
            Some(p)
        } else {
            cheat_end
        };

        if costs.get(p.0, p.1).filter(|&&c| c < cost).is_some() {
            return;
        }

        costs.set(p.0, p.1, cost);

        if maze.get(p.0, p.1).filter(|&&c| c == 'E').is_some() {
            // if cheat_steps > 1 {
            //     return;
            // }
            if let Some(start) = cheat_start {
                if let Some(end) = nce {
                    savings.push((start, end, cost));
                }
            }
        }
        
        let next = vec![
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
        ];

        let ncs = cheat_steps.saturating_sub(1);

        for np in &next {
            if ncs == 0 && maze.get(np.0, np.1).filter(|&&c| c == '#').is_some() {
                continue;
            }

            go2(
                maze,
                costs,
                *np,
                cost + 1,
                cheat_start,
                nce,
                ncs,
                cheated,
                savings,
                cheat_max,
            );
        }

        if cheated {
            return;
        }

        let mut cloned = costs.clone();

        for np in &next {
            if maze.get(np.0, np.1).filter(|&&c| c == '#').is_none() {
                continue;
            }
            
            go2(
                maze,
                &mut cloned,
                *np,
                cost + 1,
                Some(p),
                None,
                cheat_max,
                true,
                savings,
                cheat_max,
            );
        }
    }

    fn go(maze: &Array2D<char>, costs: &mut Array2D<u32>, p: (usize, usize), cost: u32) {
        if p.0 == 0 || p.1 == 0 || p.0 == maze.num_rows() - 1 || p.1 == maze.num_columns() - 1 {
            return;
        }

        if maze.get(p.0, p.1).filter(|&&c| c != '#').is_none() {
            return;
        }

        if costs.get(p.0, p.1).filter(|&&c| c < cost).is_some() {
            return;
        }

        costs.set(p.0, p.1, cost);

        if maze.get(p.0, p.1).filter(|&&c| c == 'E').is_some() {
            return;
        }

        let next = vec![
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
        ];

        for np in next {
            go(maze, costs, np, cost + 1);
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let maze = Array2D::from_rows(&rows)?;

        let (start, _) = maze.enumerate_row_major().find(|(_, &c)| c == 'S').unwrap();
        let (end, _) = maze.enumerate_row_major().find(|(_, &c)| c == 'E').unwrap();

        let mut costs = Array2D::filled_with(u32::MAX, maze.num_rows(), maze.num_columns());

        go(&maze, &mut costs, start, 0);
        let reference = *costs.get(end.0, end.1).unwrap();

        let mut savings = Vec::new();
        for cheat_max in 2..=20 {
            go2(
                &maze,
                &mut costs,
                start,
                0,
                None,
                None,
                0,
                false,
                &mut savings,
                cheat_max,
            );
        }

        let unique = savings.iter().unique().collect_vec();

        let filtered = unique
            .into_iter()
            .filter(|(_, _, c)| *c + 50 == reference)
            .collect_vec();
        let answer = filtered.len();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(84, part1(BufReader::new(TEST.as_bytes()))?);

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
