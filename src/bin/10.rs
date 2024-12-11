use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::grid::{Grid, Point};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

trait P10 {
    fn search_trail(&self, start: (usize, usize)) -> Vec<(usize, usize)>;
    fn get_trail_heads(&self) -> Vec<(usize, usize)>;
    fn count_distinct_paths(&self, start: Point) -> usize;
    fn up_hill(&self, loc: &(usize, usize)) -> Vec<(usize, usize)>;
}

impl P10 for Grid {
    fn get_trail_heads(&self) -> Vec<(usize, usize)> {
        self.array
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if *v == b'0' {
                    return Some((i / self.height, i % self.height));
                }
                None
            })
            .collect_vec()
    }

    fn search_trail(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut poten_steps: Vec<(usize, usize)> = vec![start];
        // let mut trail_steps: Vec<(usize, usize)>
        let mut discovered = HashSet::new();
        while let Some(v) = poten_steps.pop() {
            if !discovered.insert(v) {
                continue;
            }
            let path = self.up_hill(&v);
            poten_steps.extend(path);
        }
        Vec::from_iter(discovered)
    }

    fn up_hill(&self, loc: &(usize, usize)) -> Vec<(usize, usize)> {
        let height = self.get_at(*loc);
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .filter_map(|direction| {
                if let (Some(y), Some(x)) = (
                    loc.0.checked_add_signed(direction.0),
                    loc.1.checked_add_signed(direction.1),
                ) {
                    if y < self.height && x < self.width && height + 1 == self.get(y, x) {
                        return Some((y, x));
                    }
                }
                None
            })
            .collect_vec()
    }

    fn count_distinct_paths(&self, start: Point) -> usize {
        let mut trails: Vec<Vec<Point>> = vec![vec![start]];

        loop {
            let t = trails
                .iter()
                .flat_map(|trail| {
                    let adjacents = self.up_hill(&trail[trail.len() - 1]);
                    adjacents
                        .iter()
                        .map(|node| {
                            Vec::from_iter(trail.clone().into_iter().chain(std::iter::once(*node)))
                        })
                        .collect_vec()
                })
                .collect_vec();
            if t.is_empty() {
                return trails.len();
            }
            trails = t;
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader);

        let answer = grid
            .get_trail_heads()
            .iter()
            .map(|start| {
                grid.search_trail(*start)
                    .iter()
                    .filter(|loc| grid.get(loc.0, loc.1) == b'9')
                    .count()
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader);

        let answer = grid
            .get_trail_heads()
            .iter()
            .map(|start| grid.count_distinct_paths(*start))
            .sum();
        Ok(answer)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trail_heads() {
        let g = Grid::new(BufReader::new(TEST.as_bytes()));
        assert_eq!(
            g.get_trail_heads(),
            vec![
                (0, 2),
                (0, 4),
                (2, 4),
                (4, 6),
                (5, 2),
                (5, 5),
                (6, 0),
                (6, 6),
                (7, 1)
            ]
        );
    }

    #[test]
    fn test_works() {
        let g = Grid::new(BufReader::new(TEST.as_bytes()));
        assert_eq!(
            g.search_trail((0, 2))
                .iter()
                .filter(|loc| g.get(loc.0, loc.1) == b'9')
                .count(),
            5
        );
        assert_eq!(
            g.search_trail((0, 4))
                .iter()
                .filter(|loc| g.get(loc.0, loc.1) == b'9')
                .count(),
            6
        );
    }

    #[test]
    fn test_count_distinct_paths() {
        let g = Grid::new(BufReader::new(TEST.trim().as_bytes()));
        let tree = g.count_distinct_paths((0, 2));

        assert_eq!(tree, 20);
    }
}
