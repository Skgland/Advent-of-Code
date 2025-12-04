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

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut map = parse_input(input);
    let mut count = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell
                && [
                    (-1, -1),
                    (1, 1),
                    (0, -1),
                    (-1, 0),
                    (0, 1),
                    (1, 0),
                    (-1, 1),
                    (1, -1),
                ]
                .iter()
                .filter(|(r, c)| {
                    let Some(ri) = row_idx.checked_add_signed(*r) else {
                        return false;
                    };
                    let Some(ci) = col_idx.checked_add_signed(*c) else {
                        return false;
                    };

                    map.get(ri)
                        .and_then(|row| row.get(ci).copied())
                        .unwrap_or(false)
                })
                .count()
                    < 4
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
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
    assert_eq!(part2(INPUT_EXAMPLE1), 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
