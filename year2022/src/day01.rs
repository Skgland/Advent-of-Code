fn both(input: &str, top: usize) -> u32 {
    let calories = input.lines().map(|line| line.parse::<u32>());
    let mut current = 0;
    let mut elfs = vec![];

    for val in calories {
        if let Ok(val) = val {
            current += val;
        } else {
            elfs.push(current);
            current = 0;
        }
    }
    elfs.push(current);

    elfs.sort();
    elfs.reverse();

    dbg!(&elfs[..top]).iter().sum()
}

pub fn part1(input: &str) -> u32 {
    both(input, 1)
}

pub fn part2(input: &str) -> u32 {
    both(input, 3)
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day01.example.txt");
    assert_eq!(part1(input), 24000);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day01.txt"));
    assert_eq!(part1(input), 70720);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day01.example.txt");
    assert_eq!(part2(input), 45000);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day01.txt"));
    assert_eq!(part2(input), 207148);
}
