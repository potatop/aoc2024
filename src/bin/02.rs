use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer: usize = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let mut iter = line
                    .split_whitespace()
                    .flat_map(|i| i.parse::<i32>())
                    .multipeek();

                while let Some(cur) = iter.next() {
                    if let Some(&n0) = iter.peek() {
                        if let Some(&n1) = iter.peek() {
                            if is_safe_with_signum(cur, n0, n1).is_none() {
                                return false;
                            }
                        }
                    }
                }
                true
            })
            .filter(|&a| a)
            .count();

        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer: usize = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let mut row = line
                    .split_whitespace()
                    .flat_map(|i| i.parse::<i32>())
                    .collect::<Vec<i32>>();
                part2_is_safe(&mut row)
            })
            .filter(|&a| a)
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn part2_is_safe(row: &mut [i32]) -> bool {
    let mut once = true;
    let mut signum: Option<i32> = None;
    for i in 0..row.len() - 2 {
        let &cur = row.get(i).unwrap();
        let &n0 = row.get(i + 1).unwrap();
        let &n1 = row.get(i + 2).unwrap();
        if let Some(s) = is_safe_with_signum(cur, n0, n1) {
            if *signum.get_or_insert(s) == s {
                continue;
            }
        }
        if !once {
            return once;
        }
        once = false;
        if i + 3 >= row.len() {
            return true;
        }
        let &z = row.get(i + 3).unwrap();
        if let Some(s) = is_safe_with_signum(n0, n1, z) {
            if *signum.get_or_insert(s) == s {
                continue;
            }
        }

        if let Some(s) = is_safe_with_signum(cur, n1, z) {
            row[i + 1] = cur;
            if *signum.get_or_insert(s) == s {
                continue;
            }
        }
        if let Some(s) = is_safe_with_signum(cur, n0, z) {
            row[i + 1] = cur;
            row[i + 2] = n0;
            if *signum.get_or_insert(s) == s {
                continue;
            }
        }
    }
    true
}

fn is_safe_with_signum(a: i32, b: i32, c: i32) -> Option<i32> {
    let d1 = a.abs_diff(b);
    let d2 = b.abs_diff(c);
    let s1 = (a - b).signum();
    let s2 = (b - c).signum();

    if s1 == s2 && s1 != 0 && d1 <= 3 && d2 <= 3 {
        return Some(s1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_is_safe() {
        assert!(part2_is_safe(&mut [7, 6, 4, 2, 1]));
        assert!(!part2_is_safe(&mut [1, 2, 7, 8, 9]));
        assert!(!part2_is_safe(&mut [9, 7, 6, 2, 1]));
        assert!(part2_is_safe(&mut [1, 3, 2, 4, 5]));
        assert!(part2_is_safe(&mut [8, 6, 4, 4, 1]));
        assert!(part2_is_safe(&mut [1, 3, 6, 7, 9]));
        assert!(part2_is_safe(&mut [2, 1, 3, 4, 5]));
        assert!(part2_is_safe(&mut [1, 6, 3, 4, 5]));
        assert!(part2_is_safe(&mut [1, 2, 3, 4, 9]));
        assert!(part2_is_safe(&mut [3, 5, 7, 9, 8]));
        assert!(!part2_is_safe(&mut [1, 1, 1, 2, 3]));
        assert!(part2_is_safe(&mut [1, 2, 3, 2, 4]));
        assert!(!part2_is_safe(&mut [23, 20, 18, 21, 24]));
    }
}
