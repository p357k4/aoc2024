use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "13"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"; // TODO: Add the test input

#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let mut lines = reader.lines();

    let re = Regex::new(r"X\+(\d+), Y\+(\d+)")?;
    let re2 = Regex::new(r"X=(\d+), Y=(\d+)")?;

    loop {
        let line_a = lines.next().unwrap()?;
        let line_b = lines.next().unwrap()?;
        let prize = lines.next().unwrap()?;

        let caps_a = re.captures(&line_a).unwrap();
        let x_a = caps_a.get(1).unwrap().as_str().parse::<i64>()?;
        let y_a = caps_a.get(2).unwrap().as_str().parse::<i64>()?;

        let caps_b = re.captures(&line_b).unwrap();
        let x_b = caps_b.get(1).unwrap().as_str().parse::<i64>()?;
        let y_b = caps_b.get(2).unwrap().as_str().parse::<i64>()?;

        let caps_p = re2.captures(&prize).unwrap();
        let x_p = caps_p.get(1).unwrap().as_str().parse::<i64>()?;
        let y_p = caps_p.get(2).unwrap().as_str().parse::<i64>()?;

        let empty = lines.next();
        games.push(Game{
            button_a: (x_a, y_a),
            button_b: (x_b, y_b),
            prize: (x_p, y_p),
        });

        if empty.is_none() {
            break;
        }
    }

    Ok(games)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let games = parse_input(reader)?;

        let sum = solve(games);
        
        Ok(sum)
    }
    
    fn solve(games: Vec<Game>) -> i64 {
        let mut sum = 0;
        for game in games {
            let det = game.button_a.0 * game.button_b.1 - game.button_a.1 * game.button_b.0;
            let det_a = game.button_b.1 * game.prize.0 - game.button_b.0 * game.prize.1;
            let det_b = game.button_a.0 * game.prize.1 - game.button_a.1 * game.prize.0;

            if det_a % det != 0 {
                continue;
            }

            if det_b % det != 0 {
                continue;
            }

            let a = det_a / det;
            let b = det_b / det;

            if a < 0 {
                continue;
            }

            if b < 0 {
                continue;
            }

            if a > 100 {
                continue;
            }

            if b > 100 {
                continue;
            }

            sum += 3 * a + b;
        }
        sum
    }

    fn solve2(games: Vec<Game>) -> i64 {
        let mut sum = 0;
        for game in games {
            let det = game.button_a.0 * game.button_b.1 - game.button_a.1 * game.button_b.0;
            let det_a = game.button_b.1 * game.prize.0 - game.button_b.0 * game.prize.1;
            let det_b = game.button_a.0 * game.prize.1 - game.button_a.1 * game.prize.0;

            if det_a % det != 0 {
                continue;
            }

            if det_b % det != 0 {
                continue;
            }

            let a = det_a / det;
            let b = det_b / det;

            if a < 0 {
                continue;
            }

            if b < 0 {
                continue;
            }

            sum += 3 * a + b;
        }
        sum
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let games = parse_input(reader)?;

        let modified = games.iter().map(|g| Game{
            button_a: g.button_a,
            button_b: g.button_b,
            prize: (10000000000000+g.prize.0, 10000000000000+g.prize.1),
        }).collect::<Vec<_>>();
        
        let sum = solve2(modified);

        Ok(sum)
    }
    
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
