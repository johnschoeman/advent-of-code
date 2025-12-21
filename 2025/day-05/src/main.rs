use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    combinator::{all_consuming, map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 5 Part 2";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FreshIdRange {
    start: u64,
    end: u64,
}

impl FreshIdRange {
    fn contains(&self, id: u64) -> bool {
        (self.start..=self.end).contains(&id)
    }
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
}

fn fresh_id_range(input: &str) -> IResult<&str, FreshIdRange> {
    map(separated_pair(number, tag("-"), number), |(start, end)| {
        FreshIdRange { start, end }
    })
    .parse(input)
}

fn fresh_id_ranges(input: &str) -> IResult<&str, Vec<FreshIdRange>> {
    separated_list1(line_ending, fresh_id_range).parse(input)
}

fn ids(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(separated_list1(line_ending, number), multispace0).parse(input)
}

fn blank_line1(input: &str) -> IResult<&str, ()> {
    map(many1(line_ending), |_| ()).parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<FreshIdRange>, Vec<u64>)> {
    all_consuming(delimited(
        multispace0,
        separated_pair(fresh_id_ranges, blank_line1, ids),
        multispace0,
    ))
    .parse(input)
}

fn solve(id_ranges: &[FreshIdRange]) -> u64 {
    let mut sorted = id_ranges.to_vec();
    sorted.sort();

    let mut count = 0;
    let mut current = 0;

    for range in sorted {
        let mut s = range.start;
        if current >= s {
            s = current + 1;
        }
        if range.end >= s {
            count += range.end - s + 1;
        }
        current = std::cmp::max(range.end, current);
    }

    count
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, (id_ranges, _ids)) = parse(&input).expect("should parse");
    let result = solve(&id_ranges);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let expected: (Vec<FreshIdRange>, Vec<u64>) = (
            vec![
                FreshIdRange { start: 3, end: 5 },
                FreshIdRange { start: 10, end: 14 },
                FreshIdRange { start: 16, end: 20 },
                FreshIdRange { start: 12, end: 18 },
            ],
            vec![1, 5, 8, 11, 17, 32],
        );

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_4_part_2() {
        let input = "3-5
16-20
12-18
17-19
10-14

1
5
8
11
17
32";
        let (_remaining, (id_ranges, _ids)) = parse(input).expect("should parse");

        let result = solve(&id_ranges);
        let expected = 14;
        assert_eq!(result, expected);
    }
}
