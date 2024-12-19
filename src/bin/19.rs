use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut iter = reader.lines().map_while(Result::ok);
        let p_input = iter.next().unwrap();
        let patterns = p_input.split(", ").collect_vec();
        let designs = iter.skip(1).collect_vec();
        let answer = designs
            .iter()
            .filter(|&design| find_patterns(design, &patterns, &mut HashMap::new()))
            .count();

        Ok(answer)
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut iter = reader.lines().map_while(Result::ok);
        let p_input = iter.next().unwrap();
        let patterns = p_input.split(", ").collect_vec();

        let mut seen = HashMap::new();
        let designs = iter.skip(1).collect_vec();
        let answer = designs
            .iter()
            .map(|design| find_pattern_count(design, &patterns, &mut seen))
            .sum();

        Ok(answer)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn find_patterns(s: &str, patterns: &[&str], seen: &mut HashMap<String, bool>) -> bool {
    if let Some(&ret) = seen.get(s) {
        return ret;
    }
    if s.is_empty() {
        return true;
    }

    for &p in patterns {
        if let Some(remaining) = s.strip_prefix(p) {
            if find_patterns(remaining, patterns, seen) {
                seen.insert(s.to_owned(), true);
                return true;
            }
        }
    }
    seen.insert(s.to_owned(), false);
    false
}

fn find_pattern_count(s: &str, patterns: &[&str], seen: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = seen.get(s) {
        return count;
    }
    if s.is_empty() {
        return 0;
    }
    let mut count = 0;
    for &p in patterns {
        if s == p {
            count += 1;
            continue;
        }
        if let Some(remaining) = s.strip_prefix(p) {
            count += find_pattern_count(remaining, patterns, seen);
        }
    }
    seen.insert(s.to_owned(), count);
    count
}
