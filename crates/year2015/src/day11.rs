use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::{fmt::Display, str::FromStr};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2015/day11.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2015", "11", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2015", "11", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Base26 {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Base26 {
    fn next(&self) -> Self {
        match self {
            Base26::A => Self::B,
            Base26::B => Self::C,
            Base26::C => Self::D,
            Base26::D => Self::E,
            Base26::E => Self::F,
            Base26::F => Self::G,
            Base26::G => Self::H,
            Base26::H => Self::I,
            Base26::I => Self::J,
            Base26::J => Self::K,
            Base26::K => Self::L,
            Base26::L => Self::M,
            Base26::M => Self::N,
            Base26::N => Self::O,
            Base26::O => Self::P,
            Base26::P => Self::Q,
            Base26::Q => Self::R,
            Base26::R => Self::S,
            Base26::S => Self::T,
            Base26::T => Self::U,
            Base26::U => Self::V,
            Base26::V => Self::W,
            Base26::W => Self::X,
            Base26::X => Self::Y,
            Base26::Y => Self::Z,
            Base26::Z => Self::A,
        }
    }

    fn inc(&mut self) -> bool {
        let new = self.next();
        let carry = new < *self;
        *self = new;
        carry
    }
}

impl Display for Base26 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ((*self as u8 + b'a') as char).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
struct Password([Base26; 8]);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            c.fmt(f)?;
        }
        Ok(())
    }
}

impl Password {
    fn inc(&mut self) {
        for elem in self.0.iter_mut().rev() {
            if !elem.inc() {
                break;
            }
        }
    }
}

impl FromStr for Password {
    type Err = Vec<Base26>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .bytes()
            .filter_map(|elem| match elem {
                val @ b'a'..=b'z' => {
                    let mut value = Base26::A;
                    for _ in 0..(val - b'a') {
                        value.inc();
                    }
                    Some(value)
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .try_into()?;

        Ok(Self(parts))
    }
}

fn req1(example: &[Base26]) -> bool {
    example.windows(3).any(
        |window| matches!(window, [a,b,c] if a.next() == *b && b.next() == *c && a < b && b < c),
    )
}

fn req2(example: &[Base26]) -> bool {
    !example
        .iter()
        .any(|elem| [Base26::I, Base26::O, Base26::L].contains(elem))
}

fn req3(example: &[Base26]) -> bool {
    for start in 0..example.len() {
        if let [a, b, rest @ ..] = &example[start..] {
            if a == b
                && rest
                    .windows(2)
                    .any(|window| matches!(window, [c, d] if c == d && b != c))
            {
                return true;
            }
        }
    }
    false
}

fn parse_input(input: &str) -> Password {
    let Ok(password) = Password::from_str(input) else {
        panic!("Input has the wring length!")
    };
    password
}

fn next_valid(mut password: Password) -> Password {
    std::iter::from_fn(|| {
        password.inc();
        Some(password)
    })
    .find(|password| [req1, req2, req3].into_iter().all(|f| f(&password.0)))
    .unwrap()
}

pub fn part1(input: &str) -> String {
    let password = parse_input(input);
    next_valid(password).to_string()
}

pub fn part2(input: &str) -> String {
    let password = parse_input(input);
    next_valid(next_valid(password)).to_string()
}

#[test]
fn part1_example1() {
    assert_eq!(part1("abcdefgh"), "abcdffaa");
}

#[test]
fn part1_example2() {
    assert_eq!(part1("ghijklmn"), "ghjaabcc");
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), "cqjxxyzz");
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), "cqkaabcc");
}
