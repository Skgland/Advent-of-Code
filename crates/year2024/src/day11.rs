use helper::IntegerExtension;
use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day11.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "11", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "11", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> usize {
    process_stones(&parse_input(input), 25, &mut HashMap::new())
}

pub fn part2(input: &str) -> usize {
    process_stones(&parse_input(input), 75, &mut HashMap::new())
}

fn process_stones(stones: &[u64], iterations: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
    stones
        .iter()
        .map(|&stone| count_stones(stone, iterations, cache))
        .sum()
}

fn count_stones(stone: u64, iterations: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
    if iterations == 0 {
        return 1;
    }
    if let Some(&res) = cache.get(&(stone, iterations)) {
        return res;
    }

    let count = process_stones(&update_stones(&[stone]), iterations - 1, cache);
    cache.insert((stone, iterations), count);
    count
}

fn update_stones(input: &[u64]) -> Vec<u64> {
    input
        .iter()
        .copied()
        .flat_map(|stone| {
            if stone == 0 {
                [Some(1), None]
            } else if stone.length_base10() % 2 == 0 {
                let divisor = 10u64.pow(stone.length_base10() / 2);
                [Some(stone / divisor), Some(stone % divisor)]
            } else {
                [Some(stone * 2024), None]
            }
        })
        .flatten()
        .collect()
}

#[test]
fn part1_step1() {
    assert_eq!(
        update_stones(&[0, 1, 10, 99, 999]).as_slice(),
        &[1, 2024, 1, 0, 9, 9, 2021976],
    );
}

#[test]
fn part1_example1() {
    let mut stones = vec![0, 1, 10, 99, 999];

    stones = update_stones(&stones);
    assert_eq!(&stones, &[1, 2024, 1, 0, 9, 9, 2021976],);
}

#[test]
fn part1_example2() {
    let mut stones = vec![125, 17];

    stones = update_stones(&stones);
    assert_eq!(&stones, &[253000, 1, 7],);

    stones = update_stones(&stones);
    assert_eq!(&stones, &[253, 0, 2024, 14168],);

    stones = update_stones(&stones);
    assert_eq!(&stones, &[512072, 1, 20, 24, 28676032],);

    stones = update_stones(&stones);
    assert_eq!(&stones, &[512, 72, 2024, 2, 0, 2, 4, 2867, 6032],);

    stones = update_stones(&stones);
    assert_eq!(
        &stones,
        &[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
    );

    stones = update_stones(&stones);
    assert_eq!(
        &stones,
        &[
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2
        ],
    );

    for _ in 6..25 {
        stones = update_stones(&stones);
    }
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 186424);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 219838428124832);
}
