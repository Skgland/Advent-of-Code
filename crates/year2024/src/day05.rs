use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day05.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "5", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "5", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
struct Input {
    rules: HashMap<u8, HashSet<u8>>,
    updates: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let rules = (&mut lines)
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (pre, post) = line.split_once('|').unwrap();
            (pre.parse::<u8>().unwrap(), post.parse::<u8>().unwrap())
        })
        .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, (pre, post)| {
            acc.entry(pre).or_default().insert(post);
            acc
        });

    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|entry| entry.parse().unwrap())
                .collect()
        })
        .collect();

    Input { rules, updates }
}

pub fn part1(input: &str) -> u32 {
    let input = parse_input(input);

    input
        .updates
        .iter()
        .filter(|update| is_valid(update, &input.rules))
        .map(|update| u32::from(*get_middle(update)))
        .sum()
}

fn get_middle(mut values: &[u8]) -> &u8 {
    loop {
        match values {
            [_, rem @ .., _] => values = rem,
            [result] => return result,
            [] => {
                panic!("No middle")
            }
        }
    }
}

fn is_valid(mut update: &[u8], rules: &HashMap<u8, HashSet<u8>>) -> bool {
    while let [heads @ .., tail] = update {
        update = heads;
        if let Some(rule) = rules.get(tail) {
            if heads.iter().any(|pre| rule.contains(pre)) {
                return false;
            }
        }
    }
    true
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse_input(input);
    input
        .updates
        .iter_mut()
        .filter(|update| !is_valid(update, &input.rules))
        .map(|update| {
            fixup(update, &input.rules);
            assert!(is_valid(update, &input.rules));
            u32::from(*get_middle(update))
        })
        .sum()
}

fn fixup(update: &mut [u8], rules: &HashMap<u8, HashSet<u8>>) {
    let mut len = update.len();
    'outer: while let [heads @ .., tail] = &mut update[..len] {
        if let Some(rule) = rules.get(tail) {
            for head in heads.iter_mut() {
                if rule.contains(head) {
                    std::mem::swap(head, tail);
                    continue 'outer;
                }
            }
        }
        len -= 1;
    }
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day05.example.txt"
    ));
    assert_eq!(part1(input), 61 + 53 + 29);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 6267);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day05.example.txt"
    ));
    assert_eq!(part2(input), 47 + 29 + 47);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 5184);
}
