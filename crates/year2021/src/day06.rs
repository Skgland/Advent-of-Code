use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::VecDeque;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day06.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "6", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "6", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .flat_map(|line| line.split(',').map(|elem| elem.parse().unwrap()))
}

pub fn both(input: &str, days: u32) -> usize {
    let mut aging_queue = VecDeque::with_capacity(9);
    aging_queue.resize(9, 0);
    let mut counter = 0;
    for elem in parse_input(input) {
        counter += 1;
        aging_queue[elem] += 1;
    }

    for _day in 1..=days {
        let today = aging_queue.pop_front().unwrap();
        counter += today;
        aging_queue[6] += today;
        aging_queue.push_back(today);
    }
    counter
}

pub fn part1(input: &str) -> usize {
    both(input, 80)
}

pub fn part2(input: &str) -> usize {
    both(input, 256)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day06.example.txt"
    ));
    assert_eq!(part1(input), 5934);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 349549);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day06.example.txt"
    ));
    assert_eq!(part2(input), 26984457539);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1589590444365);
}
