use std::{collections::BTreeMap, num::ParseIntError, str::FromStr};

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day12.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day12.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "12", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

struct Input {
    patterns: BTreeMap<usize, Pattern>,
    regions: Vec<Region>,
}

struct Pattern {
    width: usize,
    heitgh: usize,
    units: usize,
    pattern: Vec<Vec<bool>>,
}

struct Region {
    width: usize,
    height: usize,
    used_patterns: BTreeMap<usize, usize>,
}
impl Region {
    fn check_packages_fit(&self, patterns: &BTreeMap<usize, Pattern>) -> bool {
        assert!(patterns.values().all(|p| p.width == 3 && p.heitgh == 3));

        let guaranteed_packages = (self.width / 3) * (self.height / 3);

        if guaranteed_packages >= self.used_patterns.values().copied().sum() {
            return true;
        }

        if self.width * self.height
            < self
                .used_patterns
                .iter()
                .map(|(pat_idx, pat_count)| patterns[pat_idx].units * pat_count)
                .sum()
        {
            return false;
        }

        todo!("solve the example")
    }
}

impl FromStr for Region {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (wh, cs) = s.split_once(": ").unwrap();
        let (w, h) = wh.split_once('x').unwrap();

        let used_patterns = cs
            .split(' ')
            .enumerate()
            .map(|(idx, count)| Ok((idx, count.parse()?)))
            .collect::<Result<_, _>>()?;

        Ok(Region {
            width: w.parse()?,
            height: h.parse()?,
            used_patterns,
        })
    }
}

fn parse_input(input: &str) -> Input {
    let input = input.replace("\r\n", "\n");
    let mut groups = input.split("\n\n").collect::<Vec<_>>();

    let regions = groups.pop().unwrap();

    let patterns = groups
        .iter()
        .map(|group| {
            let (idx, pattern) = group.split_once(":\n").unwrap();

            let pattern: Vec<Vec<bool>> = pattern
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();

            (
                idx.parse().unwrap(),
                Pattern {
                    width: pattern.iter().map(|line| line.len()).max().unwrap(),
                    heitgh: pattern.len(),
                    units: pattern
                        .iter()
                        .map(|line| line.iter().copied().filter(|&e| e).count())
                        .sum(),
                    pattern,
                },
            )
        })
        .collect();
    let regions = regions.lines().map(|line| line.parse().unwrap()).collect();

    Input { patterns, regions }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);

    input
        .regions
        .iter()
        .filter(|region| region.check_packages_fit(&input.patterns))
        .count()
}

#[test]
#[ignore = "unsolved"]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 2);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 587);
}
