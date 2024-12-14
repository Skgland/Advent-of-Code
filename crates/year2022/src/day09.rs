use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day09.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "9", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "9", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct State {
    positions: Vec<(isize, isize)>,
}

impl State {
    pub(crate) fn apply(&mut self, dir: Direction) {
        let head = self.positions.get_mut(0).unwrap();

        match dir {
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
        };

        for idx in 0..self.positions.len() - 1 {
            let pair = &mut self.positions[idx..][..2];
            let x_dif = pair[0].0 - pair[1].0;
            let y_dif = pair[0].1 - pair[1].1;
            if x_dif.abs() > 1 || y_dif.abs() > 1 {
                pair[1].0 += x_dif.signum();
                pair[1].1 += y_dif.signum();
            }
        }
    }

    pub(crate) fn tail_position(&self) -> (isize, isize) {
        *self.positions.last().unwrap()
    }
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, usize)> + '_ {
    input.lines().map(|elem| {
        use Direction::*;
        let (dir, dist) = elem.split_once(' ').unwrap();
        (
            match dir {
                "L" => Left,
                "R" => Right,
                "U" => Up,
                "D" => Down,
                _ => panic!(),
            },
            dist.parse().unwrap(),
        )
    })
}

fn both(input: &str, knots: usize) -> usize {
    let mut state = State {
        positions: vec![(0, 0); knots],
    };
    let mut visited = HashSet::new();
    visited.insert(state.tail_position());

    for (dir, count) in parse(input) {
        for _ in 0..count {
            state.apply(dir);
            visited.insert(state.tail_position());
        }
    }

    visited.len()
}

pub fn part1(input: &str) -> usize {
    both(input, 2)
}

pub fn part2(input: &str) -> usize {
    both(input, 10)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day09.example.txt"
    ));
    assert_eq!(part1(input), 13);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 6337);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day09.example.txt"
    ));
    assert_eq!(part2(input), 1);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 2455);
}
