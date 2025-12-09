use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2025/day09.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2025/day09.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2025", "09", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2025", "09", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> impl Iterator<Item = [u32; 2]> + '_ {
    input.lines().map(|line| {
        let mut elements = line.split(',').map(|val| val.parse().unwrap());
        std::array::from_fn::<_, 2, _>(|_| elements.next().unwrap())
    })
}

pub fn part1(input: &str) -> u64 {
    let tiles = parse_input(input).collect::<Vec<_>>();

    let mut max = 0;

    for tile1 in tiles.iter().copied() {
        for tile2 in tiles.iter().copied() {
            let area = tile1
                .into_iter()
                .zip(tile2.into_iter())
                .map(|(a, b)| (a.abs_diff(b) + 1) as u64)
                .product();
            max = max.max(area)
        }
    }
    max
}

pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 50);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 4763509452);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 5);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1262);
}
