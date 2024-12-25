use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::arena_tree::{ArenaTree, Node};

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let mut split = input.split("\n\n");

        let part = split.next().unwrap();
        let mut at = ArenaTree::new();
        let mut values = part
            .lines()
            .map(|line| {
                let mut sp = line.split(": ");
                let key = sp.next().unwrap();
                at.node(key.to_owned());
                (key.to_owned(), sp.next().unwrap().parse::<u8>().unwrap())
            })
            .collect::<HashMap<_, _>>();

        let part = split.next().unwrap();
        part.lines().for_each(|line| {
            let mut sp = line.split(" -> ");
            let children = sp
                .next()
                .unwrap()
                .split(" ")
                .map(|n| at.node(n.to_owned()))
                .collect_vec();
            let nid = at.node(sp.next().unwrap().to_owned());
            at.arena[nid].children.extend(children.iter());
        });

        let result = at
            .arena
            .iter()
            .filter(|node| node.val.starts_with("z"))
            .sorted_by(|a, b| b.val.cmp(&a.val))
            .map(|node| try_answer(node, &mut values, &at).unwrap())
            .collect_vec();

        let x = at
            .arena
            .iter()
            .filter(|node| node.val.starts_with("x"))
            .sorted_by(|a, b| b.val.cmp(&a.val))
            .map(|node| values[&node.val])
            .join("");
        let y = at
            .arena
            .iter()
            .filter(|node| node.val.starts_with("y"))
            .sorted_by(|a, b| b.val.cmp(&a.val))
            .map(|node| values[&node.val])
            .join("");
        println!("{:?}", (x, y));

        println!("{:?}", result.iter().join(""));
        Ok(usize::from_str_radix(&result.into_iter().join(""), 2).expect("Not a binary number!"))
    }

    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut at = ArenaTree::new();
        reader
            .lines()
            .map_while(Result::ok)
            .skip_while(|line| !line.contains("->"))
            .for_each(|line| {
                let mut sp = line.split(" -> ");
                let children = sp
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|n| at.node(n.to_owned()))
                    .collect_vec();
                let nid = at.node(sp.next().unwrap().to_owned());
                at.arena[nid].children.extend(children.iter());
            });

        let xori = at.node(String::from("XOR"));
        let ori = at.node(String::from("OR"));
        println!("xor index {}", xori);
        at.arena
            .iter()
            .filter(|node| node.val.starts_with("z"))
            .for_each(|node| {
                if node.val == "z45" || node.val == "z00" {
                    return;
                }
                if !node.children.contains(&xori) {
                    println!("bad sum {:?}", node);
                    return;
                }
                for ci in node.children.iter().filter(|ci| **ci != xori) {
                    let cn = &at.arena[*ci];
                    if cn.children.contains(&xori)
                    {
                        if cn
                        .children
                        .iter()
                        .filter(|ici| **ici == xori)
                        .all(|ici| at.arena[*ici].val.ends_with(&node.val[1..])){
                            println!("bad inner sum {:?} {:?}", &node.val[1..], cn);
                        }
                    } else if !cn.children.contains(&ori) {
                        println!("bad carry {:?}", cn);
                    }
                }
                
            });
        let mut result = ["z07", "bjm", "z13", "hsw", "skf", "z18", "nvr", "wkr"];
        result.sort();
        println!("{}", result.join(","));
        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn try_answer(
    node: &Node<String>,
    values: &mut HashMap<String, u8>,
    at: &ArenaTree<String>,
) -> Option<u8> {
    if let Some(v) = values.get(&node.val) {
        return Some(*v);
    }
    let mut gate = String::new();
    let mut inputs = Vec::new();
    for ci in node.children.iter() {
        let cn: &Node<String> = &at.arena[*ci];
        if cn.val == "AND" || cn.val == "OR" || cn.val == "XOR" {
            gate = cn.val.clone();
        } else if let Some(input) = try_answer(cn, values, at) {
            inputs.push(input);
        } else {
            return None;
        }
    }

    let result = match gate.as_str() {
        "AND" => inputs[0] & inputs[1],
        "OR" => inputs[0] | inputs[1],
        "XOR" => inputs[0] ^ inputs[1],
        _ => unreachable!(),
    };
    values.insert(node.val.clone(), result);
    Some(result)
}
