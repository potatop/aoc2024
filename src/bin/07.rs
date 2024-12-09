use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use radix_fmt::radix_3;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                let eq = line.split(':').collect_vec();
                assert_eq!(eq.len(), 2);
                let y = eq[0].parse::<usize>().unwrap();
                let op = eq[1]
                    .trim()
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec();

                for value in 0..2u32.pow(op.len() as u32) {
                    // let mut bits = mask & value;
                    let result = op[1..].iter().enumerate().fold(op[0], |acc, (i, &e)| {
                        if (value >> i) & 1 == 0 {
                            acc + e
                        } else {
                            acc * e
                        }
                    });
                    if y == result {
                        return Some(y);
                    }
                }
                None
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                let eq = line.split(':').collect_vec();
                assert_eq!(eq.len(), 2);
                let y = eq[0].parse::<usize>().unwrap();
                let nums = eq[1]
                    .trim()
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec();

                for t in 0..3u32.pow(nums.len() as u32) {
                    let ops = format!("{:0>20}", radix_3(t).to_string())
                        .chars()
                        .rev()
                        .collect_vec();

                    let result = nums[1..]
                        .iter()
                        .enumerate()
                        .try_fold(nums[0], |acc, (i, &n)| {
                            if acc > y {
                                return None;
                            }
                            if ops[i] == '0' {
                                acc.checked_add(n)
                            } else if ops[i] == '1' {
                                acc.checked_mul(n)
                            } else {
                                concat(acc, n)
                            }
                        });
                    if let Some(val) = result {
                        if y == val {
                            // println!("{}", true);
                            return Some(y);
                        }
                    }
                }
                None
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn concat(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(
        10usize
            .checked_pow(b.checked_ilog10().unwrap().checked_add(1).unwrap())
            .unwrap(),
    )
    .unwrap()
    .checked_add(b)
    // a * 10usize.pow(b.ilog10() + 1) + b
}
