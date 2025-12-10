use std::{num::ParseIntError, str::FromStr};

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day10.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day10.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "10", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "10", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}
impl Machine {
    fn min_lamp_pushes(&self) -> u32 {
        let mut min = self.buttons.len() as u32;

        for push_pattern in 1..(2u32.pow(self.buttons.len() as u32)) {
            let buttons_pushed = push_pattern.count_ones();
            if buttons_pushed >= min {
                continue;
            }

            if self.target == self.push_toggle_buttons(push_pattern) {
                min = buttons_pushed;
            }
        }

        min
    }

    fn push_toggle_buttons(&self, push_pattern: u32) -> u16 {
        let mut state = 0;
        for (idx, toggle) in self.buttons.iter().enumerate() {
            if push_pattern & (1 << idx) != 0 {
                state ^= toggle;
            }
        }
        state
    }

    fn min_counter_pushes(&self) -> u32 {
        todo!()
    }
}

impl FromStr for Machine {
    type Err = ParseIntError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix('[').unwrap();
        s = s.strip_suffix('}').unwrap();

        let (target, s) = s.split_once("] (").unwrap();
        let (buttons, joltage) = s.split_once(") {").unwrap();

        Ok(Machine {
            target: target.chars().enumerate().fold(0, |acc, (idx, c)| {
                if c == '#' { acc | (1 << idx) } else { acc }
            }),
            buttons: buttons
                .split(") (")
                .map(|button| {
                    let lamp_indices = button
                        .split(',')
                        .map(|lamp_idx| lamp_idx.parse::<u16>())
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(lamp_indices
                        .into_iter()
                        .fold(0, |acc, idx| acc | (1 << idx)))
                })
                .collect::<Result<Vec<_>, _>>()?,
            joltage: joltage
                .split(',')
                .map(|part| part.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn part1(input: &str) -> u32 {
    let machines = parse_input(input).collect::<Vec<_>>();

    machines
        .iter()
        .map(|machine| machine.min_lamp_pushes())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let machines = parse_input(input).collect::<Vec<_>>();

    machines
        .iter()
        .map(|machine| machine.min_counter_pushes())
        .sum()
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 2 + 3 + 2);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 466);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
