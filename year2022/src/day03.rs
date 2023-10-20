use std::collections::BTreeSet;

fn calculate_priority(item: u8) -> u8 {
    #[allow(clippy::manual_is_ascii_check)]
    if (b'a'..=b'z').contains(&item) {
        item - b'a' + 1
    } else {
        item - b'A' + 27
    }
}

fn parse_into_split_backpacks(
    input: &str,
) -> impl Iterator<Item = (BTreeSet<u8>, BTreeSet<u8>)> + '_ {
    input.lines().map(|line| {
        let (start, end) = line.split_at(line.len() / 2);
        let start_set: BTreeSet<_> = start.as_bytes().iter().copied().collect();
        let end_set: BTreeSet<_> = end.as_bytes().iter().copied().collect();
        (start_set, end_set)
    })
}

fn parse_into_backpacks(input: &str) -> impl Iterator<Item = BTreeSet<u8>> + '_ {
    input
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect())
}

pub fn part1(input: &str) -> u32 {
    parse_into_split_backpacks(input)
        .map(|(start_set, end_set)| {
            start_set
                .intersection(&end_set)
                .map(|&item| calculate_priority(item))
                .sum::<u8>() as u32
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    parse_into_backpacks(input)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunck| {
            chunck
                .iter()
                .cloned()
                .reduce(|l, r| l.intersection(&r).copied().collect())
                .unwrap()
                .into_iter()
                .map(|item| calculate_priority(item) as u32)
                .sum::<u32>()
        })
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day03.example.txt");
    assert_eq!(part1(input), 16 + 38 + 42 + 22 + 20 + 19);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day03.txt"));
    assert_eq!(part1(input), 7785);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day03.example.txt");
    assert_eq!(part2(input), 18 + 52);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day03.txt"));
    assert_eq!(part2(input), 2633);
}
