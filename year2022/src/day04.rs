use std::ops::RangeInclusive;

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let (start, end) = input.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input.lines().map(|line| {
        let (l, r) = line.split_once(',').unwrap();
        (parse_range(l), parse_range(r))
    })
}

fn subset(l: &RangeInclusive<u32>, r: &RangeInclusive<u32>) -> bool {
    l.start() <= r.start() && r.end() <= l.end()
}

fn intersect(l: &RangeInclusive<u32>, r: &RangeInclusive<u32>) -> bool {
    l.start() <= r.end() && r.start() <= l.end()
}

fn both(
    input: &str,
    filter: impl Fn(&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool,
) -> usize {
    parse(input).filter(filter).count()
}

pub fn part1(input: &str) -> usize {
    both(input, |(l, r)| subset(l, r) || subset(r, l))
}

pub fn part2(input: &str) -> usize {
    both(input, |(l, r)| intersect(l, r))
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day04.example.txt");
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day04.txt"));
    assert_eq!(part1(input), 542);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day04.example.txt");
    assert_eq!(part2(input), 4);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day04.txt"));
    assert_eq!(part2(input), 900);
}
