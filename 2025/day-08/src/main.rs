use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::delimited,
};

use std::fs;

const FILE_PATH: &str = "./input.txt";
const DAY_AND_PART: &str = "Day 8 Part 1";

#[derive(Debug, Clone, PartialEq)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse::<i64>).parse(input)
}

fn junction(input: &str) -> IResult<&str, Junction> {
    map(
        (number, tag(","), number, tag(","), number),
        |(x, _, y, _, z)| Junction { x, y, z },
    )
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Junction>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(line_ending, junction),
        multispace0,
    ))
    .parse(input)
}

fn dist(a: &Junction, b: &Junction) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }

    fn component_sizes(mut self) -> Vec<i64> {
        for i in 0..self.parent.len() {
            let _ = self.find(i);
        }
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| (i == p).then_some(self.size[i]))
            .collect()
    }
}

fn solve(junctions: &[Junction], n: usize) -> i64 {
    let mut edges: Vec<(usize, usize, i64, &Junction, &Junction)> = vec![];

    for i in 0..(junctions.len() - 1) {
        for j in (i + 1)..junctions.len() {
            let d = dist(&junctions[i], &junctions[j]);
            edges.push((i, j, d, &junctions[i], &junctions[j]));
        }
    }

    edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut circuts = Dsu::new(junctions.len());

    for &(i, j, _, _, _) in edges.iter().take(n) {
        circuts.union(i, j);
    }

    let mut sizes = circuts.component_sizes();

    sizes.sort_unstable();

    println!("{:?}", sizes);

    sizes.iter().rev().take(3).product()
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("should load input data");
    let (_remaining, parsed) = parse(&input).expect("should parse");
    let result = solve(&parsed, 1000);
    println!("[{}] Result: {}", DAY_AND_PART, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "162,817,812
57,618,57";

        let expected: Vec<Junction> = vec![
            Junction {
                x: 162,
                y: 817,
                z: 812,
            },
            Junction {
                x: 57,
                y: 618,
                z: 57,
            },
        ];

        let (_remaining, parsed) = parse(input).expect("parser should succeed");

        assert_eq!(parsed, expected);
    }

    // 162,817,812 and 425,690,689. 0 19
    // 162,817,812 and 431,825,988  0 7
    #[test]
    fn test_day_8_part_1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let (_remaining, items) = parse(input).expect("should parse");

        let result = solve(&items, 10);
        let expected = 40;
        assert_eq!(result, expected);
    }
}
