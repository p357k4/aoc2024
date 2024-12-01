use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
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
        let terrain = Array2D::from_rows(&rows)?;

        let mut antennas = HashMap::new();

        for row in 0..terrain.num_rows() {
            for column in 0..terrain.num_columns() {
                if let Some(c) = terrain.get(row, column).filter(|c| **c != '.') {
                    antennas.entry(*c).or_insert_with(Vec::new).push((row as i32, column as i32));
                }
            }
        }

        let mut antinode_positions = HashSet::new();

        for antenna in antennas.keys() {
            let antenna_positions = antennas.get(antenna).unwrap();
            for i in 0..antenna_positions.len() {
                for j in i + 1..antenna_positions.len() {
                    let antenna_a = antenna_positions.get(i).unwrap();
                    let antenna_b = antenna_positions.get(j).unwrap();

                    let delta = (antenna_b.0 - antenna_a.0, antenna_b.1 - antenna_a.1);

                    let antinode_a = (antenna_a.0 - delta.0, antenna_a.1 - delta.1);
                    let antinode_b = (antenna_b.0 + delta.0, antenna_b.1 + delta.1);

                    if 0 <= antinode_a.0 && antinode_a.0 < terrain.num_rows() as i32 && 0 <= antinode_a.1 && antinode_a.1 < terrain.num_columns() as i32 {
                        antinode_positions.insert(antinode_a);
                    }

                    if 0 <= antinode_b.0 && antinode_b.0 < terrain.num_rows() as i32 && 0 <= antinode_b.1 && antinode_b.1 < terrain.num_columns() as i32 {
                        antinode_positions.insert(antinode_b);
                    }
                }
            }
        }
        
        let result = antinode_positions.len();
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

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
        let terrain = Array2D::from_rows(&rows)?;

        let mut antennas = HashMap::new();

        for row in 0..terrain.num_rows() {
            for column in 0..terrain.num_columns() {
                if let Some(c) = terrain.get(row, column).filter(|c| **c != '.') {
                    antennas.entry(*c).or_insert_with(Vec::new).push((row as i32, column as i32));
                }
            }
        }

        let mut antinode_positions = HashSet::new();

        for antenna in antennas.keys() {
            let antenna_positions = antennas.get(antenna).unwrap();
            for i in 0..antenna_positions.len() {
                for j in i + 1..antenna_positions.len() {
                    let antenna_a = antenna_positions.get(i).unwrap();
                    let antenna_b = antenna_positions.get(j).unwrap();

                    let delta = (antenna_b.0 - antenna_a.0, antenna_b.1 - antenna_a.1);

                    let mut antinode_a = (antenna_a.0, antenna_a.1);
                    while 0 <= antinode_a.0 && antinode_a.0 < terrain.num_rows() as i32 && 0 <= antinode_a.1 && antinode_a.1 < terrain.num_columns() as i32 {
                        antinode_positions.insert(antinode_a);
                        antinode_a = (antinode_a.0 - delta.0, antinode_a.1 - delta.1);
                    }
                    
                    let mut antinode_b = (antenna_b.0, antenna_b.1);
                    while 0 <= antinode_b.0 && antinode_b.0 < terrain.num_rows() as i32 && 0 <= antinode_b.1 && antinode_b.1 < terrain.num_columns() as i32 {
                        antinode_positions.insert(antinode_b);
                        antinode_b = (antinode_b.0 + delta.0, antinode_b.1 + delta.1);
                    } 
                }
            }
        }

        let result = antinode_positions.len();
        Ok(result)
    }
    
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
