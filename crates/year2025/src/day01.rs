use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day01.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day01.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "01", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "01", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Dir, u32)> + '_ {
    input
        .lines()
        .map(|line| match line.chars().next().unwrap() {
            'L' => (Dir::Left, line.strip_prefix('L').unwrap().parse().unwrap()),
            'R' => (Dir::Right, line.strip_prefix('R').unwrap().parse().unwrap()),
            _ => unreachable!(),
        })
}

pub fn part1(input: &str) -> u32 {
    let mut pos = 50i32;
    let mut count = 0;
    let mut iter = parse_input(input);

    for (dir, distance) in iter {
        match dir {
            Dir::Left => {
                pos = pos.strict_sub_unsigned(distance);
            }
            Dir::Right => {
                pos = pos.strict_add_unsigned(distance);
            }
        }

        pos = pos.rem_euclid(100);

        if pos == 0 {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let mut pos = 50i32;
    let mut count = 0;
    let mut iter = parse_input(input);

    for (dir, mut distance) in iter {
        match dir {
            Dir::Left => {
                if pos != 0 {
                    pos -= 100;
                }
                pos = pos.strict_sub_unsigned(distance);
                count += (pos / -100).unsigned_abs();
            }
            Dir::Right => {
                pos = pos.strict_add_unsigned(distance);
                count += (pos / 100).unsigned_abs();
            }
        }

        pos = pos.rem_euclid(100);
    }

    count
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 3);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 962);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 6);
}

#[test]
fn part2_example2() {
    assert_eq!(part2("L50"), 1);
    assert_eq!(part2("L1000"), 10);
    assert_eq!(part2("L1050"), 11);
    assert_eq!(part2("R50"), 1);
    assert_eq!(part2("R1000"), 10);
    assert_eq!(part2("R1050"), 11);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 5782);
}
