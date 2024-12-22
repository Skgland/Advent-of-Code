use std::collections::BTreeMap;

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day22.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day22.example1.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day22.example2.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "22", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "22", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

struct SecretNumber {
    seed: usize,
}

impl SecretNumber {
    fn next(&mut self) -> usize {
        self.seed = Self::prng(self.seed);
        self.seed
    }

    fn prng(mut seed: usize) -> usize {
        fn mix(seed: usize, b: usize) -> usize {
            seed ^ b
        }

        fn prune(seed: usize) -> usize {
            seed % 16777216
        }

        seed = prune(mix(seed, seed * 64));
        seed = prune(mix(seed, seed / 32));
        prune(mix(seed, seed * 2048))
    }
}

struct SecretNumberIterator {
    inner: SecretNumber,
}

impl Iterator for SecretNumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next())
    }
}

impl IntoIterator for SecretNumber {
    type Item = usize;

    type IntoIter = SecretNumberIterator;

    fn into_iter(self) -> Self::IntoIter {
        SecretNumberIterator { inner: self }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = SecretNumber> + '_ {
    input.lines().map(|line| SecretNumber {
        seed: line.parse().unwrap(),
    })
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|secret| secret.into_iter().skip(1999).next().unwrap())
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .flat_map(|secret| {
            let results = std::iter::once(secret.seed)
                .chain(secret.into_iter().take(2000))
                .map(|number| number % 10)
                .collect::<Vec<_>>()
                .windows(5)
                .map(|window| {
                    let seq: [isize; 4] = window
                        .windows(2)
                        .map(|diff| diff[1] as isize - diff[0] as isize)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    (seq, window[4])
                })
                .fold(BTreeMap::new(), |mut acc, (seq, profit)| {
                    // only the first occourence of a sequenc is relevant
                    acc.entry(seq).or_insert(profit);
                    acc
                });
            results
        })
        .fold(BTreeMap::<_, usize>::new(), |mut acc, (seq, price)| {
            *acc.entry(seq).or_default() += price;
            acc
        })
        .into_iter()
        .max_by_key(|(_, elem)| *elem)
        .unwrap()
        .1
}

#[test]
fn part1_prng_demo() {
    assert!(SecretNumber { seed: 123 }.into_iter().take(10).eq([
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254
    ]))
}

#[test]
fn part1_example1() {
    assert_eq!(
        part1(INPUT_EXAMPLE1),
        [8685429, 4700978, 15273692, 8667524].into_iter().sum()
    );
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 19150344884);
}

#[test]
fn part2_example2() {
    // sequence = [-2, 1, -1, 3]
    assert_eq!(part2(INPUT_EXAMPLE2), 7 + 7 + 0 + 9);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 2121);
}
