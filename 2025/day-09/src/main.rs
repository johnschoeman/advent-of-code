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
const DAY_AND_PART: &str = "Day 9 Part 2";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    a: Position,
    b: Position,
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

fn edges(points: &[Position]) -> Vec<Edge> {
    let first = points.first().copied();
    points
        .iter()
        .copied()
        .zip(points.iter().copied().skip(1).chain(first))
        .map(move |(a, b)| Edge { a, b, })
        .collect::<Vec<Edge>>()
}

fn rect_ok_for_edge(a: Position, b: Position, e: Edge) -> bool {
    let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));
    let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));

    let (emin_x, emax_x) = (e.a.x.min(e.b.x), e.a.x.max(e.b.x));
    let (emin_y, emax_y) = (e.a.y.min(e.b.y), e.a.y.max(e.b.y));

    let left  = max_x <= emin_x;
    let right = min_x >= emax_x;
    let above = max_y <= emin_y;
    let below = min_y >= emax_y;

    left || right || above || below
}

fn solve(points: &[Position]) -> isize {
    let es = edges(points);

    let mut best = 0;

    for (i, &a) in points.iter().enumerate() {
        for &b in &points[i + 1..] {
            let candidate = area(a, b);

            if candidate <= best {
                continue;
            }

            let ok = es.iter().copied().all(|e| rect_ok_for_edge(a, b, e));
            if ok {
                best = candidate;
            }
        }
    }

    best
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
    fn test_day_9_part_2() {
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
        let expected = 24;
        assert_eq!(result, expected);
    }
}
