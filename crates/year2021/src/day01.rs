use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::VecDeque;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day01.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "1", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "1", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn both(input: &str, window_size: usize) -> u32 {
    let mut iter = parse_input(input);

    let mut window = VecDeque::with_capacity(window_size);

    window.extend((&mut iter).take(window_size));

    let mut counter = 0;

    for current in iter {
        let last = window.pop_front().unwrap();
        window.push_back(current);

        if current > last {
            counter += 1;
        }
    }

    counter
}

pub fn part1(input: &str) -> u32 {
    both(input, 1)
}

pub fn part2(input: &str) -> u32 {
    both(input, 3)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day01.example.txt"
    ));
    assert_eq!(part1(input), 7);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1292);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day01.example.txt"
    ));
    assert_eq!(part2(input), 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
