use std::collections::{HashMap, HashSet};

enum Obstacle {
    VerticalSplitter,
    HorizontalSplitter,
    NwSeMirror,
    NeSwMirror,
}
impl Obstacle {
    fn handle_beam(&self, beam: &Beam) -> Vec<Beam> {
        match (self, beam.heading) {
            (Obstacle::VerticalSplitter, Heading::Up | Heading::Down)
            | (Obstacle::HorizontalSplitter, Heading::Left | Heading::Right) => {
                beam.go().into_iter().collect()
            }
            (Obstacle::VerticalSplitter, Heading::Right | Heading::Left) => {
                let mut up = beam.clone();
                up.heading = Heading::Up;
                let mut down = beam.clone();
                down.heading = Heading::Down;
                [up.go(), down.go()].into_iter().flatten().collect()
            }
            (Obstacle::HorizontalSplitter, Heading::Down | Heading::Up) => {
                let mut left = beam.clone();
                left.heading = Heading::Left;
                let mut right = beam.clone();
                right.heading = Heading::Right;
                [left.go(), right.go()].into_iter().flatten().collect()
            }
            (Obstacle::NwSeMirror, Heading::Down | Heading::Up)
            | (Obstacle::NeSwMirror, Heading::Right | Heading::Left) => {
                beam.rotate_clockwise().go().into_iter().collect()
            }
            (Obstacle::NwSeMirror, Heading::Right | Heading::Left)
            | (Obstacle::NeSwMirror, Heading::Down | Heading::Up) => {
                beam.rotate_counter_clockwise().go().into_iter().collect()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    Right,
    Left,
    Down,
    Up,
}

struct MirrorMaze {
    obstacles: HashMap<(usize, usize), Obstacle>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    row: usize,
    column: usize,
    heading: Heading,
}

impl Beam {
    fn go(&self) -> Option<Beam> {
        match self.heading {
            Heading::Right => Some(Beam {
                row: self.row,
                column: self.column.checked_add(1)?,
                heading: self.heading,
            }),
            Heading::Left => Some(Beam {
                row: self.row,
                column: self.column.checked_sub(1)?,
                heading: self.heading,
            }),
            Heading::Down => Some(Beam {
                row: self.row.checked_add(1)?,
                column: self.column,
                heading: self.heading,
            }),
            Heading::Up => Some(Beam {
                row: self.row.checked_sub(1)?,
                column: self.column,
                heading: self.heading,
            }),
        }
    }

    fn rotate_clockwise(&self) -> Beam {
        let heading = match self.heading {
            Heading::Right => Heading::Up,
            Heading::Left => Heading::Down,
            Heading::Down => Heading::Right,
            Heading::Up => Heading::Left,
        };
        Beam {
            row: self.row,
            column: self.column,
            heading,
        }
    }

    fn rotate_counter_clockwise(&self) -> Beam {
        self.rotate_clockwise()
            .rotate_clockwise()
            .rotate_clockwise()
    }
}

impl MirrorMaze {
    fn handle_beam(&self, beam: &Beam) -> Vec<Beam> {
        if let Some(obstacle) = self.obstacles.get(&(beam.row, beam.column)) {
            let mut beams = obstacle.handle_beam(beam);
            beams.retain(|beam| self.is_in_bounds(beam));
            beams
        } else {
            beam.go()
                .filter(|beam| self.is_in_bounds(beam))
                .into_iter()
                .collect()
        }
    }

    fn is_in_bounds(&self, beam: &Beam) -> bool {
        beam.column < self.width && beam.row < self.height
    }

    fn shot_beam(&self, beam: Beam) -> usize {
        let mut cache = HashSet::new();
        cache.insert(beam.clone());

        let mut beams = vec![beam];

        while !beams.is_empty() {
            beams = beams
                .into_iter()
                .flat_map(|beam| self.handle_beam(&beam))
                .filter(|beam| cache.insert(beam.clone()))
                .collect();
        }

        cache
            .into_iter()
            .map(|beam| (beam.row, beam.column))
            .collect::<HashSet<_>>()
            .len()
    }

    fn starts(&self) -> impl Iterator<Item = Beam> + '_ {
        (0..self.width)
            .flat_map(|column| {
                [
                    Beam {
                        row: 0,
                        column,
                        heading: Heading::Down,
                    },
                    Beam {
                        row: self.height - 1,
                        column,
                        heading: Heading::Up,
                    },
                ]
            })
            .chain((0..self.height).flat_map(|row| {
                [
                    Beam {
                        row,
                        column: 0,
                        heading: Heading::Right,
                    },
                    Beam {
                        row,
                        column: self.width - 1,
                        heading: Heading::Right,
                    },
                ]
            }))
    }
}

fn parse_input(input: &str) -> MirrorMaze {
    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column, sym)| match sym {
                    '|' => Some(((row, column), Obstacle::VerticalSplitter)),
                    '-' => Some(((row, column), Obstacle::HorizontalSplitter)),
                    '\\' => Some(((row, column), Obstacle::NwSeMirror)),
                    '/' => Some(((row, column), Obstacle::NeSwMirror)),
                    _ => None,
                })
        })
        .collect();

    MirrorMaze {
        obstacles,
        height: input.lines().count(),
        width: input
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0),
    }
}

pub fn part1(input: &str) -> usize {
    let maze = parse_input(input);

    maze.shot_beam(Beam {
        row: 0,
        column: 0,
        heading: Heading::Right,
    })
}

pub fn part2(input: &str) -> usize {
    let maze = parse_input(input);
    maze.starts()
        .map(|beam| maze.shot_beam(beam))
        .max()
        .unwrap()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day16.example.txt"));
    assert_eq!(part1(input), 46);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day16.txt"));
    assert_eq!(part1(input), 6816);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day16.example.txt");
    assert_eq!(part2(input), 51);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day16.txt"));
    assert_eq!(part2(input), 8163);
}
