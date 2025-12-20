use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    combinator::{all_consuming, map, map_res, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day X Part 1";

#[derive(Clone, Debug, PartialEq)]
struct Item {
}

fn item(input: &str) -> IResult<&str, Item> {
    parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<X>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, item),
        multispace0,
    ))
    .parse(input)
}

fn solve(parsed: &[X]) -> i32 {
    1
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
        let input = "
";

        let expected: Vec<Item> = vec![];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_X_part_1() {
        let input = "
";
        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(&items);
        let expected = 6;
        assert_eq!(result, expected);
    }
}
