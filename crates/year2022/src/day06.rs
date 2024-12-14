use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::BTreeSet;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day06.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "6", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "6", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

pub fn first_distinct_run(input: &str, run: usize) -> Option<usize> {
    for (offset, window) in input.as_bytes().windows(run).enumerate() {
        if window.len() == run && BTreeSet::from_iter(window.iter().copied()).len() == run {
            return Some(offset + run);
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    first_distinct_run(input, 4).expect("There should be a matching run")
}

pub fn part2(input: &str) -> usize {
    first_distinct_run(input, 14).expect("There should be a matching run")
}

#[cfg(test)]
const EXAMPLES: &[(&str, usize, usize)] = &[
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

#[test]
fn part1_example() {
    for (input, result_part1, _) in EXAMPLES {
        assert_eq!(part1(input), *result_part1);
    }
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1909);
}

#[test]
fn part2_example() {
    for (input, _, result_part2) in EXAMPLES {
        assert_eq!(part2(input), *result_part2);
    }
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 3380);
}
