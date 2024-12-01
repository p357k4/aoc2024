use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use array2d::Array2D;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "18"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn go(corrupted: &Array2D<bool>, costs: &mut Array2D<u32>, p: (usize, usize), e: (usize, usize), cost: u32) {
        costs.set(p.0, p.1, cost);

        if p == e {
            return;
        }

        let next = vec![
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1),
            (p.0, p.1.wrapping_sub(1)),
        ];

        let nc = cost + 1;

        for np in next {
            if corrupted.get(np.0, np.1).filter(|&&c| !c).is_none() {
                continue;
            }

            if costs.get(np.0, np.1).filter(|&&c| c <= nc).is_some() {
                continue;
            }

            go(corrupted, costs, np, e, nc);
        }
    }

    fn part1<R: BufRead>(reader: R, rows : usize, columns: usize, n: usize) -> Result<u32> {
        let mut corrupted = Array2D::filled_with(false, rows, columns);
        let mut costs = Array2D::filled_with(u32::MAX, rows, columns);
        
        for line in reader.lines().take(n) {
            let line = line?;
            let split = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
            corrupted.set(*split.get(0).unwrap(), *split.get(1).unwrap(), true);
        }
        
        go(&corrupted, &mut costs, (0, 0), (rows - 1, columns - 1), 0);

        let answer = *costs.get(rows - 1, columns - 1).unwrap();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 7, 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 71, 71, 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R, rows : usize, columns: usize) -> Result<(usize,usize)> {

        let mut bytes = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let split = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
            bytes.push((*split.get(0).unwrap(), *split.get(1).unwrap()));
        }

        let mut a = 0;
        let mut b = bytes.len() - 1;
        loop {
            if a + 1 == b {
                return Ok(*bytes.get(b).unwrap());
            }
            let c = (a + b + 1) / 2;
            let mut corrupted = Array2D::filled_with(false, rows, columns);
            let mut costs = Array2D::filled_with(u32::MAX, rows, columns);
            
            for i in 0..=c {
                let bc = bytes.get(i).unwrap();
                corrupted.set(bc.0, bc.1, true);
            }
            
            go(&corrupted, &mut costs, (0, 0), (rows - 1, columns - 1), 0);

            let answer = *costs.get(rows - 1, columns - 1).unwrap();
            if answer == u32::MAX {
                b = c;
            } else {
                a = c;
            }
        }
        panic!("should not happen");
    }
    
    assert_eq!((6,1), part2(BufReader::new(TEST.as_bytes()), 7, 7)?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 71, 71)?);
    println!("Result = {:?}", result);
    //endregion

    Ok(())
}
