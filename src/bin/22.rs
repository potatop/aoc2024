use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
";

const TEST2: &str = "\
1
2
3
2024
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer: isize = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let mut secret = line.parse().unwrap();
                (0..2000).for_each(|_| secret = evolve_next(secret));
                secret
            })
            .sum();
        Ok(answer.try_into()?)
    }

    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let price_bucket = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let mut secrets = vec![line.parse::<isize>().unwrap()];
                (0..2000).for_each(|_| secrets.push(evolve_next(secrets[secrets.len() - 1])));
                secrets
                    .into_iter()
                    .rev()
                    .tuple_windows()
                    .map(|(a, b, c, d, e)| {
                        (
                            (
                                (d % 10) - (e % 10),
                                (c % 10) - (d % 10),
                                (b % 10) - (c % 10),
                                (a % 10) - (b % 10),
                            ),
                            a % 10,
                        )
                    })
                    .collect::<HashMap<_, _>>()
            })
            .fold(HashMap::new(), |mut acc, m| {
                m.into_iter()
                    .for_each(|(k, v)| *acc.entry(k).or_default() += v);
                acc
            });

        let answer: isize = *price_bucket.values().max().unwrap();
        Ok(answer.try_into()?)
    }

    assert_eq!(23, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

const PRUNE_MAGIC: isize = 16777216;
fn evolve_next(init: isize) -> isize {
    let mut result = (init ^ (init * 64)) % PRUNE_MAGIC;
    result = (result ^ (result / 32)) % PRUNE_MAGIC;
    result = (result ^ (result * 2048)) % PRUNE_MAGIC;
    result
}

#[test]
fn test_next() {
    assert_eq!(evolve_next(123), 15887950);
    assert_eq!(evolve_next(15887950), 16495136);
    assert_eq!(evolve_next(16495136), 527345);
    assert_eq!(evolve_next(527345), 704524);
    assert_eq!(evolve_next(704524), 1553684);
    assert_eq!(evolve_next(1553684), 12683156);
    assert_eq!(evolve_next(12683156), 11100544);
    assert_eq!(evolve_next(11100544), 12249484);
    assert_eq!(evolve_next(12249484), 7753432);
    assert_eq!(evolve_next(7753432), 5908254);
}
