use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day13.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "13", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "13", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
enum Axis {
    Row,
    Column,
}

struct Mirror {
    after: usize,
    axis: Axis,
}
impl Mirror {
    fn value(&self) -> usize {
        match self.axis {
            Axis::Row => (self.after + 1) * 100,
            Axis::Column => self.after + 1,
        }
    }
}

#[derive(Debug)]
struct Pattern {
    rocks: HashSet<Rock>,
    max_row: usize,
    max_column: usize,
}

impl Pattern {
    fn find_mirror(&self, defects: usize) -> Option<Mirror> {
        for row in 0..self.max_row {
            if self.row_has_mirror(row, defects) {
                return Some(Mirror {
                    after: row,
                    axis: Axis::Row,
                });
            }
        }
        for column in 0..self.max_column {
            if self.column_has_mirror(column, defects) {
                return Some(Mirror {
                    after: column,
                    axis: Axis::Column,
                });
            }
        }
        None
    }

    // how many mirrored places are out of place, mirroring after a given row
    fn of_by_row(&self, row: usize) -> usize {
        (0..=(row.min(self.max_row - row - 1)))
            .map(|row_offset| {
                let up_row = row - row_offset;
                let down_row = row + row_offset + 1;
                (0..=self.max_column)
                    .filter(|&column| {
                        let up = self.rocks.contains(&Rock {
                            row: up_row,
                            column,
                        });
                        let down = self.rocks.contains(&Rock {
                            row: down_row,
                            column,
                        });
                        up != down
                    })
                    .count()
            })
            .sum()
    }

    fn row_has_mirror(&self, row: usize, defects: usize) -> bool {
        self.of_by_row(row) == defects
    }

    // how many mirrored places are out of place, mirroring after a given column
    fn of_by_column(&self, column: usize) -> usize {
        (0..=(column.min(self.max_column - column - 1)))
            .map(|column_offset| {
                let left_column = column - column_offset;
                let right_column = column + column_offset + 1;
                (0..=self.max_row)
                    .filter(|&row| {
                        let left = self.rocks.contains(&Rock {
                            row,
                            column: left_column,
                        });
                        let right = self.rocks.contains(&Rock {
                            row,
                            column: right_column,
                        });
                        left != right
                    })
                    .count()
            })
            .sum()
    }

    fn column_has_mirror(&self, column: usize, defects: usize) -> bool {
        self.of_by_column(column) == defects
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Rock {
    row: usize,
    column: usize,
}

struct ParseState {
    max_row: usize,
    max_column: usize,
    rocks: HashSet<Rock>,
}

impl ParseState {
    fn into_pattern(self) -> Pattern {
        Pattern {
            rocks: self.rocks,
            max_row: self.max_row,
            max_column: self.max_column,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Pattern> + '_ {
    input
        .lines()
        .map(Some)
        .chain(std::iter::once(None))
        .scan(None, |state, line| match line {
            None => state
                .take()
                .map(|state: ParseState| Some(state.into_pattern())),
            Some(line) if line.trim().is_empty() => {
                Some(state.take().map(|state| state.into_pattern()))
            }
            Some(line) => {
                let row = state.as_ref().map_or(0, |state| state.max_row + 1);

                let rocks: HashSet<_> = line
                    .chars()
                    .enumerate()
                    .filter_map(|(column, c)| matches!(c, '#').then_some(Rock { row, column }))
                    .collect();

                let max_column = rocks.iter().map(|rock| rock.column).max().unwrap_or(0);

                if let Some(state) = state {
                    state.max_row = row;
                    state.max_column = state.max_column.max(max_column);
                    state.rocks.extend(rocks);
                } else {
                    *state = Some(ParseState {
                        max_row: 0,
                        max_column,
                        rocks,
                    });
                }
                Some(None)
            }
        })
        .flatten()
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .flat_map(|elem| elem.find_mirror(0))
        .map(|mirror| mirror.value())
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .flat_map(|elem| elem.find_mirror(1))
        .map(|mirror| mirror.value())
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day13.example.txt"
    ));
    assert_eq!(part1(input), 405);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 27742);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day13.example.txt"
    ));
    assert_eq!(part2(input), 400);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 32728);
}
