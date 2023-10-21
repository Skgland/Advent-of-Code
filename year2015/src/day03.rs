use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = Direction> + Clone + '_ {
    input.bytes().flat_map(|c| match c {
        b'^' => Some(Direction::Up),
        b'v' => Some(Direction::Down),
        b'<' => Some(Direction::Left),
        b'>' => Some(Direction::Right),
        _ => None,
    })
}

fn walk(path: impl Iterator<Item = Direction>) -> HashSet<(i32, i32)> {
    let (_, state): (_, HashSet<_>) = path.fold(
        ((0, 0), HashSet::from_iter([(0, 0)])),
        |(mut pos, mut past), next| {
            match next {
                Direction::Up => pos.1 += 1,
                Direction::Down => pos.1 -= 1,
                Direction::Left => pos.0 -= 1,
                Direction::Right => pos.0 += 1,
            };
            past.insert(pos);
            (pos, past)
        },
    );
    state
}

pub fn part1(input: &str) -> usize {
    walk(parse_input(input)).len()
}

pub fn part2(input: &str) -> usize {
    let iter = parse_input(input);
    let santa = walk(iter.clone().step_by(2));
    let robot = walk(iter.skip(1).step_by(2));
    santa.union(&robot).count()
}

#[test]
fn part1_example1() {
    let input = ">";
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_example2() {
    let input = "^>v<";
    assert_eq!(part1(input), 4);
}

#[test]
fn part1_example3() {
    let input = "^v^v^v^v^v";
    assert_eq!(part1(input), 2);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day03.txt"));
    assert_eq!(part1(input), 2572);
}

#[test]
fn part2_example1() {
    let input = "^v";
    assert_eq!(part2(input), 3);
}

#[test]
fn part2_example2() {
    let input = "^>v<";
    assert_eq!(part2(input), 3);
}

#[test]
fn part2_example3() {
    let input = "^v^v^v^v^v";
    assert_eq!(part2(input), 11);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day03.txt"));
    assert_eq!(part2(input), 2631);
}
