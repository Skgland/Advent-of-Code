use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day15.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "15", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "15", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(char_to_digit).collect())
        .collect()
}

fn char_to_digit(c: char) -> u8 {
    c as u8 - b'0'
}

pub fn get_neighbours(pos: (usize, usize), dim: usize) -> impl Iterator<Item = (usize, usize)> {
    let right = (pos.0.checked_add(1).filter(move |&x| x < dim), Some(pos.1));
    let left = (pos.0.checked_sub(1), Some(pos.1));
    let up = (Some(pos.0), pos.1.checked_add(1).filter(move |&y| y < dim));
    let down = (Some(pos.0), pos.1.checked_sub(1));
    [right, left, up, down]
        .into_iter()
        .flat_map(|elem| match elem {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        })
}

pub fn traverse(map: &[Vec<u8>], multiplier: usize) -> u32 {
    let mut candidates = HashMap::from([((0, 0), 0)]);
    let mut done = HashSet::new();
    let dim = map.len();
    let destination = (dim * multiplier - 1, dim * multiplier - 1);

    while let Some((pos, cost)) = candidates
        .iter()
        .min_by_key(|&(_key, value)| value)
        .map(|(&pos, &cost)| (pos, cost))
    {
        if pos == destination {
            return cost;
        }
        candidates.remove(&pos);
        done.insert(pos);
        get_neighbours(pos, map.len() * multiplier)
            .filter(|neighbor| !done.contains(neighbor))
            .for_each(|neighbor_pos| {
                let neighbour_cost = (map[neighbor_pos.0 % map.len()][neighbor_pos.1 % dim] as u32
                    + (neighbor_pos.0 / dim + neighbor_pos.1 / dim) as u32
                    - 1)
                    % 9
                    + 1;
                let total_cost = cost + neighbour_cost;
                let value = candidates.entry(neighbor_pos).or_insert(total_cost);
                *value = (*value).min(total_cost)
            });
    }
    panic!("Never reached the exit!");
}

pub fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    traverse(&map, 1)
}

pub fn part2(input: &str) -> u32 {
    let map = parse_input(input);
    traverse(&map, 5)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day15.example.txt"
    ));
    assert_eq!(part1(input), 40);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 562);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day15.example.txt"
    ));
    assert_eq!(part2(input), 315);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 2874);
}
