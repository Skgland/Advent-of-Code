use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day12.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "12", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "12", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

pub struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

fn parse_input(input: &str) -> Graph<'_> {
    let edges = input.lines().flat_map(|elem| elem.split_once('-')).fold(
        HashMap::new(),
        |mut map: HashMap<_, Vec<_>>, elem| {
            map.entry(elem.0).or_default().push(elem.1);
            map.entry(elem.1).or_default().push(elem.0);
            map
        },
    );
    Graph { edges }
}

pub enum SmallCaveDuplicateStrategy {
    NoDuplicates,
    AtMostOneDuplicateInTotal,
}

pub fn no_duplicates<'a>(graph: &Graph<'a>, path: &mut Vec<&'a str>) -> u32 {
    let mut unexplored_branches =
        Vec::from([graph.edges.get(path.last().unwrap()).unwrap().as_slice()]);

    let mut count = 0;

    while let Some(branches) = unexplored_branches.pop() {
        match branches {
            [] => {
                path.pop();
            }
            &[head, ref tail @ ..] => {
                unexplored_branches.push(tail);

                if head == "end" {
                    count += 1;
                } else if head.chars().next().unwrap().is_uppercase() || !path.contains(&head) {
                    path.push(head);
                    unexplored_branches.push(graph.edges.get(head).unwrap())
                }
            }
        }
    }

    count
}

pub fn at_most_one_duplicate<'a>(graph: &Graph<'a>, path: &mut Vec<&'a str>) -> u32 {
    let mut unexplored_branches = Vec::from([graph.edges.get("start").unwrap().as_slice()]);

    let mut count = 0;

    while let Some(branches) = unexplored_branches.pop() {
        match branches {
            [] => {
                path.pop();
            }
            &[head, ref tail @ ..] => {
                unexplored_branches.push(tail);

                if head == "end" {
                    count += 1;
                } else if head == "start" {
                } else {
                    let uppercase = head.chars().next().unwrap().is_uppercase();
                    let contained = path.contains(&head);
                    path.push(head);
                    if uppercase || !contained {
                        unexplored_branches.push(graph.edges.get(head).unwrap())
                    } else {
                        count += no_duplicates(graph, path);
                    }
                }
            }
        }
    }

    count
}

pub fn part1(input: &str) -> u32 {
    let graph = parse_input(input);
    no_duplicates(&graph, &mut vec!["start"])
}

pub fn part2(input: &str) -> u32 {
    let graph = parse_input(input);
    at_most_one_duplicate(&graph, &mut vec!["start"])
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day12.example1.txt"
    ));
    assert_eq!(part1(input), 10);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day12.example2.txt"
    ));
    assert_eq!(part1(input), 19);
}

#[test]
fn part1_example3() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day12.example3.txt"
    ));
    assert_eq!(part1(input), 226);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 4720);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day12.example1.txt"
    ));
    assert_eq!(part2(input), 36);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day12.example2.txt"
    ));
    assert_eq!(part2(input), 103);
}

#[test]
fn part2_example3() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day12.example3.txt"
    ));
    assert_eq!(part2(input), 3509);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 147848);
}
