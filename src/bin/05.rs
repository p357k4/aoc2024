use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut lists = Vec::new();
        let mut parsing_pairs = true;
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                // If an empty line is encountered, switch to parsing lists
                parsing_pairs = false;
                continue;
            }

            if parsing_pairs {
                // Parse the `a|b` pairs
                if let Some((a, b)) = line.split_once('|') {
                    let a: i32 = a.trim().parse().unwrap();
                    let b: i32 = b.trim().parse().unwrap();
                    map.entry(a).or_default().insert(b);
                }
            } else {
                // Parse the comma-separated lists
                let list: Vec<i32> = line
                    .split(',')
                    .map(|num| num.trim().parse().unwrap())
                    .collect();
                lists.push(list);
            }
        }

        let mut sum = 0;
        for list in lists {
            let mut subset: HashSet<i32> = list.iter().cloned().into_iter().collect();
            for element in list.iter() {
                subset.remove(element);
                if subset.is_empty() {
                    break;
                }
                if let Some(set) = map.get(&element) {
                    if !subset.is_subset(map.get(element).unwrap()) {
                        break;
                    }
                } else {
                    println!("{} -> {}", element, subset.len());
                    break;
                }
            }
            if subset.is_empty() {
                sum = sum + list.get(list.len() / 2).unwrap_or(&0);
            }
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut lists = Vec::new();
        let mut parsing_pairs = true;
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                // If an empty line is encountered, switch to parsing lists
                parsing_pairs = false;
                continue;
            }

            if parsing_pairs {
                // Parse the `a|b` pairs
                if let Some((a, b)) = line.split_once('|') {
                    let a: i32 = a.trim().parse().unwrap();
                    let b: i32 = b.trim().parse().unwrap();
                    map.entry(a).or_default().insert(b);
                }
            } else {
                // Parse the comma-separated lists
                let list: Vec<i32> = line
                    .split(',')
                    .map(|num| num.trim().parse().unwrap())
                    .collect();
                lists.push(list);
            }
        }

        let mut sum = 0;
        for list in lists {
            let mut subset: HashSet<i32> = list.iter().cloned().into_iter().collect();
            for element in list.iter() {
                subset.remove(element);
                if subset.is_empty() {
                    break;
                }
                if let Some(set) = map.get(&element) {
                    if !subset.is_subset(map.get(element).unwrap()) {
                        break;
                    }
                } else {
                    println!("{} -> {}", element, subset.len());
                    break;
                }
            }

            if !subset.is_empty() {
                let mut incorrect: HashSet<i32> = list.iter().cloned().into_iter().collect();

                let mut correct = Vec::new();
                loop {
                    {
                        if incorrect.len() == 1 {
                            correct.push(*incorrect.iter().next().unwrap());
                            break;
                        }
                    }

                    {
                        let mut correct_as_set: HashSet<i32> = correct.iter().cloned().into_iter().collect();
                        
                        for element in incorrect.difference(&correct_as_set) {
                            let me = map.get(element);
                            if me.is_none() {
                                continue;
                            }
                            let set = me.unwrap();
                            let fixed: HashSet<i32> =
                                incorrect.iter().cloned().filter(|x| x != element).collect();
                            if fixed.is_subset(set) {
                                correct.push(*element);
                            }
                        }
                    }
                    {
                        incorrect.remove(correct.last().unwrap());
                    }
                }

                sum = sum + *correct.get(correct.len() / 2).unwrap_or(&&0);
            }
        }

        Ok(sum)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
