use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::str::FromStr;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day02.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "2", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "2", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
#[derive(Debug)]
struct Game {
    idx: usize,
    plays: Vec<BallSet>,
}

enum Color {
    Red,
    Blue,
    Green,
}

impl Game {
    fn is_possible(&self, total: &BallSet) -> bool {
        self.plays.iter().all(|set| set.is_possible(total))
    }

    fn min_cubes(&self) -> BallSet {
        self.plays.iter().fold(
            BallSet {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, cur| {
                acc.red = acc.red.max(cur.red);
                acc.blue = acc.blue.max(cur.blue);
                acc.green = acc.green.max(cur.green);
                acc
            },
        )
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").ok_or(())?;
        let (idx, plays) = s.split_once(':').ok_or(())?;

        let plays = plays
            .split(';')
            .map(|set| {
                set.split(',')
                    .map(|colors| {
                        let (count, color) = colors.trim().split_once(' ').unwrap();
                        let color = match color {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => panic!("Invalid Color"),
                        };
                        (count.parse::<usize>().unwrap(), color)
                    })
                    .fold(
                        BallSet {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |mut acc, (count, color)| {
                            match color {
                                Color::Red => acc.red += count,
                                Color::Blue => acc.blue += count,
                                Color::Green => acc.green += count,
                            }
                            acc
                        },
                    )
            })
            .collect();

        Ok(Self {
            idx: idx.parse().map_err(|_| ())?,
            plays,
        })
    }
}

#[derive(Debug)]
struct BallSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl BallSet {
    fn is_possible(&self, total: &BallSet) -> bool {
        self.red <= total.red && self.green <= total.green && self.blue <= total.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(|line| line.parse::<Game>().unwrap())
}

pub fn part1(input: &str) -> usize {
    let total = BallSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    parse_input(input)
        .filter_map(|game| game.is_possible(&total).then_some(game.idx))
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|game| game.min_cubes().power())
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day02.example.txt"
    ));
    assert_eq!(part1(input), 8);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 2265);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day02.example.txt"
    ));
    assert_eq!(part2(input), 48 + 12 + 1560 + 630 + 36);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 64097);
}
