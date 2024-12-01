use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::Rem;

const DAY: &str = "14"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"; // TODO: Add the test input

struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn read_input<R: BufRead>(reader: R) -> Result<(Vec<(i64, i64)>, Vec<(i64, i64)>), Error> {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

        let mut ps = Vec::new();
        let mut vs = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let caps = re.captures(&line).unwrap();
            let px = caps.get(1).unwrap().as_str().parse::<i64>()?;
            let py = caps.get(2).unwrap().as_str().parse::<i64>()?;
            let vx = caps.get(3).unwrap().as_str().parse::<i64>()?;
            let vy = caps.get(4).unwrap().as_str().parse::<i64>()?;

            ps.push((px, py));
            vs.push((vx, vy));
        }
        Ok((ps, vs))
    }

    fn go(
        ps: &Vec<(i64, i64)>,
        vs: &Vec<(i64, i64)>,
        k: i64,
        wide: i64,
        tall: i64,
    ) -> Vec<(i64, i64)> {
        ps.iter()
            .zip(vs)
            .map(|(p, v)| {
                (
                    ((p.0 + k * v.0).rem(wide) + wide).rem(wide),
                    ((p.1 + k * v.1).rem(tall) + tall).rem(tall),
                )
            })
            .collect()
    }

    fn part1<R: BufRead>(reader: R, wide: i64, tall: i64) -> Result<usize> {
        let (ps, vs) = read_input(reader)?;

        let k = 100;
        let ns = go(&ps, &vs, k, wide, tall);

        let mut sum_left_top = 0;
        let mut sum_right_top = 0;
        let mut sum_left_bottom = 0;
        let mut sum_right_bottom = 0;

        for p in ns {
            if p.0 < wide / 2 && p.1 < tall / 2 {
                sum_left_top += 1;
            }
            if p.0 > wide / 2 && p.1 < tall / 2 {
                sum_right_top += 1;
            }
            if p.0 < wide / 2 && p.1 > tall / 2 {
                sum_left_bottom += 1;
            }
            if p.0 > wide / 2 && p.1 > tall / 2 {
                sum_right_bottom += 1;
            }
        }

        let answer = sum_left_top * sum_right_top * sum_left_bottom * sum_right_bottom;
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn check(p0: &Vec<(i64, i64)>) -> bool {
        const W: usize = 9;

        let map0 = p0.iter().into_group_map_by(|&p| p.0);
        let max0 = map0.iter().max_by_key(|&p| *p.0);

        if max0.filter(|p| p.1.len() > W).is_none() {
            return false;
        }

        let map1 = p0.iter().into_group_map_by(|&p| p.1);
        let max1 = map0.iter().max_by_key(|&p| *p.0);

        max1.filter(|p| p.1.len() > W).is_some()
    }

    fn part2<R: BufRead>(reader: R, wide: i64, tall: i64) -> Result<i64> {
        let (ps, vs) = read_input(reader)?;

        let k = 6398;
        
        let ns = go(&ps, &vs, k, wide, tall);

        for t in 0..tall {
            for w in 0..wide {
                if ns.iter().contains(&(w, t)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        Ok(0)
    }
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()), 101, 103)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
