use adv_code_2024::*;
use anyhow::*;
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn visit(prev: char, terrain: &Array2D<char>, visited: &mut Array2D<bool>, row: usize, column: usize, perimeter: &mut u32, area: &mut u32) {
        if row >= terrain.num_rows() || column >= terrain.num_columns() {
            return;
        }
        if visited.get(row, column).filter(|&&x| x == true).is_some() {
            return;
        }
        if terrain.get(row, column).filter(|&&c| c != prev).is_some() {
            return;
        }
        *area += 1;

        if terrain.get(row.wrapping_add(1), column).filter(|&&c| c == prev).is_none() {
            *perimeter += 1;
        }
        if terrain.get(row.wrapping_sub(1), column).filter(|&&c| c == prev).is_none() {
            *perimeter += 1;
        }
        if terrain.get(row, column.wrapping_add(1)).filter(|&&c| c == prev).is_none() {
            *perimeter += 1;
        }
        if terrain.get(row, column.wrapping_sub(1)).filter(|&&c| c == prev).is_none() {
            *perimeter += 1;
        }
        
        visited.set(row, column, true);
        visit(prev, terrain, visited, row.wrapping_add(1), column, perimeter, area);
        visit(prev, terrain, visited, row.wrapping_sub(1), column, perimeter, area);
        visit(prev, terrain, visited, row, column.wrapping_add(1), perimeter, area);
        visit(prev, terrain, visited, row, column.wrapping_sub(1), perimeter, area);
    }
    
    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let terrain = Array2D::from_rows(&rows)?;
        let mut visited = Array2D::filled_with(false, terrain.num_rows(), terrain.num_columns());

        let mut sum = 0;
        for row in 0..terrain.num_rows() {
            for column in 0..terrain.num_columns() {
                if visited.get(row, column).filter(|&&x| x == true).is_some() {
                    continue;
                }

                let mut perimeter = 0; // virtual common edge
                let mut area = 0;
                visit(*terrain.get(row, column).unwrap(), &terrain, &mut visited, row, column, &mut perimeter, &mut area);
                sum += perimeter * area;
            }
        }
        
        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(0)
    }
    
    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
