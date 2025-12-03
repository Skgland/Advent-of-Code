use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2015/day09.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2015", "9", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2015", "9", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> HashMap<(&'_ str, &'_ str), usize> {
    input
        .lines()
        .flat_map(
            |elem| match elem.split(' ').collect::<Vec<_>>().as_slice() {
                [a, "to", b, "=", dist] => [
                    ((*a, *b), dist.parse().unwrap()),
                    ((*b, *a), dist.parse().unwrap()),
                ],
                _ => panic!("Invalid line: {elem}"),
            },
        )
        .collect()
}

fn preferred_distance(
    distances: &HashMap<(&str, &str), usize>,
    known: &mut HashMap<(&str, BTreeSet<&str>), usize>,
    to_visit: &BTreeSet<&str>,
    from: &str,
    comparator: impl Fn(&usize, &usize) -> Ordering + Copy,
) -> Option<usize> {
    if to_visit.is_empty() {
        Some(0)
    } else if let Some(&result) = known.get(&(from, to_visit.clone())) {
        Some(result)
    } else {
        let mut min = None;

        for next in to_visit {
            let Some(&next_dist) = distances.get(&(from, next)) else {
                continue;
            };

            let mut clone = to_visit.clone();
            clone.remove(next);
            let Some(rest_dist) = preferred_distance(distances, known, &clone, next, comparator)
            else {
                continue;
            };

            let dist = next_dist + rest_dist;
            if min.is_none() || min.is_some_and(|min| comparator(&dist, &min).is_lt()) {
                min = Some(dist)
            }
        }

        min
    }
}

pub fn part1(input: &str) -> usize {
    let distances = parse_input(input);
    let rem = distances.keys().map(|(a, _)| *a).collect::<BTreeSet<_>>();
    let mut known = HashMap::new();

    rem.iter()
        .flat_map(|elem| {
            let mut to_visit = rem.clone();
            to_visit.remove(elem);
            preferred_distance(&distances, &mut known, &to_visit, elem, usize::cmp)
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let distances = parse_input(input);
    let rem = distances.keys().map(|(a, _)| *a).collect::<BTreeSet<_>>();
    let mut known = HashMap::new();

    rem.iter()
        .flat_map(|elem| {
            let mut to_visit = rem.clone();
            to_visit.remove(elem);
            preferred_distance(&distances, &mut known, &to_visit, elem, |a, b| {
                a.cmp(b).reverse()
            })
        })
        .max()
        .unwrap()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day09.example.txt"
    ));
    assert_eq!(part1(input), 605);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 117);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day09.example.txt"
    ));
    assert_eq!(part2(input), 982);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 909);
}
