use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::grid::Grid;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

trait P12 {
    fn get_connected_region(&self, index: usize) -> Vec<usize>;
    fn build_region_map(&self) -> HashMap<usize, i32>;
    fn get_neighbors(&self, start: usize) -> Vec<usize>;
}

impl P12 for Grid<u8> {
    fn get_connected_region(&self, index: usize) -> Vec<usize> {
        let value = self.array[index];
        let mut v = vec![index];
        let mut visited = Vec::new();
        visited.push(index);
        while let Some(idx) = v.pop() {
            let neighbors = self.get_neighbors(idx);
            let next_iter = neighbors.iter().filter(|&loc| self.array[*loc] == value);
            for ni in next_iter {
                if !visited.contains(ni) {
                    visited.push(*ni);
                    v.push(*ni);
                }
            }
        }
        visited
    }

    // return map of index to region id
    fn build_region_map(&self) -> HashMap<usize, i32> {
        let mut region_map: HashMap<usize, i32> = HashMap::new();
        let mut region_id = -1;
        for i in 0..self.array.len() {
            if region_map.contains_key(&i) {
                continue;
            }
            region_id += 1;
            let connected = self.get_connected_region(i);
            for j in connected {
                region_map.insert(j, region_id);
            }
        }
        region_map
    }

    fn get_neighbors(&self, start: usize) -> Vec<usize> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(|direction| {
                if let (Some(y), Some(x)) = (
                    (start / self.height).checked_add_signed(direction.0),
                    (start % self.height).checked_add_signed(direction.1),
                ) {
                    if y < self.height && x < self.width {
                        return Some(y * self.height + x);
                    }
                }
                None
            })
            .collect_vec()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::<u8>::from_reader(reader);
        let region_map = grid.build_region_map();
        let region_area = region_map.iter().fold(HashMap::new(), |mut acc, entry| {
            let counter = acc.entry(*entry.1).or_insert(0);
            *counter += 1;
            acc
        });
        let region_perimeter = region_map.iter().fold(HashMap::new(), |mut acc, entry| {
            let region_id = entry.1;
            let perimeter_count = grid
                .get_neighbors(*entry.0)
                .iter()
                .filter(|loc| region_map[loc] == *region_id)
                .count();
            let value = acc.entry(*region_id).or_insert(0);
            *value += 4 - perimeter_count;
            acc
        });
        let answer = region_area
            .iter()
            .map(|(rid, area)| region_perimeter[rid] * area)
            .sum();
        Ok(answer)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::<u8>::from_reader(reader);
        let region_map = grid.build_region_map();
        let region_area = (0..grid.array.len()).fold(HashMap::new(), |mut acc, index| {
            let region_id = region_map[&index];
            let counter = acc.entry(region_id).or_insert(0);
            *counter += 1;
            acc
        });

        let region_sides = count_sides(grid, region_map);
        for i in 0..11 {
            println!("{:?} {}", region_area[&i], region_sides[&i]);
        }
        let answer = region_area
            .iter()
            .map(|(rid, area)| region_sides[rid] * area)
            .sum();
        Ok(answer)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn count_sides(grid: Grid<u8>, region_map: HashMap<usize, i32>) -> HashMap<i32, usize> {
    (0..grid.array.len()).fold(HashMap::new(), |mut acc, index| {
        let region_id = region_map[&index];

        let neighbors = grid
            .get_neighbors(index)
            .into_iter()
            .filter(|loc| region_map[loc] == region_id)
            .collect_vec();
        let sides_count: usize = match neighbors.len() {
            0 => 4,
            1 => 2,
            _ => {
                let mut s = 0;
                if neighbors.len() == 2 {
                    let n1 = (neighbors[0] / grid.height, neighbors[0] % grid.height);
                    let n2 = (neighbors[1] / grid.height, neighbors[1] % grid.height);
                    if n1.0 != n2.0 && n1.1 != n2.1 {
                        s += 1;
                    }
                };

                let val = grid.array[index];
                if let (Some(y1), x1, y2, Some(x2)) = (
                    (index / grid.height).checked_add_signed(-1),
                    index % grid.height,
                    index / grid.height,
                    (index % grid.height).checked_add_signed(-1),
                ) {
                    if neighbors.contains(&(y1 * grid.height + x1))
                        && neighbors.contains(&(y2 * grid.height + x2))
                        && grid.get(&(y1, x2)) != val
                    {
                        s += 1
                    }
                }
                if let (Some(y1), x1, y2, Some(x2)) = (
                    (index / grid.height).checked_add_signed(-1),
                    index % grid.height,
                    index / grid.height,
                    (index % grid.height).checked_add_signed(1),
                ) {
                    if x2 < grid.width
                        && neighbors.contains(&(y1 * grid.height + x1))
                        && neighbors.contains(&(y2 * grid.height + x2))
                        && grid.get(&(y1, x2)) != val
                    {
                        s += 1
                    }
                }
                if let (Some(y1), x1, y2, Some(x2)) = (
                    (index / grid.height).checked_add_signed(1),
                    index % grid.height,
                    index / grid.height,
                    (index % grid.height).checked_add_signed(-1),
                ) {
                    if y1 < grid.height
                        && neighbors.contains(&(y1 * grid.height + x1))
                        && neighbors.contains(&(y2 * grid.height + x2))
                        && grid.get(&(y1, x2)) != val
                    {
                        s += 1
                    }
                }
                if let (Some(y1), x1, y2, Some(x2)) = (
                    (index / grid.height).checked_add_signed(1),
                    index % grid.height,
                    index / grid.height,
                    (index % grid.height).checked_add_signed(1),
                ) {
                    if y1 < grid.height
                        && x2 < grid.width
                        && neighbors.contains(&(y1 * grid.height + x1))
                        && neighbors.contains(&(y2 * grid.height + x2))
                        && grid.get(&(y1, x2)) != val
                    {
                        s += 1
                    }
                }
                s
            }
        };
        let value = acc.entry(region_id).or_insert(0);
        *value += sides_count;
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::TEST;

    use super::*;

    #[test]
    fn test_build_region_map() {
        let grid = Grid::<u8>::from_reader(BufReader::new(TEST.as_bytes()));
        let region_map = grid.build_region_map();
        assert_eq!(region_map[&0], 0);
        assert_eq!(region_map[&1], 0);
        assert_eq!(region_map[&2], 0);
        assert_eq!(region_map[&3], 0);
        assert_eq!(region_map[&4], 1);
        assert_eq!(region_map[&5], 1);
        assert_eq!(region_map[&6], 2);
        assert_eq!(region_map[&7], 2);
        assert_eq!(region_map[&8], 3);
        assert_eq!(region_map[&9], 3);
        let region_area = region_map.iter().fold(HashMap::new(), |mut acc, entry| {
            let counter = acc.entry(entry.1).or_insert(0);
            *counter += 1;
            acc
        });
        assert_eq!(region_area.len(), 11);
        assert_eq!(region_area[&0], 12);
        assert_eq!(region_area[&1], 4);
        assert_eq!(region_area[&2], 14);
        assert_eq!(region_area[&3], 10);
    }

    #[test]
    fn test_count_sides() {
        let test = "\
AAAA
BBCD
BBCC
EEEC
";
        let grid = Grid::<u8>::from_reader(BufReader::new(test.as_bytes()));
        let region_map = grid.build_region_map();
        assert_eq!(
            count_sides(grid, region_map),
            HashMap::from([(0, 4), (1, 4), (2, 8), (3, 4), (4, 4)])
        )
    }
}
