use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::{cell::Cell, collections::HashMap};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day12.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "12", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "12", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
struct Input {
    heights: HashMap<(isize, isize), u8>,
    start: (isize, isize),
    end: (isize, isize),
}

fn parse(input: &str) -> Input {
    let start = Cell::new((0, 0));
    let end = Cell::new((0, 0));

    // we don't want to move start and end into the move closure below
    // so we prepare a reference outside to be captured by move instead
    // the closure need to be move as it otherwise ties to capture y by reference,
    // which correctly fails the borrow check
    let start_ref = &start;
    let end_ref = &end;
    let heights = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().map(move |(x, height)| {
                let pos = (x as isize, y as isize);
                let height = match height {
                    b'S' => {
                        start_ref.set(pos);
                        println!("Found start at {:?}", start_ref.get());
                        0
                    }
                    b'E' => {
                        end_ref.set(pos);
                        println!("Found end at {:?}", end_ref.get());
                        25
                    }
                    b'a'..=b'z' => height - b'a',
                    _ => panic!(),
                };
                (pos, height)
            })
        })
        .collect();
    Input {
        heights,
        start: start.get(),
        end: end.get(),
    }
}

fn dijkstra(
    start: (isize, isize),
    target: impl Fn((isize, isize), u8) -> bool,
    valid: impl Fn(u8, u8) -> bool,
    heights: HashMap<(isize, isize), u8>,
) -> u32 {
    let mut heights: HashMap<_, _> = heights
        .into_iter()
        .map(|(key, value)| (key, (value, None)))
        .collect();

    heights.get_mut(&start).unwrap().1 = Some((0, (0, 0)));
    let mut current = start;

    loop {
        // remove from hights to mark position as visited
        let (current_height, distance) = heights.remove(&current).unwrap();
        let (current_distance, _predecessor) = distance.unwrap();

        // stop if we reached a valid destination/start
        if target(current, current_height) {
            break current_distance;
        }

        for neighbor_pos in [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|offset| (current.0 + offset.0, current.1 + offset.1))
        {
            if let Some((neightbor_height, distance)) = heights.get_mut(&neighbor_pos) {
                if valid(current_height, *neightbor_height) {
                    *distance = Some(distance.map_or(
                        (current_distance + 1, current),
                        |neighbor_distance| {
                            if neighbor_distance.0 > (current_distance + 1) {
                                (current_distance + 1, current)
                            } else {
                                neighbor_distance
                            }
                        },
                    ))
                }
            }
        }

        let (key, _distance) = heights
            .iter()
            .filter_map(|(key, (_height, distance))| distance.map(|distance| (key, distance)))
            .min_by_key(|(_key, distance)| *distance)
            .unwrap();
        current = *key;
    }
}

pub fn part1(input: &str) -> u32 {
    let input = parse(input);
    // go forward till we reach the end
    dijkstra(
        input.start,
        |current, _| current == input.end,
        |current, next| next <= current + 1,
        input.heights,
    )
}

// part1 and part2 could be made more similar by going backwards in both versions,
// then part1 would use the same valid function as part2
// both would pass input.end as the start of the search
// the target functions would be the only difference
pub fn part1_alt(input: &str) -> u32 {
    let input = parse(input);
    // go backwards till be we reach the start
    dijkstra(
        input.end,
        |current, _| current == input.start,
        |current, next| current <= next + 1,
        input.heights,
    )
}

pub fn part2(input: &str) -> u32 {
    let input = parse(input);
    // go backwards till be reach a tile with height 0 (a)
    dijkstra(
        input.end,
        |_, height| height == 0,
        |current, next| current <= next + 1,
        input.heights,
    )
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day12.example.txt"
    ));
    assert_eq!(part1(input), 31);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 339);
}

#[test]
fn part1_alt_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day12.example.txt"
    ));
    assert_eq!(part1_alt(input), 31);
}

#[test]
fn part1_alt_full() {
    assert_eq!(part1_alt(INPUT), 339);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day12.example.txt"
    ));
    assert_eq!(part2(input), 29);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 332);
}
