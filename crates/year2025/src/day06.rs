use std::collections::VecDeque;

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day06.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day06.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "06", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "06", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum Op {
    Add,
    Mul,
}

struct Calc {
    args: Vec<u64>,
    op: Op,
}
impl Calc {
    fn eval(&self) -> u64 {
        match self.op {
            Op::Add => self.args.iter().sum(),
            Op::Mul => self.args.iter().product(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Calc> {
    let mut lines = input
        .lines()
        .map(|line| line.split(' ').filter(|val|!val.is_empty()).collect())
        .collect::<Vec<VecDeque<_>>>();

    let (ops, argss) = lines.split_last_mut().unwrap();

    ops.iter()
        .map(|op| {
            let op = match *op {
                "+" => Op::Add,
                "*" => Op::Mul,
                op => unreachable!("Invalid operator {op}"),
            };

            let args = argss
                .iter_mut()
                .map(|arg| arg.pop_front().unwrap().parse().unwrap())
                .collect();

            Calc { args, op }
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    parse_input(input).iter().map(|calc| calc.eval()).sum()
}

pub fn part2(input: &str) -> u64 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 33210 + 490 + 4243455 + 401);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 5977759036837);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
