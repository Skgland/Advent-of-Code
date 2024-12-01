fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    todo!("parse_input WIP");
    std::iter::empty()
}

pub fn part1(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part1 WIP")
}

pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);
    todo!("part2 WIP")
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day25.example.txt"
    ));
    assert_eq!(part1(input), 9 * 6);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day25.txt"
    ));
    assert_eq!(part1(input), 1292);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day25.example.txt"
    ));
    assert_eq!(part2(input), 5);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day25.txt"
    ));
    assert_eq!(part2(input), 1262);
}
