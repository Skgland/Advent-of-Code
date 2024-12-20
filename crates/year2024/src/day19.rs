use std::collections::BTreeMap;

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day19.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day19.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "19", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "19", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

struct Input {
    available: Vec<Vec<Color>>,
    desired: Vec<Vec<Color>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let available = lines.next().unwrap().split(", ").map(parse_towl).collect();
    let desired = lines.skip(1).map(parse_towl).collect();
    Input { available, desired }
}

fn parse_towl(towl: &str) -> Vec<Color> {
    towl.chars()
        .map(|c| match c {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => panic!("Unexpected Color {c}"),
        })
        .collect()
}

fn towl_combinations<'a>(
    input: &Input,
    pattern: &'a [Color],
    cache: &mut BTreeMap<&'a [Color], usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(count) = cache.get(&pattern) {
        return *count;
    }

    let mut count = 0;
    for available in &input.available {
        if available.len() <= pattern.len() && pattern[..available.len()] == *available {
            count += towl_combinations(input, &pattern[available.len()..], cache);
        }
    }
    cache.insert(pattern, count);
    count
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let mut cache = BTreeMap::new();
    input
        .desired
        .iter()
        .map(|towl| towl_combinations(&input, towl, &mut cache))
        .filter(|count| *count != 0)
        .count()
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let mut cache = BTreeMap::new();
    input
        .desired
        .iter()
        .map(|towl| towl_combinations(&input, towl, &mut cache))
        .sum()
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 6);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 272);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 16);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1041529704688380);
}
