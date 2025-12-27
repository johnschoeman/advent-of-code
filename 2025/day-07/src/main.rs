use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0},
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::delimited,
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 7 Part 2";

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Start,
    Split,
}

type Node = usize;

#[derive(Debug, PartialEq)]
struct Grid {
    start_node: Node,
    height: usize,
    width: usize,
    splits: Vec<Vec<bool>>,
}

fn cell(input: &str) -> IResult<&str, Cell> {
    alt((
        value(Cell::Empty, tag(".")),
        value(Cell::Start, tag("S")),
        value(Cell::Split, tag("^")),
    ))
    .parse(input)
}

fn line(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(cell).parse(input)
}

fn parse(input: &str) -> IResult<&str, Grid> {
    let (rest, grid) = all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, line),
        multispace0,
    ))
    .parse(input)?;

    let start_node = grid[0]
        .iter()
        .position(|&cell| cell == Cell::Start)
        .expect("start exists");

    let splits: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| row.iter().map(|&cell| cell == Cell::Split).collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let out: Grid = Grid {
        start_node,
        width,
        height,
        splits,
    };

    Ok((rest, out))
}

fn solve(grid: &Grid) -> u64 {
    let mut curr = vec![0u64; grid.width];
    curr[grid.start_node] = 1;

    for row_idx in 1..grid.height {
        let splits = &grid.splits[row_idx];
        let mut next = vec![0u64; grid.width];

        for col_idx in 0..grid.width {
            let path_count = curr[col_idx];

            if splits[col_idx] {
                next[col_idx - 1] += path_count;
                next[col_idx + 1] += path_count;
            } else {
                next[col_idx] += path_count;
            }
        }

        curr = next;
    }

    curr.iter().sum()
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, grid) = parse(&input).expect("should parse");
    let result = solve(&grid);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let expected_start_node = 7;
        let expected_height = 16;
        let expected_width = 15;
        let expected_splits = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ];

        let (_remaining, grid) = parse(input).expect("parser should succeed");

        assert_eq!(grid.width, expected_width);
        assert_eq!(grid.height, expected_height);
        assert_eq!(grid.start_node, expected_start_node);
        assert_eq!(grid.splits[0], expected_splits);
    }

    #[test]
    fn test_day_7_part_2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let (_remaining, grid) = parse(input).expect("should parse");

        let result = solve(&grid);
        let expected = 40;
        assert_eq!(result, expected);
    }
}
