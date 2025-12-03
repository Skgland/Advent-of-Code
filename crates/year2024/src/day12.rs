use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};

type Pos = (isize, isize);
use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day12.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "12", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "12", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> BTreeMap<Pos, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, c)| ((column as isize, row as isize), c))
        })
        .collect()
}

pub fn group_plots(mut tiles: BTreeMap<Pos, char>) -> Vec<BTreeSet<Pos>> {
    let mut plots = Vec::new();
    while let Some((pos, kind)) = tiles.pop_first() {
        let mut plot = BTreeSet::from([pos]);
        let mut todo = vec![pos];

        while let Some(pos) = todo.pop() {
            for neighbor_pos in neighbors(pos) {
                let Entry::Occupied(neighbor) = tiles.entry(neighbor_pos) else {
                    continue;
                };
                if *neighbor.get() == kind {
                    neighbor.remove();
                    plot.insert(neighbor_pos);
                    todo.push(neighbor_pos);
                }
            }
        }

        plots.push(plot);
    }
    plots
}

fn neighbors(pos: (isize, isize)) -> [(isize, isize); 4] {
    [
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}

fn calculate_perimiter(plot: &BTreeSet<Pos>) -> usize {
    plot.iter()
        .map(|&pos| {
            neighbors(pos)
                .into_iter()
                .filter(|pos| !plot.contains(pos))
                .count()
        })
        .sum()
}

fn calculate_sides(plot: &BTreeSet<Pos>) -> usize {
    count_sides(
        plot,
        |pos| (pos.0, pos.1 - 1),
        |pos| [(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
    ) + count_sides(
        plot,
        |pos| (pos.0, pos.1 + 1),
        |pos| [(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
    ) + count_sides(
        plot,
        |pos| (pos.0 - 1, pos.1),
        |pos| [(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
    ) + count_sides(
        plot,
        |pos| (pos.0 + 1, pos.1),
        |pos| [(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
    )
}

fn count_sides(
    plot: &BTreeSet<(isize, isize)>,
    must_be_empty: impl Fn((isize, isize)) -> (isize, isize),
    adjacent_tiles: impl Fn((isize, isize)) -> [(isize, isize); 2],
) -> usize {
    let mut count = 0;
    let mut tiles = plot.clone();
    while let Some(pos) = tiles.pop_first() {
        if !plot.contains(&must_be_empty(pos)) {
            count += 1;
            let mut todo = vec![pos];
            while let Some(tile) = todo.pop() {
                for tile in adjacent_tiles(tile) {
                    if tiles.remove(&tile) && !plot.contains(&must_be_empty(tile)) {
                        todo.push(tile);
                    }
                }
            }
        }
    }
    count
}

pub fn part1(input: &str) -> usize {
    both(input, calculate_perimiter)
}

pub fn part2(input: &str) -> usize {
    both(input, calculate_sides)
}

fn both(input: &str, length: fn(&BTreeSet<Pos>) -> usize) -> usize {
    group_plots(parse_input(input))
        .into_iter()
        .map(|plot| plot.len() * length(&plot))
        .sum()
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example1.txt"
    ));
    assert_eq!(part1(input), 4 * 10 + 4 * 8 + 4 * 10 + 1 * 4 + 3 * 8);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example2.txt"
    ));
    assert_eq!(part1(input), 21 * 36 + 1 * 4 * 4);
}

#[test]
fn part1_example3() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example3.txt"
    ));
    assert_eq!(
        part1(input),
        12 * 18
            + 4 * 8
            + 14 * 28
            + 10 * 18
            + 13 * 20
            + 11 * 20
            + 1 * 4
            + 13 * 18
            + 14 * 22
            + 5 * 12
            + 3 * 8
    );
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1361494);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example1.txt"
    ));
    assert_eq!(part2(input), 16 + 16 + 32 + 4 + 12);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example2.txt"
    ));
    assert_eq!(part2(input), 436);
}

#[test]
fn part2_example3() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example3.txt"
    ));
    assert_eq!(part2(input), 1206);
}

#[test]
fn part2_example4() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example4.txt"
    ));
    assert_eq!(part2(input), 236);
}

#[test]
fn part2_example5() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day12.example5.txt"
    ));
    assert_eq!(part2(input), 368);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 830516);
}
