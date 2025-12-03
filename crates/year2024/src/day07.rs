use std::collections::HashSet;

use helper::IntegerExtension;
use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day07.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "7", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "7", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct Test {
    result: u64,
    arguments: Vec<u64>,
}

impl Test {
    fn is_possible<const N: usize>(&self, ops: impl Fn(u64, u64) -> [u64; N]) -> bool {
        self.arguments[1..]
            .iter()
            .copied()
            .fold(HashSet::from([self.arguments[0]]), |accs, arg| {
                accs.iter()
                    .copied()
                    .flat_map(|acc| ops(acc, arg))
                    .filter(|&val| val <= self.result)
                    .collect()
            })
            .contains(&self.result)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Test> + '_ {
    input.lines().map(|line| {
        let (result, args) = line.split_once(": ").unwrap();

        Test {
            result: result.parse().unwrap(),
            arguments: args.split(' ').map(|arg| arg.parse().unwrap()).collect(),
        }
    })
}

pub fn part1(input: &str) -> u64 {
    both(input, part1_ops)
}

fn part1_ops(acc: u64, arg: u64) -> [u64; 2] {
    [acc + arg, acc * arg]
}

pub fn part2(input: &str) -> u64 {
    both(input, part2_ops)
}

fn part2_ops(acc: u64, arg: u64) -> [u64; 3] {
    [acc + arg, acc * arg, acc * arg.next_power_of_ten() + arg]
}

fn both<const N: usize>(input: &str, ops: impl Fn(u64, u64) -> [u64; N] + Copy) -> u64 {
    parse_input(input)
        .filter(|test| test.is_possible(ops))
        .map(|test| test.result)
        .sum()
}

#[test]
fn edge_case() {
    let test = Test {
        result: 5,
        arguments: vec![2, 5],
    };
    assert!(!test.is_possible(part1_ops));
    assert!(!test.is_possible(part2_ops));
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day07.example.txt"
    ));
    assert_eq!(part1(input), 3749);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 28730327770375);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day07.example.txt"
    ));
    assert_eq!(part2(input), 11387);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 424977609625985);
}
