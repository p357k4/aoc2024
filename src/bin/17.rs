use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{BitXor, Div, Shl, Shr};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "17"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

struct State {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        // TODO: Solve Part 1 of the puzzle
        let lines = reader
            .lines()
            .filter_map(Result::ok)
            .filter(|s| !s.is_empty())
            .collect_vec();

        let l = lines.iter().map(|s| s.split(": ").last()).flatten().collect_vec();
        let program= l.get(3).unwrap().split(",").map(|s| s.parse::<u8>()).filter_map(Result::ok).collect_vec();

        let mut state = State{
            a: l.get(0).unwrap().parse::<u64>()?,
            b: l.get(1).unwrap().parse::<u64>()?,
            c: l.get(2).unwrap().parse::<u64>()?,
            pc: 0,
        };

        let outputs = run(&program, state);

        let answer = outputs.iter().map(|s| s.to_string()).join(",");
        Ok(answer)
    }

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        // TODO: Solve Part 1 of the puzzle
        let lines = reader
            .lines()
            .filter_map(Result::ok)
            .filter(|s| !s.is_empty())
            .collect_vec();

        let l = lines.iter().map(|s| s.split(": ").last()).flatten().collect_vec();
        let program= l.get(3).unwrap().split(",").map(|s| s.parse::<u8>()).filter_map(Result::ok).collect_vec();

        // for c in 0o1035510045136000..=0o1035510046000000 {
        // for c in 0o10355100451300..0o10355100451400 {
        for c in 0o1035510045136764..0o1035510045136765 {
            let mut state = State {
                a: c,
                b: 0,
                c: 0,
                pc: 0,
            };

            let outputs = run(&program, state);
            let answer = outputs.iter().map(|s| s.to_string()).join(",");
            println!("{} {:o} {} {} {}", answer, c, c, program.len(), outputs.len());
        }
        Ok("".into())
    }

    fn run(program: &Vec<u8>, initial: State) -> Vec<u8> {
        let mut outputs = Vec::new();

        let mut state = initial;
        while let Some(code) = program.get(state.pc..state.pc + 2) {
            state = match code[0] {
                0 => State { // adv
                    a: state.a.shr(combo(&state, code[1])),
                    pc: state.pc + 2,
                    ..state
                },
                1 => State { // bxl
                    b: state.b.bitxor(code[1] as u64),
                    pc: state.pc + 2,
                    ..state
                },
                2 => State { // bst
                    b: combo(&state, code[1]) & 0x7,
                    pc: state.pc + 2,
                    ..state
                },
                3 => if state.a != 0 {
                    State { // jnz
                        b: combo(&state, code[1]),
                        pc: code[1] as usize,
                        ..state
                    }
                } else {
                    State {
                        pc: state.pc + 2,
                        ..state
                    }
                },
                4 => State { // bxc
                    b: state.b.bitxor(state.c),
                    pc: state.pc + 2,
                    ..state
                },
                5 => { // out
                    let output = combo(&state, code[1]) & 0x7;
                    outputs.push(output as u8);
                    State {
                        pc: state.pc + 2,
                        ..state
                    }
                },
                6 => State { // bdv
                    b: state.a.shr(combo(&state, code[1])),
                    pc: state.pc + 2,
                    ..state
                },
                7 => State { // cdv
                    c: state.a.shr(combo(&state, code[1])),
                    pc: state.pc + 2,
                    ..state
                },
                _ => panic!("invalid instruction"),
            }
        }
        outputs
    }

    fn combo(p0: &State, p: u8) -> u64 {
        match p {
            0..=3 => p as u64,
            4 => p0.a,
            5 => p0.b,
            6 => p0.c,
            _ => panic!("invalid combo"),
        }
    }

    // TODO: Set the expected answer for the test input
    assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
