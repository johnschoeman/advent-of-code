use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 2 Part 1";

#[derive(Clone, Debug, PartialEq)]
struct IDRange(u64, u64);

fn id(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
}

fn id_range(input: &str) -> IResult<&str, IDRange> {
    map(separated_pair(id, tag("-"), id), |(start, end)| {
        IDRange(start, end)
    })
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<IDRange>> {
    all_consuming(delimited(
        many0(line_ending),
        separated_list0(tag(","), id_range),
        many1(line_ending),
    ))
    .parse(input)
}

fn is_valid_id(id: &str) -> bool {
    let is_odd = id.len() % 2 == 1;
    if is_odd {
        return true;
    }

    let mid = id.len() / 2;

    let (left, right) = id.split_at(mid);

    left != right
}

fn invalid_in_range(id_range: &IDRange) -> Vec<u64> {
    let mut invalid_ids = vec![];
    let (start, end) = (id_range.0, id_range.1 + 1);

    for id in start..end {
        let id_str: &str = &id.to_string();
        if !is_valid_id(id_str) {
            invalid_ids.push(id);
        }
    }

    invalid_ids
}

fn solve(parsed: &[IDRange]) -> u64 {
    let result = parsed.iter().fold(0, |acc, range| {
        let invalid_ids = invalid_in_range(range);
        let sum_of_range = invalid_ids.into_iter().fold(0, |a, id| a + id);
        acc + sum_of_range
    });
    result
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
            IDRange(11, 22),
            IDRange(95, 115),
            IDRange(998, 1012),
            IDRange(1188511880, 1188511890),
            IDRange(222220, 222224),
            IDRange(1698522, 1698528),
            IDRange(446443, 446449),
            IDRange(38593856, 38593862),
            IDRange(565653, 565659),
            IDRange(824824821, 824824827),
            IDRange(2121212118, 2121212124),
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_is_valid_id() {
        let id1 = "11";
        let result1 = is_valid_id(id1);
        assert_eq!(result1, false, "id: 11");

        let id2 = "55";
        let result2 = is_valid_id(id2);
        assert_eq!(result2, false, "id: 55");

        let id3 = "6464";
        let result3 = is_valid_id(id3);
        assert_eq!(result3, false, "id: 6464");

        let id4 = "423423";
        let result4 = is_valid_id(id4);
        assert_eq!(result4, false, "id: 423423");

        let id5 = "0505";
        let result5 = is_valid_id(id5);
        assert_eq!(result5, false, "id: 0505");

        let id6 = "606";
        let result6 = is_valid_id(id6);
        assert_eq!(result6, true, "id: 606");

        let id7 = "1234";
        let result7 = is_valid_id(id7);
        assert_eq!(result7, true, "id: 1234");
    }

    #[test]
    fn test_invalid_in_range() {
        let id_range1 = IDRange(11, 22);
        let result1 = invalid_in_range(&id_range1);
        assert_eq!(result1, vec![11, 22], "id range: 11-22");

        // 95-115 has one invalid ID, 99.
        let id_range2 = IDRange(95, 115);
        let result2 = invalid_in_range(&id_range2);
        assert_eq!(result2, vec![99], "id range: 95-115");

        // 998-1012 has one invalid ID, 1010.
        let id_range3 = IDRange(998, 1012);
        let result3 = invalid_in_range(&id_range3);
        assert_eq!(result3, vec![1010], "id range: 998-1012");

        // 1188511880-1188511890 has one invalid ID, 1188511885.
        let id_range4 = IDRange(1188511880, 1188511890);
        let result4 = invalid_in_range(&id_range4);
        assert_eq!(result4, vec![1188511885], "id range: 1188511880-1188511890");

        // 222220-222224 has one invalid ID, 222222.
        let id_range5 = IDRange(222220, 222224);
        let result5 = invalid_in_range(&id_range5);
        assert_eq!(result5, vec![222222], "id range: 222220-222224");

        // 1698522-1698528 contains no invalid IDs.
        let id_range6 = IDRange(1698522, 1698528);
        let result6 = invalid_in_range(&id_range6);
        assert_eq!(result6, vec![], "id range: 1698522-1698528");
    }

    #[test]
    fn test_day_2_part_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let (_remaining, id_ranges) = parse(input).expect("should parse");

        let result = solve(&id_ranges);
        let expected = 1227775554;
        assert_eq!(result, expected);
    }
}
