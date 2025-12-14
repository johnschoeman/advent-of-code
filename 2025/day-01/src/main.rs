use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map, map_res, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair},
};

use std::fs;

const FILE_PATH: &str = "./input1.txt";
const DAY_AND_PART: &str = "Day 1 Part 2";

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Rotation {
    dir: Direction,
    steps: i32,
}

fn rot(dir: Direction, steps: i32) -> Rotation {
    Rotation { dir, steps }
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    ))
    .parse(input)
}

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse).parse(input)
}

fn rotation(input: &str) -> IResult<&str, Rotation> {
    map(pair(direction, number), |(dir, steps)| rot(dir, steps)).parse(input)
}

fn rotations(input: &str) -> IResult<&str, Vec<Rotation>> {
    all_consuming(delimited(
        many0(line_ending),
        separated_list1(line_ending, rotation),
        many0(line_ending),
    ))
    .parse(input)
}

fn rotate_dial(dial_size: i32, dial_pos: i32, rotation: &Rotation) -> (i32, i32) {
    debug_assert!(dial_size > 0);
    debug_assert!(dial_pos < dial_size);

    let steps = rotation.steps as i32;

    let full_spins = steps.div_euclid(dial_size);

    let step_mod = steps.rem_euclid(dial_size);

    let delta = match rotation.dir {
        Direction::Left => -step_mod,
        Direction::Right => step_mod,
    };

    let next = (dial_pos + delta).rem_euclid(dial_size);

    let crossed_zero = dial_pos != 0
        && match rotation.dir {
            Direction::Left => next > dial_pos || next == 0,
            Direction::Right => next < dial_pos || next == 0,
        };

    (next, full_spins + i32::from(crossed_zero))
}

fn find_password(rotations: &[Rotation]) -> i32 {
    let dial_size = 100;

    let (_final_pos, zeros_hit) =
        rotations
            .iter()
            .fold((50i32, 0i32), |(dial_pos, zeros_hit), rotation| {
                let (next_dial_pos, next_zeros_hit) = rotate_dial(dial_size, dial_pos, &rotation);

                (next_dial_pos, zeros_hit + next_zeros_hit)
            });

    zeros_hit
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, rotations) = rotations(&input).expect("should parse");
    let result = find_password(&rotations);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
L68
R48
";

        let expected: Vec<Rotation> = vec![rot(Direction::Left, 68), rot(Direction::Right, 48)];

        let (_remaining, parsed) = rotations(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    // The dial starts by pointing at 50.
    //
    // 1 The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
    // 2 The dial is rotated L30 to point at 52.
    // 3 The dial is rotated R48 to point at 0.
    // 4 The dial is rotated L5 to point at 95.
    // 5 The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
    // 6 The dial is rotated L55 to point at 0.
    // 7 The dial is rotated L1 to point at 99.
    // 8 The dial is rotated L99 to point at 0.
    // 9 The dial is rotated R14 to point at 14.
    // 10 The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.

    #[test]
    fn test_rotate_dial() {
        let dial_size = 100;

        let (pos1, hit1) = rotate_dial(dial_size, 50, &rot(Direction::Left, 68));
        assert_eq!((pos1, hit1), (82, 1), "step 1");

        let (pos2, hit2) = rotate_dial(dial_size, 82, &rot(Direction::Left, 30));
        assert_eq!((pos2, hit2), (52, 0), "step 2");

        let (pos3, hit3) = rotate_dial(dial_size, 52, &rot(Direction::Right, 48));
        assert_eq!((pos3, hit3), (0, 1), "step 3");

        let (pos4, hit4) = rotate_dial(dial_size, 0, &rot(Direction::Left, 5));
        assert_eq!((pos4, hit4), (95, 0), "step 4");

        let (pos5, hit5) = rotate_dial(dial_size, 95, &rot(Direction::Right, 60));
        assert_eq!((pos5, hit5), (55, 1), "step 5");

        let (pos6, hit6) = rotate_dial(dial_size, 55, &rot(Direction::Left, 55));
        assert_eq!((pos6, hit6), (0, 1), "step 6");

        let (pos7, hit7) = rotate_dial(dial_size, 0, &rot(Direction::Left, 1));
        assert_eq!((pos7, hit7), (99, 0), "step 7");

        let (pos8, hit8) = rotate_dial(dial_size, 99, &rot(Direction::Left, 99));
        assert_eq!((pos8, hit8), (0, 1), "step 8");

        let (pos9, hit9) = rotate_dial(dial_size, 0, &rot(Direction::Right, 14));
        assert_eq!((pos9, hit9), (14, 0), "step 9");

        let (pos10, hit10) = rotate_dial(dial_size, 14, &rot(Direction::Left, 82));
        assert_eq!((pos10, hit10), (32, 1), "step 10");
    }

    #[test]
    fn test_day_1_part_2() {
        let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let (_remaining, rotations) = rotations(input).expect("should parse");

        let result = find_password(&rotations);
        let expected = 6;
        assert_eq!(result, expected);
    }
}
