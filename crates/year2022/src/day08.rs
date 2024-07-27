fn is_visible(input: &[Vec<u8>], x: usize, y: usize) -> bool {
    let height = input[x][y];

    input[0..x].iter().all(|row| row[y] < height)
        || input[x + 1..].iter().all(|row| row[y] < height)
        || input[x][0..y].iter().all(|&elem| elem < height)
        || input[x][y + 1..].iter().all(|&elem| elem < height)
}

fn scenic_score(input: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut lxs = 0;
    let mut rxs = 0;
    let mut lys = 0;
    let mut rys = 0;
    let height = input[x][y];

    for x in (0..x).rev() {
        lxs += 1;
        if input[x][y] >= height {
            break;
        }
    }
    for column in &input[x + 1..input.len()] {
        rxs += 1;
        if column[y] >= height {
            break;
        }
    }
    for y in (0..y).rev() {
        lys += 1;
        if input[x][y] >= height {
            break;
        }
    }
    for y in y + 1..input[0].len() {
        rys += 1;
        if input[x][y] >= height {
            break;
        }
    }

    lxs * rxs * lys * rys
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|elem| elem - b'0').collect())
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let input = parse(input);
    let mut count = 0;
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if is_visible(&input, x, y) {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &str) -> usize {
    let input = parse(input);

    let mut max_scenic = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            let score = scenic_score(&input, x, y);
            max_scenic = max_scenic.max(score);
        }
    }
    max_scenic
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day08.example.txt"
    ));
    assert_eq!(part1(input), 21);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day08.txt"
    ));
    assert_eq!(part1(input), 1782);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day08.example.txt"
    ));
    assert_eq!(part2(input), 8);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day08.txt"
    ));
    assert_eq!(part2(input), 474606);
}
