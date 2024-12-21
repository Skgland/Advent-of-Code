use std::{
    collections::{BTreeMap, BTreeSet},
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
    fn prefix(code: &[Digit]) -> usize {
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

trait Neighbors {
    fn neighbors(&self) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    door_robot: Digit,
    move_robots: Vec<Move>,
}

impl State {
    fn new(move_robot_count: usize) -> Self {
        State {
            door_robot: Digit::A,
            move_robots: vec![Move::A; move_robot_count],
        }
    }

    fn ready_for(self: &State, end: &Digit) -> bool {
        self.door_robot == *end && self.move_robots.iter().all(|robot| *robot == Move::A)
    }
}

trait Robot {
    fn apply(&mut self, action: Move, chain: &mut [&mut dyn Robot]) -> Option<()>;
}

impl Robot for Move {
    fn apply(&mut self, action: Move, chain: &mut [&mut dyn Robot]) -> Option<()> {
        *self = match (self.clone(), action) {
            (current, Move::A) => {
                let (next, others) = chain.split_first_mut().unwrap();
                return next.apply(current.clone(), others);
            }
            (Move::Up, Move::Down) => Some(Self::Down),
            (Move::Up, Move::Right) => Some(Self::A),
            (Move::Down, Move::Up) => Some(Self::Up),
            (Move::Down, Move::Left) => Some(Self::Left),
            (Move::Down, Move::Right) => Some(Self::Right),
            (Move::Left, Move::Right) => Some(Self::Down),
            (Move::Right, Move::Up) => Some(Self::A),
            (Move::Right, Move::Left) => Some(Self::Down),
            (Move::A, Move::Down) => Some(Self::Right),
            (Move::A, Move::Left) => Some(Self::Up),
            _ => None,
        }?;
        Some(())
    }
}

impl Robot for Digit {
    fn apply(&mut self, action: Move, _chain: &mut [&mut dyn Robot]) -> Option<()> {
        *self = match (self.clone(), action) {
            (old, Move::A) => Some(old),
            (Digit::Zero, Move::Up) => Some(Self::Two),
            (Digit::Zero, Move::Right) => Some(Self::A),
            (Digit::One, Move::Up) => Some(Self::Four),
            (Digit::One, Move::Right) => Some(Self::Two),
            (Digit::Two, Move::Up) => Some(Self::Five),
            (Digit::Two, Move::Down) => Some(Self::Zero),
            (Digit::Two, Move::Left) => Some(Self::One),
            (Digit::Two, Move::Right) => Some(Self::Three),
            (Digit::Three, Move::Up) => Some(Self::Six),
            (Digit::Three, Move::Down) => Some(Self::A),
            (Digit::Three, Move::Left) => Some(Self::Two),
            (Digit::Four, Move::Up) => Some(Self::Seven),
            (Digit::Four, Move::Down) => Some(Self::One),
            (Digit::Four, Move::Right) => Some(Self::Five),
            (Digit::Five, Move::Up) => Some(Self::Eight),
            (Digit::Five, Move::Down) => Some(Self::Two),
            (Digit::Five, Move::Left) => Some(Self::Four),
            (Digit::Five, Move::Right) => Some(Self::Six),
            (Digit::Six, Move::Up) => Some(Self::Nine),
            (Digit::Six, Move::Down) => Some(Self::Three),
            (Digit::Six, Move::Left) => Some(Self::Five),
            (Digit::Seven, Move::Down) => Some(Self::Four),
            (Digit::Seven, Move::Right) => Some(Self::Eight),
            (Digit::Eight, Move::Down) => Some(Self::Five),
            (Digit::Eight, Move::Left) => Some(Self::Seven),
            (Digit::Eight, Move::Right) => Some(Self::Nine),
            (Digit::Nine, Move::Down) => Some(Self::Six),
            (Digit::Nine, Move::Left) => Some(Self::Eight),
            (Digit::A, Move::Up) => Some(Self::Three),
            (Digit::A, Move::Left) => Some(Self::Zero),
            _ => None,
        }?;
        Some(())
    }
}

impl State {
    fn neighbors(&self) -> Vec<(State, Move)> {
        [Move::Left, Move::Right, Move::Down, Move::Up, Move::A]
            .into_iter()
            .flat_map(|action| {
                self.clone()
                    .apply(action.clone())
                    .map(|new_state| (new_state, action))
            })
            .collect()
    }

    fn apply(&self, action: Move) -> Option<Self> {
        let mut new_state = self.clone();
        let (first, others) = new_state.move_robots.split_first_mut().unwrap();
        first.apply(
            action,
            others
                .iter_mut()
                .map(|r| r as &mut dyn Robot)
                .chain(std::iter::once::<&mut dyn Robot>(&mut new_state.door_robot))
                .collect::<Vec<&mut dyn Robot>>()
                .as_mut_slice(),
        )?;
        Some(new_state)
    }
}

fn dijkstra(start: State, end: &Digit) -> usize {
    let mut visisted = BTreeMap::new();
    let mut todo = BTreeMap::<_, _>::from([(0, vec![start])]);

    loop {
        let Some((current_score, todos)) = todo.pop_first() else {
            panic!("No Path found");
        };

        for current in todos {
            if visisted.contains_key(&current) {
                continue;
            }
            visisted.insert(current.clone(), current_score);

            if current.ready_for(end) {
                return current_score;
            }

            for (next, _) in current.neighbors() {
                if !visisted.contains_key(&next) {
                    todo.entry(current_score + 1).or_default().push(next);
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

fn shortest_paths(code: &[Digit], move_robots: usize) -> usize {
    let mut current = State::new(move_robots);
    code.iter()
        .map(|next| {
            let res = dijkstra(current.clone(), next) + 1;
            current.door_robot = next.clone();
            res
        })
        .sum::<usize>()
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|code| {
            let prefix = Digit::prefix(&code);
            let path_length = shortest_paths(&code, 2);
            prefix * path_length
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|code| {
            let prefix = Digit::prefix(&code);
            let path_length = shortest_paths(&code, 25);
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
    assert_eq!(part2(INPUT), 1262);
}
