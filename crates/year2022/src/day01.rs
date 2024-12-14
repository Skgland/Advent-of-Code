use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day01.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "1", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "1", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn both(input: &str, top: usize) -> u32 {
    let calories = input.lines().map(|line| line.parse::<u32>());
    let mut current = 0;
    let mut elfs = vec![];

    for val in calories {
        if let Ok(val) = val {
            current += val;
        } else {
            elfs.push(current);
            current = 0;
        }
    }
    elfs.push(current);

    elfs.sort();
    elfs.reverse();

    elfs[..top].iter().sum()
}

pub fn part1(input: &str) -> u32 {
    both(input, 1)
}

pub fn part2(input: &str) -> u32 {
    both(input, 3)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day01.example.txt"
    ));
    assert_eq!(part1(input), 24000);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 70720);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day01.example.txt"
    ));
    assert_eq!(part2(input), 45000);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 207148);
}
