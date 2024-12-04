use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        match read(reader) {
            Result::Ok((mut left, mut right)) => {
                left.sort();
                right.sort();
                let answer: usize = left
                    .into_iter()
                    .zip(right)
                    .map(|(l, r)| (l.abs_diff(r)))
                    .sum();
                Ok(answer)
            }
            Err(e) => Err(e),
        }
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        match read(reader) {
            Result::Ok((left, right)) => {
                let answer: usize = left
                    .into_iter()
                    .map(|e| e * right.iter().filter(|&&ee| ee == e).count())
                    .sum();
                Ok(answer)
            }
            Err(e) => Err(e),
        }
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>)> {
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip())
}
