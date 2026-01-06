use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, space1},
    combinator::{map, map_res, value},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 12 Part 1";
const N_SHAPES: usize = 6;

#[derive(Clone, Debug, PartialEq)]
struct Parsed {
    shapes: [Shape; N_SHAPES],
    areas: Vec<Area>,
}

type Cell = usize;
type Shape = usize;

#[derive(Clone, Debug, PartialEq)]
struct Area {
    width: usize,
    height: usize,
    shapes: [usize; N_SHAPES],
}

fn cell(input: &str) -> IResult<&str, Cell> {
    alt((value(1, tag("#")), value(0, tag(".")))).parse(input)
}

fn shape(input: &str) -> IResult<&str, Shape> {
    let (input, grid) = separated_list1(line_ending, many1(cell)).parse(input)?;

    let count = grid.iter().flatten().sum();
    Ok((input, count))
}

fn shape_item(input: &str) -> IResult<&str, Shape> {
    preceded((digit1, tag(":"), line_ending), shape).parse(input)
}

fn shapes(input: &str) -> IResult<&str, [Shape; N_SHAPES]> {
    map(separated_list1(multispace0, shape_item), |v| {
        v.try_into().expect("should be array of size 6")
    })
    .parse(input)
}

fn num_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

fn area(input: &str) -> IResult<&str, Area> {
    map(
        separated_pair(
            separated_pair(num_usize, tag("x"), num_usize),
            tag(": "),
            map(separated_list1(space1, num_usize), |s| {
                s.try_into().expect("should have 6 shapes")
            }),
        ),
        |((width, height), shapes)| Area {
            width,
            height,
            shapes,
        },
    )
    .parse(input)
}

fn areas(input: &str) -> IResult<&str, Vec<Area>> {
    separated_list1(line_ending, area).parse(input)
}

fn parse(input: &str) -> IResult<&str, Parsed> {
    let (input, shapes) = shapes(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, areas) = areas(input)?;

    let parsed = Parsed { shapes, areas };

    Ok((input, parsed))
}

fn is_packable(area: &Area, shapes: &[Shape; N_SHAPES]) -> bool {
    let total = area.width * area.height;
    let needed = area
        .shapes
        .iter()
        .zip(shapes.iter())
        .map(|(&n, &size)| n * size)
        .sum::<usize>();

    total > needed
}

fn solve(parsed: &Parsed) -> usize {
    parsed
        .areas
        .iter()
        .filter(|a| is_packable(a, &parsed.shapes))
        .count()
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
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let expected: Parsed = Parsed {
            shapes: [7, 7, 7, 7, 7, 7],
            areas: vec![
                Area {
                    width: 4,
                    height: 4,
                    shapes: [0, 0, 0, 0, 2, 0],
                },
                Area {
                    width: 12,
                    height: 5,
                    shapes: [1, 0, 1, 0, 2, 2],
                },
                Area {
                    width: 12,
                    height: 5,
                    shapes: [1, 0, 1, 0, 3, 2],
                },
            ],
        };

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_12_part_1() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(&items);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
