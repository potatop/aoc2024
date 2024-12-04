use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::u32;
use nom::combinator::{peek, recognize, value};
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
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;

        let (_, list) = part2_parser(&string).expect("cannot read string");

        let answer: u32 = list.iter().map(|(a, b)| a * b).sum();

        Ok(answer as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
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

fn part2_parser(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let mut res = Vec::new();
    let (mut remaining, (_, p)) = tuple((parse_junk, parse_multiply))(input)?;
    // println!("******{:?}", remaining);
    // println!("######{:?}", p);
    res.push(p);

    loop {
        (remaining, _) = parse_junk(remaining)?;
        // println!("******{:?}", remaining);

        if let Result::Ok((rem, mul_input)) = recognize(parse_multiply)(remaining) {
            let (_, p) = parse_multiply(mul_input)?;
            // println!("######{:?}", p);
            res.push(p);
            remaining = rem;
        } else if let Result::Ok((rem, _)) = recognize(parse_dont)(remaining) {
            remaining = rem;
        } else {
            break;
        }
    }

    Result::Ok((remaining, res))
}

fn parse_junk(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = value((), take_till(|c| c == 'm' || c == 'd'))(input)?;

    if remaining.starts_with("m") {
        if peek(parse_multiply)(remaining).is_ok() {
            return Result::Ok((remaining, ()));
        }
        return parse_junk(&remaining[1..]);
    }
    let res: IResult<&str, &str> = tag("don't()")(remaining);
    if let Result::Ok((rem, _)) = res {
        let res: IResult<&str, &str> = take_until("do()")(rem);
        return match res {
            Result::Ok(_) => Result::Ok((remaining, ())),
            Err(_) => Result::Ok(("", ())),
        };
    } else if remaining.starts_with("d") {
        return parse_junk(&remaining[1..]);
    }
    assert!(remaining.is_empty());
    Result::Ok((remaining, ()))
}

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

    #[test]
    fn test_parser() {
        let (r, v) = part2_parser(TEST2).unwrap();
        assert_eq!(r, "");
        assert_eq!(v, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn test_parse_junk() {
        let test = vec![
            ("xmul(2,4)", "mul(2,4)"),
            (
                "&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)",
                "don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)",
            ),
            ("x", ""),
            ("don't()x?mul(8,5)", ""),
        ];
        for (i, res) in test {
            let (r, _) = parse_junk(i).unwrap();
            assert_eq!(r, res);
        }
    }
}
