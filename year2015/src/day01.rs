enum Direction {
    Up,
    Down,
}

fn parse_input(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input.bytes().flat_map(|c| match c {
        b'(' => Some(Direction::Up),
        b')' => Some(Direction::Down),
        _ => None
    })
}

pub fn part1(input: &str) -> i32 {
    parse_input(input).map(|elem| match elem {
        Direction::Up => 1,
        Direction::Down => -1,
    }).sum()
}

pub fn part2(input: &str) -> usize {
    let mut acc = 0;
    for (idx, dir ) in     parse_input(input).map(|elem| match elem {
        Direction::Up => 1,
        Direction::Down => -1,
    }).enumerate() {
        acc += dir;
        if acc < 0 {
            return idx+1
        }
    }
    0
}

#[test]
fn part1_example1a() {
    let input = include_str!(concat!("../input/day01.example1a.txt"));
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_example1b() {
    let input = include_str!(concat!("../input/day01.example1b.txt"));
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_example2a() {
    let input = include_str!(concat!("../input/day01.example2a.txt"));
    assert_eq!(part1(input), 3);
}

#[test]
fn part1_example2b() {
    let input = include_str!(concat!("../input/day01.example2b.txt"));
    assert_eq!(part1(input), 3);
}

#[test]
fn part1_example3() {
    let input = include_str!(concat!("../input/day01.example3.txt"));
    assert_eq!(part1(input), 3);
}

#[test]
fn part1_example4a() {
    let input = include_str!(concat!("../input/day01.example4a.txt"));
    assert_eq!(part1(input), -1);
}

#[test]
fn part1_example4b() {
    let input = include_str!(concat!("../input/day01.example4b.txt"));
    assert_eq!(part1(input), -1);
}

#[test]
fn part1_example5a() {
    let input = include_str!(concat!("../input/day01.example5a.txt"));
    assert_eq!(part1(input), -3);
}

#[test]
fn part1_example5b() {
    let input = include_str!(concat!("../input/day01.example5b.txt"));
    assert_eq!(part1(input), -3);
}


#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day01.txt"));
    assert_eq!(part1(input), 232);
}

#[test]
fn part2_example1a() {
    let input = include_str!("../input/day01.example1a.txt");
    assert_eq!(part2(input), 0);
}

#[test]
fn part2_example1b() {
    let input = include_str!("../input/day01.example1b.txt");
    assert_eq!(part2(input), 0);
}

#[test]
fn part2_example2a() {
    let input = include_str!("../input/day01.example2a.txt");
    assert_eq!(part2(input), 0);
}

#[test]
fn part2_example2b() {
    let input = include_str!("../input/day01.example2b.txt");
    assert_eq!(part2(input), 0);
}

#[test]
fn part2_example3() {
    let input = include_str!("../input/day01.example3.txt");
    assert_eq!(part2(input), 1);
}

#[test]
fn part2_example4a() {
    let input = include_str!("../input/day01.example4a.txt");
    assert_eq!(part2(input), 3);
}

#[test]
fn part2_example4b() {
    let input = include_str!("../input/day01.example4b.txt");
    assert_eq!(part2(input), 1);
}

#[test]
fn part2_example5a() {
    let input = include_str!("../input/day01.example5a.txt");
    assert_eq!(part2(input), 1);
}

#[test]
fn part2_example5b() {
    let input = include_str!("../input/day01.example5b.txt");
    assert_eq!(part2(input), 1);
}

#[test]
fn part2_example6() {
    let input = include_str!("../input/day01.example6.txt");
    assert_eq!(part2(input), 1);
}

#[test]
fn part2_example7() {
    let input = include_str!("../input/day01.example7.txt");
    assert_eq!(part2(input), 5);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day01.txt"));
    assert_eq!(part2(input), 1783);
}
