use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};

use pathfinding::prelude::count_paths;
use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 11 Part 2";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Device<'a> {
    name: &'a str,
    outputs: Vec<&'a str>,
}

fn device(input: &str) -> IResult<&str, Device<'_>> {
    map(
        separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
        |(name, outputs)| Device { name, outputs },
    )
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Device<'_>>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, device),
        multispace0,
    ))
    .parse(input)
}

type Node = usize;
type Index<'a> = HashMap<&'a str, Node>;

fn build_index<'a>(devices: &'a [Device<'a>]) -> Index<'a> {
    devices
        .iter()
        .enumerate()
        .map(|(i, d)| (d.name, i))
        .collect()
}

fn successors<'a>(n: &Node, index: &Index<'a>, devices: &[Device<'_>]) -> Vec<Node> {
    devices[*n].outputs.iter().map(|name| index[name]).collect()
}

fn paths(from: Node, to: Node, index: &Index, devices: &[Device<'_>]) -> usize {
    count_paths(
        from,
        |n: &Node| successors(n, index, devices),
        |p: &Node| *p == to,
    )
}

fn solve<'a>(parsed: &'a [Device<'a>]) -> usize {
    let mut devices = parsed.to_vec();
    devices.push(Device {
        name: "out",
        outputs: vec![],
    });

    let index = build_index(&devices);

    let svr = index["svr"];
    let dac = index["dac"];
    let fft = index["fft"];
    let out = index["out"];

    let svr_dac = paths(svr, dac, &index, &devices);
    let svr_fft = paths(svr, fft, &index, &devices);
    let fft_dac = paths(fft, dac, &index, &devices);
    let dac_fft = paths(dac, fft, &index, &devices);
    let dac_out = paths(dac, out, &index, &devices);
    let fft_out = paths(fft, out, &index, &devices);

    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
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
        let input = "aaa: you hhh
you: bbb ccc";

        let expected: Vec<Device> = vec![
            Device {
                name: "aaa",
                outputs: vec!["you", "hhh"],
            },
            Device {
                name: "you",
                outputs: vec!["bbb", "ccc"],
            },
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_day_11_part_2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let (_remaining, devices) = parse(input).expect("should parse");

        let result = solve(&devices);
        let expected = 2;
        assert_eq!(result, expected);
    }
}
