#![allow(clippy::identity_op)]

struct Dimensions(usize, usize, usize);

fn parse_input(input: &str) -> impl Iterator<Item = Dimensions> + '_ {
    input.lines().map(|line| {
        let mut split = line.split('x');
        let l = split.next().unwrap().parse().unwrap();
        let w = split.next().unwrap().parse().unwrap();
        let h = split.next().unwrap().parse().unwrap();
        Dimensions(l, w, h)
    })
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|Dimensions(l, w, h)| {
            let a = l * w;
            let b = l * h;
            let c = w * h;
            2 * a + 2 * b + 2 * c + a.min(b).min(c)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|Dimensions(l, w, h)| {
            let a = 2 * l + 2 * w;
            let b = 2 * l + 2 * h;
            let c = 2 * w + 2 * h;
            a.min(b).min(c) + l * w * h
        })
        .sum()
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day02.example1.txt"
    ));
    assert_eq!(part1(input), 2 * 6 + 2 * 12 + 2 * 8 + 6);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day02.example2.txt"
    ));
    assert_eq!(part1(input), 2 * 1 + 2 * 10 + 2 * 10 + 1);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day02.txt"
    ));
    assert_eq!(part1(input), 1588178);
}

#[test]
fn part2_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day02.example1.txt"
    ));
    assert_eq!(part2(input), 2 + 2 + 3 + 3 + 2 * 3 * 4);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day02.example2.txt"
    ));
    assert_eq!(part2(input), 1 + 1 + 1 + 1 + 1 * 1 * 10);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day02.txt"
    ));
    assert_eq!(part2(input), 3783758);
}
