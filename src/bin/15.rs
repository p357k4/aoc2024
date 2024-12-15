use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"; // TODO: Add the test input

const TEST2: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let (terrain_input, moves_input) = input(reader)?;

        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut start = (0, 0);
        let moves = moves_input.iter().flatten().collect::<Vec<_>>();

        for (row_index, row) in terrain_input.iter().enumerate() {
            for (column_index, c) in row.iter().enumerate() {
                let c = &row[column_index];
                let p = (row_index as i32, column_index as i32);

                if *c == '#' {
                    walls.insert(p);
                } else if *c == 'O' {
                    boxes.insert(p);
                } else if *c == '@' {
                    start = p;
                }
            }
        }

        let deltas = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
        let mut p = start;

        for command in moves {
            let delta = deltas.get(command).unwrap();

            let np = (p.0 + delta.0, p.1 + delta.1);
            if go1(&np, delta, &walls, &mut boxes) {
                p = np;
            }

            // render(&mut walls, &mut boxes, &mut p);
        }

        let mut sum = 0;
        for b in boxes.iter() {
            sum += 100 * b.0 + b.1;
        }

        Ok(sum)
    }
    fn input<R: BufRead>(reader: R) -> Result<(Vec<Vec<char>>, Vec<Vec<char>>), Error> {
        let mut terrain_flag = true;
        let mut terrain_input = Vec::new();
        let mut moves_input = Vec::new();
        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                terrain_flag = false;
            }

            let cs = line.chars().collect::<Vec<char>>();

            if terrain_flag {
                terrain_input.push(cs);
            } else {
                moves_input.push(cs);
            }
        }
        Ok((terrain_input, moves_input))
    }

    fn go1(
        p: &(i32, i32),
        delta: &(i32, i32),
        walls: &HashSet<(i32, i32)>,
        boxes: &mut HashSet<(i32, i32)>,
    ) -> bool {
        let np = (p.0 + delta.0, p.1 + delta.1);

        if walls.contains(&p) {
            return false;
        }

        if !boxes.contains(&p) {
            return true; // empty space
        }

        if !go1(&np, delta, walls, boxes) {
            return false;
        }

        boxes.remove(&p);
        boxes.insert(np);
        true
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let (terrain_input, moves_input) = input(reader)?;

        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut start = (0, 0);
        let moves = moves_input.iter().flatten().collect::<Vec<_>>();

        for (row_index, row) in terrain_input.iter().enumerate() {
            for (column_index, c) in row.iter().enumerate() {
                let c = &row[column_index];
                let p0 = (row_index as i32, 2 * column_index as i32);
                let p1 = (p0.0, p0.1 + 1);

                if *c == '#' {
                    walls.insert(p0);
                    walls.insert(p1);
                } else if *c == 'O' {
                    boxes.insert((p0, p1));
                } else if *c == '@' {
                    start = p0;
                }
            }
        }

        let deltas = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
        let mut p = start;

        for command in moves {
            let mut moved = HashSet::new();
            let mut marked = HashSet::new();
            let delta = deltas.get(command).unwrap();

            let np = (p.0 + delta.0, p.1 + delta.1);
            if go2(&np, delta, &walls, &mut boxes, &mut moved, &mut marked) {
                p = np;
                for m in moved {
                    boxes.insert(m);
                }
            } else {
                for m in marked {
                    boxes.insert(m);
                }
            }
        }

        let mut sum = 0;
        for b in boxes.iter() {
            sum += 100 * b.0 .0 + b.0 .1;
        }

        Ok(sum)
    }

    fn go2(
        p: &(i32, i32),
        delta: &(i32, i32),
        walls: &HashSet<(i32, i32)>,
        boxes: &mut HashSet<((i32, i32), (i32, i32))>,
        moved: &mut HashSet<((i32, i32), (i32, i32))>,
        marked: &mut HashSet<((i32, i32), (i32, i32))>,
    ) -> bool {
        if walls.contains(&p) {
            return false;
        }

        let is_box = boxes.iter().find(|(r, q)| r == p || q == p);
        if is_box.is_none() {
            return true; // empty space
        }

        let b = *is_box.unwrap();
        marked.insert(b);
        boxes.remove(&b);

        let nb = (
            (b.0 .0 + delta.0, b.0 .1 + delta.1),
            (b.1 .0 + delta.0, b.1 .1 + delta.1),
        );

        moved.insert(nb);
        if !go2(&nb.0, delta, walls, boxes, moved, marked) {
            return false;
        }

        if !go2(&nb.1, delta, walls, boxes, moved, marked) {
            return false;
        }

        true
    }

    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
