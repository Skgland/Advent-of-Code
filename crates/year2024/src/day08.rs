use std::collections::{HashMap, HashSet};

type Position = (isize, isize);

struct Map {
    nodes: HashMap<char, Vec<Position>>,
    width: isize,
    height: isize,
}

fn parse_input(input: &str) -> Map {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().flat_map(move |(column, tile)| {
                let pos = (column as isize, row as isize);

                match tile {
                    '.' => None,
                    c @ ('a'..='z' | 'A'..='Z' | '0'..='9') => Some((c, pos)),
                    _ => {
                        eprintln!("unexpected input char: {tile:?}");
                        None
                    }
                }
            })
        })
        .fold(
            HashMap::<char, Vec<Position>>::new(),
            |mut acc, (name, pos)| {
                acc.entry(name).or_default().push(pos);
                acc
            },
        );

    Map {
        nodes,
        height: input.lines().count() as isize,
        width: input.lines().next().unwrap().len() as isize,
    }
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let anti_nodes = map
        .nodes
        .values()
        .flat_map(|poss| {
            let mut rem = poss.as_slice();
            let mut anti_nodes = HashSet::with_capacity(poss.len());
            while let [head, tail @ ..] = rem {
                rem = tail;
                for other in tail {
                    let dx = head.0 - other.0;
                    let dy = head.1 - other.1;
                    anti_nodes.insert((head.0 + dx, head.1 + dy));
                    anti_nodes.insert((other.0 - dx, other.1 - dy));
                }
            }
            anti_nodes
        })
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < map.width && y < map.height)
        .collect::<HashSet<_>>();
    anti_nodes.len()
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let anti_nodes = map
        .nodes
        .values()
        .flat_map(|poss| {
            let mut rem = poss.as_slice();
            let mut anti_nodes = HashSet::with_capacity(poss.len());
            let max_dim = map.width.max(map.height);
            while let [head, tail @ ..] = rem {
                rem = tail;
                for other in tail {
                    let dx = head.0 - other.0;
                    let dy = head.1 - other.1;
                    for t in -max_dim..=max_dim {
                        anti_nodes.insert((head.0 + t * dx, head.1 + t * dy));
                    }
                }
            }
            anti_nodes
        })
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < map.width && y < map.height)
        .collect::<HashSet<_>>();
    anti_nodes.len()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day08.example1.txt"
    ));
    assert_eq!(part1(input), 14);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day08.txt"
    ));
    assert_eq!(part1(input), 371);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day08.example1.txt"
    ));
    assert_eq!(part2(input), 34);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day08.example2.txt"
    ));
    assert_eq!(part2(input), 9);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day08.txt"
    ));
    assert_eq!(part2(input), 1229);
}
