use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::u32;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer: u32 = reader
            .lines()
            .flat_map(|l| {
                let line = l.unwrap();
                let (_, list) = part1_parser(&line).unwrap();
                list
            })
            .map(|(a, b)| a * b)
            .sum();

        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");

    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     let answer: u32 = reader
    //         .lines()
    //         .flat_map(|l| {
    //             let line = l.unwrap();
    //             let (_, list) = part2_parser(&line).unwrap();
    //             list
    //         })
    //         .map(|(a, b)| a * b)
    //         .sum();

    //     Ok(answer as usize)
    // }

    // assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_integer_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag(","), u32)(input)
}

fn search_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (remaining, _) = take_until("mul(")(input)?;
    // println!("{:?}", remaining);
    let mut result = parse_multiply(remaining);
    while let Err(nom::Err::Error(ref e)) = result {
        result = search_mul(e.input);
    }
    result
}

fn parse_multiply(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(tag("mul("), parse_integer_pair, tag(")"))(input)
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("don't()"), take_until("do()"), tag("do()"))))(input)
}

fn part1_parser(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many0(search_mul)(input)
}

// fn part2_parser(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
//     let mut vec: Vec<(u32, u32)> = Vec::new();
//     let (remaining, pair) = parse_x(input)?;
//     vec.push(pair);
//     let (remaining, mut list) = many0(alt((parse_dont, parse_x)))(remaining)?;
//     vec.append(&mut list);
//     std::result::Result::Ok((remaining, vec))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiply() {
        let (r, p) = parse_multiply(r"mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))").unwrap();
        assert_eq!(r, "+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(p, (5, 5));
    }

    #[test]
    fn test_part1_parser() {
        let (r, list) = part1_parser(TEST).unwrap();
        println!("{:?}", r);
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn test_parse_dont() {
        let (r, _) = parse_dont(r"don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))").unwrap();
        assert_eq!(r, "?mul(8,5))");
    }

    // #[test]
    // fn test_part2_parser() {
    //     let (r, list) = part2_parser(TEST2).unwrap();
    //     println!("{:?}", list);
    //     assert_eq!(list.len(), 3);
    // }
}
