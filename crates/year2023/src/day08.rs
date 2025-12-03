use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{
    borrow::Borrow,
    collections::{BTreeSet, HashMap},
};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day08.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "8", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "8", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: false,
};

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    name: String,
}

impl Borrow<str> for Node {
    fn borrow(&self) -> &str {
        &self.name
    }
}

fn parse_input(input: &str) -> (Vec<Dir>, HashMap<Node, (Node, Node)>) {
    let mut lines = input.lines();

    let dirs = lines
        .next()
        .iter()
        .flat_map(|line| line.chars())
        .filter_map(|elem| match elem {
            'L' => Some(Dir::Left),
            'R' => Some(Dir::Right),
            _ => None,
        })
        .collect();

    lines.next();

    let nodes = lines
        .flat_map(|line| {
            let (name, dsts) = line.split_once(" = (")?;
            let (left, right) = dsts.trim_end_matches(')').split_once(", ")?;
            Some((
                Node {
                    name: name.to_string(),
                },
                (
                    Node {
                        name: left.to_string(),
                    },
                    Node {
                        name: right.to_string(),
                    },
                ),
            ))
        })
        .collect();

    (dirs, nodes)
}

fn navigate_step<'s>(
    from: &str,
    nodes: &'s HashMap<Node, (Node, Node)>,
    dir: &Dir,
) -> Option<&'s str> {
    nodes.get(from).map(|(left, right)| match dir {
        Dir::Left => left.name.as_str(),
        Dir::Right => right.name.as_str(),
    })
}

#[derive(Debug)]
struct Cycle {
    // finish state before the start of the cycle
    prefix: Vec<usize>,
    // iteration the cycle starts
    start: usize,
    // steps before the first repetition
    len: usize,
    // finish states relative to cycle start
    cycle: Vec<usize>,
}

impl Cycle {
    fn merge(&self, other: &Cycle) -> Cycle {
        let common_start = self.start.max(other.start);
        let common_len = helper::lcm(self.len, other.len);

        let a: BTreeSet<_> = (0..)
            .flat_map(|offset| {
                self.cycle
                    .iter()
                    .map(move |elem| elem + self.start + self.len * offset)
            })
            .take_while(|&elem| elem < common_start + common_len)
            .collect();
        let b: BTreeSet<_> = (0..)
            .flat_map(|offset| {
                other
                    .cycle
                    .iter()
                    .map(move |elem| elem + other.start + other.len * offset)
            })
            .take_while(|&elem| elem < common_start + common_len)
            .collect();

        let mut cycle: Vec<_> = a.intersection(&b).map(|elem| elem - common_start).collect();
        cycle.sort();

        Cycle {
            prefix: (1..common_start)
                .filter(|&elem| self.contains(elem) && other.contains(elem))
                .collect(),
            start: common_start,
            len: common_len,
            cycle,
        }
    }

    fn find(from: &str, map: &HashMap<Node, (Node, Node)>, dirs: &[Dir]) -> Cycle {
        let mut idx = 0;

        let dirs = dirs.iter().enumerate().collect::<Vec<_>>();

        let mut states = HashMap::new();

        let mut win = vec![];

        let mut at = from;

        let mut iter = std::iter::repeat(dirs).flatten();

        let (start, len) = loop {
            let (offset, step) = iter.next().unwrap();

            if let Some(first_idx) = states.insert((at, offset), idx) {
                break (first_idx, idx - first_idx);
            }

            idx += 1;
            at = navigate_step(at, map, step).unwrap();

            if at.ends_with('Z') {
                win.push(idx);
            }
        };

        Cycle {
            prefix: win
                .iter()
                .copied()
                .take_while(|elem| elem < &start)
                .collect(),
            start,
            len,
            cycle: win
                .iter()
                .copied()
                .skip_while(|elem| elem < &start)
                .map(|elem| elem - start)
                .collect(),
        }
    }

    fn contains(&self, val: usize) -> bool {
        self.prefix.contains(&val)
            || (val >= self.start
                && self
                    .cycle
                    .iter()
                    .any(|&cyc| (val - self.start) % self.len == cyc))
    }

    fn first(&self) -> usize {
        self.prefix
            .first()
            .copied()
            .or_else(|| self.cycle.first().map(|offset| self.start + offset))
            .unwrap()
    }
}

pub fn part1(input: &str) -> u32 {
    let (dirs, nodes) = parse_input(input);

    std::iter::repeat(&dirs)
        .flatten()
        .scan(("AAA", 0), |(at, step_count), step| {
            let next = navigate_step(at, &nodes, step)?;

            *at = next;
            *step_count += 1;

            if next == "ZZZ" {
                Some(Some(*step_count))
            } else {
                Some(None)
            }
        })
        .flatten()
        .next()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let (dirs, nodes) = parse_input(input);

    let cycle = nodes
        .keys()
        .filter(|elem| elem.name.ends_with('A'))
        .map(|elem| Cycle::find(&elem.name, &nodes, &dirs))
        .reduce(|a, b| a.merge(&b))
        .unwrap();

    cycle.first()
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day08.example1.txt"
    ));
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day08.example2.txt"
    ));
    assert_eq!(part1(input), 6);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 13939);
}

#[test]
fn part2_example3() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day08.example3.txt"
    ));
    assert_eq!(part2(input), 6);
}

#[test]
#[ignore = "too slow"]
fn part2_full() {
    assert_eq!(part2(INPUT), 8906539031197);
}
