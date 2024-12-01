use adv_code_2024::*;
use anyhow::*;
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

const DAY: &str = "16"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn go(
        maze: &Array2D<char>,
        costs: &mut Array2D<u32>,
        p: (usize, usize),
        orientation: char,
        cost: u32,
        path: &Vec<(usize, usize)>,
        paths: &mut Vec<(u32, Vec<(usize, usize)>)>,
    ) {
        if maze.get(p.0, p.1).filter(|&&c| c != '#').is_none() {
            return;
        }
        
        if p.0 == 9 && p.1 == 3 {
            println!("hit {}", cost);
        }

        if costs.get(p.0, p.1).filter(|&&c| c < cost).is_some() {
            return;
        }

        let mut new_path = path.clone();
        new_path.push(p);
            
        if maze.get(p.0, p.1).filter(|&&c| c == 'E').is_some() {
            costs.set(p.0, p.1, cost);
            paths.push((cost, new_path));
            return;
        }

        let all_orientations = "^>v<";

        
        let orientations = [vec![orientation], all_orientations.chars().filter(|&c| c != orientation).collect_vec()].concat();
        
        for no in orientations {
            let np = match no {
                '^' => (p.0 - 1, p.1),
                '>' => (p.0, p.1 + 1),
                'v' => (p.0 + 1, p.1),
                '<' => (p.0, p.1 - 1),
                _ => panic!("Invalid orientation"),
            };
            let rotation_cost = if no == orientation { 0 } else { 1000 };
            let nc = cost + rotation_cost;
            costs.set(p.0, p.1, nc);


            go(maze, costs, np, no, nc + 1, &new_path, paths);
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let maze = Array2D::from_rows(&rows)?;

        let (start, _) = maze.enumerate_row_major().find(|(_, &c)| c == 'S').unwrap();
        let (end, _) = maze.enumerate_row_major().find(|(_, &c)| c == 'E').unwrap();

        let mut costs = Array2D::filled_with(u32::MAX, maze.num_rows(), maze.num_columns());

        let mut paths = Vec::new();
        go(&maze, &mut costs, start, '>', 0, &vec![], &mut paths);

        let cost_min = paths.iter().map(|(cost, _)| cost).min().unwrap();

        let spots = paths.iter().filter(|(cost, _)| cost == cost_min).map(|(_, path)| path).flatten().collect::<HashSet<_>>();

        println!("{}", spots.len());
        let result = *costs.get(end.0, end.1).unwrap();
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);

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
