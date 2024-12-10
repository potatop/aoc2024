use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
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
    layout: Vec<String>,
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
        self.layout.extend({
            if self.is_file_block {
                let r = std::iter::repeat(self.id.to_string()).take(digit);
                self.id += 1;
                r
            } else {
                std::iter::repeat(String::from(".")).take(digit)
            }
        });
        self.is_file_block = !self.is_file_block;
    }

    fn defrag(&mut self) {
        while let Some(idx) = self.layout.iter().position(|o| o == ".") {
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
            .filter_map(|(i, e)| {
                if e != "." {
                    Some(e.parse::<usize>().unwrap() * i)
                } else {
                    None
                }
            })
            .sum()
    }

    fn defrag2(&mut self) {
        let mut i = 0;
        let mut fpos = Vec::new();
        while let Some(t) = self.layout[i..].iter().find_position(|&o| *o != ".") {
            i += t.0;
            fpos.push((i, String::from(t.1)));
            i += 1;
        }
        let mut fps = fpos.chunk_by(|a, b| a.1 == b.1).collect_vec();
        fps.reverse();

        for chunk in fps {
            let spos = self
                .layout
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if e == "." { Some(i) } else { None })
                .collect_vec();
            let sps = spos.chunk_by(|a, b| a.abs_diff(*b) == 1).collect_vec();

            if let Some(space) = sps
                .into_iter()
                .find(|&sp_chunk| sp_chunk.len() >= chunk.len())
            {
                for i in 0..chunk.len() {
                    let (fi, fb) = &chunk[i];
                    let si = space[i];
                    if *fi < si {
                        continue;
                    }
                    self.layout[si] = String::from(fb);
                    self.layout[*fi] = String::from(".");
                }
            }
        }
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.layout.iter().map(|o| o.as_str()).collect::<String>()
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk_map = DiskMap::new();

        for line in reader.lines().map_while(Result::ok) {
            for c in line.chars() {
                disk_map.unpack(c.to_digit(10).unwrap() as usize)
            }
        }
        disk_map.defrag2();
        let answer = disk_map.checksum();
        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

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

    #[test]
    fn test_defrag2() {
        let mut disk_map = DiskMap::new();
        for c in TEST.trim().chars() {
            disk_map.unpack(c.to_digit(10).unwrap() as usize)
        }
        disk_map.defrag2();
        assert_eq!(
            disk_map.to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        )
    }
}
