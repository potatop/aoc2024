use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";
#[derive(Debug, PartialEq, Eq, Hash)]
struct DiskMap {
    layout: Vec<Option<u32>>,
    is_file_block: bool,
    id: u32,
}

impl DiskMap {
    fn new() -> Self {
        Self {
            layout: Vec::new(),
            is_file_block: true,
            id: 0,
        }
    }
    fn unpack(&mut self, digit: usize) {
        self.layout.extend(if self.is_file_block {
            let r = std::iter::repeat(Some(self.id)).take(digit);
            self.id += 1;
            r
        } else {
            std::iter::repeat(None).take(digit)
        });
        self.is_file_block = !self.is_file_block;
    }

    fn defrag(&mut self) {
        while let Some(idx) = self.layout.iter().position(|&o| o.is_none()) {
            if let Some(e) = self.layout.pop() {
                self.layout[idx] = e;
            } else {
                return;
            }
        }
    }

    fn checksum(&self) -> usize {
        self.layout
            .iter()
            .enumerate()
            .filter_map(|(i, e)| e.as_ref().map(|id| *id as usize * i))
            .sum()
    }
}
impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.layout
                .iter()
                .map(|o| {
                    if let Some(id) = o {
                        char::from_digit(*id, 10).unwrap()
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        )
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk_map = DiskMap::new();
        for line in reader.lines().map_while(Result::ok) {
            for c in line.chars() {
                disk_map.unpack(c.to_digit(10).unwrap() as usize)
            }
        }
        disk_map.defrag();
        let answer = disk_map.checksum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::usize;

    #[test]
    fn test_unpack() {
        let mut disk_map = DiskMap::new();
        for c in TEST.trim().chars() {
            disk_map.unpack(c.to_digit(10).unwrap() as usize)
        }
        assert_eq!(
            disk_map.to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        )
    }

    #[test]
    fn test_defrag() {
        let mut disk_map = DiskMap::new();
        for c in TEST.trim().chars() {
            disk_map.unpack(c.to_digit(10).unwrap() as usize)
        }
        disk_map.defrag();
        assert_eq!(disk_map.to_string(), "0099811188827773336446555566")
    }

    #[test]
    fn test_checksum() {
        let mut disk_map = DiskMap::new();
        for c in TEST.trim().chars() {
            disk_map.unpack(c.to_digit(10).unwrap() as usize)
        }
        disk_map.defrag();
        assert_eq!(disk_map.checksum(), 1928)
    }
}
