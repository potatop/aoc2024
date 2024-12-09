use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::u32;
use nom::combinator::{opt, value};
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

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .expect("cannot read string");
        let (_, answer) = parse(input.as_str()).expect("");

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .expect("cannot read string");
        let (_, answer) = parse2(input.as_str()).expect("");

        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, PartialEq)]
enum State {
    Mul,
    Dont,
    End,
}

fn filler(input: &str, suppress: bool) -> IResult<&str, State> {
    let mut rest = input;
    if suppress {
        let res: IResult<&str, (&str, &str)> = tuple((take_until("do()"), tag("do()")))(input);
        match res {
            Result::Ok((rem, _)) => rest = rem,
            Err(_) => return Result::Ok(("", State::End)),
        }
    }
    if !rest.starts_with("m") && !rest.starts_with("d") {
        (rest, _) = value((), is_not("md"))(rest)?;
    }
    if rest.starts_with('m') {
        return Result::Ok((rest, State::Mul));
    } else if rest.starts_with('d') {
        return Result::Ok((rest, State::Dont));
    }
    Result::Ok((rest, State::End))
}

fn mul(input: &str) -> IResult<&str, Option<usize>> {
    if let (rest, Some((a, b))) = opt(delimited(
        tag("mul("),
        separated_pair(u32, tag(","), u32),
        tag(")"),
    ))(input)?
    {
        return Result::Ok((rest, Some((a * b) as usize)));
    }
    Result::Ok((&input[1..], None))
}

fn parse(input: &str) -> IResult<&str, usize> {
    let mut out: usize = 0;
    let mut rem: &str = input;
    loop {
        let (rest, _) = filler(rem, false)?;
        if rest.is_empty() {
            break;
        }
        let (rest, opt) = mul(rest)?;
        rem = rest;
        if let Some(value) = opt {
            out += value;
        }
    }
    Result::Ok(("", out))
}

fn parse2(input: &str) -> IResult<&str, usize> {
    let mut out: usize = 0;
    let mut rem: &str = input;
    let mut suppress = false;
    loop {
        let (rest, state) = filler(rem, suppress)?;
        suppress = false;
        match state {
            State::Mul => {
                let (rest, opt) = mul(rest)?;
                rem = rest;
                if let Some(value) = opt {
                    out += value;
                }
            }
            State::Dont => {
                let (rest, s) = dont(rest)?;
                suppress = s;
                rem = rest;
            }
            State::End => break,
        }
    }
    Result::Ok(("", out))
}

fn dont(input: &str) -> IResult<&str, bool> {
    if let (rest, Some(_)) = opt(tag("don't()"))(input)? {
        return Result::Ok((rest, true));
    }
    Result::Ok((&input[1..], false))
}

#[cfg(test)]
mod tests {
    use super::*;
    // use nom::error::Error;
    // use nom::error::ErrorKind;
    #[test]
    fn test_filler() {
        assert_eq!(filler("xmul", false), Result::Ok(("mul", State::Mul)));
        assert_eq!(filler("mfd", false), Result::Ok(("mfd", State::Mul)));
        assert_eq!(filler("x23do", false), Result::Ok(("do", State::Dont)));
        assert_eq!(filler(TEST, true), Result::Ok(("", State::End)));
        assert_eq!(filler(TEST2, true), Result::Ok(("mul(8,5))\n", State::Mul)));
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul("mul(2,4)"), Result::Ok(("", Some(8))));
        assert_eq!(mul("mul(2,4]"), Result::Ok(("ul(2,4]", None)));
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(TEST), Result::Ok(("", 161)));
    }
}
