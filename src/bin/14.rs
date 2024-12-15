use adv_code_2024::util::grid::Grid;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

struct Wrapping(isize, isize);

impl Wrapping {
    fn wrapping_add(&self, lhs: isize, rhs: isize) -> isize {
        let y = lhs + rhs;
        if y < self.0 {
            self.1 + y
        } else if y >= self.1 {
            y - self.1
        } else {
            y
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let (height, width) = if input.lines().count() > 20 {
            (103, 101)
        } else {
            (7, 11)
        };

        let mut guards = input
            .lines()
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|part| {
                        part.trim_start_matches(|c: char| c != '-' && !c.is_ascii_digit())
                            .split(",")
                            .map(|nums| nums.parse::<isize>().unwrap())
                    })
                    .tuples()
                    .map(|(sx, sy, vx, vy)| {
                        assert!(sy < height && sx < width);
                        ((sy, sx), (vy, vx))
                    })
            })
            .collect_vec();

        let wrap_h = Wrapping(0, height);
        let wrap_w = Wrapping(0, width);
        for _ in 0..100 {
            for (s, v) in guards.iter_mut() {
                s.0 = wrap_h.wrapping_add(s.0, v.0);
                s.1 = wrap_w.wrapping_add(s.1, v.1);
            }
        }

        let sectors = guards.iter().fold(HashMap::new(), |mut acc, (l, _)| {
            match l {
                (y, x) if *y < height / 2 && *x < width / 2 => {
                    let entry = acc.entry(0).or_insert(0);
                    *entry += 1;
                }
                (y, x) if *y < height / 2 && *x > width / 2 => {
                    let entry = acc.entry(1).or_insert(0);
                    *entry += 1;
                }
                (y, x) if *y > height / 2 && *x < width / 2 => {
                    let entry = acc.entry(2).or_insert(0);
                    *entry += 1;
                }
                (y, x) if *y > height / 2 && *x > width / 2 => {
                    let entry = acc.entry(3).or_insert(0);
                    *entry += 1;
                }
                (_, _) => (),
            };

            acc
        });

        let answer = sectors.iter().fold(1, |acc, entry| acc * entry.1);
        Ok(answer)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let (height, width) = if input.lines().count() > 20 {
            (103, 101)
        } else {
            (7, 11)
        };

        let mut guards = input
            .lines()
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|part| {
                        part.trim_start_matches(|c: char| c != '-' && !c.is_ascii_digit())
                            .split(",")
                            .map(|nums| nums.parse::<isize>().unwrap())
                    })
                    .tuples()
                    .map(|(sx, sy, vx, vy)| {
                        assert!(sy < height && sx < width);
                        ((sy, sx), (vy, vx))
                    })
            })
            .collect_vec();

        let wrap_h = Wrapping(0, height);
        let wrap_w = Wrapping(0, width);

        for i in 1.. {
            for (s, v) in guards.iter_mut() {
                s.0 = wrap_h.wrapping_add(s.0, v.0);
                s.1 = wrap_w.wrapping_add(s.1, v.1);
            }

            if guards.iter().map(|(l, _)| l).all_unique() {
                let mut grid = Grid::new(
                    vec!["."; (height * width) as usize],
                    height as usize,
                    width as usize,
                );
                for (l, _) in guards.iter() {
                    // println!("{:?}", l);

                    grid.set(&(l.0 as usize, l.1 as usize), "@");
                }
                println!("{}", grid);
                return Ok(i as usize);
            }
        }

        Ok(0)
    }

    assert_eq!(1, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
