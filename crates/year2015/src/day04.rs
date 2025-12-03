use std::collections::VecDeque;

use helper::{TASKS, Task};
use linkme::distributed_slice;
use md5::digest::Digest;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2015/day04.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2015", "4", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2015", "4", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: false,
};

#[inline(always)]
fn find_md5_prefix(input: &str, prefix: &str) -> u32 {
    let input = input.trim();
    let md5_prefix = md5::Md5::new_with_prefix(input.as_bytes());

    let mut todo = VecDeque::new();

    for digit in 1..=9 {
        let md5 = md5_prefix.clone().chain_update([digit as u8 + b'0']);

        if format!("{:x}", md5.clone().finalize()).starts_with(prefix) {
            return digit;
        } else {
            todo.push_back((digit, md5));
        }
    }

    while let Some((val, md5_prefix)) = todo.pop_front() {
        for digit in 0..=9 {
            let md5 = md5_prefix.clone().chain_update([digit as u8 + b'0']);

            if format!("{:x}", md5.clone().finalize()).starts_with(prefix) {
                return val * 10 + digit;
            } else {
                todo.push_back((val * 10 + digit, md5));
            }
        }
    }
    panic!("No suffix found that produces the desired prefix");
}

pub fn part1(input: &str) -> u32 {
    find_md5_prefix(input, "00000")
}

pub fn part2(input: &str) -> u32 {
    find_md5_prefix(input, "000000")
}

#[test]
fn part1_example1() {
    let input = "abcdef";
    assert_eq!(part1(input), 609043);
}

#[test]
fn part1_example2() {
    let input = "pqrstuv";
    assert_eq!(part1(input), 1048970);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 282749);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 9962624);
}
