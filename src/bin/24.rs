use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "24"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"; // TODO: Add the test input

const TEST2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"; // TODO: Add the test input

type Wire = String;

#[derive(Clone)]
struct Gate {
    t: String,
    left: Wire,
    right: Wire,
    output: Wire,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn evaluate(
        wire: &Wire,
        gates: &[Gate],
        values: &mut HashMap<Wire, u8>,
        visited: &mut Vec<Wire>,
    ) -> bool {
        if values.contains_key(wire) {
            return true;
        }

        if visited.contains(wire) {
            return false;
        }

        let gate = gates.iter().find(|&g| g.output.eq(wire)).unwrap();

        visited.push(gate.output.clone());

        if !values.contains_key(&gate.left) {
            if !evaluate(&gate.left, gates, values, visited) {
                return false;
            }
        }

        if !values.contains_key(&gate.right) {
            if !evaluate(&gate.right, gates, values, visited) {
                return false;
            }
        }
        visited.remove(visited.len() - 1);

        let value = match gate.t.as_str() {
            "AND" => values.get(&gate.left).unwrap() & values.get(&gate.right).unwrap(),
            "OR" => values.get(&gate.left).unwrap() | values.get(&gate.right).unwrap(),
            "XOR" => values.get(&gate.left).unwrap() ^ values.get(&gate.right).unwrap(),
            _ => 0,
        };

        values.insert(gate.output.clone(), value);
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut values = HashMap::new();
        let mut initials_flag = true;

        let mut gates = Vec::new();

        let lines = reader.lines().filter_map(Result::ok).collect_vec();

        for line in lines {
            if line.is_empty() {
                initials_flag = false;
                continue;
            }

            if initials_flag {
                let split = line.split(": ").collect::<Vec<_>>();
                let wire = split[0].to_string() as Wire;
                let value = split[1].parse::<u8>()?;
                values.insert(wire, value);
            } else {
                //x00 AND y00 -> z00
                let split = line.split(" -> ").collect::<Vec<_>>();
                let logic = split[0].split(" ").collect::<Vec<_>>();
                let gate = Gate {
                    t: logic[1].to_string(),
                    left: logic[0].to_string() as Wire,
                    right: logic[2].to_string() as Wire,
                    output: split[1].to_string() as Wire,
                };
                gates.push(gate);
            }
        }

        let mut visited = Vec::new();
        for gate in &gates {
            evaluate(&gate.output, &gates, &mut values, &mut visited);
        }

        let result = values
            .iter()
            .filter(|(key, value)| key.starts_with("z"))
            .sorted_by_key(|i| i.0)
            .map(|i| i.1)
            .rev()
            .join("");

        let answer = usize::from_str_radix(&result, 2)?;
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    //assert_eq!(4, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(2024, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut values = HashMap::new();
        let mut initials_flag = true;

        let mut gates = Vec::new();

        let lines = reader.lines().filter_map(Result::ok).collect_vec();

        for line in lines {
            if line.is_empty() {
                initials_flag = false;
                continue;
            }

            if initials_flag {
                let split = line.split(": ").collect::<Vec<_>>();
                let wire = split[0].to_string() as Wire;
                let value = split[1].parse::<u8>()?;
                values.insert(wire, value);
            } else {
                //x00 AND y00 -> z00
                let split = line.split(" -> ").collect::<Vec<_>>();
                let logic = split[0].split(" ").collect::<Vec<_>>();
                let gate = Gate {
                    t: logic[1].to_string(),
                    left: logic[0].to_string() as Wire,
                    right: logic[2].to_string() as Wire,
                    output: split[1].to_string() as Wire,
                };
                gates.push(gate);
            }
        }

        let wires = gates
            .iter()
            .flat_map(|gate| vec![gate.left.clone(), gate.right.clone(), gate.output.clone()])
            .filter(|w| !(w.starts_with("x") || w.starts_with("y")))
            .unique()
            .collect_vec();

        let swap = ("".to_string(), "".to_string());

        let initial_fit = fit(&mut gates);
        let initial_fit0 = fit0(&mut gates);

        for (i, p) in wires
            .iter()
            .permutations(2)
            .map(|p| (p[0].clone(), p[1].clone()))
            .enumerate()
        {
            let mut ng = gates.iter().cloned().collect::<Vec<_>>();
            swap_outputs(&mut ng, &p);
            if fit(&mut ng) < initial_fit && fit0(&mut ng) < initial_fit0 {
                println!("{:?}", p);
            }
        }

        //print(&gates);

        Ok(0)
    }

    fn swap_outputs(gates: &mut Vec<Gate>, p1: &(String, String)) {
        for gate in gates {
            if gate.output == p1.0 {
                gate.output = p1.1.clone();
                continue;
            }
            if gate.output == p1.1 {
                gate.output = p1.0.clone();
                continue;
            }
            if gate.left == p1.0 {
                gate.left = p1.1.clone();
                continue;
            }
            if gate.left == p1.1 {
                gate.left = p1.0.clone();
                continue;
            }
            if gate.right == p1.0 {
                gate.right = p1.1.clone();
                continue;
            }
            if gate.right == p1.1 {
                gate.right = p1.0.clone();
                continue;
            }
        }
    }

    fn fit(gates: &[Gate]) -> u32 {
        let mut errors = 0;
        let mut visited = Vec::new();

        for k in 0..=44 {
            let mut values = HashMap::new();

            for i in 0..=44 {
                values.insert(format!("x{:02}", i), if i == k { 1 } else { 0 });
                values.insert(format!("y{:02}", i), if i == k { 1 } else { 0 });
            }

            for gate in gates {
                if !evaluate(&gate.output, gates, &mut values, &mut visited) {
                    return u32::MAX;
                }
            }

            if values.get(&format!("z{:02}", k + 1)).unwrap().ne(&1) {
                errors += 1;
            }
        }

        errors
    }
    
    fn fit0(gates: &[Gate]) -> u32 {
        let mut errors = 0;
        let mut visited = Vec::new();

        let mut values = HashMap::new();

        for i in 0..=44 {
            values.insert(format!("x{:02}", i), 1);
            values.insert(format!("y{:02}", i), 1);
        }

        for gate in gates {
            if !evaluate(&gate.output, gates, &mut values, &mut visited) {
                return u32::MAX;
            }
        }

        for k in 1..=45 {
            if values.get(&format!("z{:02}", k)).unwrap().ne(&1) {
                errors += 0x100;
            }
        }
        errors
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
