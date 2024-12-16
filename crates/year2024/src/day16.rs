use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet},
};

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day16.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day16.example1.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day16.example2.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "16", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "16", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct Input {
    map: BTreeSet<Pos>,
    start: Pos,
    end: Pos,
}

type Pos = [isize; 2];

fn parse_input(input: &str) -> Input {
    let start = Cell::new([0, 0]);
    let end = Cell::new([0, 0]);

    let map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .flat_map({
            |(row, line)| {
                line.chars().enumerate().flat_map({
                    let start = &start;
                    let end = &end;
                    move |(column, char)| {
                        let pos = [column as isize, row as isize];
                        match char {
                            '.' => Some(pos),
                            'S' => {
                                start.set(pos);
                                Some(pos)
                            }
                            'E' => {
                                end.set(pos);
                                Some(pos)
                            }
                            '#' => None,
                            _ => {
                                log::warn!("Unexpected char {char} in input");
                                None
                            }
                        }
                    }
                })
            }
        })
        .collect::<BTreeSet<_>>();
    Input {
        map,
        start: start.get(),
        end: end.get(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn apply_to(&self, pos: [isize; 2]) -> [isize; 2] {
        match self {
            Direction::North => [pos[0], pos[1] - 1],
            Direction::East => [pos[0] + 1, pos[1]],
            Direction::South => [pos[0], pos[1] + 1],
            Direction::West => [pos[0] - 1, pos[1]],
        }
    }

    fn cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn ccw(&self) -> Direction {
        self.cw().cw().cw()
    }
}

fn dijkstra(map: &BTreeSet<[isize; 2]>, start: [isize; 2], end: [isize; 2]) -> usize {
    let mut visisted = BTreeSet::from([(start, Direction::East)]);
    let mut todo = BTreeMap::from([(0, vec![(start, Direction::East)])]);

    while let Some((current_score, todos)) = todo.pop_first() {
        for (pos, dir) in todos {
            if pos == end {
                return current_score;
            }

            for (neighbor_cost, neighbor) in neighbors((pos, dir)) {
                if map.contains(&neighbor.0) && visisted.insert(neighbor) {
                    todo.entry(neighbor_cost + current_score)
                        .or_default()
                        .push(neighbor);
                }
            }
        }
    }
    panic!("No path found");
}

fn neighbors((pos, dir): ([isize; 2], Direction)) -> [(usize, (Pos, Direction)); 4] {
    [
        // continue forward
        (1, (dir.apply_to(pos), dir)),
        // turn cw and move forward
        (1000 + 1, (dir.cw().apply_to(pos), dir.cw())),
        // turn ccw and move forward
        (1000 + 1, (dir.ccw().apply_to(pos), dir.ccw())),
        // turn twice and move forward
        (2000 + 1, (dir.cw().cw().apply_to(pos), dir.cw().cw())),
    ]
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    dijkstra(&input.map, input.start, input.end)
}

pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 7036);
}
#[test]
fn part1_example2() {
    assert_eq!(part1(INPUT_EXAMPLE2), 11048);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 115500);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 45);
}

#[test]
fn part2_example2() {
    assert_eq!(part2(INPUT_EXAMPLE2), 64);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
