use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    fn turn_right(&self) -> Self {
        match self {
            Directions::Up => Directions::Right,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
            Directions::Right => Directions::Down,
        }
    }

    /// Returns (row, col) offsets.
    fn offsets(&self) -> (isize, isize) {
        match self {
            Directions::Up => (-1, 0),
            Directions::Down => (1, 0),
            Directions::Left => (0, -1),
            Directions::Right => (0, 1),
        }
    }
}

struct Grid {
    data: Vec<Vec<u8>>,
    cols: usize,
    rows: usize,
}

impl Grid {
    fn new<R: BufRead>(reader: R) -> Self {
        let d = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.bytes().collect_vec())
            .collect_vec();
        let rsize = d.len();
        let csize = d[0].len();

        for row in &d {
            assert!(row.len() == csize)
        }
        // println!("{:?}", rsize);
        // println!("{:?}", csize);
        // println!("{:?}", d);
        Grid {
            data: d,
            cols: csize,
            rows: rsize,
        }
    }

    fn find(&self, val: u8) -> Option<(usize, usize)> {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .find(|(r, c)| self.data[*r][*c] == val)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let grid = &Grid::new(reader);

        if let Some((row, col)) = grid.find(b'^') {
            let mut direction = Directions::Up;
            let mut pos = (Some(row), Some(col));

            while let (Some(newr), Some(newc)) = pos {
                if newr < grid.rows && newc < grid.cols {
                    if grid.data[newr][newc] == b'#' {
                        pos = (
                            newr.checked_add_signed(
                                -direction.offsets().0 | direction.turn_right().offsets().0,
                            ),
                            newc.checked_add_signed(
                                -direction.offsets().1 | direction.turn_right().offsets().1,
                            ),
                        );
                        direction = direction.turn_right();
                    } else {
                        seen.insert((newr, newc));
                        pos = (
                            newr.checked_add_signed(direction.offsets().0),
                            newc.checked_add_signed(direction.offsets().1),
                        );
                    }
                } else {
                    break;
                }
            }
        }
        Ok(seen.len())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = &Grid::new(reader);

        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let ostart = grid.find(b'^');
        if let Some((row, col)) = ostart {
            let mut direction = Directions::Up;
            let mut pos = (Some(row), Some(col));

            while let (Some(newr), Some(newc)) = pos {
                if newr < grid.rows && newc < grid.cols {
                    if grid.data[newr][newc] == b'#' {
                        pos = (
                            newr.checked_add_signed(
                                -direction.offsets().0 | direction.turn_right().offsets().0,
                            ),
                            newc.checked_add_signed(
                                -direction.offsets().1 | direction.turn_right().offsets().1,
                            ),
                        );
                        direction = direction.turn_right();
                    } else {
                        seen.insert((newr, newc));
                        pos = (
                            newr.checked_add_signed(direction.offsets().0),
                            newc.checked_add_signed(direction.offsets().1),
                        );
                    }
                } else {
                    break;
                }
            }
        }

        let answer = seen
            .iter()
            .filter(|&loc| {
                let mut seen2: HashSet<(usize, usize, Directions)> = HashSet::new();
                // let mut answer = 0;
                if let Some((row, col)) = ostart {
                    let mut direction = Directions::Up;
                    let mut pos = (Some(row), Some(col));
                    while let (Some(newr), Some(newc)) = pos {
                        if newr < grid.rows && newc < grid.cols {
                            if grid.data[newr][newc] == b'#' || *loc == (newr, newc) {
                                pos = (
                                    newr.checked_add_signed(
                                        -direction.offsets().0 | direction.turn_right().offsets().0,
                                    ),
                                    newc.checked_add_signed(
                                        -direction.offsets().1 | direction.turn_right().offsets().1,
                                    ),
                                );
                                direction = direction.turn_right();
                            } else {
                                if !seen2.insert((newr, newc, direction)) {
                                    return true;
                                }
                                pos = (
                                    newr.checked_add_signed(direction.offsets().0),
                                    newc.checked_add_signed(direction.offsets().1),
                                );
                            }
                        } else {
                            return false;
                        }
                    }
                }
                false
            })
            .count();

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
