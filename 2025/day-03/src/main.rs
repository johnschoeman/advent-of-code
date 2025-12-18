use nom::Parser;
use nom::{
    IResult,
    character::complete::{digit1, line_ending, multispace0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::delimited,
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 3 Part 1";

#[derive(Debug, PartialOrd, PartialEq)]
struct BatteryBank {
    digits: Vec<u8>,
}

fn digits_to_u64(digits: &[u8]) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn battery_bank(input: &str) -> IResult<&str, BatteryBank> {
    digit1
        .map(|s: &str| s.bytes().map(|b| b - b'0').collect())
        .map(|digits| BatteryBank { digits })
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<BatteryBank>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, battery_bank),
        multispace0,
    ))
    .parse(input)
}

fn find_largest_joltage(joltage_size: usize, battery_bank: &BatteryBank) -> u64 {
    let n = battery_bank.digits.len();

    if n <= joltage_size {
        return digits_to_u64(&battery_bank.digits);
    }

    let mut max_bank = BatteryBank { digits: vec![0] };
    for idx in 0..n {
        let mut next_bank = battery_bank.digits.clone();
        next_bank.remove(idx);
        let next_battery_bank = BatteryBank { digits: next_bank };
        if next_battery_bank > max_bank {
            max_bank = next_battery_bank;
        }
    }

    find_largest_joltage(12, &max_bank)
}

fn solve(parsed: &[BatteryBank]) -> u64 {
    parsed
        .iter()
        .map(|bank| find_largest_joltage(12, bank))
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
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let expected: Vec<BatteryBank> = vec![
            BatteryBank {
                digits: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            },
            BatteryBank {
                digits: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            },
            BatteryBank {
                digits: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            },
            BatteryBank {
                digits: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
            },
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_find_largest_joltage() {
        let cases = [
            (
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                987654321111,
            ),
            (
                vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                811111111119,
            ),
            (
                vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                434234234278,
            ),
            (
                vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                888911112111,
            ),
        ];

        for (digits, expected) in cases {
            let battery_bank = BatteryBank { digits: digits.clone() };
            let result = find_largest_joltage(12, &battery_bank);
            assert_eq!(result, expected, "{}", format!("bank: {:?}", digits))
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
        let expected = 3121910778619;
        assert_eq!(result, expected);
    }
}
