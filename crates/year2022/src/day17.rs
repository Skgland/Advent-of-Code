use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{collections::HashMap, ops::ControlFlow};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day17.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "17", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "17", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

type Position = (u8, u64);
struct Cave(HashMap<u64, u8>);

impl Cave {
    fn new() -> Cave {
        Self(Default::default())
    }

    fn contains(&self, (x_pos, y_pos): Position) -> bool {
        self.0
            .get(&y_pos)
            .is_some_and(|row| (row & (1 << x_pos)) != 0)
    }

    fn insert(&mut self, (x, y): Position) {
        *self.0.entry(y).or_default() |= 1 << x;
    }

    fn copy_range(&mut self, src: u64, target: u64, length: u64) {
        for offset in 0..length {
            if let Some(&entry) = self.0.get(&(src + offset)) {
                self.0.insert(target + offset, entry);
            }
        }
    }

    fn match_ranges(&self, start_a: u64, start_b: u64, length: u64) -> bool {
        (0..length).all(|offset| self.0.get(&(start_a + offset)) == self.0.get(&(start_b + offset)))
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Rock {
    Line,
    Plus,
    Corner,
    Pipe,
    Square,
}

impl Rock {
    const SEQUENCE: [Self; 5] = [
        Self::Line,
        Self::Plus,
        Self::Corner,
        Self::Pipe,
        Self::Square,
    ];

    fn push(self, pos @ (x_pos, y_pos): Position, cave: &Cave, next: Direction) -> Position {
        let new_pos = match next {
            Direction::Left => {
                if x_pos == 0 {
                    return pos;
                }
                (x_pos - 1, y_pos)
            }
            Direction::Right => (x_pos + 1, y_pos),
        };

        if self.has_collision(new_pos, cave) {
            pos
        } else {
            new_pos
        }
    }

    fn fall(self, (x_pos, y_pos): Position, cave: &Cave) -> ControlFlow<(), Position> {
        if y_pos == 0 {
            ControlFlow::Break(())
        } else {
            let new_pos = (x_pos, y_pos - 1);
            if self.has_collision(new_pos, cave) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(new_pos)
            }
        }
    }

    fn offsets(self) -> &'static [Position] {
        match self {
            Rock::Line => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Rock::Corner => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Pipe => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Square => &[(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    fn occupies(self, (x_pos, y_pos): Position) -> impl Iterator<Item = Position> {
        self.offsets()
            .iter()
            .map(move |(x_offset, y_offset)| (x_pos + x_offset, y_pos + y_offset))
    }

    fn has_collision(&self, pos: Position, cave: &Cave) -> bool {
        self.occupies(pos)
            .any(|pos @ (x_pos, _y_pos)| x_pos > 6 || cave.contains(pos))
    }

    fn simulate(
        self: Rock,
        max: &mut u64,
        cave: &mut Cave,
        streams: &mut impl Iterator<Item = (usize, Direction)>,
    ) -> u8 {
        let mut pos = (2, *max + 3);
        loop {
            if log::log_enabled!(log::Level::Debug) {
                print_cave(cave, *max, Some((self, pos)));
            }

            pos = self.push(pos, &*cave, streams.next().unwrap().1);
            pos = match self.fall(pos, &*cave) {
                ControlFlow::Continue(pos) => pos,
                ControlFlow::Break(()) => {
                    for (x, y) in self.occupies(pos) {
                        cave.insert((x, y));
                        *max = (*max).max(y + 1)
                    }
                    break pos.0;
                }
            };
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            _ => None,
        })
        .collect()
}

fn print_cave(cave: &Cave, max: u64, rock_pos: Option<(Rock, Position)>) {
    let max = if let Some((rock, pos)) = rock_pos {
        max.max(
            rock.occupies(pos)
                .map(|(_x, y)| y)
                .max()
                .unwrap_or_default(),
        )
    } else {
        max
    };

    for y in (0..=max).rev() {
        print!("|");
        for x in 0..7 {
            let symbol = if cave.contains((x, y)) {
                '#'
            } else if rock_pos
                .is_some_and(|(rock, pos)| rock.occupies(pos).any(|pos| pos == (x, y)))
            {
                '@'
            } else {
                '.'
            };
            print!("{}", symbol);
        }
        println!("|");
    }
    println!("+-------+\n")
}

fn both(input: &str, iterations: usize) -> u64 {
    let mut current_max = 0;
    let streams = parse(input);

    let mut streams = streams.into_iter().enumerate().cycle().peekable();
    let mut cave = Cave::new();

    let mut state: HashMap<_, Vec<_>> = HashMap::new();

    let mut idx = 0;
    let mut search_period = true;

    while idx < iterations {
        let rock = Rock::SEQUENCE[idx % Rock::SEQUENCE.len()];
        let x = rock.simulate(&mut current_max, &mut cave, &mut streams);

        if search_period {
            let streams_idx = streams.peek().unwrap().0;

            let state_key = (rock, x, streams_idx);
            let val = (idx, current_max);

            state
                .entry(state_key)
                .and_modify(|old| {
                    log::debug!("Updating entry {:?} to include {:?}", state_key, val);

                    for &(old_idx, entry_max) in &*old {
                        let diff = current_max - entry_max;
                        if entry_max >= diff {
                            let entry_base = entry_max - diff;
                            let max_base = current_max - diff;
                            if cave.match_ranges(entry_base, max_base, diff) {
                                // we only use the first repetition
                                search_period = false;
                                let period = idx - old_idx;
                                println!(
                                    "Repetition from {} to {}, with height {} and period {}",
                                    entry_max, current_max, diff, period
                                );

                                let remaining = iterations - idx;
                                let repetitions = remaining / period;

                                println!("Skipping {} occurrences of the repetition", repetitions);

                                // skip forward, till we can't fit the repetition anymore into the remaining steps
                                current_max += repetitions as u64 * diff;
                                idx += repetitions * period;

                                // copy the repetition pattern to the end of the skip so that we can resume the simulation steps
                                let new_base = current_max - diff;
                                cave.copy_range(max_base, new_base, diff);
                            }
                        }
                    }
                    old.push(val);
                })
                .or_insert_with(|| {
                    log::debug!("Adding entry {:?} with {:?}", state_key, val);
                    vec![val]
                });
        }

        idx += 1;
    }

    current_max
}

pub fn part1(input: &str) -> u64 {
    both(input, 2022)
}

pub fn part2(input: &str) -> u64 {
    both(input, 1_000_000_000_000)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day17.example.txt"
    ));
    assert_eq!(part1(input), 3068);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 3085);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day17.example.txt"
    ));
    assert_eq!(part2(input), 1514285714288);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1535483870924);
}
