use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    combinator::{all_consuming, map, map_res, value},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 6 Part 1";

#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq)]
struct Parsed {
    numbers: Vec<Vec<u64>>,
    ops: Vec<Op>,
}

fn u64_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
}

fn number_line(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(
        delimited(space0, separated_list1(space1, u64_number), space0),
        line_ending,
    )
    .parse(input)
}

fn op(input: &str) -> IResult<&str, Op> {
    alt((value(Op::Add, tag("+")), value(Op::Mul, tag("*")))).parse(input)
}

fn ops_line(input: &str) -> IResult<&str, Vec<Op>> {
    terminated(
        delimited(space0, separated_list1(space1, op), space0),
        line_ending,
    )
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Parsed> {
    map(
        all_consuming((many1(number_line), ops_line)),
        |(numbers, ops)| Parsed { numbers, ops },
    )
    .parse(input)
}

fn solve(parsed: &Parsed) -> u64 {
    parsed
        .ops
        .iter()
        .enumerate()
        .map(|(col, &op)| {
            let values = parsed.numbers.iter().map(|row| row[col]);

            match op {
                Op::Add => values.sum::<u64>(),
                Op::Mul => values.product::<u64>(),
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, parsed) = parse(&input).expect("should parse");
    let result = solve(&parsed);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";

        let expected: Parsed = Parsed {
            numbers: vec![
                vec![123, 328, 51, 64],
                vec![45, 64, 387, 23],
                vec![6, 98, 215, 314],
            ],
            ops: vec![Op::Mul, Op::Add, Op::Mul, Op::Add],
        };

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_6_part_1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";
        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(&items);
        let expected = 4277556;
        assert_eq!(result, expected);
    }
}
