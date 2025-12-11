use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

use helper::{TASKS, Task};
use linkme::distributed_slice;
use scryer_prolog::{LeafAnswer, Term};

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

#[derive(Debug)]
struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
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
        let mut count = 0;
        let mut states = BTreeSet::from([self.joltages.clone()]);
        let target = vec![0; self.joltages.len()];

        while !states.contains(&target) {
            count += 1;
            for state in std::mem::take(&mut states) {
                for button in &self.buttons {
                    if let Some(new_state) = apply_button_to_state(&state, button) {
                        states.insert(new_state);
                    }
                }
            }
        }

        count
    }
}

fn apply_button_to_state(state: &[u16], button: &u16) -> Option<Vec<u16>> {
    state
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, count)| {
            if (button & (1 << idx)) != 0 {
                count.checked_sub(1)
            } else {
                Some(count)
            }
        })
        .collect::<Option<Vec<_>>>()
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
            joltages: joltage
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

    let mut machine = scryer_prolog::MachineBuilder::new().build();
    machine.consult_module_string("day10.pl", include_str!("./day10.pl"));

    let query = build_query(&machines);

    let result = machine.run_query(query).next().unwrap().unwrap();

    match result {
        LeafAnswer::LeafAnswer { bindings, .. } => {
            let res = &bindings["Res"];
            match res {
                scryer_prolog::Term::Integer(ibig) => ibig.try_into().unwrap(),
                scryer_prolog::Term::Rational(rat) => rat.to_int().unwrap().try_into().unwrap(),
                _ => unreachable!(),
            }
        }
        LeafAnswer::False => panic!("Unexpected no solution"),
        LeafAnswer::True => panic!("Unexpected no bindings"),
        LeafAnswer::Exception(exc) => panic!("Unexpected exception: {exc:?}"),
    }
}

fn build_query(machines: &[Machine]) -> String {
    let machines = machines
        .iter()
        .map(|machine| format!("machine({:?}, {:?})", machine.buttons, machine.joltages))
        .reduce(|l, r| format!("{l},\n\t\t{r}"))
        .unwrap();

    format!("part2([\n\t\t{machines}\n\t], Res).")
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
    let query = build_query(&parse_input(INPUT_EXAMPLE1).collect::<Vec<_>>());
    let _ = std::fs::write("./day10-part2-example.pl", format!("test(Res) :- {query}"));

    assert_eq!(part2(INPUT_EXAMPLE1), 10 + 12 + 11);
}

#[test]
fn part2_full() {
    let query = build_query(&parse_input(INPUT).collect::<Vec<_>>());
    let _ = std::fs::write("./day10-part2-full.pl", format!("test(Res) :- {query}"));

    let res = part2(INPUT);

    assert!(8788 < res);
    assert_eq!(res, 17214);
}
