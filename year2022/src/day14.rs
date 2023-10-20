use std::{collections::HashSet, ops::ControlFlow};

struct Input {
    rock: HashSet<(isize, usize)>,
    max_depth: usize,
}

fn parse(input: &str) -> Input {
    let rock: HashSet<_> = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let (x, depth) = coord.split_once(',').unwrap();
                    (x.parse().unwrap(), depth.parse().unwrap())
                })
                .collect::<Vec<(isize, usize)>>()
                .windows(2)
                .flat_map(|window| {
                    let start = window[0];
                    let end = window[1];

                    let min_x = start.0.min(end.0);
                    let max_x = start.0.max(end.0);

                    let min_depth = start.1.min(end.1);
                    let max_depth = start.1.max(end.1);
                    (min_x..=max_x)
                        .flat_map(move |x| (min_depth..=max_depth).map(move |depth| (x, depth)))
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    let max_depth = rock.iter().map(|(_, depth)| *depth).max().unwrap();

    Input { rock, max_depth }
}

fn simulate(input: &mut Input, arg: (isize, usize), floor: bool) -> ControlFlow<u32, u32> {
    use ControlFlow::*;

    if input.rock.contains(&arg) || (arg.1 == input.max_depth + 2) {
        // can't land here as there is rock/floor here
        Continue(0)
    } else if input.max_depth == arg.1 && !floor {
        // we are at the height of the deepest rock,
        // as such no rock is below us (to catch us),
        // therefor we reached the abyss
        Break(0)
    } else {
        let down = simulate(input, (arg.0, arg.1 + 1), floor)?;
        let left = match simulate(input, (arg.0 - 1, arg.1 + 1), floor) {
            Continue(left) => left,
            Break(left) => return Break(down + left),
        };
        let right = match simulate(input, (arg.0 + 1, arg.1 + 1), floor) {
            Continue(right) => right,
            Break(right) => return Break(down + left + right),
        };
        println!("Sand came to rest at {:?}", arg);
        input.rock.insert(arg);
        Continue(1 + left + right + down)
    }
}

pub fn part1(input: &str) -> u32 {
    let mut input = parse(input);

    match simulate(&mut input, (500, 0), false) {
        ControlFlow::Continue(_) => {
            panic!("We filled the cave without the sand reaching the abyss")
        }
        ControlFlow::Break(result) => result,
    }
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse(input);

    match simulate(&mut input, (500, 0), true) {
        ControlFlow::Continue(result) => result,
        ControlFlow::Break(_) => panic!("The floor should have stopped us"),
    }
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day14.example.txt");
    assert_eq!(part1(input), 24);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day14.txt"));
    assert_eq!(part1(input), 832);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day14.example.txt");
    assert_eq!(part2(input), 93);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day14.txt"));
    assert_eq!(part2(input), 27601);
}
