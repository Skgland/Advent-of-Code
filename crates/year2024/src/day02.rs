use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day02.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "2", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "2", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|part| part.parse().unwrap())
            .collect()
    })
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .filter(|report| is_safe_part1(report))
        .count()
}

fn is_safe_part1(report: &[i32]) -> bool {
    let diffs = report
        .windows(2)
        .map(|windows| windows[0] - windows[1])
        .collect::<Vec<_>>();
    diffs.iter().all(|diff| (1..=3).contains(&diff.abs()))
        && (diffs
            .windows(2)
            .all(|diff| diff[0].signum() == diff[1].signum()))
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .filter(|report| is_safe_part2(report))
        .count()
}

fn is_safe_part2(report: &[i32]) -> bool {
    if is_safe_part1(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(i);
        if is_safe_part1(&report) {
            return true;
        }
    }
    false
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day02.example.txt"
    ));
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 526);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day02.example.txt"
    ));
    assert_eq!(part2(input), 4);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 566);
}
