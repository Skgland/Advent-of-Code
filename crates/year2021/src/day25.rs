use crate::day25::Spot::{Down, Empty, Right};
use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day25.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "25", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

enum Spot {
    Down,
    Right,
    Empty,
}

fn parse_input(input: &str) -> Vec<Vec<Spot>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'v' => Down,
                    '>' => Right,
                    '.' => Empty,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut game_board = parse_input(input);

    let mut empty = vec![];

    for (row_idx, row) in game_board.iter().enumerate() {
        for (column_idx, entry) in row.iter().enumerate() {
            match entry {
                Empty => {
                    empty.push((row_idx, column_idx));
                }
                Right | Down => {}
            }
        }
    }

    let mut empty_down = vec![];
    let mut empty_right = empty;
    let mut not_moved;
    let mut iterations = 0;

    loop {
        not_moved = true;

        empty_right
            .drain(..)
            .flat_map(|(row, column)| {
                let origin = (
                    row,
                    (column + game_board[row].len() - 1) % game_board[row].len(),
                );
                if matches! { game_board[origin.0][origin.1], Right } {
                    empty_down.push(origin);
                    not_moved = false;
                    Some((origin, (row, column)))
                } else {
                    empty_down.push((row, column));
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(origin, (row, column))| {
                game_board[origin.0][origin.1] = Empty;
                game_board[row][column] = Right;
            });

        empty_down
            .drain(..)
            .flat_map(|(row, column)| {
                let origin = ((row + game_board.len() - 1) % game_board.len(), column);
                if matches! { game_board[origin.0][origin.1], Down } {
                    empty_right.push(origin);
                    not_moved = false;
                    Some((origin, (row, column)))
                } else {
                    empty_right.push((row, column));
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(origin, (row, column))| {
                game_board[origin.0][origin.1] = Empty;
                game_board[row][column] = Down;
            });

        iterations += 1;

        if not_moved {
            break;
        }
    }

    iterations
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day25.example.txt"
    ));
    assert_eq!(part1(input), 58);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 509);
}
