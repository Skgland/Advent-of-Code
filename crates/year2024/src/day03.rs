use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day03.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "3", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "3", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul { l: u32, r: u32 },
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.split("do()").flat_map(|part| {
        let (enabled, disabled) = part.split_once("don't()").unwrap_or((part, ""));
        std::iter::once(Instruction::Do)
            .chain(parse_mult(enabled))
            .chain(std::iter::once(Instruction::Dont))
            .chain(parse_mult(disabled))
    })
}

fn parse_mult(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.split("mul(").skip(1).flat_map(|entry| {
        let (args, _) = entry.split_once(')')?;
        let (l, r) = args.split_once(',')?;
        if l.len() > 3 || r.len() > 3 {
            return None;
        }
        Some(Instruction::Mul {
            l: l.parse().ok()?,
            r: r.parse().ok()?,
        })
    })
}

pub fn part1(input: &str) -> u32 {
    parse_input(input).fold(0, |mut acc, inst| {
        match inst {
            Instruction::Do => {}
            Instruction::Dont => {}
            Instruction::Mul { l, r } => acc += l * r,
        }
        acc
    })
}

pub fn part2(input: &str) -> u32 {
    parse_input(input).fold(0, {
        let mut enabled = true;
        move |mut acc, inst| {
            match inst {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Mul { l, r } => {
                    if enabled {
                        acc += l * r
                    }
                }
            }
            acc
        }
    })
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day03.example1.txt"
    ));
    assert_eq!(part1(input), 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 178886550);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day03.example2.txt"
    ));
    assert_eq!(part2(input), 2 * 4 + 8 * 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 87163705);
}
