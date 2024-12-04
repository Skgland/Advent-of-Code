use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct Graph<'a> {
    nodes: BTreeMap<&'a str, HashSet<&'a str>>,
}

fn parse_input(input: &str) -> Graph<'_> {
    Graph {
        nodes: input.lines().fold(BTreeMap::new(), |mut acc, next| {
            let (src, dests) = next.split_once(": ").unwrap();
            for dest in dests.split(' ') {
                acc.entry(src).or_default().insert(dest);
            }
            acc
        }),
    }
}

pub fn part1(input: &str) -> u32 {
    let mut graph = parse_input(input);
    todo!("part1 WIP: {graph:?}")
}

pub fn part2(input: &str) -> u32 {
    let mut graph = parse_input(input);
    todo!("part2 WIP: {graph:?}")
}

#[test]
#[ignore = "NYI"]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day25.example.txt"
    ));
    assert_eq!(part1(input), 9 * 6);
}

#[test]
#[ignore = "NYI"]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day25.txt"
    ));
    assert_eq!(part1(input), 1292);
}

#[test]
#[ignore = "NYI"]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day25.example.txt"
    ));
    assert_eq!(part2(input), 5);
}

#[test]
#[ignore = "NYI"]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day25.txt"
    ));
    assert_eq!(part2(input), 1262);
}
