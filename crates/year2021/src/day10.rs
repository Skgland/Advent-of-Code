use crate::day10::Delimiter::{AngleBracket, Brace, Bracket, Parenthesis};
use crate::day10::Side::{Close, Open};
use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day10.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "10", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "10", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
enum Side {
    Open(Delimiter),
    Close(Delimiter),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Delimiter {
    Parenthesis,
    Bracket,
    Brace,
    AngleBracket,
}

impl Delimiter {
    pub fn score1(&self) -> usize {
        match self {
            Parenthesis => 3,
            Bracket => 57,
            Brace => 1197,
            AngleBracket => 25137,
        }
    }
    pub fn score2(&self) -> usize {
        match self {
            Parenthesis => 1,
            Bracket => 2,
            Brace => 3,
            AngleBracket => 4,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = Side> + '_> + '_ {
    input.lines().map(|line| {
        line.chars().flat_map(|char| match char {
            '(' => Some(Open(Parenthesis)),
            '[' => Some(Open(Bracket)),
            '{' => Some(Open(Brace)),
            '<' => Some(Open(AngleBracket)),
            ')' => Some(Close(Parenthesis)),
            ']' => Some(Close(Bracket)),
            '}' => Some(Close(Brace)),
            '>' => Some(Close(AngleBracket)),
            _ => None,
        })
    })
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|line| {
            let mut stack = vec![];
            for delim in line {
                match delim {
                    Open(delim) => stack.push(delim),
                    Close(delim) => {
                        if Some(delim) != stack.pop() {
                            return delim.score1();
                        }
                    }
                }
            }
            0
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut completions: Vec<_> = parse_input(input)
        .flat_map(|line| {
            let mut stack = vec![];
            for delim in line {
                match delim {
                    Open(delim) => stack.push(delim),
                    Close(delim) => {
                        if Some(delim) != stack.pop() {
                            return None;
                        }
                    }
                }
            }

            let res = stack
                .iter()
                .rev()
                .fold(0, |acc, delim| acc * 5 + delim.score2());
            if res == 0 {
                None
            } else {
                Some(res)
            }
        })
        .collect();
    completions.sort_unstable();
    completions[completions.len() / 2]
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day10.example.txt"
    ));
    assert_eq!(part1(input), 2 * 3 + 57 + 1197 + 25137);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 318081);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day10.example.txt"
    ));
    assert_eq!(part2(input), 288957);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 4361305341);
}
