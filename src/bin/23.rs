use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "23"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().filter_map(Result::ok).collect_vec();

        let mut edges = Vec::new();
        let mut vertices = HashMap::new();

        for line in &lines {
            let split: Vec<_> = line.split('-').collect();

            vertices
                .entry(split[0])
                .and_modify(|vs: &mut Vec<&str>| vs.push(split[1]))
                .or_insert(vec![split[1]]);
            vertices
                .entry(split[1])
                .and_modify(|vs: &mut Vec<&str>| vs.push(split[0]))
                .or_insert(vec![split[0]]);
            edges.push((split[0], split[1]));
        }

        let mut triples = HashSet::new();
        for (&u, us) in vertices.iter() {
            for &v in us {
                for &w in us {
                    if u == w {
                        continue;
                    }
                    
                    if v == w {
                        continue;
                    }
                    
                    if u == v {
                        continue;
                    }
                    
                    if !edges.contains(&(v, w)) {
                        continue;
                    }
                    
                    let mut t = vec![u, v, w];
                    if !t.iter().any(|s| s.starts_with("t")) {
                        continue;
                    }
                    
                    t.sort();
                    triples.insert(t);
                }
            }
        }

        for t in triples.iter().sorted() {
            println!("{:?}", t);
        }

        let answer = triples.iter().count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

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
