use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map, map_res, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair},
};

use std::fs;

const FILE_PATH: &str = "./input1.txt";
const DAY_AND_PART: &str = "Day 1 Part 1";

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Rotation {
    dir: Direction,
    steps: i32,
}

fn rot(dir: Direction, steps: i32) -> Rotation {
    Rotation { dir, steps }
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    ))
    .parse(input)
}

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse).parse(input)
}

fn rotation(input: &str) -> IResult<&str, Rotation> {
    map(pair(direction, number), |(dir, steps)| rot(dir, steps)).parse(input)
}

fn rotations(input: &str) -> IResult<&str, Vec<Rotation>> {
    all_consuming(delimited(
        many0(line_ending),
        separated_list1(line_ending, rotation),
        many0(line_ending),
    ))
    .parse(input)
}

fn rotate_dial(rotations: &[Rotation]) -> i32 {
    let dial_size = 100;

    let (_final_pos, zeros_hit) =
        rotations
            .iter()
            .fold((50i32, 0i32), |(dial_pos, zeros_hit), rotation| {
                let zeros_hit = zeros_hit + i32::from(dial_pos == 0);

                let delta = match rotation.dir {
                    Direction::Left => -(rotation.steps as i32),
                    Direction::Right => rotation.steps as i32,
                };

                let dial_pos = (dial_pos + delta).rem_euclid(dial_size);

                (dial_pos, zeros_hit)
            });

    zeros_hit
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, rotations) = rotations(&input).expect("should parse");
    let result = rotate_dial(&rotations);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
L68
R48
";

        let expected: Vec<Rotation> = vec![rot(Direction::Left, 68), rot(Direction::Right, 48)];

        let (_remaining, parsed) = rotations(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_1() {
        let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let (_remaining, rotations) = rotations(input).expect("should parse");

        let result = rotate_dial(&rotations);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
