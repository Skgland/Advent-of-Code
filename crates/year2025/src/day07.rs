use std::collections::{BTreeMap, BTreeSet};

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day07.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day07.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "07", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "07", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum Symbols {
    Start,
    Splitter,
    Empty,
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<Symbols>> + '_ {
    input.lines().map(|line| {
        line.chars()
            .map(|c| match c {
                'S' => Symbols::Start,
                '^' => Symbols::Splitter,
                '.' => Symbols::Empty,
                sym => unreachable!("Invalid symbol {sym}"),
            })
            .collect()
    })
}

pub fn part1(input: &str) -> u32 {
    let mut iter = parse_input(input);
    let pos = iter
        .next()
        .unwrap()
        .iter()
        .position(|e| matches!(e, Symbols::Start))
        .unwrap();

    let mut beams = BTreeSet::from([pos]);

    let mut splits = 0;

    for row in iter {
        for beam in std::mem::take(&mut beams) {
            match &row[beam] {
                Symbols::Start => unreachable!("Start should only occour in the first row"),
                Symbols::Splitter => {
                    beams.extend([beam - 1, beam + 1]);
                    splits += 1;
                }
                Symbols::Empty => {
                    beams.insert(beam);
                }
            }
        }
    }

    splits
}

pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 21);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1615);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
