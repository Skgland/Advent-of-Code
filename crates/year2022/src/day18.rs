use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day18.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "18", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "18", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse(input: &str) -> impl Iterator<Item = [isize; 3]> + '_ {
    input.lines().map(|line| {
        line.split(',')
            .map(|number| number.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    })
}

fn neighbors(pos: [isize; 3]) -> [[isize; 3]; 6] {
    [
        [pos[0] + 1, pos[1], pos[2]],
        [pos[0] - 1, pos[1], pos[2]],
        [pos[0], pos[1] + 1, pos[2]],
        [pos[0], pos[1] - 1, pos[2]],
        [pos[0], pos[1], pos[2] + 1],
        [pos[0], pos[1], pos[2] - 1],
    ]
}

pub fn part1(input: &str) -> usize {
    let points = parse(input).collect::<HashSet<_>>();

    points
        .iter()
        .map(|pos| {
            6 - neighbors(*pos)
                .into_iter()
                .filter(|neighbor| points.contains(neighbor))
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let points = parse(input).collect::<HashSet<_>>();

    let (min, max) = points.iter().fold(
        {
            let elem = points.iter().next().unwrap();
            (*elem, *elem)
        },
        |(min, max), next| {
            (
                [0, 1, 2].map(|idx| min[idx].min(next[idx])),
                [0, 1, 2].map(|idx| max[idx].max(next[idx])),
            )
        },
    );

    let start = [min[0] - 1, min[1] - 1, min[2] - 1];

    let mut accessible = HashSet::new();
    accessible.insert(start);

    let mut surface = 0;

    let mut todo = vec![start];

    while let Some(entry) = todo.pop() {
        for neighbor in neighbors(entry) {
            if points.contains(&neighbor) {
                surface += 1;
            } else if is_reasonable(neighbor, min, max) && accessible.insert(neighbor) {
                todo.push(neighbor);
            }
        }
    }

    surface
}

fn is_reasonable(entry: [isize; 3], min: [isize; 3], max: [isize; 3]) -> bool {
    [0, 1, 2]
        .map(|idx| min[idx] - 1 <= entry[idx] && entry[idx] <= max[idx] + 1)
        .into_iter()
        .all(|elem| elem)
}

#[test]
fn part1_tiny_example() {
    let input = "\
    1,1,1\n\
    2,1,1\
    ";
    assert_eq!(part1(input), 10);
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day18.example.txt"
    ));
    assert_eq!(part1(input), 64);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 4628);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day18.example.txt"
    ));
    assert_eq!(part2(input), 58);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 2582);
}
