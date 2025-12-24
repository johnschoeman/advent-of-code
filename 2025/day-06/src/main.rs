use nom::Parser;
use nom::{
    IResult,
    character::complete::{line_ending, not_line_ending},
    multi::separated_list1,
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 6 Part 2";

#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

type Parsed = Vec<(Op, Vec<u64>)>;

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if grid.is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let cols = grid[0].len();

    (0..cols)
        .map(|c| (0..rows).map(|r| grid[r][c]).collect())
        .collect()
}

fn parse(input: &str) -> IResult<&str, Parsed> {
    let (rest, lines) = separated_list1(line_ending, not_line_ending).parse(input)?;

    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let columns = transpose(&grid);

    let parsed: Parsed = columns
        .split(|col| col.iter().all(|&c| c == ' '))
        .map(|group| {
            let op_char = *group[0].last().expect("should be non empty column");
            let op = Op::try_from(op_char).expect("should be an operator");

            let numbers = group
                .iter()
                .map(|col| {
                    let s: String = col[..col.len() - 1].iter().collect();
                    s.trim().parse::<u64>().expect("should be a number")
                })
                .collect::<Vec<u64>>();

            (op, numbers)
        })
        .collect();

    Ok((rest, parsed))
}

fn solve(parsed: &Parsed) -> u64 {
    parsed
        .iter()
        .map(|p| match p {
            (Op::Mul, numbers) => numbers.iter().product::<u64>(),
            (Op::Add, numbers) => numbers.iter().sum::<u64>(),
        })
        .sum::<u64>()
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
*   +   *   +  ";

        let expected: Parsed = vec![
            (Op::Mul, vec![1, 24, 356]),
            (Op::Add, vec![369, 248, 8]),
            (Op::Mul, vec![32, 581, 175]),
            (Op::Add, vec![623, 431, 4]),
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_6_part_2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(&items);
        let expected = 3263827;
        assert_eq!(result, expected);
    }
}
