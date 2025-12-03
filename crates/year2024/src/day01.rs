use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day01.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day01.example.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "1", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "1", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let l = l.trim().parse::<u64>().unwrap();
            let r = r.trim().parse::<u64>().unwrap();
            (l, r)
        })
        .unzip()
}

pub fn part1(input: &str) -> u64 {
    let (mut l, mut r) = parse_input(input);

    l.sort();
    r.sort();

    l.into_iter().zip(r).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn part2(input: &str) -> u64 {
    let (l, r) = parse_input(input);
    let l = l.into_iter().fold(HashMap::new(), |mut acc, val| {
        *acc.entry(val).or_default() += 1;
        acc
    });
    r.into_iter().flat_map(|r| l.get(&r).map(|l| r * l)).sum()
}

#[test]
fn part1_example() {
    assert_eq!(part1(INPUT_EXAMPLE1), 11);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 936063);
}

#[test]
fn part2_example() {
    assert_eq!(part2(INPUT_EXAMPLE1), 31);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 23150395);
}
