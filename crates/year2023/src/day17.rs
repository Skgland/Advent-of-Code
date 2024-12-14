use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::{BTreeMap, HashSet};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day17.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "17", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "17", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
struct Input {
    heat_loss: Vec<Vec<u8>>,
}

impl Input {
    fn find_path(&self, ultra_crucible: bool) -> usize {
        let mut queue = BTreeMap::new();
        queue.insert(
            0,
            vec![
                Position {
                    row: 0,
                    column: 0,
                    last_turn: 0,
                    direction: Direction::Down,
                },
                Position {
                    row: 0,
                    column: 0,
                    last_turn: 0,
                    direction: Direction::Right,
                },
            ],
        );

        let mut visited = HashSet::new();

        loop {
            let (cost, todo) = queue.pop_first().unwrap();

            if todo.iter().any(|elem| self.is_goal(elem, ultra_crucible)) {
                return cost;
            }

            for entry in todo {
                if visited.contains(&entry) {
                    continue;
                }
                let paths = entry.next(ultra_crucible);
                visited.insert(entry);
                for next in paths {
                    if let Some(&heat_loss) = self
                        .heat_loss
                        .get(next.row)
                        .and_then(|entry| entry.get(next.column))
                    {
                        queue
                            .entry(cost + heat_loss as usize)
                            .or_default()
                            .push(next);
                    }
                }
            }
        }
    }

    fn is_goal(&self, elem: &Position, ultra_crucible: bool) -> bool {
        elem.row == self.heat_loss.len() - 1
            && elem.column == self.heat_loss.last().unwrap().len() - 1
            && (!ultra_crucible || elem.last_turn >= 4)
    }
}

fn parse_input(input: &str) -> Input {
    Input {
        heat_loss: input
            .lines()
            .map(|line| line.bytes().map(|num| num - b'0').collect())
            .collect(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize,
    direction: Direction,
    last_turn: u8,
}

impl Position {
    fn next(&self, ultra_crucible: bool) -> Vec<Position> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .map(|dir| self.go(dir, ultra_crucible))
        .into_iter()
        .flatten()
        .collect()
    }

    fn go(&self, dir: Direction, ultra_crucible: bool) -> Option<Self> {
        if self.direction == dir.opposite()
            || (!self.can_turn(ultra_crucible) && self.direction != dir)
            || (self.must_turn(ultra_crucible) && self.direction == dir)
        {
            None
        } else {
            let (row, column) = match dir {
                Direction::Up => (self.row.checked_sub(1)?, self.column),
                Direction::Down => (self.row.checked_add(1)?, self.column),
                Direction::Left => (self.row, self.column.checked_sub(1)?),
                Direction::Right => (self.row, self.column.checked_add(1)?),
            };
            Some(Self {
                row,
                column,
                direction: dir,
                last_turn: if self.direction == dir {
                    self.last_turn + 1
                } else {
                    1
                },
            })
        }
    }

    fn can_turn(&self, ultra_crucible: bool) -> bool {
        !ultra_crucible || self.last_turn >= 4
    }

    fn must_turn(&self, ultra_crucible: bool) -> bool {
        self.last_turn >= if ultra_crucible { 10 } else { 3 }
    }
}

pub fn part1(input: &str) -> usize {
    parse_input(input).find_path(false)
}

pub fn part2(input: &str) -> usize {
    parse_input(input).find_path(true)
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day17.example1.txt"
    ));
    assert_eq!(part1(input), 102);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day17.txt"
    ));
    assert_eq!(part1(input), 861);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day17.example1.txt"
    ));
    assert_eq!(part2(input), 94);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day17.example2.txt"
    ));
    assert_eq!(part2(input), 71);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day17.txt"
    ));
    assert_eq!(part2(input), 1037);
}
