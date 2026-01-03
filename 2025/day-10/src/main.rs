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
use z3::{Optimize, ast::Int};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 10 Part 2";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Light {
    On,
    Off,
}

type LightDiagram = Vec<Light>;
type ButtonWiring = Vec<Int>;
type Joltage = Vec<u64>;

#[derive(Clone, Debug, PartialEq)]
struct Machine {
    button_wirings: Vec<ButtonWiring>,
    light_target: LightDiagram,
    joltage_target: Joltage,
}

fn light(input: &str) -> IResult<&str, Light> {
    alt((value(Light::On, tag("#")), value(Light::Off, tag(".")))).parse(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    digit1.map_res(str::parse).parse(input)
}

fn number_u64(input: &str) -> IResult<&str, u64> {
    digit1.map_res(str::parse).parse(input)
}

fn light_diagram(input: &str) -> IResult<&str, LightDiagram> {
    delimited(tag("["), many1(light), tag("]")).parse(input)
}

fn buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        tag(" "),
        delimited(tag("("), separated_list1(tag(","), number), tag(")")),
    )
    .parse(input)
}

fn joltage(input: &str) -> IResult<&str, Joltage> {
    delimited(tag("{"), separated_list1(tag(","), number_u64), tag("}")).parse(input)
}

fn build_button_matrix(values: Vec<Vec<usize>>, n: usize) -> Vec<ButtonWiring> {
    let m = values.len();
    let mut button_wirings: Vec<ButtonWiring> = Vec::with_capacity(n);

    for i in 0..n {
        let mut wiring = Vec::with_capacity(m);
        for j in 0..m {
            if values[j].iter().find(|&x| *x == i).is_some() {
                wiring.push(Int::from_u64(1));
            } else {
                wiring.push(Int::from_u64(0));
            }
        }

        button_wirings.push(wiring);
    }

    button_wirings
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, light_target) = light_diagram.parse(input)?;
    let (input, buttons) = preceded(tag(" "), buttons).parse(input)?;
    let (input, joltage_target) = preceded(tag(" "), joltage).parse(input)?;

    let n = joltage_target.len();
    let button_wirings = build_button_matrix(buttons, n);

    Ok((
        input,
        Machine {
            button_wirings,
            light_target,
            joltage_target,
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

// x1  x2    x3  x4    x5    x6
// (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

// 0x1 + 0x2 + 0x3 + 0x4 + 1x5 + 1x6 = 3
// 0x1 + 1x2 + 0x3 + 0x4 + 0x5 + 1x6 = 5
// 0x1 + 0x2 + 1x3 + 1x4 + 1x5 + 0x6 = 4
// 1x1 + 1x2 + 0x3 + 1x4 + 0x5 + 0x6 = 7

// Ax = b
fn solve_machine(machine: &Machine) -> usize {
    let joltage = &machine.joltage_target;
    let n = joltage.len();

    let buttons = &machine.button_wirings;
    let m = buttons[0].len();

    let opt = Optimize::new();

    let mut x = Vec::with_capacity(m);
    let mut count = Int::from_u64(0);
    for i in 0..m {
        let xn = Int::new_const(format!("x{}", i));
        opt.assert(&xn.ge(0));
        count = count + xn.clone();
        x.push(xn);
    }
    opt.minimize(&count);

    for i in 0..n {
        let wiring = &buttons[i];
        let mut eq = Int::from_u64(0);
        for j in 0..m {
            eq = eq + wiring[j].clone() * &x[j];
        }
        opt.assert(&eq.eq(&Int::from_u64(joltage[i])));
    }

    if opt.check(&[]) == z3::SatResult::Sat {
        let model = opt.get_model().unwrap();

        let mut count = 0u64;
        for i in 0..m {
            let xn = model.eval(&x[i], true).unwrap().as_u64().unwrap();
            count += xn;
        }

        return count.try_into().unwrap();
    } else {
        println!("No solution found.");
        return 0;
    }
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
            button_wirings: vec![
                vec![
                    Int::from_u64(0),
                    Int::from_u64(0),
                    Int::from_u64(0),
                    Int::from_u64(0),
                    Int::from_u64(1),
                    Int::from_u64(1),
                ],
                vec![
                    Int::from_u64(0),
                    Int::from_u64(1),
                    Int::from_u64(0),
                    Int::from_u64(0),
                    Int::from_u64(0),
                    Int::from_u64(1),
                ],
                vec![
                    Int::from_u64(0),
                    Int::from_u64(0),
                    Int::from_u64(1),
                    Int::from_u64(1),
                    Int::from_u64(1),
                    Int::from_u64(0),
                ],
                vec![
                    Int::from_u64(1),
                    Int::from_u64(1),
                    Int::from_u64(0),
                    Int::from_u64(1),
                    Int::from_u64(0),
                    Int::from_u64(0),
                ],
            ],
            light_target: vec![Light::Off, Light::On, Light::On, Light::Off],
            joltage_target: vec![3, 5, 4, 7],
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
        let expected = 33;
        assert_eq!(result, expected);
    }
}
