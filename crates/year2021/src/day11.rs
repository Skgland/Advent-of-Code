use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day11.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "11", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "11", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

fn parse_input(input: &str) -> [[u8; 10]; 10] {
    let mut lines = input.lines();
    [(); 10].map(|_| {
        let mut line = lines.next().unwrap().trim().chars();
        [(); 10].map(|_| line.next().unwrap() as u8 - b'0')
    })
}

pub fn neighbours(x: isize, y: isize, map: &[[u8; 10]; 10]) -> Vec<(usize, usize)> {
    let mut elems = vec![];

    for x_off in -1..=1 {
        for y_off in -1..=1 {
            let x_idx = x + x_off;
            let y_idx = y + y_off;
            if (x, y) != (x + x_off, y + y_off)
                && (0..map.len() as isize).contains(&x_idx)
                && (0..map[x_idx as usize].len() as isize).contains(&y_idx)
            {
                elems.push((x_idx as usize, y_idx as usize));
            }
        }
    }

    elems
}

fn iterate(input: &mut [[u8; 10]; 10]) -> usize {
    input
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|elem| *elem += 1);

    let mut to_flash = Vec::with_capacity(100);
    let mut flashed = HashSet::with_capacity(100);

    let ready = input.iter().enumerate().flat_map(|(x, row)| {
        row.iter()
            .enumerate()
            .filter(|&(_, &v)| v > 9)
            .map(move |(y, _)| (x, y))
    });

    to_flash.extend(ready);

    while let Some(elem) = to_flash.pop() {
        if flashed.insert(elem) {
            let mut neighbours = neighbours(elem.0 as isize, elem.1 as isize, input);
            neighbours.iter().for_each(|&(x, y)| input[x][y] += 1);
            neighbours.retain(|elem| input[elem.0][elem.1] > 9);
            to_flash.extend(neighbours)
        }
    }

    input
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .filter(|elem| **elem > 9)
        .for_each(|elem| *elem = 0);

    flashed.len()
}

pub fn part1(input: &str) -> usize {
    let mut input = parse_input(input);
    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += iterate(&mut input);
    }
    flash_count
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse_input(input);
    let mut iteration_count = 0;

    loop {
        iteration_count += 1;
        if iterate(&mut input) == 100 {
            return iteration_count;
        }
    }
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day11.example.txt"
    ));
    assert_eq!(part1(input), 1656);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1640);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day11.example.txt"
    ));
    assert_eq!(part2(input), 195);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 312);
}
