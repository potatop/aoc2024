use adv_code_2024::util::grid::Point;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[derive(Debug)]
struct Map {
    start: Point,
    end: Point,
    walls: Vec<Point>,
    track: Vec<Point>,
}

impl Map {
    fn from<R: BufRead>(reader: R) -> Self {
        let mut walls = Vec::new();
        let mut track = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => walls.push((y, x)),
                    '.' => track.push((y, x)),
                    'S' => {
                        start = (y, x);
                        track.push(start)
                    }
                    'E' => {
                        end = (y, x);
                        track.push(end)
                    }
                    _ => unreachable!(),
                }
            }
        }

        Self {
            start,
            end,
            walls,
            track,
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = Map::from(reader);
        let mut dist_map = HashMap::new();
        let total = shortest_path(&map, &mut dist_map).unwrap();

        let answer = map
            .track
            .iter()
            .flat_map(|pt| {
                [(0, -1), (0, 1), (-1, 0), (1, 0)]
                    .iter()
                    .filter_map(|offset| {
                        if let (Some(y), Some(x)) = (
                            pt.0.checked_add_signed(offset.0 * 2),
                            pt.1.checked_add_signed(offset.1 * 2),
                        ) {
                            let p1 = (
                                pt.0.checked_add_signed(offset.0).unwrap(),
                                pt.1.checked_add_signed(offset.1).unwrap(),
                            );
                            let p2 = (y, x);
                            if map.walls.contains(&p1) && map.track.contains(&p2) {
                                if let Some(saved) =
                                    (total - dist_map[pt]).checked_sub(total - dist_map[&p2] + 2)
                                {
                                    if saved >= 100 {
                                        return Some(saved);
                                    }
                                }
                            }
                        }
                        None
                    })
                    .collect_vec()
            })
            .count();
        Ok(answer)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = Map::from(reader);
        let mut dist_map = HashMap::new();
        let total = shortest_path(&map, &mut dist_map).unwrap();
        dist_map = dist_map.iter().map(|(k, v)| (*k, total - v)).collect();
        // let dist_cache = &mut HashMap::<(Point, Point), usize>::new();
        let answer = map
            .track
            .iter()
            .flat_map(|pt| {
                let candidates = (-20..21)
                    .rev()
                    .flat_map(|i| {
                        (-20..21)
                            .flat_map(|j| {
                                if isize::abs(i) + isize::abs(j) <= 20
                                    && (isize::abs(i) >= 2 || isize::abs(j) >= 2)
                                {
                                    if let (Some(y), Some(x)) =
                                        (pt.0.checked_add_signed(i), pt.1.checked_add_signed(j))
                                    {
                                        let p2 = (y, x);
                                        if map.track.contains(&p2) && dist_map[pt] > dist_map[&p2] {
                                            return Some((p2, isize::abs(i) + isize::abs(j)));
                                        }
                                    }
                                }
                                None
                            })
                            .collect::<HashSet<_>>()
                    })
                    .collect::<HashSet<_>>();

                candidates
                    .iter()
                    .filter_map(|(p2, dist)| {
                        if let Some(saved) = dist_map[pt]
                            .checked_sub(dist_map[p2].checked_add_signed(*dist).unwrap())
                        {
                            if saved >= 100 {
                                return Some(saved);
                            }
                        }
                        None
                    })
                    .collect_vec()
            })
            .count();

        Ok(answer)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
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
fn shortest_path(map: &Map, dist: &mut HashMap<Point, usize>) -> Option<usize> {
    let mut heap = BinaryHeap::from([State {
        cost: 0,
        position: map.start,
    }]);

    dist.insert(map.start, 0);
    while let Some(State { cost, position }) = heap.pop() {
        if position == map.end {
            return Some(cost);
        }
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }
        for next in find_next_paths(map, &position) {
            let state = State {
                cost: next.cost + cost,
                position: next.position,
            };

            if state.cost < *dist.get(&state.position).unwrap_or(&usize::MAX) {
                heap.push(state);
                dist.insert(state.position, state.cost);
            }
        }
    }
    None
}

fn find_next_paths(map: &Map, location: &Point) -> Vec<State> {
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .filter_map(|offset| {
            let position = (
                location.0.checked_add_signed(offset.0).unwrap(),
                location.1.checked_add_signed(offset.1).unwrap(),
            );
            if map.track.contains(&position) {
                Some(State { cost: 1, position })
            } else {
                None
            }
        })
        .collect_vec()
}
