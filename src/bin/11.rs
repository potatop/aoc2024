use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                // iteration -> [stone: count]
                let cache: HashMap<usize, RefCell<HashMap<usize, usize>>> =
                    HashMap::from_iter((0..=25).map(|i| (i, RefCell::new(HashMap::new()))));
                line.split(" ")
                    .map(|st| st.parse().unwrap())
                    .map(|stone| count_change(stone, 25, &cache))
                    .sum()
            })
            .collect_vec();
        Ok(answer[0])
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

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
            .map(|line| {
                let cache: HashMap<usize, RefCell<HashMap<usize, usize>>> =
                    HashMap::from_iter((0..=75).map(|i| (i, RefCell::new(HashMap::new()))));
                line.split(" ")
                    .map(|st| st.parse().unwrap())
                    .map(|stone| count_change(stone, 75, &cache))
                    .sum()
            })
            .collect_vec();
        Ok(answer[0])
    }

    assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn count_change(
    stone: usize,
    iteration: usize,
    cache: &HashMap<usize, RefCell<HashMap<usize, usize>>>,
) -> usize {
    if iteration == 0 {
        return 1;
    }

    *cache[&iteration]
        .borrow_mut()
        .entry(stone)
        .or_insert_with(|| {
            let s = stone.to_string();
            if stone == 0 {
                count_change(1, iteration - 1, cache)
            } else if s.len() % 2 == 0 {
                let t = s.split_at(s.len() / 2);
                count_change(t.0.parse().unwrap(), iteration - 1, cache)
                    + count_change(t.1.parse().unwrap(), iteration - 1, cache)
            } else {
                count_change(stone * 2024, iteration - 1, cache)
            }
        })
}

fn blink(stones: &[usize]) -> Vec<usize> {
    stones
        .iter()
        .flat_map(|num| -> Vec<usize> {
            let snum = num.to_string();
            if *num == 0 {
                vec![1]
            } else if snum.len() % 2 == 0 {
                let t = snum.split_at(snum.len() / 2);
                vec![t.0.parse().unwrap(), t.1.parse().unwrap()]
            } else {
                vec![num * 2024]
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(blink(&[125, 17]), vec![253000, 1, 7]);
        assert_eq!(blink(&[253000, 1, 7]), vec![253, 0, 2024, 14168]);
        assert_eq!(
            blink(&[253, 0, 2024, 14168]),
            vec![512072, 1, 20, 24, 28676032]
        );
        let mut state = vec![125, 17];
        for _ in 0..25 {
            state = blink(&state);
        }
        assert_eq!(state.len(), 55312);
    }
}
