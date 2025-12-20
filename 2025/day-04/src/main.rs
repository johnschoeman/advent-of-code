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
const DAY_AND_PART: &str = "Day 4 Part 2";

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Dot,
    At,
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn idx(&self, row_idx: usize, col_idx: usize) -> usize {
        row_idx * self.width + col_idx
    }

    fn get(&self, row_idx: isize, col_idx: isize) -> Option<Cell> {
        if (row_idx < 0) || (col_idx < 0) {
            return None;
        }

        let (row_idx, col_idx) = (row_idx as usize, col_idx as usize);
        if (row_idx >= self.height) || (col_idx >= self.width) {
            return None;
        }

        Some(self.cells[self.idx(row_idx, col_idx)])
    }

    fn neighbors8(&self, row_idx: usize, col_idx: usize) -> impl Iterator<Item = Cell> + '_ {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let (r, c) = (row_idx as isize, col_idx as isize);

        OFFSETS
            .into_iter()
            .filter_map(move |(dr, dc)| self.get(r + dr, c + dc))
    }
}

fn cell(input: &str) -> IResult<&str, Cell> {
    alt((value(Cell::At, tag("@")), value(Cell::Dot, tag(".")))).parse(input)
}

fn row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(cell).parse(input)
}

fn parse(input: &str) -> IResult<&str, Grid> {
    let (rest, rows) = all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, row),
        multispace0,
    ))
    .parse(input)?;

    let height = rows.len();
    let width = rows.first().map(|r| r.len()).unwrap_or(0);

    debug_assert!(rows.iter().all(|r| r.len() == width));

    let cells: Vec<Cell> = rows.into_iter().flatten().collect();

    Ok((
        rest,
        Grid {
            width,
            height,
            cells,
        },
    ))
}

fn remove_accessible(grid: &Grid) -> (u32, Grid) {
    let mut accessible = 0u32;
    let mut next_cells: Vec<Cell> = Vec::with_capacity(grid.cells.len());

    for row_idx in 0..grid.height {
        for col_idx in 0..grid.width {
            if grid.cells[grid.idx(row_idx, col_idx)] == Cell::Dot {
                next_cells.push(Cell::Dot);
                continue;
            }

            let adjacent_ats = grid
                .neighbors8(row_idx, col_idx)
                .filter(|c| *c == Cell::At)
                .count();

            if adjacent_ats < 4 {
                next_cells.push(Cell::Dot);
                accessible += 1;
            } else {
                next_cells.push(Cell::At);
            }
        }
    }

    let next_grid: Grid = Grid {
        width: grid.width,
        height: grid.height,
        cells: next_cells,
    };

    (accessible, next_grid)
}

fn solve(mut grid: Grid) -> u32 {
    let mut removed = 0u32;

    loop {
        let (count_removed, next_grid) = remove_accessible(&grid);
        removed += count_removed;

        if count_removed == 0 {
            break removed;
        }
        grid = next_grid;
    }
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, parsed) = parse(&input).expect("should parse");
    let result = solve(parsed);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "..@
@@@
@.@";

        let expected: Grid = Grid {
            height: 3,
            width: 3,
            cells: vec![
                Cell::Dot,
                Cell::Dot,
                Cell::At,
                Cell::At,
                Cell::At,
                Cell::At,
                Cell::At,
                Cell::Dot,
                Cell::At,
            ],
        };

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_remove_accesible() {
        let input = "..@
@@@
@.@";

        let (_remaining, grid) = parse(input).expect("should parse");

        let result = remove_accessible(&grid);

        let expected_grid_input = "...
.@.
...";
        let (_remaining, next_grid) = parse(expected_grid_input).expect("should parse");

        let expected = (5, next_grid);

        assert_eq!(result, expected);
    }

    //  0123456789
    //0 ..xx.xx@x.
    //1 x@@.@.@.@@
    //2 @@@@@.x.@@
    //3 @.@@@@..@.
    //4 x@.@@@@.@x
    //5 .@@@@@@@.@
    //6 .@.@.@.@@@
    //7 x.@@@.@@@@
    //8 .@@@@@@@@.
    //9 x.x.@@@.x.

    #[test]
    fn test_day_4_part_2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(items);
        let expected = 43;
        assert_eq!(result, expected);
    }
}
