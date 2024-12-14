use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::{BTreeSet, HashMap, HashSet};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day14.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "14", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "14", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Rock {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Platform {
    width: usize,
    height: usize,
    square_rocks: HashSet<Rock>,
    round_rocks: BTreeSet<Rock>,
}

enum TiltDirection {
    North,
    West,
    South,
    East,
}

impl Platform {
    fn tilt(&mut self, direction: TiltDirection) {
        let rocks = self.round_rocks.len();
        match direction {
            TiltDirection::North => {
                // skip rocks in row 0 as they have nowhere to roll
                for row in 1..self.height {
                    for column in 0..self.width {
                        if !self.contains(row - 1, column)
                            && self.round_rocks.remove(&Rock { row, column })
                        {
                            let target_row = (1..row)
                                .rev()
                                .find(|&row| self.contains(row - 1, column))
                                .unwrap_or(0);
                            self.round_rocks.insert(Rock {
                                row: target_row,
                                column,
                            });
                        }
                    }
                }
            }
            TiltDirection::West => {
                // skip rocks in column 0 as they have nowhere to roll
                for column in 1..self.width {
                    for row in 0..self.height {
                        if !self.contains(row, column - 1)
                            && self.round_rocks.remove(&Rock { row, column })
                        {
                            let target_column = (1..column)
                                .rev()
                                .find(|&column| self.contains(row, column - 1))
                                .unwrap_or(0);
                            self.round_rocks.insert(Rock {
                                row,
                                column: target_column,
                            });
                        }
                    }
                }
            }
            TiltDirection::South => {
                // skip rocks in row height-1 as they have nowhere to roll
                for row in (0..(self.height - 1)).rev() {
                    for column in 0..self.width {
                        if !self.contains(row + 1, column)
                            && self.round_rocks.remove(&Rock { row, column })
                        {
                            let target_row = (row + 1..self.height - 1)
                                .find(|&row| self.contains(row + 1, column))
                                .unwrap_or(self.height - 1);
                            self.round_rocks.insert(Rock {
                                row: target_row,
                                column,
                            });
                        }
                    }
                }
            }
            TiltDirection::East => {
                // skip rocks in column width-1 as they have nowhere to roll
                for column in (0..(self.width - 1)).rev() {
                    for row in 0..self.height {
                        if !self.contains(row, column + 1)
                            && self.round_rocks.remove(&Rock { row, column })
                        {
                            let target_column = (column + 1..self.width - 1)
                                .find(|&column| self.contains(row, column + 1))
                                .unwrap_or(self.width - 1);
                            self.round_rocks.insert(Rock {
                                row,
                                column: target_column,
                            });
                        }
                    }
                }
            }
        }
        assert_eq!(self.round_rocks.len(), rocks);
    }

    fn cycle(&mut self) {
        self.tilt(TiltDirection::North);
        self.tilt(TiltDirection::West);
        self.tilt(TiltDirection::South);
        self.tilt(TiltDirection::East);
    }

    fn contains(&self, row: usize, column: usize) -> bool {
        self.square_rocks.contains(&Rock { row, column })
            || self.round_rocks.contains(&Rock { row, column })
    }

    fn load(&self) -> usize {
        self.round_rocks
            .iter()
            .map(|rock| self.height - rock.row)
            .sum()
    }
}

fn parse_input(input: &str) -> Platform {
    let height = input.lines().count();
    let width = input
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let square_rocks = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, symbol)| matches!(symbol, '#'))
                .map(move |(column, _)| Rock { row, column })
        })
        .collect();

    let round_rocks = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, symbol)| matches!(symbol, 'O'))
                .map(move |(column, _)| Rock { row, column })
        })
        .collect();

    Platform {
        width,
        height,
        square_rocks,
        round_rocks,
    }
}

pub fn part1(input: &str) -> usize {
    let mut platform = parse_input(input);
    platform.tilt(TiltDirection::North);
    platform.load()
}

pub fn part2(input: &str) -> usize {
    let mut platform = parse_input(input);
    let mut formation_cache = HashMap::new();
    const CYCLE_COUNT: usize = 1_000_000_000;
    for cycle in 0..CYCLE_COUNT {
        if let Some(&first_occurrence) = formation_cache.get(&platform.round_rocks) {
            // found cycle
            let end_in = (CYCLE_COUNT - first_occurrence) % (cycle - first_occurrence);
            println!("Found cycle from {first_occurrence} to {cycle}, next cycle identical to cycle {CYCLE_COUNT} in {end_in} cycles!");
            for _ in 0..end_in {
                platform.cycle();
            }
            return platform.load();
        } else {
            formation_cache.insert(platform.round_rocks.clone(), cycle);
            platform.cycle();
        }
    }
    platform.load()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day14.example.txt"
    ));
    assert_eq!(part1(input), 136);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day14.txt"
    ));
    assert_eq!(part1(input), 108935);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day14.example.txt"
    ));
    assert_eq!(part2(input), 64);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day14.txt"
    ));
    assert_eq!(part2(input), 100876);
}
