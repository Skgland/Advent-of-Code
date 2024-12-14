use helper::{Task, TASKS};
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
    (1..)
        .find(|suf| {
            let digest = md5_prefix.clone().chain_update(format!("{suf}")).finalize();
            let hash = format!("{:x}", digest);
            hash.starts_with(prefix)
        })
        .unwrap()
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

#[ignore = "too slow"]
#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 9962624);
}
