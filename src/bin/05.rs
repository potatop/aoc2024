use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, orders) = rules_orders_from_reader(reader);

        let answer = orders
            .iter()
            .filter(|&o| is_order_correct(o, &rules))
            .map(|o| o[o.len() / 2])
            .sum();

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, orders) = rules_orders_from_reader(reader);
        let answer = orders
            .iter()
            .filter(|&o| !is_order_correct(o, &rules))
            .map(|o| {
                let mut oo = o.to_owned();
                oo.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        return Ordering::Less;
                    }
                    Ordering::Greater
                });
                oo[oo.len() / 2]
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn rules_orders_from_reader<R: BufRead>(reader: R) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    reader.lines().map_while(Result::ok).fold(
        (HashSet::new(), Vec::new()),
        |(mut rule, mut order): (HashSet<(usize, usize)>, Vec<Vec<usize>>), line: String| {
            if line.is_empty() {
                return (rule, order);
            }
            if line.contains("|") {
                let d = line.split('|').collect_vec();
                rule.insert((d[0].parse().unwrap(), d[1].parse().unwrap()));
            } else {
                order.push(line.split(',').map(|s| s.parse().unwrap()).collect_vec());
            }
            (rule, order)
        },
    )
}

fn is_order_correct(order: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    for (i, p) in order[..order.len() - 1].iter().enumerate() {
        for e in &order[i + 1..] {
            if !rules.contains(&(*p, *e)) {
                return false;
            }
        }
    }
    true
}
