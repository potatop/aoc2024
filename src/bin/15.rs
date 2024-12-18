use adv_code_2024::util::grid::{Grid, Point};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const BOX: u8 = b'O';
const WALL: u8 = b'#';
const ROBOT: u8 = b'@';
const EMPTY: u8 = b'.';

trait P15 {
    fn find_robot_position(&self) -> Option<Point>;
    fn look_ahead_current_box(&self, cur: Point, direction: &char) -> Vec<Point>;
    fn look_ahead_current_2w_box(
        &self,
        cur: Point,
        direction: &char,
    ) -> Vec<HashSet<(Point, Point)>>;
}

impl P15 for Grid<u8> {
    fn find_robot_position(&self) -> Option<Point> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .find(|pt| self.get(pt) == ROBOT)
    }

    fn look_ahead_current_box(&self, cur: Point, direction: &char) -> Vec<Point> {
        let mut result = Vec::new();
        let mut m = cur;
        while {
            m = m.next_move(direction);
            result.push(m);
            self.get(&m) == BOX
        } {}
        result
    }

    fn look_ahead_current_2w_box(
        &self,
        cur: Point,
        direction: &char,
    ) -> Vec<HashSet<(Point, Point)>> {
        fn next_move(
            grid: &Grid<u8>,
            cur: &HashSet<(Point, Point)>,
            dir: &char,
        ) -> HashSet<(Point, Point)> {
            let mut result = HashSet::new();
            let mut move_closure = |next: Point, next_v: u8| {
                if next_v == b'[' || next_v == b']' {
                    Some(result.insert(complete_box(&next, next_v)))
                } else if next_v == EMPTY {
                    Some(result.insert((next, next)))
                } else {
                    None
                }
            };
            for (lhs, rhs) in cur {
                if lhs == rhs {
                    assert_eq!(grid.get(lhs), EMPTY);
                    continue;
                }
                let next_l = lhs.next_move(dir);
                let next_r = rhs.next_move(dir);
                if move_closure(next_l, grid.get(&next_l)).is_none() {
                    return HashSet::new();
                }
                if move_closure(next_r, grid.get(&next_r)).is_none() {
                    return HashSet::new();
                }
            }
            if !result
                .iter()
                .all(|(lhs, rhs)| grid.get(lhs) == EMPTY && grid.get(rhs) == EMPTY)
            {
                result = result
                    .into_iter()
                    .filter(|(lhs, rhs)| *lhs != *rhs)
                    .collect::<HashSet<_>>();
            }
            result
        }

        let mut result = Vec::new();
        match self.get(&cur) {
            b'[' | b']' => {
                if *direction == '<' || *direction == '>' {
                    let mut hres = HashSet::new();
                    let mut cur_box = complete_box(&cur, self.get(&cur));
                    while {
                        cur_box = (
                            cur_box.0.next_move(direction),
                            cur_box.1.next_move(direction),
                        );
                        hres.insert(cur_box);

                        (self.get(&cur_box.0) == b'[' || self.get(&cur_box.0) == b']')
                            && (self.get(&cur_box.1) == b'[' || self.get(&cur_box.1) == b']')
                    } {}
                    result.push(hres);
                } else {
                    let mut next = HashSet::from([complete_box(&cur, self.get(&cur))]);
                    while {
                        next = next_move(self, &next, direction);
                        if next.is_empty() {
                            return Vec::new();
                        }
                        result.push(next.clone());
                        !next
                            .iter()
                            .all(|(lhs, rhs)| self.get(lhs) == EMPTY && self.get(rhs) == EMPTY)
                    } {}
                }
            }
            _ => unreachable!(),
        }

        result
    }
}

fn complete_box(half: &Point, val: u8) -> (Point, Point) {
    if val == b'[' {
        (*half, (half.0, half.1 + 1))
    } else {
        ((half.0, half.1 - 1), *half)
    }
}

trait Move {
    fn next_move(&self, direction: &char) -> Point;
}

