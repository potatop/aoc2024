use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
...........#.....#......
...................#....
...#.....##.............
......................#.
..................#.....
..#.....................
....................#...
........................
.#........^.............
..........#..........#..
..#.....#..........#....
........#.....#..#......
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
    width: usize,
    height: usize,
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
            width: csize,
            height: rsize,
        }
    }

    fn find(&self, val: u8) -> Option<(usize, usize)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .find(|(r, c)| self.data[*r][*c] == val)
    }
}

struct Matrix {
    array: Vec<Vec<u8>>,
    height: usize,
    // width: usize,
}
impl Matrix {
    fn new(h: usize, w: usize) -> Self {
        Matrix {
            array: vec![vec![0; w * h]; w * h],
            height: h,
        }
    }

    fn set(&mut self, p1: &(usize, usize), p2: &(usize, usize)) {
        self.array[p1.0 * self.height + p1.1][p2.0 * self.height + p2.1] = 1;
    }

    fn get_ns(&self, p: &(usize, usize)) -> Vec<(usize, usize)> {
        self.array[p.0 * self.height + p.1]
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| {
                if n == 1 {
                    return Some((i / self.height, i % self.height));
                }
                None
            })
            .collect_vec()
    }
    // fn get(&self, p1: &(usize, usize), p2: &(usize, usize)) -> u8 {
    //     self.array[p1.0 * self.height + p1.1][p2.0 * self.height + p2.1]
    // }
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
                if newr < grid.height && newc < grid.width {
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

    assert_eq!(91, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // let mut answer = 0;
        let grid = &Grid::new(reader);
        let mut seen = Matrix::new(grid.height, grid.width);
        let mut path: HashSet<(usize, usize, Directions)> = HashSet::new();

        let start = grid.find(b'^');
        if let Some((row, col)) = start {
            let mut direction = Directions::Up;
            let mut pos = (Some(row), Some(col));
            let mut source = None;
            while let (Some(newr), Some(newc)) = pos {
                if newr < grid.height && newc < grid.width {
                    if grid.data[newr][newc] == b'#' {
                        let cur_pos = (
                            newr.saturating_add_signed(-direction.offsets().0),
                            newc.saturating_add_signed(-direction.offsets().1),
                        );
                        if let Some(s) = source {
                            seen.set(&s, &cur_pos);
                        }
                        source = Some(cur_pos);
                        direction = direction.turn_right();
                        pos = (
                            cur_pos.0.checked_add_signed(direction.offsets().0),
                            cur_pos.1.checked_add_signed(direction.offsets().1),
                        );
                    } else {
                        // if is_cycle(&(newr, newc, direction), grid, &seen) {
                        //     // println!("{:?}", (newr, newc, direction));
                        //     answer += 1;
                        // }
                        path.insert((newr, newc, direction));
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
        let res = path.iter().filter(|&pos| is_cycle(pos, &grid, &seen));
        println!("{:?}", res);
        let answer = res.count();
        Ok(answer)
    }

    assert_eq!(19, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {:?}", result);
    //endregion

    Ok(())
}

fn is_cycle(pos: &(usize, usize, Directions), grid: &Grid, matrix: &Matrix) -> bool {
    let mut discovered = HashSet::new();
    discovered.insert((pos.0, pos.1));
    let mut neighbors = find_neighbors(pos, grid);
    // let mut previous = None;
    while let Some(ne) = neighbors.pop_front() {
        if !discovered.insert(ne) {
            // println!("discovered = {:?}", discovered);
            return true;
        }
        for x in matrix.get_ns(&ne) {
            // if !discovered.contains(&x) {
            neighbors.push_back(x);
            // }
        }
        // previous = Some(ne);
    }
    false
}

fn find_neighbors(
    anchor: &(usize, usize, Directions),
    // other: Option<(usize, usize)>,
    // visited: &HashSet<(usize, usize)>,
    grid: &Grid,
) -> VecDeque<(usize, usize)> {
    let mut result = VecDeque::new();
    let directions = match anchor.2 {
        Directions::Up => (Directions::Left, Directions::Right),
        Directions::Down => (Directions::Right, Directions::Left),
        Directions::Left => (Directions::Up, Directions::Down),
        Directions::Right => (Directions::Down, Directions::Up),
    };

    let range = match anchor.2 {
        Directions::Up => anchor.0..grid.height,
        Directions::Down => 0..anchor.0,
        Directions::Left => 1..anchor.0,
        Directions::Right => anchor.0..grid.height - 1,
    };
    for i in range {
        if grid.data[i.saturating_add_signed(directions.0.offsets().0)]
            [anchor.1.saturating_add_signed(directions.0.offsets().1)]
            == b'#'
        {
            result.push_back((i, anchor.1))
        }
    }
    let range = match anchor.2 {
        Directions::Up => anchor.1..grid.width - 1,
        Directions::Down => 1..anchor.1,
        Directions::Left => anchor.1..grid.width,
        Directions::Right => 0..anchor.1,
    };
    for i in range {
        if grid.data[anchor.0.saturating_add_signed(directions.1.offsets().0)]
            [i.saturating_add_signed(directions.1.offsets().1)]
            == b'#'
        {
            result.push_back((anchor.0, i))
        }
    }
    // println!("{:?}{:?}", anchor,result);
    result
}

// fn grid_add(lhs: usize, rhs: isize, max: usize) -> usize {
//     std::cmp::min(lhs.saturating_add_signed(rhs), max - 1)
// }
//
// fn cal_limit(
//     directions: Directions,
//     previous: Option<(usize, usize, Directions)>,
//     max: (usize, usize),
// ) -> (usize, usize) {
//     match directions {
//         Directions::Up => {
//             if let Some(prev) = previous {
//                 if directions.turn_right() == prev.2 {
//                     if directions.turn_right() == prev.2 {
//                         return (max.0, prev.1);
//                     } else if prev.2.turn_right() == directions {
//                         return (prev.0, max.1 - 1);
//                     }
//                 }
//             }
//             (max.0, max.1 - 1)
//         }
//         Directions::Down => {
//             if let Some(prev) = previous {
//                 if directions.turn_right() == prev.2 {
//                     if directions.turn_right() == prev.2 {
//                         return (1, prev.1);
//                     } else if prev.2.turn_right() == directions {
//                         return (prev.0, 0);
//                     }
//                 }
//             }
//             (0, 1)
//         }
//         Directions::Left => {
//             if let Some(prev) = previous {
//                 if directions.turn_right() == prev.2 {
//                     return (prev.0, max.1);
//                 } else if prev.2.turn_right() == directions {
//                     return (1, prev.1);
//                 }
//             }
//             (1, max.1)
//         }
//         Directions::Right => {
//             if let Some(prev) = previous {
//                 if directions.turn_right() == prev.2 {
//                     return (prev.0, 0);
//                 } else if prev.2.turn_right() == directions {
//                     return (max.0 - 1, prev.1);
//                 }
//             }
//             (max.0 - 1, 0)
//         }
//     }
// }
