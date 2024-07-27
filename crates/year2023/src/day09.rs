fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input
        .lines()
        .map(|elem| elem.split(' ').map(|elem| elem.parse().unwrap()).collect())
}

fn next_element(seq: &[i32]) -> i32 {
    let mut row: Vec<i32>;
    let mut prev_row = seq;
    let mut end_stack = vec![];

    loop {
        end_stack.push(*prev_row.last().unwrap());
        row = prev_row
            .windows(2)
            .map(|items| items[1] - items[0])
            .collect::<Vec<_>>();
        prev_row = &row.as_slice();

        if prev_row.iter().all(|&elem| elem == 0) {
            break;
        }
    }

    end_stack.into_iter().sum()
}

pub fn part1(input: &str) -> i32 {
    parse_input(input).map(|list| next_element(&list)).sum()
}

pub fn part2(input: &str) -> i32 {
    parse_input(input)
        .map(|mut list| {
            list.reverse();
            next_element(&list)
        })
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day09.example.txt"
    ));
    assert_eq!(part1(input), 114);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day09.txt"
    ));
    assert_eq!(part1(input), 2105961943);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day09.example.txt"
    ));
    assert_eq!(part2(input), 2);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day09.txt"
    ));
    assert_eq!(part2(input), 1019);
}
