use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    vec,
};

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day21.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day21.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "21", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "21", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Digit {
    fn prefix(code: &[Digit]) -> u128 {
        let mut current = 0;
        let mut rem = code;
        while let [next, remainder @ ..] = rem {
            rem = remainder;
            match next {
                Digit::Zero => current *= 10,
                Digit::One => current = current * 10 + 1,
                Digit::Two => current = current * 10 + 2,
                Digit::Three => current = current * 10 + 3,
                Digit::Four => current = current * 10 + 4,
                Digit::Five => current = current * 10 + 5,
                Digit::Six => current = current * 10 + 6,
                Digit::Seven => current = current * 10 + 7,
                Digit::Eight => current = current * 10 + 8,
                Digit::Nine => current = current * 10 + 9,
                Digit::A => break,
            }
        }
        current
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    A,
}

trait Robot {
    fn neighbors(&self, end: &Self) -> Vec<(Self, Move)>
    where
        Self: Sized;
}

impl Robot for Move {
    fn neighbors(&self, end: &Self) -> Vec<(Self, Move)>
    where
        Self: Sized,
    {
        if self != end {
            match self {
                Move::Up => vec![(Self::Down, Move::Down), (Self::A, Move::Right)],
                Move::Down => vec![
                    (Self::Up, Move::Up),
                    (Self::Left, Move::Left),
                    (Self::Right, Move::Right),
                ],
                Move::Left => vec![(Self::Down, Move::Right)],
                Move::Right => vec![(Self::Down, Move::Left), (Self::A, Move::Up)],
                Move::A => vec![(Self::Right, Move::Down), (Self::Up, Move::Left)],
            }
        } else {
            vec![(self.clone(), Move::A)]
        }
    }
}

impl Robot for Digit {
    fn neighbors(&self, end: &Self) -> Vec<(Self, Move)>
    where
        Self: Sized,
    {
        if self != end {
            match self {
                Digit::Zero => vec![(Self::A, Move::Right), (Self::Two, Move::Up)],
                Digit::One => vec![(Self::Four, Move::Up), (Self::Two, Move::Right)],
                Digit::Two => vec![
                    (Self::One, Move::Left),
                    (Self::Zero, Move::Down),
                    (Self::Three, Move::Right),
                    (Self::Five, Move::Up),
                ],
                Digit::Three => vec![
                    (Self::Two, Move::Left),
                    (Self::A, Move::Down),
                    (Self::Six, Move::Up),
                ],
                Digit::Four => vec![
                    (Self::One, Move::Down),
                    (Self::Five, Move::Right),
                    (Self::Seven, Move::Up),
                ],
                Digit::Five => vec![
                    (Self::Four, Move::Left),
                    (Self::Two, Move::Down),
                    (Self::Six, Move::Right),
                    (Self::Eight, Move::Up),
                ],
                Digit::Six => vec![
                    (Self::Five, Move::Left),
                    (Self::Three, Move::Down),
                    (Self::Nine, Move::Up),
                ],
                Digit::Seven => vec![(Self::Four, Move::Down), (Self::Eight, Move::Right)],
                Digit::Eight => vec![
                    (Self::Seven, Move::Left),
                    (Self::Five, Move::Down),
                    (Self::Nine, Move::Right),
                ],
                Digit::Nine => vec![(Self::Eight, Move::Left), (Self::Six, Move::Down)],
                Digit::A => vec![(Self::Zero, Move::Left), (Self::Three, Move::Up)],
            }
        } else {
            vec![(self.clone(), Move::A)]
        }
    }
}

fn dijkstra<R: Robot + Eq + Clone + Ord + Debug>(
    current_robot: R,
    depth: usize,
    end: &R,
    cache: &mut BTreeMap<(usize, Move, Move), u128>,
) -> u128 {
    if depth == 0 {
        return 0;
    }

    let mut visisted = BTreeSet::new();
    let mut todo = BTreeMap::<_, _>::from([(0, vec![(current_robot, Move::A)])]);

    let target = (end.clone(), Move::A);
    loop {
        let Some((current_score, todos)) = todo.pop_first() else {
            panic!("No Path found");
        };

        for current in todos {
            if !visisted.insert(current.clone()) {
                continue;
            }

            if current == target {
                return current_score;
            }

            for next in current.0.neighbors(end) {
                if !visisted.contains(&next) {
                    let cache_key = (depth - 1, current.1.clone(), next.1.clone());
                    let cost = if let Some(&cost) = cache.get(&cache_key) {
                        cost
                    } else {
                        let mut cost = dijkstra(current.1.clone(), depth - 1, &next.1, cache);
                        if !matches!(next.1, Move::A) {
                            cost += 1;
                        }
                        cache.insert(cache_key, cost);
                        cost
                    };
                    todo.entry(current_score + cost).or_default().push(next);
                }
            }
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<Digit>> + '_ {
    input.lines().map(|line| {
        line.chars()
            .map(|c| match c {
                '0' => Digit::Zero,
                '1' => Digit::One,
                '2' => Digit::Two,
                '3' => Digit::Three,
                '4' => Digit::Four,
                '5' => Digit::Five,
                '6' => Digit::Six,
                '7' => Digit::Seven,
                '8' => Digit::Eight,
                '9' => Digit::Nine,
                'A' => Digit::A,
                _ => panic!("Unecpected Input"),
            })
            .collect()
    })
}

fn shortest_paths(code: &[Digit], move_robots: usize) -> u128 {
    let mut current = Digit::A;
    let mut cache = BTreeMap::new();
    code.iter()
        .map(move |next| {
            dijkstra(
                std::mem::replace(&mut current, next.clone()),
                move_robots + 1,
                next,
                &mut cache,
            ) + 1
        })
        .sum::<u128>()
}

pub fn part1(input: &str) -> u128 {
    both(input, 2)
}

pub fn part2(input: &str) -> u128 {
    both(input, 25)
}

fn both(input: &str, move_robots: usize) -> u128 {
    parse_input(input)
        .map(|code| {
            let prefix = Digit::prefix(&code);
            let path_length = shortest_paths(&code, move_robots);
            prefix * path_length
        })
        .sum()
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 126384);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 128962);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 159684145150108);
}

#[test]
fn so_many_robots() {
    assert_eq!(both(INPUT, 86), 221895079889046776830013740791852600572);
}
