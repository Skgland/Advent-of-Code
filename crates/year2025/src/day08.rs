use std::collections::{BTreeMap, HashMap};

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day08.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day08.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "08", "part1"],
    run: || println!("{}", part1(INPUT, 1000)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "08", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = [u32; 3]> + '_ {
    input.lines().map(|line| {
        let mut elements = line.split(',').map(|val| val.parse().unwrap());
        std::array::from_fn::<_, 3, _>(|_| elements.next().unwrap())
    })
}

fn distance_square(a: [u32; 3], b: [u32; 3]) -> u64 {
    a.into_iter()
        .zip(b)
        .map(|(x, y)| x.abs_diff(y) as u64)
        .map(|x| x * x)
        .sum::<u64>()
}

fn pairs(points: &[[u32; 3]]) -> impl Iterator<Item = ([u32; 3], [u32; 3])> + '_ {
    (0..points.len() - 1)
        .flat_map(move |i| (i + 1..points.len()).map(move |j| (points[i], points[j])))
}

pub fn part1(input: &str, connections: usize) -> usize {
    let junction_boxes = parse_input(input).collect::<Vec<_>>();

    let mut circuits = junction_boxes
        .iter()
        .enumerate()
        .map(|(idx, &pos)| (pos, idx))
        .collect::<HashMap<_, _>>();

    let mut distances = pairs(&junction_boxes)
        .map(|(a, b)| (a, b, distance_square(a, b)))
        .collect::<Vec<_>>();

    distances.sort_by_key(|&(_, _, dist)| dist);

    for (a, b, _) in distances.iter().take(connections) {
        let min = circuits[a].min(circuits[b]);
        let max = circuits[a].max(circuits[b]);

        for circuit in circuits.values_mut() {
            if *circuit == max {
                *circuit = min;
            }
        }
    }

    let mut circuits = circuits
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, (_, comp)| {
            *acc.entry(comp).or_insert(0) += 1usize;
            acc
        })
        .into_iter()
        .collect::<Vec<_>>();

    circuits.sort_by_key(|&(_, size)| size);
    circuits.reverse();
    circuits.into_iter().take(3).map(|(_, size)| size).product()
}

pub fn part2(input: &str) -> u32 {
    let junction_boxes = parse_input(input).collect::<Vec<_>>();

    let mut circuits = junction_boxes
        .iter()
        .enumerate()
        .map(|(idx, &pos)| (pos, idx))
        .collect::<HashMap<_, _>>();

    let mut distances = pairs(&junction_boxes)
        .map(|(a, b)| (a, b, distance_square(a, b)))
        .collect::<Vec<_>>();

    distances.sort_by_key(|&(_, _, dist)| dist);

    let mut last_pair = ([0; 3], [0; 3]);

    for (a, b, _) in distances {
        if circuits[&a] != circuits[&b] {
            last_pair = (a, b);

            let min = circuits[&a].min(circuits[&b]);
            let max = circuits[&a].max(circuits[&b]);

            for circuit in circuits.values_mut() {
                if *circuit == max {
                    *circuit = min;
                }
            }
        }
    }

    last_pair.0[0] * last_pair.1[0]
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1, 10), 5 * 4 * 2);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT, 1000), 50568);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 216 * 117);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 36045012);
}
