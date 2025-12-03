use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day04.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "4", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "4", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct ScratchCard {
    wining: HashSet<u8>,
    having: HashSet<u8>,
}

impl ScratchCard {
    fn score(&self) -> u32 {
        let common = self.matches();
        if common > 0 { 1 << (common - 1) } else { 0 }
    }

    fn matches(&self) -> usize {
        self.wining.intersection(&self.having).count()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = ScratchCard> + '_ {
    input.lines().map(|line| {
        let (_, line) = line.split_once(':').unwrap();
        let (winning, having) = line.split_once('|').unwrap();
        let wining = winning
            .split(' ')
            .filter(|elem| !elem.trim().is_empty())
            .map(|elem| elem.parse().unwrap())
            .collect();
        let having = having
            .split(' ')
            .filter(|elem| !elem.trim().is_empty())
            .map(|elem| elem.parse().unwrap())
            .collect();
        ScratchCard { wining, having }
    })
}

pub fn part1(input: &str) -> u32 {
    parse_input(input).map(|card| card.score()).sum()
}

pub fn part2(input: &str) -> u32 {
    parse_input(input)
        .map(|card| card.matches())
        .scan(VecDeque::new(), |multipliers, matches| {
            let multiplier = multipliers.pop_front().unwrap_or(1);
            if multipliers.len() < matches {
                multipliers.resize(matches, 1);
            }
            for entry in &mut multipliers.make_contiguous()[0..matches] {
                *entry += multiplier;
            }
            Some(multiplier)
        })
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day04.example.txt"
    ));
    assert_eq!(part1(input), 13);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 23941);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day04.example.txt"
    ));
    assert_eq!(part2(input), 30);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 5571760);
}
