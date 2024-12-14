use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day04.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "4", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "4", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let (start, end) = input.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input.lines().map(|line| {
        let (l, r) = line.split_once(',').unwrap();
        (parse_range(l), parse_range(r))
    })
}

fn subset(l: &RangeInclusive<u32>, r: &RangeInclusive<u32>) -> bool {
    l.start() <= r.start() && r.end() <= l.end()
}

fn intersect(l: &RangeInclusive<u32>, r: &RangeInclusive<u32>) -> bool {
    l.start() <= r.end() && r.start() <= l.end()
}

fn both(
    input: &str,
    filter: impl Fn(&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool,
) -> usize {
    parse(input).filter(filter).count()
}

pub fn part1(input: &str) -> usize {
    both(input, |(l, r)| subset(l, r) || subset(r, l))
}

pub fn part2(input: &str) -> usize {
    both(input, |(l, r)| intersect(l, r))
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day04.example.txt"
    ));
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 542);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day04.example.txt"
    ));
    assert_eq!(part2(input), 4);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 900);
}
