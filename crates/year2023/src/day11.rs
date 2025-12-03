use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day11.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "11", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "11", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct Galaxy {
    row: usize,
    column: usize,
}

fn parse_input(input: &str) -> Vec<Galaxy> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(column_idx, char)| {
                    matches!(char, '#').then_some(Galaxy {
                        row: row_idx,
                        column: column_idx,
                    })
                })
        })
        .collect()
}

fn expand(galaxies: &mut Vec<Galaxy>, multiplier: usize) {
    let max_row = galaxies.iter().map(|galaxy| galaxy.row).max().unwrap();
    let empty_rows = (0..max_row)
        .filter(|&row| galaxies.iter().all(|galaxy| row != galaxy.row))
        .collect::<Vec<_>>();
    let max_column = galaxies.iter().map(|galaxy| galaxy.column).max().unwrap();
    let empty_column = (0..max_column)
        .filter(|&column| galaxies.iter().all(|galaxy| column != galaxy.column))
        .collect::<Vec<_>>();
    for galaxy in galaxies {
        galaxy.column += empty_column
            .iter()
            .filter(|&&column| column < galaxy.column)
            .count()
            * (multiplier - 1);
        galaxy.row += empty_rows.iter().filter(|&&row| row < galaxy.row).count() * (multiplier - 1);
    }
}

fn distances(galaxies: &[Galaxy]) -> usize {
    let mut remaining_first = galaxies;

    let mut sum = 0;

    while let Some((first, rest)) = remaining_first.split_first() {
        remaining_first = rest;
        let mut remaining_second = rest;
        while let Some((second, rest)) = remaining_second.split_first() {
            remaining_second = rest;
            sum += manhattan_distance(first, second);
        }
    }

    sum
}

fn manhattan_distance(first: &Galaxy, second: &Galaxy) -> usize {
    first.row.abs_diff(second.row) + first.column.abs_diff(second.column)
}

pub fn part1(input: &str) -> usize {
    let mut galaxies = parse_input(input);

    expand(&mut galaxies, 2);
    distances(&galaxies)
}

pub fn part2(input: &str) -> usize {
    let mut galaxies = parse_input(input);

    expand(&mut galaxies, 1_000_000);
    distances(&galaxies)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day11.example.txt"
    ));
    assert_eq!(part1(input), 374);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 9177603);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day11.example.txt"
    ));

    let mut galaxies = parse_input(input);
    expand(&mut galaxies, 10);

    assert_eq!(distances(&galaxies), 1030);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day11.example.txt"
    ));

    let mut galaxies = parse_input(input);
    expand(&mut galaxies, 100);

    assert_eq!(distances(&galaxies), 8410);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 632003913611);
}