impl Move for Point {
    fn next_move(&self, direction: &char) -> Point {
        match direction {
            '^' => (self.0.saturating_add_signed(-1), self.1),
            'v' => (self.0 + 1, self.1),
            '<' => (self.0, self.1.saturating_add_signed(-1)),
            '>' => (self.0, self.1 + 1),
            _ => unreachable!(),
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

        let mut split = input.split("\n\n");
        let grid = &mut Grid::<u8>::from_reader(BufReader::new(split.next().unwrap().as_bytes()));
        let moves = split
            .next()
            .unwrap()
            .lines()
            .flat_map(|line| line.chars())
            .collect_vec();
        let mut robot = grid.find_robot_position().unwrap();
        for m in &moves {
            let next_move = robot.next_move(m);
            match grid.get(&next_move) {
                EMPTY => {
                    grid.set(&robot, EMPTY);
                    grid.set(&next_move, ROBOT);
                    robot = next_move;
                }
                BOX => {
                    let look_ahead = grid.look_ahead_current_box(next_move, m);
                    if grid.get(&look_ahead[look_ahead.len() - 1]) == EMPTY {
                        grid.set(&robot, EMPTY);
                        grid.set(&next_move, ROBOT);
                        grid.set(&look_ahead[look_ahead.len() - 1], BOX);
                        robot = next_move;
                    }
                }
                WALL => (),
                _ => unreachable!("huh {}", char::from(grid.get(&next_move))),
            }
        }

        let answer = grid
            .array
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if *v == BOX {
                    let pt = grid.map_index(i);
                    pt.0 * 100 + pt.1
                } else {
                    0
                }
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let mut split = input.split("\n\n");
        let mut w = 0;
        let mut h = 0;
        let mut v = Vec::new();
        for line in split.next().unwrap().lines() {
            if w == 0 {
                w = line.len() * 2;
            } else {
                assert_eq!(w, line.len() * 2);
            }
            for c in line.chars() {
                match c {
                    'O' => v.extend([b'[', b']']),
                    '@' => v.extend([b'@', b'.']),
                    '#' | '.' => v.extend([c as u8].repeat(2)),
                    _ => unreachable!(),
                }
            }
            h += 1;
        }
        let mut grid = Grid::<u8>::new(v, h, w);
        let moves = split
            .next()
            .unwrap()
            .lines()
            .flat_map(|line| line.chars())
            .collect_vec();
        let mut robot = grid.find_robot_position().unwrap();

        // let mut debug = false;
        for m in &moves {
            // if robot == (3,13) && *m == 'v' {
            //     debug = true;
            //     println!("HERE");
            // }
            let next_move = robot.next_move(m);
            match grid.get(&next_move) {
                EMPTY => {
                    grid.set(&robot, EMPTY);
                    grid.set(&next_move, ROBOT);
                    robot = next_move;
                }
                b'[' | b']' => {
                    let look_ahead = grid.look_ahead_current_2w_box(next_move, m);
                    // let (lhs, rhs) = look_ahead[look_ahead.len()-1];
                    match m {
                        '<' | '>' => {
                            let v = &look_ahead[0];
                            let term = if *m == '<' {
                                v.iter().fold((usize::MAX, usize::MAX), |acc, (l, _)| {
                                    if l.1 < acc.1 {
                                        *l
                                    } else {
                                        acc
                                    }
                                })
                            } else {
                                v.iter().fold(
                                    (0, 0),
                                    |acc, (_, r)| {
                                        if r.1 > acc.1 {
                                            *r
                                        } else {
                                            acc
                                        }
                                    },
                                )
                            };
                            if grid.get(&term) == EMPTY {
                                grid.set(&robot, EMPTY);
                                grid.set(&next_move, ROBOT);
                                if *m == '<' {
                                    for i in (term.1..next_move.1).step_by(2) {
                                        grid.set(&(term.0, i), b'[');
                                        grid.set(&(term.0, i + 1), b']');
                                    }
                                } else {
                                    for i in (next_move.1 + 1..=term.1).rev().step_by(2) {
                                        grid.set(&(term.0, i - 1), b'[');
                                        grid.set(&(term.0, i), b']');
                                    }
                                }
                                robot = next_move;
                            }
                        }
                        '^' | 'v' => {
                            if !look_ahead.is_empty() {
                                let offset = if *m == '^' { -1 } else { 1 };
                                for v in look_ahead.iter().rev() {
                                    if v.iter()
                                        .all(|(l, r)| grid.get(l) == EMPTY && grid.get(r) == EMPTY)
                                    {
                                        continue;
                                    }
                                    for (l, r) in v {
                                        grid.set(
                                            &(l.0.saturating_add_signed(offset), l.1),
                                            grid.get(l),
                                        );
                                        grid.set(
                                            &(r.0.saturating_add_signed(offset), r.1),
                                            grid.get(r),
                                        );
                                        grid.set(l, EMPTY);
                                        grid.set(r, EMPTY);
                                    }
                                }
                                grid.set(&robot, EMPTY);
                                let (l, r) = complete_box(&next_move, grid.get(&next_move));
                                grid.set(&(l.0.saturating_add_signed(offset), l.1), grid.get(&l));
                                grid.set(&(r.0.saturating_add_signed(offset), r.1), grid.get(&r));
                                grid.set(&next_move, ROBOT);
                                if l == next_move {
                                    grid.set(&r, EMPTY);
                                } else {
                                    grid.set(&l, EMPTY);
                                }
                                robot = next_move;
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                WALL => (),
                _ => unreachable!("huh {}", char::from(grid.get(&next_move))),
            }
            // if grid.height > 20 {
            //     println!("{}", m);
            //     let s = (0..grid.height).fold(String::new(), |mut output, i| {
            //         let _ = writeln!(
            //             output,
            //             "{:?}",
            //             grid.array[grid.width * i..grid.width * (i + 1)].iter().map(|u| char::from(*u)).collect::<String>()
            //         );
            //         output
            //     });
            //     println!("{}", s);
            // }
        }
        let s = (0..grid.height).fold(String::new(), |mut output, i| {
            let _ = writeln!(
                output,
                "{:?}",
                grid.array[grid.width * i..grid.width * (i + 1)]
                    .iter()
                    .map(|u| char::from(*u))
                    .collect::<String>()
            );
            output
        });
        println!("{}", s);
        let answer = grid
            .array
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if *v == b'[' {
                    let pt = grid.map_index(i);
                    pt.0 * 100 + pt.1
                } else {
                    0
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
