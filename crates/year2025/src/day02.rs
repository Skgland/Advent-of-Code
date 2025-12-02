use std::ops::RangeInclusive;

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day02.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day02.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "02", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "02", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> + '_ {
    input.trim().split(',').map(|entry| {
        let (start, end) = entry.split_once('-').unwrap();
        (start.parse().unwrap())..=(end.parse().unwrap())
    })
}

pub fn part1(input: &str) -> u128 {
    let mut sum: u128 = 0;

    for range in parse_input(input) {
        for id in range {
            let s = format!("{id}");
            if s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..] {
                sum += id as u128;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u128 {
    let mut sum: u128 = 0;

    for range in parse_input(input) {
        for id in range {
            let s = format!("{id}");
            for chunk_len in 1..=s.len() / 2 {
                let mut chunks = s.as_bytes().chunks_exact(chunk_len);
                let first = chunks.next().unwrap();
                if chunks.remainder().is_empty() && chunks.all(|chunk| chunk == first) {
                    sum += id as u128;
                    break;
                }
            }
        }
    }

    sum
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 1227775554);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 24157613387);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 4174379265);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 33832678380);
}
