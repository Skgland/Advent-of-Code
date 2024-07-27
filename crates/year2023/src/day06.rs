#[derive(Debug)]
struct Race {
    time: u64,
    highscore: u64,
}

impl Race {
    fn winning_holdings(&self) -> usize {
        (1..self.time)
            .filter(|&press| self.highscore < distance(press, self.time))
            .count()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Race> + '_ {
    let mut lines = input.lines();
    let times = lines
        .next()
        .and_then(|line| line.strip_prefix("Time: "))
        .into_iter()
        .flat_map(|line| line.split(' '))
        .flat_map(|elem| elem.parse());
    let distances = lines
        .next()
        .and_then(|line| line.strip_prefix("Distance: "))
        .into_iter()
        .flat_map(|line| line.split(' '))
        .flat_map(|elem| elem.parse());

    times
        .zip(distances)
        .map(|(time, highscore)| Race { time, highscore })
}

pub fn distance(pressed: u64, total: u64) -> u64 {
    // d = t_p * (t_t - t_p)
    pressed * (total - pressed)
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|race| race.winning_holdings())
        .product()
}

pub fn part2(input: &str) -> usize {
    let race = parse_input(input).fold(
        Race {
            time: 0,
            highscore: 0,
        },
        |acc, cur| Race {
            time: acc.time * 10u64.pow(cur.time.ilog10() + 1) + cur.time,
            highscore: acc.highscore * 10u64.pow(cur.highscore.ilog10() + 1) + cur.highscore,
        },
    );
    race.winning_holdings()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day06.example.txt"
    ));
    assert_eq!(part1(input), 4 * 8 * 9);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day06.txt"
    ));
    assert_eq!(part1(input), 771628);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day06.example.txt"
    ));
    assert_eq!(part2(input), 71503);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day06.txt"
    ));
    assert_eq!(part2(input), 27363861);
}
