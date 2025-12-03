use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{collections::VecDeque, num::Wrapping};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day15.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "15", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "15", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
struct Instruction<'a> {
    label: &'a str,
    kind: InstructionKind,
}

enum InstructionKind {
    Remove,
    Insert(u8),
}

fn parse_input1(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.split(',')
}

fn parse_input2(input: &str) -> impl Iterator<Item = Instruction<'_>> + '_ {
    input.split(',').map(|elem| {
        if let Some(idx) = elem.find('-') {
            Instruction {
                label: &elem[..idx],
                kind: InstructionKind::Remove,
            }
        } else if let Some(idx) = elem.find('=') {
            Instruction {
                label: &elem[..idx],
                kind: InstructionKind::Insert(elem.as_bytes()[idx + 1] - b'0'),
            }
        } else {
            panic!()
        }
    })
}

fn hash(input: &str) -> u8 {
    input
        .bytes()
        .fold(Wrapping(0), |mut acc, c| {
            acc += c;
            acc *= 17;
            acc
        })
        .0
}

#[test]
fn test_hash() {
    let examples = [
        ("HASH", 52),
        ("rn=1", 30),
        ("cm-", 253),
        ("qp=3", 97),
        ("cm=2", 47),
        ("qp-", 14),
        ("pc=4", 180),
        ("ot=9", 9),
        ("ab=5", 197),
        ("pc-", 48),
        ("pc=6", 214),
        ("ot=7", 231),
    ];

    for (val, res) in examples {
        assert_eq!(hash(val), res);
    }
}

pub fn part1(input: &str) -> u32 {
    parse_input1(input.lines().next().unwrap())
        .map(hash)
        .map(|hash| hash as u32)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let iter = parse_input2(input);
    let mut boxes: [VecDeque<(&str, u8)>; 256] = [(); 256].map(|_| VecDeque::new());

    for inst in iter {
        let carton = &mut boxes[hash(inst.label) as usize];
        match inst.kind {
            InstructionKind::Remove => {
                carton.retain(|elem| elem.0 != inst.label);
            }
            InstructionKind::Insert(focus) => {
                if let Some(entry) = carton.iter_mut().find(|elem| elem.0 == inst.label) {
                    entry.1 = focus;
                } else {
                    carton.push_back((inst.label, focus));
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_id, carton)| {
            let box_power = box_id + 1;
            carton
                .iter()
                .enumerate()
                .map(|(slot_id, lense)| {
                    let slot_power = slot_id + 1;
                    box_power * slot_power * lense.1 as usize
                })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day15.example.txt"
    ));
    assert_eq!(part1(input), 1320);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 518107);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day15.example.txt"
    ));
    assert_eq!(part2(input), 145);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 303404);
}
