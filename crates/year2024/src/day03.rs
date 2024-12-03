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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day03.txt"
    ));
    assert_eq!(part1(input), 178886550);
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day03.txt"
    ));
    assert_eq!(part2(input), 87163705);
}
