use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Instruction {
    dir: Direction,
    dist: u32,
    alt_dir: Direction,
    alt_dist: u32,
}

impl Instruction {
    fn switch_to_alt(self) -> Self {
        Self {
            dir: self.alt_dir,
            dist: self.alt_dist,
            alt_dir: self.dir,
            alt_dist: self.dist,
        }
    }
}

fn calc_alt(color: &str) -> (u32, Direction) {
    let alt_dist = u32::from_str_radix(&color[0..5], 16).unwrap();
    let alt_dir = match &color[5..6] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!(),
    };
    (alt_dist, alt_dir)
}

#[test]
fn test_alt() {
    let examples = [
        ("70c710", (461937, Direction::Right)),
        ("0dc571", (56407, Direction::Down)),
        ("5713f0", (356671, Direction::Right)),
        ("d2c081", (863240, Direction::Down)),
        ("59c680", (367720, Direction::Right)),
        ("411b91", (266681, Direction::Down)),
        ("8ceee2", (577262, Direction::Left)),
        ("caa173", (829975, Direction::Up)),
        ("1b58a2", (112010, Direction::Left)),
        ("caa171", (829975, Direction::Down)),
        ("7807d2", (491645, Direction::Left)),
        ("a77fa3", (686074, Direction::Up)),
        ("015232", (5411, Direction::Left)),
        ("7a21e3", (500254, Direction::Up)),
    ];

    for (color, result) in examples {
        assert_eq!(calc_alt(color), result);
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        let mut parts = line.split(' ');
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap();
        let color = parts.next().unwrap();
        let dir = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let dist = u32::from_str_radix(dist, 10).unwrap();
        let color = color.trim_start_matches("(#").trim_end_matches(')');
        let (alt_dist, alt_dir) = calc_alt(color);
        Instruction {
            dir,
            dist,
            alt_dir,
            alt_dist,
        }
    })
}

pub fn part1(input: &str) -> u128 {
    let instruction = parse_input(input);
    let border = determine_border(instruction);
    determine_hole_size(&border)
}

pub fn part2(input: &str) -> u128 {
    let instruction = parse_input(input);
    let border = determine_border(instruction.map(|inst| inst.switch_to_alt()));
    determine_hole_size(&border)
}

#[derive(Debug)]
struct Border {
    flip_parity: bool,
    length: u32,
}

fn determine_hole_size(border: &BTreeMap<i64, BTreeMap<i64, Border>>) -> u128 {
    #[derive(Debug)]
    enum State {
        Inside(i64),
        Outside,
    }

    let mut count: u128 = 0;

    for parts in border.values() {
        let mut state = State::Outside;
        for (cur_column, part) in parts {
            count += part.length as u128;
            match &state {
                State::Inside(start) => {
                    count += (cur_column - start) as u128;
                    if part.flip_parity {
                        state = State::Outside;
                    } else {
                        state = State::Inside(cur_column + part.length as i64);
                    }
                }
                State::Outside => {
                    if part.flip_parity {
                        state = State::Inside(cur_column + part.length as i64);
                    }
                }
            }
        }
    }

    count
}

fn determine_border(
    iter: impl Iterator<Item = Instruction>,
) -> BTreeMap<i64, BTreeMap<i64, Border>> {
    let mut pos = (0, 0);
    let mut border: BTreeMap<i64, BTreeMap<i64, Border>> = BTreeMap::<_, BTreeMap<_, _>>::new();

    let mut instructions: Vec<_> = iter.collect();

    let last = instructions.last().cloned().unwrap();
    let first = instructions.first().cloned().unwrap();

    instructions.insert(0, last);
    instructions.push(first);

    for window in instructions.windows(3) {
        let [prev, inst, next] = window else {
            panic!("Invalid window size");
        };

        match inst.dir {
            Direction::Up => {
                pos.0 -= inst.dist as i64;
                for offset in 1..inst.dist as i64 {
                    border.entry(pos.0 + offset).or_default().insert(
                        pos.1,
                        Border {
                            flip_parity: true,
                            length: 1,
                        },
                    );
                }
            }
            Direction::Down => {
                for offset in 1..inst.dist as i64 {
                    border.entry(pos.0 + offset).or_default().insert(
                        pos.1,
                        Border {
                            flip_parity: true,
                            length: 1,
                        },
                    );
                }
                pos.0 += inst.dist as i64;
            }
            Direction::Left => {
                pos.1 -= inst.dist as i64;
                border.entry(pos.0).or_default().insert(
                    pos.1,
                    Border {
                        flip_parity: prev.dir == next.dir,
                        length: inst.dist + 1,
                    },
                );
            }
            Direction::Right => {
                border.entry(pos.0).or_default().insert(
                    pos.1,
                    Border {
                        flip_parity: prev.dir == next.dir,
                        length: inst.dist + 1,
                    },
                );
                pos.1 += inst.dist as i64;
            }
        }
    }

    border
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day18.example.txt"
    ));
    assert_eq!(part1(input), 62);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day18.txt"
    ));
    assert_eq!(part1(input), 34329);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day18.example.txt"
    ));
    assert_eq!(part2(input), 952408144115);
}

#[test]
#[ignore = "too slow"]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day18.txt"
    ));
    assert_eq!(part2(input), 42617947302920);
}
