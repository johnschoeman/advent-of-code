use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
};

use std::collections::{HashSet, VecDeque};
use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 10 Part 1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Light {
    On,
    Off,
}
impl Light {
    fn toggle(&self) -> Self {
        match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

type LightDiagram = Vec<Light>;
type ButtonWiring = Vec<usize>;
type Joltage = Vec<usize>;

#[derive(Clone, Debug, PartialEq)]
struct Machine {
    light_diagram: LightDiagram,
    button_wirings: Vec<ButtonWiring>,
    joltage: Joltage,
}

fn light(input: &str) -> IResult<&str, Light> {
    alt((value(Light::On, tag("#")), value(Light::Off, tag(".")))).parse(input)
}

fn num_usize(input: &str) -> IResult<&str, usize> {
    digit1.map_res(str::parse).parse(input)
}

fn light_diagram(input: &str) -> IResult<&str, LightDiagram> {
    delimited(tag("["), many1(light), tag("]")).parse(input)
}

fn button_wirings(input: &str) -> IResult<&str, Vec<ButtonWiring>> {
    separated_list1(
        tag(" "),
        delimited(tag("("), separated_list1(tag(","), num_usize), tag(")")),
    )
    .parse(input)
}

fn joltage(input: &str) -> IResult<&str, Joltage> {
    delimited(tag("{"), separated_list1(tag(","), num_usize), tag("}")).parse(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, light_diagram) = light_diagram.parse(input)?;
    let (input, button_wirings) = preceded(tag(" "), button_wirings).parse(input)?;
    let (input, joltage) = preceded(tag(" "), joltage).parse(input)?;

    Ok((
        input,
        Machine {
            light_diagram,
            button_wirings,
            joltage,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, machine),
        multispace0,
    ))
    .parse(input)
}

fn solve_machine(machine: &Machine) -> usize {
    let start: LightDiagram = vec![Light::Off; machine.light_diagram.len()];

    let mut q = VecDeque::from([(start.clone(), 0)]);
    let mut seen = HashSet::from([start]);

    while let Some((state, depth)) = q.pop_front() {
        for wiring in machine.button_wirings.iter() {
            let next = toggle_lights(&state, wiring);

            if next == machine.light_diagram {
                return depth + 1;
            }
            if seen.insert(next.clone()) {
                q.push_back((next, depth + 1))
            }
        }
    }

    0
}

fn toggle_lights(light_diagram: &LightDiagram, button_wiring: &ButtonWiring) -> LightDiagram {
    let mut next = light_diagram.clone();
    for button in button_wiring.iter() {
        next[*button] = next[*button].toggle();
    }
    next
}

fn solve(machines: &[Machine]) -> usize {
    machines.iter().map(solve_machine).sum()
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
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

        let expected: Vec<Machine> = vec![Machine {
            light_diagram: vec![Light::Off, Light::On, Light::On, Light::Off],
            button_wirings: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage: vec![3, 5, 4, 7],
        }];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_10_part_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let (_remaining, machines) = parse(input).expect("should parse");

        let result = solve(&machines);
        let expected = 7;
        assert_eq!(result, expected);
    }
}
