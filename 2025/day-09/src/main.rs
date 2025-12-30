use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 9 Part 1";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

fn number(input: &str) -> IResult<&str, isize> {
    digit1.map_res(str::parse).parse(input)
}

fn position(input: &str) -> IResult<&str, Position> {
    map(separated_pair(number, tag(","), number), |(x, y)| {
        Position { x, y }
    })
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Position>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, position),
        multispace0,
    ))
    .parse(input)
}

fn area(a: Position, b: Position) -> isize {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn solve(points: &[Position]) -> isize {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| points[i + 1..].iter().map(move |&b| area(a, b)))
        .max()
        .unwrap_or(0)
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
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let expected: Vec<Position> = vec![
            Position { x: 7, y: 1 },
            Position { x: 11, y: 1 },
            Position { x: 11, y: 7 },
            Position { x: 9, y: 7 },
            Position { x: 9, y: 5 },
            Position { x: 2, y: 5 },
            Position { x: 2, y: 3 },
            Position { x: 7, y: 3 },
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_9_part_1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let (_remaining, tiles) = parse(input).expect("should parse");

        let result = solve(&tiles);
        let expected = 50;
        assert_eq!(result, expected);
    }
}
