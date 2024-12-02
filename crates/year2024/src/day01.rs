use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let l = l.trim().parse::<u64>().unwrap();
            let r = r.trim().parse::<u64>().unwrap();
            (l, r)
        })
        .unzip()
}

pub fn part1(input: &str) -> u64 {
    let (mut l, mut r) = parse_input(input);

    l.sort();
    r.sort();

    l.into_iter()
        .zip(r.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (l, r) = parse_input(input);
    let l = l.into_iter().fold(HashMap::new(), |mut acc, val| {
        *acc.entry(val).or_default() += 1;
        acc
    });
    r.into_iter().flat_map(|r| l.get(&r).map(|l| r * l)).sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day01.example.txt"
    ));
    assert_eq!(part1(input), 11);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day01.txt"
    ));
    assert_eq!(part1(input), 936063);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day01.example.txt"
    ));
    assert_eq!(part2(input), 31);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day01.txt"
    ));
    assert_eq!(part2(input), 23150395);
}
