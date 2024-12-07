use std::collections::HashSet;

use helper::IntegerExtension;

struct Test {
    result: u64,
    arguments: Vec<u64>,
}

impl Test {
    fn is_possible<const N: usize>(&self, ops: impl Fn(u64, u64) -> [u64; N]) -> bool {
        self.arguments
            .iter()
            .copied()
            .fold(HashSet::from([0]), |accs, arg| {
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
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day07.example.txt"
    ));
    assert_eq!(part1(input), 3749);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day07.txt"
    ));
    assert_eq!(part1(input), 28730327770375);
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day07.txt"
    ));
    assert_eq!(part2(input), 424977609625985);
}
