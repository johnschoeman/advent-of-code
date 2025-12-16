use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 2 Part 2";

#[derive(Clone, Debug, PartialEq)]
struct IDRange {
    start: u64,
    end: u64,
}

fn id(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
}

fn id_range(input: &str) -> IResult<&str, IDRange> {
    map(separated_pair(id, tag("-"), id), |(start, end)| IDRange {
        start,
        end,
    })
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<IDRange>> {
    all_consuming(delimited(
        multispace0,
        separated_list0(tag(","), id_range),
        multispace0,
    ))
    .parse(input)
}

fn is_valid_id(id: &str) -> bool {
    let bytes = id.as_bytes();
    let n = bytes.len();
    let mid = n / 2;

    for chunk_size in 1..=mid {
        if n % chunk_size != 0 {
            continue;
        }

        let pattern = &bytes[..chunk_size];
        let repeats = bytes.chunks_exact(chunk_size).all(|chunk| chunk == pattern);

        if repeats {
            return false;
        }
    }

    true
}

fn invalid_in_range_sum(id_range: &IDRange) -> u64 {
    (id_range.start..=id_range.end)
        .filter(|id| !is_valid_id(&id.to_string()))
        .sum()
}

fn solve(parsed: &[IDRange]) -> u64 {
    parsed.iter().map(invalid_in_range_sum).sum()
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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let expected: Vec<IDRange> = vec![
            IDRange { start: 11, end: 22 },
            IDRange {
                start: 95,
                end: 115,
            },
            IDRange {
                start: 998,
                end: 1012,
            },
            IDRange {
                start: 1188511880,
                end: 1188511890,
            },
            IDRange {
                start: 222220,
                end: 222224,
            },
            IDRange {
                start: 1698522,
                end: 1698528,
            },
            IDRange {
                start: 446443,
                end: 446449,
            },
            IDRange {
                start: 38593856,
                end: 38593862,
            },
            IDRange {
                start: 565653,
                end: 565659,
            },
            IDRange {
                start: 824824821,
                end: 824824827,
            },
            IDRange {
                start: 2121212118,
                end: 2121212124,
            },
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_is_valid_id() {
        let cases = [
            ("11", false),
            ("55", false),
            ("6464", false),
            ("423423", false),
            ("0505", false),
            ("606", true),
            ("1234", true),
            ("121212", false),
        ];

        for (id, expected) in cases {
            assert_eq!(is_valid_id(id), expected, "id: {id}")
        }
    }

    #[test]
    fn test_invalid_in_range_sum() {
        let cases = [
            (IDRange { start: 11, end: 22 }, 11 + 22),
            (
                IDRange {
                    start: 95,
                    end: 115,
                },
                99 + 111,
            ),
            (
                IDRange {
                    start: 998,
                    end: 1012,
                },
                999 + 1010,
            ),
            (
                IDRange {
                    start: 1188511880,
                    end: 1188511890,
                },
                1188511885,
            ),
            (
                IDRange {
                    start: 1698522,
                    end: 1698528,
                },
                0,
            ),
        ];

        for (range, expected) in cases {
            assert_eq!(invalid_in_range_sum(&range), expected);
        }
    }

    #[test]
    fn test_day_2_part_2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let (_remaining, id_ranges) = parse(input).expect("should parse");

        let result = solve(&id_ranges);
        let expected = 4174379265;
        assert_eq!(result, expected);
    }
}
