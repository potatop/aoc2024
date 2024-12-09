use adv_code_2024::util::grid::Grid;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

trait P8 {
    fn locate_antennas(&self, f: &u8) -> Vec<(usize, usize)>;
    fn get_frequency_types(&self) -> HashSet<&u8>;
}

impl P8 for Grid {
    fn locate_antennas(&self, f: &u8) -> Vec<(usize, usize)> {
        self.array
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                if b == f {
                    return Some((i / self.height, i % self.height));
                }
                None
            })
            .collect_vec()
    }

    fn get_frequency_types(&self) -> HashSet<&u8> {
        self.array
            .iter()
            .filter(|&b| b.is_ascii_alphanumeric())
            .collect::<HashSet<_>>()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader);

        let answer = grid
            .get_frequency_types()
            .into_iter()
            .flat_map(|f| {
                let locations = grid.locate_antennas(f);
                locations
                    .iter()
                    .flat_map(|loc| {
                        let mut hashset = HashSet::new();
                        for another in locations.iter() {
                            if another != loc {
                                if let (Some(dy), Some(dx)) = (
                                    (another.0 as isize).checked_sub_unsigned(loc.0),
                                    (another.1 as isize).checked_sub_unsigned(loc.1),
                                ) {
                                    if let (Some(y), Some(x)) = (
                                        loc.0.checked_add_signed(-dy),
                                        loc.1.checked_add_signed(-dx),
                                    ) {
                                        if y < grid.height && x < grid.width {
                                            hashset.insert((y, x));
                                        }
                                    }
                                }
                            }
                        }
                        hashset
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>()
            .len();
        Ok(answer)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader);

        let answer = grid
            .get_frequency_types()
            .into_iter()
            .flat_map(|f| {
                let locations = grid.locate_antennas(f);
                locations
                    .iter()
                    .flat_map(|loc| {
                        let mut hashset: HashSet<(usize, usize)> =
                            HashSet::from_iter(locations.clone().into_iter());
                        for another in locations.iter() {
                            if another != loc {
                                if let (Some(dy), Some(dx)) = (
                                    (another.0 as isize).checked_sub_unsigned(loc.0),
                                    (another.1 as isize).checked_sub_unsigned(loc.1),
                                ) {
                                    let mut pt = *loc;
                                    while let (Some(y), Some(x)) =
                                        (pt.0.checked_add_signed(-dy), pt.1.checked_add_signed(-dx))
                                    {
                                        if y < grid.height && x < grid.width {
                                            pt = (y, x);
                                            hashset.insert(pt);
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        hashset
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>()
            .len();
        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

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
    fn it_works() {
        let grid = Grid::new(BufReader::new(TEST.as_bytes()));
        assert_eq!(grid.get_frequency_types(), HashSet::from([&b'0', &b'A']));
        assert_eq!(
            grid.locate_antennas(&b'0'),
            vec![(1, 8), (2, 5), (3, 7), (4, 4)]
        );
        assert_eq!(grid.locate_antennas(&b'A'), vec![(5, 6), (8, 8), (9, 9)]);
    }
}
