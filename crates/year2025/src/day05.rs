use std::ops::RangeInclusive;

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day05.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day05.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "05", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "05", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ingrediens: Vec<u64>,
}

fn parse_input(input: &str) -> Input {
    let (ranges, ingrediens) = input.split_once("\n\n").unwrap();

    Input {
        ranges: ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                (start.parse().unwrap())..=(end.parse().unwrap())
            })
            .collect(),
        ingrediens: ingrediens
            .lines()
            .map(|line| line.parse().unwrap())
            .collect(),
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);

    input
        .ingrediens
        .iter()
        .filter(|ingredient| input.ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 3);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 558);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 14);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
