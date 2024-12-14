use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day23.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "23", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "23", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn offset(&self, dir: &Direction) -> Position {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

fn parse(input: &str) -> HashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().flat_map(move |(x, byte)| {
                (byte == b'#').then_some(Position {
                    x: x as _,
                    y: y as _,
                })
            })
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct State {
    elfs: HashSet<Position>,
    check_order: VecDeque<Direction>,
}

impl State {
    fn round(&mut self) -> bool {
        // map of proposed position => original position
        let mut new_elfs = HashMap::<Position, Vec<Position>>::new();

        // step one, collect proposals
        for elf in &self.elfs {
            let mut blocked = [false; 4];

            for offset in -1..=1 {
                if self.elfs.contains(&Position {
                    x: elf.x + 1,
                    y: elf.y + offset,
                }) {
                    blocked[Direction::East as usize] = true;
                }

                if self.elfs.contains(&Position {
                    x: elf.x - 1,
                    y: elf.y + offset,
                }) {
                    blocked[Direction::West as usize] = true;
                }

                if self.elfs.contains(&Position {
                    x: elf.x + offset,
                    y: elf.y + 1,
                }) {
                    blocked[Direction::South as usize] = true;
                }

                if self.elfs.contains(&Position {
                    x: elf.x + offset,
                    y: elf.y - 1,
                }) {
                    blocked[Direction::North as usize] = true;
                }
            }

            let (all, none) = blocked
                .into_iter()
                .fold((true, true), |(all, none), b| (all && b, none && !b));
            if all || none {
                new_elfs.insert(*elf, vec![*elf]);
            } else {
                let dir = self
                    .check_order
                    .iter()
                    .find(|&dir| !blocked[*dir as usize])
                    .unwrap();
                new_elfs.entry(elf.offset(dir)).or_default().push(*elf);
            }
        }

        // step two, move elfs simultaneously
        let mut moved = false;
        self.elfs = new_elfs
            .into_iter()
            .flat_map(|(key, val)| match val.as_slice() {
                [val] => {
                    // only a single elf wanted to move here keep him
                    if key != *val {
                        moved = true;
                    }
                    vec![key]
                }
                // multiple elfs wanted to move here keep their original positions
                _ => val,
            })
            .collect();

        // step three, move first to end
        self.check_order.rotate_left(1);
        moved
    }

    fn size(&self) -> (isize, isize, usize, usize) {
        let elf = *self.elfs.iter().next().unwrap();
        let (min_x, max_x, min_y, max_y) = self.elfs.iter().fold(
            (elf.x, elf.x, elf.y, elf.y),
            |(min_x, max_x, min_y, max_y), pos| {
                (
                    pos.x.min(min_x),
                    pos.x.max(max_x),
                    pos.y.min(min_y),
                    pos.y.max(max_y),
                )
            },
        );
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        (min_x, min_y, width, height)
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (x, y, width, height) = self.size();

        for y in y..y + height as isize {
            for x in x..x + width as isize {
                if self.elfs.contains(&Position { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("\n");
        }
    }
}

pub fn part1(input: &str) -> usize {
    let elfs = parse(input);
    let check_order = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut state = State { elfs, check_order };

    for _ in 0..10 {
        state.round();
    }

    let (_, _, width, height) = state.size();
    (width * height) - state.elfs.len()
}

pub fn part2(input: &str) -> usize {
    let elfs = parse(input);
    let check_order = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut state = State { elfs, check_order };

    for round in 0usize.. {
        if !state.round() {
            return round + 1;
        }
    }
    panic!("Didn't stop moving!")
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day23.example.txt"
    ));
    assert_eq!(part1(input), 110);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 4068);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day23.example.txt"
    ));
    assert_eq!(part2(input), 20);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 968);
}
