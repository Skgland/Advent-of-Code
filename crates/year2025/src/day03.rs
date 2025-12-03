use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day03.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day03.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "03", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "03", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = Vec<u8>> + '_ {
    input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>()
    })
}

pub fn part1(input: &str) -> u128 {
    let mut sum = 0;

    for bank in parse_input(input) {
        sum += max_joltage(&bank, 2)
    }

    sum
}

fn max_joltage(bank: &[u8], digits: usize) -> u128 {
    let mut joltage = 0;

    let mut remainder = bank;

    for i in 1..=digits {
        let skip_from_end = digits - i;
        let prefix = &remainder[..remainder.len() - skip_from_end];
        let (idx, max_digit) = prefix
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_idx, a)| **a)
            .unwrap();
        remainder = &remainder[idx + 1..];
        joltage = joltage * 10 + (*max_digit as u128)
    }

    joltage
}

pub fn part2(input: &str) -> u128 {
    let mut sum = 0;

    for bank in parse_input(input) {
        sum += max_joltage(&bank, 12)
    }

    sum
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 98 + 89 + 78 + 92);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 17311);
}

#[test]
fn part2_example1() {
    assert_eq!(
        part2(INPUT_EXAMPLE1),
        987654321111 + 811111111119 + 434234234278 + 888911112111
    );
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 171419245422055);
}
