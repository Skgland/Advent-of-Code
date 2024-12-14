use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::str::FromStr;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day02.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "2", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "2", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => return Err(()),
        })
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, i32)> + '_ {
    input.lines().map(|line| {
        let (dir, dist) = line.split_once(' ').unwrap();
        (dir.parse::<Direction>().unwrap(), dist.parse().unwrap())
    })
}

pub fn part1(input: &str) -> i32 {
    let iter = parse_input(input);

    let (depth, distance) = iter.fold((0, 0), |(depth, distance), (dir, dist)| match dir {
        Direction::Forward => (depth, distance + dist),
        Direction::Down => (depth + dist, distance),
        Direction::Up => (depth - dist, distance),
    });

    depth * distance
}

pub fn part2(input: &str) -> i32 {
    let iter = parse_input(input);

    let (_, depth, distance) =
        iter.fold((0, 0, 0), |(aim, depth, distance), (dir, dist)| match dir {
            Direction::Forward => (aim, depth + aim * dist, distance + dist),
            Direction::Down => (aim + dist, depth, distance),
            Direction::Up => (aim - dist, depth, distance),
        });

    depth * distance
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day02.example.txt"
    ));
    assert_eq!(part1(input), 10 * 15);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1882980);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day02.example.txt"
    ));
    assert_eq!(part2(input), 15 * 60);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1971232560);
}
