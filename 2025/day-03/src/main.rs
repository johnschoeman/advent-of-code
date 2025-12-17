use nom::Parser;
use nom::{
    IResult,
    character::complete::{digit1, line_ending},
    combinator::all_consuming,
    multi::{many0, separated_list1},
    sequence::delimited,
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 3 Part 1";

type BatteryBank = Vec<u32>;

fn battery_bank(input: &str) -> IResult<&str, BatteryBank> {
    digit1
        .map(|s: &str| s.bytes().map(|b| (b - b'0') as u32).collect())
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<BatteryBank>> {
    all_consuming(delimited(
        many0(line_ending),
        separated_list1(line_ending, battery_bank),
        many0(line_ending),
    ))
    .parse(input)
}

fn bank_has_jolt(bank: &[u32], jolt: u32) -> bool {
    let first = jolt / 10;
    let second = jolt % 10;

    bank.iter()
        .position(|&x| x == first)
        .is_some_and(|i| bank[i + 1..].iter().any(|&y| y == second))
}

fn find_largest_joltage(bank: &[u32]) -> u32 {
    (10..=99)
        .rev()
        .find(|&jolt| bank_has_jolt(bank, jolt))
        .unwrap_or(0)
}

fn solve(parsed: &[BatteryBank]) -> u32 {
    parsed.iter().map(|bank| find_largest_joltage(bank)).sum()
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
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let expected: Vec<BatteryBank> = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_find_largest_joltage() {
        let cases = [
            (vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 98),
            (vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 89),
            (vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 78),
            (vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 92),
        ];

        for (bank, expected) in cases {
            let result = find_largest_joltage(&bank);
            assert_eq!(result, expected, "{}", format!("bank: {:?}", bank))
        }
    }

    #[test]
    fn test_day_3_part_1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(&items);
        let expected = 357;
        assert_eq!(result, expected);
    }
}
