use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use ndarray::{arr1, arr2, Array1, Array2};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

#[derive(Debug, PartialEq)]
struct Machine {
    buttons: Array2<f64>,
    prize: Array1<f64>,
}

fn read_input<R: BufRead>(mut reader: R) -> Result<Vec<Machine>> {
    fn read_line(line: &str) -> Option<(f64, f64)> {
        line.split(",")
            .map(|s| {
                s.trim_matches(|c: char| !c.is_ascii_digit())
                    .parse::<f64>()
                    .expect("failed to parse f64")
            })
            .collect_tuple()
    }

    let mut result = Vec::new();
    let mut input: String = String::new();
    reader.read_to_string(&mut input)?;
    for b in input.split("\n\n") {
        let mut iter = b.lines();

        if let (Some((a0, a1)), Some((b0, b1)), Some((p0, p1))) = (
            read_line(iter.next().expect("Failed to read line for Button A")),
            read_line(iter.next().expect("Failed to read line for Button B")),
            read_line(iter.next().expect("Failed to read line for Prize")),
        ) {
            result.push(Machine {
                buttons: arr2(&[[a0, a1], [b0, b1]]),
                prize: arr1(&[p0, p1]),
            });
        }
    }
    Ok(result)
}

fn inverse_matrix(matrix: &Array2<f64>) -> Array2<f64> {
    let determinant = (matrix[[0, 0]] * matrix[[1, 1]]) - (matrix[[0, 1]] * matrix[[1, 0]]);
    let adjoint: Array2<f64> = arr2(&[
        [matrix[[1, 1]], -matrix[[0, 1]]],
        [-matrix[[1, 0]], matrix[[0, 0]]],
    ]);
    // print!("{:?}", (determinant, &adjoint));
    1. / determinant * adjoint
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let machines = read_input(reader)?;
        let answer: f64 = machines
            .iter()
            .filter_map(|machine| {
                let moves: Array1<f64> = machine.prize.dot(&inverse_matrix(&machine.buttons));

                if (moves[[0]] - moves[[0]].round()).abs() < 1e-4
                    && (moves[[1]] - moves[[1]].round()).abs() < 1e-4
                {
                    // println!("{}", moves);
                    let cost: f64 = moves.dot(&arr1(&[3., 1.]));
                    return Some(cost);
                }
                None
            })
            .sum();

        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file: BufReader<File> = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let error = 10000000000000.;
        let machines = read_input(reader)?;
        let answer: f64 = machines
            .iter()
            .filter_map(|machine| {
                let prize: Array1<f64> = error + &machine.prize;
                let moves: Array1<f64> = prize.dot(&inverse_matrix(&machine.buttons));
                if (moves[[0]] - moves[[0]].round()).abs() < 1e-4
                    && (moves[[1]] - moves[[1]].round()).abs() < 1e-4
                {
                    let cost: f64 = moves.dot(&arr1(&[3., 1.]));
                    return Some(cost);
                }
                None
            })
            .sum();

        Ok(answer as usize)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

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
    fn test_read_input() {
        let machines = read_input(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(
            machines[0],
            Machine {
                buttons: arr2(&[[94., 34.], [22., 67.]]),
                prize: arr1(&[8400., 5400.])
            }
        );
        assert_eq!(
            machines[1],
            Machine {
                buttons: arr2(&[[26., 66.], [67., 21.]]),
                prize: arr1(&[12748., 12176.])
            }
        );
    }

    #[test]
    fn test_inverse_matrix() {
        let machines = read_input(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(
            inverse_matrix(&machines[0].buttons),
            arr2(&[[67. / 5550., -34. / 5550.], [-22. / 5550., 94. / 5550.]])
        );
    }
}
