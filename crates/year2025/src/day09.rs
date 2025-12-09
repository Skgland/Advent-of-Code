use std::collections::BTreeSet;

use helper::{InPoligon, TASKS, Task, point_in_polygon};
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

fn parse_input(input: &str) -> impl Iterator<Item = [i32; 2]> + '_ {
    input.lines().map(|line| {
        let mut elements = line.split(',').map(|val| val.parse().unwrap());
        std::array::from_fn::<_, 2, _>(|_| elements.next().unwrap())
    })
}

fn calculate_area(tile1: [i32; 2], tile2: [i32; 2]) -> u64 {
    tile1
        .into_iter()
        .zip(tile2.into_iter())
        .map(|(a, b)| (a.abs_diff(b) + 1) as u64)
        .product()
}

// all edges are axis aligned
pub fn point_in_axis_aligned_polygon(point: [i32; 2], polygon: &[[i32; 2]]) -> bool {
    // use a ray from the point up the y-Axis
    let mut inside = false;

    let last = [polygon[0], polygon[polygon.len() - 1]];
    for edge in polygon.windows(2).chain(std::iter::once(last.as_slice())) {
        let vertex_a = edge[0];
        let vertex_b = edge[1];

        // same x coordinate, so parallel to ray, skip
        if vertex_a[0] == vertex_b[0] {
            continue;
        }

        // below or at the point
        if vertex_a[1] <= point[1] {
            continue;
        }

        let min_x = vertex_a[0].min(vertex_b[0]);
        let max_x = vertex_a[0].max(vertex_b[0]);

        if (min_x..=max_x).contains(&point[0]) {
            inside ^= true;
        }
    }
    inside
}

pub fn part1(input: &str) -> u64 {
    let tiles = parse_input(input).collect::<Vec<_>>();

    let mut max = 0;

    for tile_a in tiles.iter().copied() {
        for tile_b in tiles.iter().copied() {
            if tile_a < tile_b {
                let area = calculate_area(tile_a, tile_b);
                max = max.max(area)
            }
        }
    }
    max
}

pub fn part2(input: &str) -> u64 {
    let tiles = parse_input(input).collect::<Vec<_>>();

    let mut pairs = vec![];

    for tile_a in tiles.iter().copied() {
        for tile_b in tiles.iter().copied() {
            if tile_a < tile_b {
                pairs.push((tile_a, tile_b))
            }
        }
    }

    // sort by area
    pairs.sort_by_cached_key(|&(tile1, tile2)| calculate_area(tile1, tile2));
    // largest first
    pairs.reverse();

    let mut bad_points = BTreeSet::<[_; 2]>::new();

    'pairs: for pair in pairs {
        let min_x = pair.0[0].min(pair.1[0]);
        let max_x = pair.0[0].max(pair.1[0]);
        let min_y = pair.0[1].min(pair.1[1]);
        let max_y = pair.0[1].max(pair.1[1]);

        if bad_points
            .iter()
            .any(|[x, y]| (min_x..=max_x).contains(x) && (min_y..=max_y).contains(y))
        {
            continue 'pairs;
        }

        // check edges first
        for x in min_x..=max_x {
            for y in [min_y, max_y] {
                if matches!(point_in_polygon([x, y], &tiles), InPoligon::Out) {
                    bad_points.insert([x, y]);
                    continue 'pairs;
                }
            }
        }

        for y in min_y..=max_y {
            for x in [min_x, max_x] {
                if matches!(point_in_polygon([x, y], &tiles), InPoligon::Out) {
                    bad_points.insert([x, y]);
                    continue 'pairs;
                }
            }
        }

        /*
        // assuming we don't have holes checking the border is sufficent

        for x in min_x + 1..max_x {
            for y in min_y + 1..max_y {
                if matches!(point_in_polygon([x, y], &tiles), InPoligon::Out) {
                    bad_points.insert([x, y]);
                    continue 'pairs;
                }
            }
        }
         */

        return calculate_area(pair.0, pair.1);
    }

    unreachable!("No squares possible")
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
    assert_eq!(part2(INPUT_EXAMPLE1), 24);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1516897893);
}
