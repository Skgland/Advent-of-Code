use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day13.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "13", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "13", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
struct Game {
    delta_a: (isize, isize),
    delta_b: (isize, isize),
    prize: (isize, isize),
}

fn parse_input(input: &str) -> impl Iterator<Item = Game> + '_ {
    let mut lines = input.lines();
    std::iter::from_fn(move || {
        let a = lines.next()?;
        let b = lines.next()?;
        let prize = lines.next()?;
        // all but the last game are followed by an empty line
        let _ = lines.next();

        let (a_x, a_y) = a
            .strip_prefix("Button A: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let (b_x, b_y) = b
            .strip_prefix("Button B: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let (p_x, p_y) = prize
            .strip_prefix("Prize: ")
            .unwrap()
            .split_once(", ")
            .unwrap();

        Some(Game {
            delta_a: (
                a_x.strip_prefix("X+").unwrap().parse().unwrap(),
                a_y.strip_prefix("Y+").unwrap().parse().unwrap(),
            ),
            delta_b: (
                b_x.strip_prefix("X+").unwrap().parse().unwrap(),
                b_y.strip_prefix("Y+").unwrap().parse().unwrap(),
            ),
            prize: (
                p_x.strip_prefix("X=").unwrap().parse().unwrap(),
                p_y.strip_prefix("Y=").unwrap().parse().unwrap(),
            ),
        })
    })
}

fn optimal_game(game: &Game) -> Option<usize> {
    // I.  a * adx + b * bdx = px   | * ady
    // II. a * ady + b * bdy = py   | * adx
    //
    // I.  a * adx * ady + b * bdx * ady = px * ady
    // II. a * ady * adx + b * bdy * adx = py * adx
    //
    // I. - II.
    // (b * bdx * ady - b * bdy * adx) = px * ady - py * adx
    // b * (bdx * ady - bdy * adx) = px * ady - py * adx     | / (bdx * ady - bdy * adx)
    //
    // b = (px * ady - py * adx) / (bdx * ady - bdy * adx)
    let b = (game.prize.0 * game.delta_a.1 - game.prize.1 * game.delta_a.0)
        / (game.delta_b.0 * game.delta_a.1 - game.delta_b.1 * game.delta_a.0);

    if b < 0 {
        return None;
    }

    let a = (game.prize.0 - b * game.delta_b.0) / game.delta_a.0;

    if a < 0 {
        return None;
    }

    if a * game.delta_a.0 + b * game.delta_b.0 == game.prize.0
        && a * game.delta_a.1 + b * game.delta_b.1 == game.prize.1
    {
        return Some((a * 3 + b) as usize);
    }

    None // min_cost
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .flat_map(|game| optimal_game(&game))
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|mut game| {
            game.prize.0 += 10_000_000_000_000;
            game.prize.1 += 10_000_000_000_000;
            game
        })
        .flat_map(|game| optimal_game(&game))
        .sum()
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day13.example1.txt"
    ));
    assert_eq!(part1(input), 480);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 26299);
}

#[test]
fn part1_compare_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day13.example1.txt"
    ));

    for game in parse_input(input) {
        assert_eq!(optimal_game(&game), optimal_game_simple(&game))
    }
}

#[test]
fn part1_compare_full() {
    for game in parse_input(INPUT) {
        assert_eq!(optimal_game(&game), optimal_game_simple(&game))
    }
}

#[cfg(test)]
fn optimal_game_simple(game: &Game) -> Option<usize> {
    let max_bs = (game.prize.0 / game.delta_b.0).min(game.prize.1 / game.delta_b.1);
    for b in (0..=max_bs).rev() {
        let rem_x = game.prize.0 - b * game.delta_b.0;
        let a = rem_x / game.delta_a.0;

        if a * game.delta_a.0 + b * game.delta_b.0 == game.prize.0
            && a * game.delta_a.1 + b * game.delta_b.1 == game.prize.1
        {
            return Some((a * 3 + b) as usize);
        }
    }
    None
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day13.example1.txt"
    ));
    assert_eq!(part2(input), 875318608908);
}

#[test]
fn part2_full() {
    let result = part2(INPUT);
    assert_eq!(result, 107824497933339);
}
