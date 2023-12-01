fn parse_input(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines()
}

pub fn part1(input: &str) -> u32 {
    parse_input(input)
        .map(|elem| {
            let first = elem.chars().find(|c| c.is_numeric()).unwrap();
            let last = elem.chars().rev().find(|c| c.is_numeric()).unwrap();
            let first = char::to_digit(first, 10).unwrap();
            let last = char::to_digit(last, 10).unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    parse_input(input)
        .map(|elem| {
            let (first, last) = find_numbers(elem);
            first * 10 + last
        })
        .sum()
}

fn find_numbers(elem: &str) -> (u32, u32) {
    const PATTERNS: &[(u32, &str)] = &[
        (1, "1"),
        (1, "one"),
        (2, "2"),
        (2, "two"),
        (3, "3"),
        (3, "three"),
        (4, "4"),
        (4, "four"),
        (5, "5"),
        (5, "five"),
        (6, "6"),
        (6, "six"),
        (7, "7"),
        (7, "seven"),
        (8, "8"),
        (8, "eight"),
        (9, "9"),
        (9, "nine"),
    ];
    let first = PATTERNS
        .iter()
        .flat_map(|&(num, pat)| elem.find(pat).map(|idx| (idx, num)))
        .min_by_key(|(idx, _)| *idx)
        .unwrap()
        .1;
    let last = PATTERNS
        .iter()
        .flat_map(|&(num, pat)| elem.rfind(pat).map(|idx| (idx, num)))
        .max_by_key(|(idx, _)| *idx)
        .unwrap()
        .1;
    (first, last)
}

#[test]
fn problem_case() {
    assert_eq!(find_numbers("fpfqp7three7"), (7, 7))
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day01.example1.txt"));
    assert_eq!(part1(input), 12 + 38 + 15 + 77);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day01.txt"));
    assert_eq!(part1(input), 54159);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day01.example2.txt");
    assert_eq!(part2(input), 29 + 83 + 13 + 24 + 42 + 14 + 76);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day01.txt"));
    assert_eq!(part2(input), 53866);
}
