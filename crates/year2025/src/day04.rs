use std::collections::BTreeSet;

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day04.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day04.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "04", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "04", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> Map {
    Map {
        cells: input
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect(),
    }
}

struct Map {
    cells: Vec<Vec<bool>>,
}

const NEIGHBOR_OFFSETS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Map {
    fn get(&self, (row, col): (usize, usize)) -> Option<bool> {
        self.cells.get(row).and_then(|r| r.get(col).copied())
    }

    fn is_roll_accessible(&self, entry: (usize, usize)) -> bool {
        NEIGHBOR_OFFSETS
            .iter()
            .filter_map(|&offset| offset_position(entry, offset))
            .filter(|&(r, c)| self.get((r, c)).unwrap_or(false))
            .count()
            < 4
    }
}

fn offset_position((row, col): (usize, usize), (r, c): (isize, isize)) -> Option<(usize, usize)> {
    Some((row.checked_add_signed(r)?, col.checked_add_signed(c)?))
}

pub fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    let mut count = 0;

    for (row_idx, row) in map.cells.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }
            if map.is_roll_accessible((row_idx, col_idx)) {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let mut map = parse_input(input);

    let mut todo: BTreeSet<_> = map
        .cells
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_idx, &cell)| cell.then_some((row_idx, col_idx)))
        })
        .collect();

    let mut count = 0;

    while let Some(entry) = todo.pop_first() {
        if map.get(entry).is_some_and(|v| v) {
            if map.is_roll_accessible(entry) {
                count += 1;
                map.cells[entry.0][entry.1] = false;
                for entry in NEIGHBOR_OFFSETS
                    .iter()
                    .filter_map(|&offset| offset_position(entry, offset))
                    .filter(|&entry| map.get(entry).is_some_and(|v| v))
                {
                    todo.insert(entry);
                }
            }
        }
    }

    count
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 13);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1449);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 43);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 8746);
}
