use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day10.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "10", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "10", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

type Pos = (isize, isize);

struct Map {
    tiles: HashMap<Pos, u8>,
}
impl Map {
    fn trails(&self) -> impl Iterator<Item = HashMap<Pos, usize>> + '_ {
        self.tiles
            .iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(&trail_head, _)| self.reachable_from(trail_head, 0))
    }

    fn reachable_from(&self, start: Pos, current_height: u8) -> HashMap<Pos, usize> {
        if current_height == 9 {
            HashMap::from([(start, 1)])
        } else {
            [
                (start.0 - 1, start.1),
                (start.0 + 1, start.1),
                (start.0, start.1 - 1),
                (start.0, start.1 + 1),
            ]
            .into_iter()
            .filter_map(|new_pos| {
                self.tiles
                    .get(&new_pos)
                    .map(|&new_height| (new_pos, new_height))
            })
            .filter(|&(_, new_height)| new_height == current_height + 1)
            .flat_map(|(pos, height)| self.reachable_from(pos, height))
            .fold(HashMap::new(), |mut acc, (pos, count)| {
                *acc.entry(pos).or_default() += count;
                acc
            })
        }
    }
}

fn parse_input(input: &str) -> Map {
    let tiles: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().flat_map(move |(column, c)| {
                c.to_digit(10)
                    .map(|digit| ((column as isize, row as isize), digit as u8))
            })
        })
        .collect();
    Map { tiles }
}

pub fn part1(input: &str) -> usize {
    parse_input(input).trails().map(|trail| trail.len()).sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .trails()
        .map(|trail| trail.values().sum::<usize>())
        .sum()
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day10.example1.txt"
    ));
    assert_eq!(part1(input), 5 + 6 + 5 + 3 + 1 + 3 + 5 + 3 + 5);
}
#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day10.example2.txt"
    ));
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 682);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day10.example1.txt"
    ));
    assert_eq!(part2(input), 20 + 24 + 10 + 4 + 1 + 4 + 5 + 8 + 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1511);
}
