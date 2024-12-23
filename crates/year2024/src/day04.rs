use helper::iter::search_grid;
use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day04.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "4", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "4", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Xmas {
    X,
    M,
    A,
    S,
}

fn parse_input(input: &str) -> Vec<Vec<Xmas>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => Xmas::X,
                    'M' => Xmas::M,
                    'A' => Xmas::A,
                    'S' => Xmas::S,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    use Xmas::*;
    search_grid(&parse_input(input), &|c| {
        matches!(c, [X, M, A, S] | [S, A, M, X])
    })
}

pub fn part2(input: &str) -> usize {
    use Xmas::*;
    let grid = parse_input(input);

    let mut count = 0;

    // iterator over all coordinates that could be the center
    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            // check that the center is A and that the diag ends are the either SM or MS
            if let (A, [S, M] | [M, S], [S, M] | [M, S]) = (
                grid[x][y],                               // center
                [grid[x - 1][y - 1], grid[x + 1][y + 1]], // tl br diag
                [grid[x - 1][y + 1], grid[x + 1][y - 1]], // bl tr diag
            ) {
                count += 1
            }
        }
    }
    count
}

#[allow(unstable_name_collisions)]
#[test]
fn part1_example_sanity() {
    use helper::iter::{diag_bl_tr_iter, vertical_iter, IteratorExtension as _};
    use Xmas::*;
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day04.example.txt"
    ));
    let haystack = parse_input(input);
    assert!(vertical_iter(&haystack).eq_by(
        [
            [M, M, A, M, X, X, S, S, M, M].iter(),
            [M, S, M, S, M, X, M, A, A, X].iter(),
            [M, A, X, A, A, A, S, X, M, M].iter(),
            [S, M, S, M, S, M, M, A, M, X].iter(),
            [X, X, X, A, A, M, S, M, M, A].iter(),
            [X, M, M, S, M, X, A, A, X, X].iter(),
            [M, S, A, M, X, X, S, S, M, M].iter(),
            [A, M, A, S, A, A, X, A, M, A].iter(),
            [S, S, M, M, M, M, S, A, M, S].iter(),
            [M, A, M, X, M, A, S, A, M, X].iter(),
        ]
        .into_iter(),
        |l, r| l.eq(r),
    ));

    assert!(diag_bl_tr_iter(&haystack).eq_by(
        [
            [M].iter(),
            [M, M].iter(),
            [A, S, M].iter(),
            [M, M, A, S].iter(),
            [X, S, X, M, X].iter(),
            [X, M, A, S, X, X].iter(),
            [S, X, A, M, X, M, M].iter(),
            [S, M, A, S, A, M, S, A].iter(),
            [M, A, S, M, A, S, A, M, S].iter(),
            [M, A, X, M, M, M, M, A, S, M].iter(),
            [X, M, A, S, X, X, S, M, A].iter(),
            [M, M, M, A, X, A, M, M].iter(),
            [X, M, A, S, A, M, X].iter(),
            [A, X, S, X, M, M].iter(),
            [X, M, A, S, A].iter(),
            [M, M, A, S].iter(),
            [A, M, A].iter(),
            [S, M].iter(),
            [X].iter(),
        ]
        .into_iter(),
        |l, r| l.eq(r),
    ));
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day04.example.txt"
    ));
    assert_eq!(part1(input), 18);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 2534);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day04.example.txt"
    ));
    assert_eq!(part2(input), 9);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1866);
}
