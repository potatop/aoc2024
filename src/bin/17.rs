use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use derive_more::TryFrom;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

#[derive(Debug)]
struct Operand(u8);

impl Operand {
    fn fetch(&self, registers: &Registers) -> isize {
        match self.0 {
            0..=3 => self.0.into(),
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, TryFrom)]
#[try_from(repr)]
#[repr(u8)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Registers {
    a: isize,
    b: isize,
    c: isize,
    pc: usize,
}

impl Registers {
    fn run(&mut self, program: &[u8]) -> Result<String> {
        let mut result = Vec::new();
        while let Some(r) = self.try_run(program)? {
            result.push(r);
        }
        Ok(result.iter().join(","))
    }

    fn try_run(&mut self, program: &[u8]) -> Result<Option<isize>> {
        use Instruction::*;

        while self.pc < program.len() {
            let operand = Operand(program[self.pc + 1]);
            let ins = Instruction::try_from(program[self.pc])?;
            match ins {
                Adv => {
                    self.adv(&operand);
                    self.pc += 2;
                }
                Bxl => {
                    self.bxl(&operand);
                    self.pc += 2;
                }
                Bst => {
                    self.bst(&operand);
                    self.pc += 2;
                }
                Jnz => {
                    if !self.jnz(&operand) {
                        self.pc += 2;
                    }
                }
                Bxc => {
                    self.bxc();
                    self.pc += 2;
                }
                Out => {
                    let out = self.out(&operand);
                    self.pc += 2;
                    return Ok(Some(out));
                }
                Bdv => {
                    self.bdv(&operand);
                    self.pc += 2;
                }
                Cdv => {
                    self.cdv(&operand);
                    self.pc += 2;
                }
            };
        }
        Ok(None)
    }

    /// The adv instruction (opcode 0) performs division. The numerator is the
    /// value in the A register. The denominator is found by raising 2 to the power
    /// of the instruction's combo operand. (So, an operand of 2 would divide A by
    /// 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division
    /// operation is truncated to an integer and then written to the A register.
    fn adv(&mut self, operand: &Operand) {
        self.a /= 2_f64.powi(operand.fetch(self).try_into().unwrap()) as isize;
    }

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
    /// the instruction's literal operand, then stores the result in register B.
    fn bxl(&mut self, operand: &Operand) {
        self.b ^= <u8 as Into<isize>>::into(operand.0);
    }

    /// The bst instruction (opcode 2) calculates the value of its combo operand
    /// modulo 8 (thereby keeping only its lowest 3 bits), then writes that value
    /// to the B register.
    fn bst(&mut self, operand: &Operand) {
        self.b = operand.fetch(self) % 8;
    }

    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction
    /// pointer to the value of its literal operand; if this instruction jumps, the
    /// instruction pointer is not increased by 2 after this instruction.
    fn jnz(&mut self, operand: &Operand) -> bool {
        if self.a != 0 {
            self.pc = operand.0.into();
            true
        } else {
            false
        }
    }

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
    /// register C, then stores the result in register B. (For legacy reasons, this
    /// instruction reads an operand but ignores it.)
    fn bxc(&mut self) {
        self.b ^= self.c;
    }

    /// The out instruction (opcode 5) calculates the value of its combo operand
    /// modulo 8, then outputs that value. (If a program outputs multiple values,
    /// they are separated by commas.)
    fn out(&mut self, operand: &Operand) -> isize {
        operand.fetch(self) % 8
    }

    /// The bdv instruction (opcode 6) works exactly like the adv instruction
    /// except that the result is stored in the B register. (The numerator is still
    /// read from the A register.)
    fn bdv(&mut self, operand: &Operand) {
        self.b = self.a / 2_f64.powi(operand.fetch(self).try_into().unwrap()) as isize;
    }

    /// The cdv instruction (opcode 7) works exactly like the adv instruction
    /// except that the result is stored in the C register. (The numerator is still
    /// read from the A register.)
    fn cdv(&mut self, operand: &Operand) {
        self.c = self.a / 2_f64.powi(operand.fetch(self).try_into().unwrap()) as isize;
    }
}

fn parse(input: &str) -> IResult<&str, Registers> {
    use nom::character::complete::i32;
    let (rest, (_, a, _, _, b, _, _, c, _)) = tuple((
        tag("Register A: "),
        i32,
        newline,
        tag("Register B: "),
        i32,
        newline,
        tag("Register C: "),
        i32,
        newline,
    ))(input)?;
    Result::Ok((
        rest,
        Registers {
            a: a as isize,
            b: b as isize,
            c: c as isize,
            pc: 0,
        },
    ))
}

fn program(input: &str) -> IResult<&str, Vec<u8>> {
    use nom::character::complete::u8;
    let (rest, (_, _, vec, _)) = tuple((
        newline,
        tag("Program: "),
        separated_list1(tag(","), u8),
        newline,
    ))(input)?;
    Result::Ok((rest, vec))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let (rest, mut registers) = parse(&input).expect("bad registers");
        let (_, program) = program(rest).expect("bad program");

        let result = registers.run(&program)?;
        println!("result: {}", result);
        Ok(0)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let (rest, _) = parse(&input).expect("bad registers");
        let (_, program) = program(rest).expect("bad program");
        let out = &mut Vec::new();
        search_init_val(0, &program, &program, 1, out)?;

        Ok(*out.iter().min().unwrap() as usize)
    }

    assert_eq!(117440, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn search_init_val(
    some_a: isize,
    values: &[u8],
    program: &[u8],
    depth: usize,
    out: &mut Vec<isize>,
) -> Result<()> {
    if values.is_empty() {
        return Ok(());
    }
    let tail = values[values.len() - 1];
    let mut candidates = HashSet::new();
    for i in 0..8 {
        let mut registers = Registers {
            a: some_a + i,
            b: 0,
            c: 0,
            pc: 0,
        };
        if let Some(o) = registers.try_run(program)? {
            if o == tail as isize {
                candidates.insert(i);
                if depth == program.len() {
                    out.push(some_a + i);
                    // println!("{}", some_a + i)
                }
            }
        }
    }
    for c in candidates {
        search_init_val(
            (some_a + c) << 3,
            &values[0..values.len() - 1],
            program,
            depth + 1,
            out,
        )?
    }
    Ok(())
}
