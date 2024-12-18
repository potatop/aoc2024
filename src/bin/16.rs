use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::grid::{Grid, Point};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    fn advance(&self, pt: &Point) -> Option<Point> {
        use Direction::*;
        if let (Some(y), Some(x)) = match self {
            East => (Some(pt.0), pt.1.checked_add_signed(1)),
            West => (Some(pt.0), pt.1.checked_add_signed(-1)),
            North => (pt.0.checked_add_signed(-1), Some(pt.1)),
            South => (pt.0.checked_add_signed(1), Some(pt.1)),
        } {
            return Some((y, x));
        }
        None
    }

    fn reverse(&self, pt: &Point) -> Option<Point> {
        use Direction::*;
        if let (Some(y), Some(x)) = match self {
            East => (Some(pt.0), pt.1.checked_add_signed(-1)),
            West => (Some(pt.0), pt.1.checked_add_signed(1)),
            North => (pt.0.checked_add_signed(1), Some(pt.1)),
            South => (pt.0.checked_add_signed(-1), Some(pt.1)),
        } {
            return Some((y, x));
        }
        None
    }

    fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            East => North,
            West => South,
            North => West,
            South => East,
        }
    }

    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            East => South,
            West => North,
            North => East,
            South => West,
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    direction: Direction,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grid: &Grid<char>,
    start: &State,
    dist: &mut HashMap<(Point, Direction), usize>,
) -> Option<usize> {
    // let mut dist = [usize::MAX].repeat(grid.array.len());
    let mut heap = BinaryHeap::from([*start]);

    // dist[start.position.0 * grid.width + start.position.1] = 0;
    dist.insert((start.position, start.direction), 0);
    while let Some(State {
        cost,
        direction,
        position,
    }) = heap.pop()
    {
        if grid.get(&position) == END {
            return Some(cost);
        }
        if cost > *dist.get(&(position, direction)).unwrap_or(&usize::MAX) {
            // if cost > dist[position.0 * grid.width + position.1] {
            continue;
        }
        for mut next in find_next_paths(grid, &direction, &position) {
            next.cost += cost;
            if next.cost
                < *dist
                    .get(&(next.position, next.direction))
                    .unwrap_or(&usize::MAX)
            {
                // if next.cost < dist[next.position.0 * grid.width + next.position.1] {
                heap.push(next);
                // println!("{:?}", next);
                // dist[next.position.0 * grid.width + next.position.1] = next.cost;
                dist.insert((next.position, next.direction), next.cost);
            }
        }
    }
    None
}

fn find_next_paths(grid: &Grid<char>, direction: &Direction, pos: &Point) -> Vec<State> {
    (0..3)
        .filter_map(|d| {
            if d == 0 {
                if let Some(pt) = direction.advance(pos) {
                    if grid.get(&pt) != WALL {
                        return Some(State {
                            cost: 1,
                            direction: *direction,
                            position: pt,
                        });
                    }
                }
                return None;
            }
            Some(State {
                cost: 1000,
                direction: if d == 1 {
                    direction.turn_left()
                } else {
                    direction.turn_right()
                },
                position: *pos,
            })
        })
        .collect_vec()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::<char>::from_reader_char(reader);
        let start = grid.find(START).unwrap();
        let answer = dijkstra(
            &grid,
            &State {
                cost: 0,
                direction: Direction::East,
                position: start,
            },
            &mut HashMap::new(),
        )
        .unwrap();

        Ok(answer)
    }

    assert_eq!(11048, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::<char>::from_reader_char(reader);
        let start = grid.find(START).unwrap();
        let seen = &mut HashMap::new();
        let answer = dijkstra(
            &grid,
            &State {
                cost: 0,
                direction: Direction::East,
                position: start,
            },
            seen,
        )
        .unwrap();

        // println!("{:?}", seen);
        let mut queue = VecDeque::new();
        let end = grid.find(END).unwrap();
        for d in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if let Some(cost) = seen.get(&(end, d)) {
                if *cost == answer {
                    queue.push_back(State {
                        cost: *cost,
                        direction: d,
                        position: end,
                    });
                }
            }
        }

        let mut path = HashSet::new();
        while let Some(State {
            cost,
            direction,
            position,
        }) = queue.pop_front()
        {
            path.insert(position);
            if position == start {
                break;
            }

            for node in (0..3).filter_map(|d| {
                if d == 0 {
                    if let Some(pt) = direction.reverse(&position) {
                        if grid.get(&pt) != WALL {
                            return Some(State {
                                cost: cost - 1,
                                direction,
                                position: pt,
                            });
                        }
                    }
                    return None;
                }
                cost.checked_add_signed(-1000).map(|c| State {
                    cost: c,
                    direction: if d == 1 {
                        direction.turn_left()
                    } else {
                        direction.turn_right()
                    },
                    position,
                })
            }) {
                if let Some(so) = seen.get(&(node.position, node.direction)) {
                    if *so == node.cost {
                        queue.push_back(node);
                    }
                }
            }
        }
        // for node in &path {
        //     grid.set(node, 'O');
        // }
        // let s = (0..grid.height).fold(String::new(), |mut output, i| {
        //     for c in &grid.array[grid.width * i..grid.width * (i + 1)] {
        //         let _ = write!(output, "{}", c);
        //     }
        //     let _ = writeln!(output);
        //     output
        // });
        // println!("{}", s);
        Ok(path.len())
    }

    assert_eq!(64, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
