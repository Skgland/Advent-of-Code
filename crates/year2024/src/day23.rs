use std::collections::{BTreeMap, BTreeSet};

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day23.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day23.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "23", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "23", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct Input<'a> {
    map: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

fn parse_input(input: &str) -> Input<'_> {
    Input {
        map: input
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .fold(BTreeMap::new(), |mut acc, (a, b)| {
                acc.entry(a).or_default().insert(b);
                acc.entry(b).or_default().insert(a);
                acc
            }),
    }
}

fn find_clique_3<'a>(input: &Input<'a>) -> BTreeSet<BTreeSet<&'a str>> {
    let mut c3s = BTreeSet::new();
    for (&a_id, a_con) in &input.map {
        for &b_id in a_con {
            if a_id < b_id {
                let b_con = &input.map[b_id];
                for &c_id in a_con.intersection(b_con) {
                    if b_id < c_id {
                        c3s.insert(BTreeSet::from([a_id, b_id, c_id]));
                    }
                }
            }
        }
    }
    c3s
}

pub fn part1(input: &str) -> usize {
    find_clique_3(&parse_input(input))
        .into_iter()
        .filter(|entry| entry.iter().any(|node| node.starts_with('t')))
        .count()
}

pub fn part2(input: &str) -> String {
    let input = parse_input(input);
    // find all cliques of size 3
    let mut cliques = find_clique_3(&input);

    let largest = loop {
        let mut current_cliques = std::mem::take(&mut cliques);
        if current_cliques.len() == 1 {
            break current_cliques.pop_first().unwrap();
        }
        for clique in current_cliques {
            // finding common connected nodes
            let mut intersection = input.map[clique.first().unwrap()].clone();
            for entry in &clique {
                intersection = intersection
                    .intersection(&input.map[entry])
                    .copied()
                    .collect();
            }
            for entry in intersection {
                // reduce dupplicates by only adding entries larger than all existing entries
                if clique.iter().all(|exitsting| exitsting < &entry) {
                    let mut new = clique.clone();
                    new.insert(entry);
                    cliques.insert(new);
                }
            }
        }
    };
    largest.into_iter().collect::<Vec<_>>().join(",")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 7);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1110);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), "co,de,ka,ta");
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), "ej,hm,ks,ms,ns,rb,rq,sc,so,un,vb,vd,wd");
}
