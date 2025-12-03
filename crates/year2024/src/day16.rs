use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet},
};

use helper::{TASKS, Task};
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

fn dijkstra(
    map: &BTreeSet<[isize; 2]>,
    start: [isize; 2],
    end: [isize; 2],
) -> (usize, BTreeSet<Pos>) {
    let mut visisted = BTreeMap::from([((start, Direction::East), BTreeSet::from([start]))]);
    let mut todo = BTreeMap::<_, BTreeMap<_, BTreeSet<_>>>::new();

    for (score, neighbor) in neighbors((start, Direction::East)) {
        if map.contains(&neighbor.0) {
            todo.entry(score)
                .or_default()
                .entry(neighbor)
                .or_default()
                .insert(start);
        }
    }

    let min_path_length = loop {
        let Some((current_score, todos)) = todo.pop_first() else {
            panic!("No path found");
        };

        let mut reached_end = false;
        for (state, paths) in todos {
            if visisted.contains_key(&state) {
                continue;
            }

            let mut new_paths = paths.clone();
            new_paths.insert(state.0);

            if state.0 == end {
                println!("Start: {start:?}, Goal: {end:?}, Paths: {new_paths:?}");
                reached_end = true;
            }

            visisted
                .entry(state)
                .or_default()
                .append(&mut new_paths.clone());

            for (neighbor_cost, neighbor) in neighbors(state) {
                if map.contains(&neighbor.0) && !visisted.contains_key(&neighbor) {
                    todo.entry(neighbor_cost + current_score)
                        .or_default()
                        .entry(neighbor)
                        .or_default()
                        .append(&mut new_paths.clone());
                }
            }
        }
        if reached_end {
            break current_score;
        }
    };

    (
        min_path_length,
        visisted
            .into_iter()
            .filter(|((pos, _), _)| *pos == end)
            .flat_map(|(_, paths)| paths)
            .collect(),
    )
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
    dijkstra(&input.map, input.start, input.end).0
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    dijkstra(&input.map, input.start, input.end).1.len()
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
    assert_eq!(part2(INPUT), 679);
}
