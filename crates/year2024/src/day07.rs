use std::collections::HashSet;

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
    parse_input(input)
        .filter(|test| test.is_possible(|acc, arg| [acc + arg, acc * arg]))
        .map(|test| test.result)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse_input(input)
        .filter(|test| {
            test.is_possible(|acc, arg| {
                [acc + arg, acc * arg, acc * next_multiple_of_ten(arg) + arg]
            })
        })
        .map(|test| test.result)
        .sum()
}

fn next_multiple_of_ten(arg: u64) -> u64 {
    match arg {
        0..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        _ => unreachable!("input only contains numbers up to three digits"),
    }
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
