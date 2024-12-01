use std::cmp::max_by;
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
    println!("\n=== Part 2 test ===");


    // BronKerbosch(R, P, X):
    //     if P is empty and X is empty:
    //         report R as a maximal clique
    //         return
    //     for each vertex v in P:
    //         BronKerbosch(R ∪ {v}, P ∩ N(v), X ∩ N(v))
    //         P ← P \ {v}
    //         X ← X ∪ {v}
    // R starts as an empty set
    // P initially contains all vertices of the graph.
    // X is initially empty

    fn bk(r : Vec<&str>, p: Vec<&str>, x: Vec<&str>, n: &HashMap<&str, Vec<&str>>, cliques : &mut Vec<String>) {
        if p.is_empty() && x.is_empty() {
            let m = r.iter().sorted().join(",");
            cliques.push(m);
            return; // report R
        }
        
        let mut x = x.clone();

        for i in (0..p.len()).rev() {
            let v = p.get(i).unwrap();
            let nv = n.get(v).unwrap();
            let mut r_prim = r.clone();
            r_prim.push(v);

            let p_and_nv = p[0..=i].iter().filter(|&pv| nv.contains(pv)).cloned().collect::<Vec<_>>();
            let x_and_nv = x.iter().filter(|&pv| nv.contains(pv)).cloned().collect::<Vec<_>>();
            bk(
                r_prim,
                p_and_nv,
                x_and_nv,
                n,
                cliques,
            );

            x.push(v);
        }
    }

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let lines = reader.lines().filter_map(Result::ok).collect_vec();

        let mut neighbours = HashMap::new();

        let mut all_vertices = Vec::new();
        for line in &lines {
            let split: Vec<_> = line.split('-').collect();
            all_vertices.push(split[0]);
            all_vertices.push(split[1]);
            neighbours
                .entry(split[0])
                .and_modify(|vs: &mut Vec<&str>| vs.push(split[1]))
                .or_insert(vec![split[1]]);
            neighbours
                .entry(split[1])
                .and_modify(|vs: &mut Vec<&str>| vs.push(split[0]))
                .or_insert(vec![split[0]]);
        }

        let mut cliques = Vec::new();
        let vertices = all_vertices.iter().unique().cloned().collect::<Vec<_>>();
        bk(Vec::new(), vertices, Vec::new(), &neighbours, &mut cliques);
        
        let m = cliques.iter().max_by_key(|c| c.len()).cloned().unwrap();

        Ok(m)
    }

    assert_eq!("co,de,ka,ta", part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
