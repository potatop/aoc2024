use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

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
        Grid {
            data: d,
            cols: csize,
            rows: rsize,
        }
    }

    fn count_xmax(&self) -> usize {
        let mut count: usize = 0;
        for (y, row) in self.data.iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                count += match byte {
                    b'X' => self.count_mas(y, x),
                    _ => continue,
                }
            }
        }
        count
    }

    fn count_mas(&self, r: usize, c: usize) -> usize {
        let directions: [(i32, i32); 8] = [
            (0, -1),
            (-1, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];
        let target = b"MAS";
        directions
            .iter()
            .filter(|(dy, dx)| {
                target.iter().enumerate().all(|(i, &byte)| {
                    let (rr, cc) = (
                        (r as i32 + dy * (i as i32 + 1)),
                        (c as i32 + dx * (i as i32 + 1)),
                    );
                    rr >= 0
                        && rr < self.rows as i32
                        && cc >= 0
                        && cc < self.rows as i32
                        && self.data[rr as usize][cc as usize] == byte
                })
            })
            .count()
    }

    fn count_x_max(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, &byte)| {
                        byte == b'A'
                            && y >= 1
                            && y + 1 < self.rows
                            && *x >= 1
                            && x + 1 < self.cols
                            && (self.data[y - 1][x - 1] == b'M' || self.data[y - 1][x - 1] == b'S')
                            // distance from 'M' to 'S' is 6
                            // upper left to lower right
                            && self.data[y - 1][x - 1].abs_diff(self.data[y + 1][x + 1]) == 6
                            && (self.data[y - 1][x + 1] == b'M' || self.data[y - 1][x + 1] == b'S')
                            // distance from 'M' to 'S' is 6
                            // upper right to lower left
                            && self.data[y - 1][x + 1].abs_diff(self.data[y + 1][x - 1]) == 6
                    })
                    .count()
            })
            .sum()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader);

        let answer = grid.count_xmax();
        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader);

        let answer = grid.count_x_max();
        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
