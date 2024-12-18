use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::grid::{Grid, Point};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

const WALL: char = '#';

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let incoming = &reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let mut parts = line.split(",").map(|part| part.parse::<usize>().unwrap());
                let (x, y) = (parts.next().unwrap(), parts.next().unwrap());
                (y, x)
            })
            .collect_vec();
        let mut grid = if incoming.len() < 50 {
            Grid::new(['.'].repeat(7 * 7), 7, 7)
        } else {
            Grid::new(['.'].repeat(71 * 71), 71, 71)
        };

        for (i, b) in incoming.iter().enumerate() {
            if i >= if incoming.len() < 50 { 12 } else { 1024 } {
                break;
            }
            grid.set(b, '#');
        }
        // println!("{}", grid);
        let answer = shortest_path(&grid, &(0, 0), &(grid.height - 1, grid.width - 1)).unwrap();
        // let answer = incoming.len();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let incoming = &reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let mut parts = line.split(",").map(|part| part.parse::<usize>().unwrap());
                let (x, y) = (parts.next().unwrap(), parts.next().unwrap());
                (y, x)
            })
            .collect_vec();
        let mut grid = if incoming.len() < 50 {
            Grid::new(['.'].repeat(7 * 7), 7, 7)
        } else {
            Grid::new(['.'].repeat(71 * 71), 71, 71)
        };

        let mut iteration = 0;
        for (i, b) in incoming.iter().enumerate() {
            if i >= iteration {
                break;
            }
            grid.set(b, WALL);
        }

        while shortest_path(&grid, &(0, 0), &(grid.height - 1, grid.width - 1)).is_some() {
            iteration += 1;
            let b = &incoming[iteration];
            grid.set(b, WALL);
        }
        println!("[Y,X]: {:?}", incoming[iteration]);
        Ok(iteration)
    }

    assert_eq!(20, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
        }
    }
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

fn shortest_path(grid: &Grid<char>, start: &(usize, usize), end: &(usize, usize)) -> Option<usize> {
    let mut dist = [usize::MAX].repeat(grid.array.len());
    let mut heap = BinaryHeap::from([State {
        cost: 0,
        position: *start,
    }]);

    dist[start.0 * grid.width + start.1] = 0;
    // seen.insert((start.position, start.direction), 0);
    while let Some(State { cost, position }) = heap.pop() {
        if &position == end {
            return Some(cost);
        }
        if cost > dist[position.0 * grid.width + position.1] {
            continue;
        }
        for next in find_next_paths(grid, &position) {
            let state = State {
                cost: next.cost + cost,
                position: next.position,
            };

            if state.cost < dist[state.position.0 * grid.width + state.position.1] {
                heap.push(state);
                dist[state.position.0 * grid.width + state.position.1] = state.cost;
            }
        }
    }
    None
}

fn find_next_paths(grid: &Grid<char>, pos: &Point) -> Vec<State> {
    // EAST, NORTH, SOUTH, WEST
    [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .iter()
    .filter_map(|d| {
        if let (Some(y), Some(x)) = (
            pos.0.checked_add_signed(d.offset().0),
            pos.1.checked_add_signed(d.offset().1),
        ) {
            if y >= grid.height || x >= grid.width || grid.get(&(y, x)) == WALL {
                return None;
            }
            return Some(State {
                cost: 1,
                position: (y, x),
            });
        }
        None
    })
    .collect_vec()
}
