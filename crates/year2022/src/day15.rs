use std::{collections::HashSet, ops::RangeInclusive};

struct Sensor {
    x: isize,
    y: isize,
}

struct Beacon {
    x: isize,
    y: isize,
}

fn parse(input: &str) -> Vec<(Sensor, Beacon)> {
    input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(": ").unwrap();
            let (sensor_x, sensor_y) = sensor
                .strip_prefix("Sensor at ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            let sensor = Sensor {
                x: sensor_x.strip_prefix("x=").unwrap().parse().unwrap(),
                y: sensor_y.strip_prefix("y=").unwrap().parse().unwrap(),
            };
            let (sensor_x, sensor_y) = beacon
                .strip_prefix("closest beacon is at ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            let beacon = Beacon {
                x: sensor_x.strip_prefix("x=").unwrap().parse().unwrap(),
                y: sensor_y.strip_prefix("y=").unwrap().parse().unwrap(),
            };
            (sensor, beacon)
        })
        .collect()
}

fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as isize
}

fn blocked_at_row(
    sensor: &Sensor,
    beacon: &Beacon,
    y: isize,
    include_beacon: bool,
) -> RangeInclusive<isize> {
    let diff = manhattan_distance((sensor.x, sensor.y), (beacon.x, beacon.y));
    let y_diff = sensor.y.abs_diff(y) as isize;
    let rem_diff = diff - y_diff;

    // if rem is negative we don't contribute blocks to the y line
    // and the range will be empt as x_start will be lager than x_end
    let x_start = sensor.x - rem_diff;
    let x_end = sensor.x + rem_diff;

    if beacon.y == y && !include_beacon {
        if beacon.x == x_start {
            x_start + 1..=x_end
        } else {
            x_start..=x_end - 1
        }
    } else {
        x_start..=x_end
    }
}

fn p1(input: &str, y: isize) -> usize {
    let pairs = parse(input);
    pairs
        .iter()
        .flat_map(|(s, b)| blocked_at_row(s, b, y, false))
        .collect::<HashSet<_>>()
        .len()
}

fn p2(input: &str, max: isize) -> usize {
    let pairs = parse(input);

    for y in 0..=max {
        let mut x = 0;

        let blocked_ranges = pairs
            .iter()
            .map(|(s, b)| blocked_at_row(s, b, y, true))
            .collect::<Vec<_>>();

        while x <= max {
            if let Some(range) = blocked_ranges.iter().find(|range| range.contains(&x)) {
                x = range.end() + 1;
            } else {
                return (x * 4000000 + y) as usize;
            }
        }
    }
    panic!("We didn't find a match!")
}

pub fn part1(input: &str) -> usize {
    p1(input, 2000000)
}

pub fn part2(input: &str) -> usize {
    p2(input, 4000000)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day15.example.txt"
    ));
    assert_eq!(p1(input, 10), 26);
}

#[ignore = "slow"]
#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day15.txt"
    ));
    assert_eq!(part1(input), 5394423);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day15.example.txt"
    ));
    assert_eq!(p2(input, 20), 14 * 4000000 + 11);
}

#[ignore = "slow"]
#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day15.txt"
    ));
    assert_eq!(part2(input), 11840879211051);
}
