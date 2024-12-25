use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::arena_tree::ArenaTree;

const DAY: &str = "23";
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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut arena_graph = ArenaTree::<String>::new();
        reader.lines().map_while(Result::ok).for_each(|line| {
            if let Some((a, b)) = line.split('-').collect_tuple() {
                let c1 = arena_graph.node(a.to_owned());
                let c2 = arena_graph.node(b.to_owned());
                arena_graph.arena[c1].children.insert(c2);
                arena_graph.arena[c2].children.insert(c1);
            }
        });

        let mut v = HashSet::new();
        for node in &arena_graph.arena {
            if node.val.starts_with("t") {
                for c in &node.children {
                    for c1 in node.children.intersection(&arena_graph.arena[*c].children) {
                        let mut triangle = [node.idx, *c, *c1];
                        triangle.sort();
                        v.insert(triangle);
                    }
                }
            }
        }

        let answer = v.len();
        Ok(answer)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // let mut arena_graph = ArenaTree::<String>::new();
        let mut computers = HashSet::new();
        let mut connections = HashSet::new();
        reader.lines().map_while(Result::ok).for_each(|line| {
            if let Some((a, b)) = line.split('-').collect_tuple() {
                computers.extend([a.to_owned(), b.to_owned()]);
                connections.extend([(a.to_owned(), b.to_owned()), (b.to_owned(), a.to_owned())]);
            }
        });

        let mut networks = computers.iter().map(|c| HashSet::from([c])).collect_vec();

        for network in &mut networks {
            for c1 in computers.iter() {
                if network
                    .iter()
                    .all(|&c0| connections.contains(&(c0.clone(), c1.clone())))
                {
                    network.insert(c1);
                }
            }
        }

        let n = networks.iter().max_by_key(|s| s.len()).unwrap();
        println!("{:?}", n.iter().sorted().join(","));

        Ok(n.len())
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
