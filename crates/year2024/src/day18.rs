use std::collections::{BTreeMap, BTreeSet};

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day18.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day18.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "18", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "18", "part2"],
    run: || {
        let [x, y] = part2(INPUT);
        println!("{x},{y}")
    },
    include_in_all: true,
};

type Pos = [i8; 2];

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect::<Vec<Pos>>()
}

fn dijkstra(start: Pos, end: Pos, valid: impl Fn(usize, Pos) -> bool) -> Option<Vec<Pos>> {
    let mut visisted = BTreeSet::new();
    let mut todo = BTreeMap::<_, _>::from([(0, vec![start])]);
    let mut predecessor = BTreeMap::new();

    loop {
        let (current_score, todos) = todo.pop_first()?;

        let mut reached_end = false;
        for current in todos {
            if !visisted.insert(current) {
                continue;
            }

            if current == end {
                reached_end = true;
            }

            for next in neighbors(current) {
                if next[0] < 0 || next[1] < 0 || next[0] > end[0] || next[1] > end[1] {
                    continue;
                }
                if valid(current_score + 1, next) && !visisted.contains(&next) {
                    predecessor.insert(next, current);
                    todo.entry(current_score + 1).or_default().push(next);
                }
            }
        }

        if reached_end {
            break;
        }
    }

    let mut cur = end;
    let mut path = vec![end];
    while let Some(prev) = predecessor.get(&cur).copied() {
        cur = prev;
        path.push(prev);
    }
    Some(path)
}

fn neighbors(current: Pos) -> [Pos; 4] {
    [
        [current[0] + 1, current[1]],
        [current[0] - 1, current[1]],
        [current[0], current[1] + 1],
        [current[0], current[1] - 1],
    ]
}

fn part1_impl(blocked: &[Pos], end: [i8; 2], limit: usize) -> Option<Vec<Pos>> {
    dijkstra([0, 0], end, |_, pos| !blocked[..limit].contains(&pos))
}

fn part2_impl(blocked: &[Pos], end: Pos) -> Pos {
    let mut limit = 0;
    while let Some(path) = part1_impl(blocked, end, limit) {
        for (index, item) in blocked.iter().enumerate() {
            if path.contains(item) {
                limit = index + 1;
                break;
            }
        }
    }
    blocked[limit - 1]
}

pub fn part1(input: &str) -> usize {
    let blocked = parse_input(input);
    part1_impl(&blocked, [70, 70], 1024).unwrap().len() - 1
}

pub fn part2(input: &str) -> Pos {
    let blocked = parse_input(input);
    part2_impl(&blocked, [70, 70])
}

#[test]
fn part1_example1() {
    let blocked = parse_input(INPUT_EXAMPLE1);
    assert_eq!(part1_impl(&blocked, [6, 6], 12).unwrap().len() - 1, 22);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 334);
}

#[test]
fn part2_example1() {
    let blocked = parse_input(INPUT_EXAMPLE1);
    assert_eq!(part2_impl(&blocked, [6, 6]), [6, 1]);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), [20, 12]);
}
