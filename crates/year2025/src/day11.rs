use std::collections::{BTreeMap, BTreeSet};

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day11.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day11.example1.txt"
));


#[cfg(test)]
const INPUT_EXAMPLE2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day11.example2.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "11", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "11", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    input.lines().map(|line| {
        let (from, tos) = line.split_once(": ").unwrap();
        let tos = tos.split(' ').collect();
        (from, tos)
    }).collect()
}

fn count_paths(graph: &BTreeMap<&str, BTreeSet<&str>>, start: &str, target: &str) -> u64 {
    let mut paths = BTreeMap::from([(start, 1)]);

    let mut counts = 0;

    while !paths.is_empty() {
        for (pos, count) in std::mem::take(&mut paths) {
            if pos == target {
                counts += count;
            } else if let Some(outs) = graph.get(pos) {
                for out in outs {
                    *paths.entry(out).or_default() += count;
                }
            }
        }
    }
    counts
}

pub fn part1(input: &str) -> u64 {
    let graph = parse_input(input);

    count_paths(&graph, "you", "out")
}


pub fn part2(input: &str) -> u64 {
    let graph = parse_input(input);

    // the graph can't have any cycles otherwise there would be infinit paths from start to goal
    // hint in task node name dac i.e. directed-acyclic-graph

    let fft_dac = count_paths(&graph, "fft", "dac");
    let dac_fft = count_paths(&graph, "dac", "fft");

    let (first, second, mid_count) = match (fft_dac, dac_fft) {
        (0,0) => panic!("impossible"),
        (0, count) => {
            ("dac", "fft", count)
        }

        (count, 0) => {
            ("fft", "dac", count)
        }
        (_,_) => panic!("infinit")
    };


    let start_count = count_paths(&graph, "svr", first);
    let end_count = count_paths(&graph,  second, "out");



    start_count * mid_count * end_count
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 5);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 708);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE2), 2);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 545394698933400);
}
