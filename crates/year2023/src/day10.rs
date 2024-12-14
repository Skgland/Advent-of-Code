use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day10.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "10", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "10", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Start,
    PipeNS,
    PipeEW,
    PipeNE,
    PipeSE,
    PipeSW,
    PipeNW,
}

impl Tile {
    fn connects(&self, dir: &Dir) -> bool {
        matches!(
            (self, dir),
            (Self::Start, _)
                | (Self::PipeNS, Dir::North | Dir::South)
                | (Self::PipeEW, Dir::East | Dir::West)
                | (Self::PipeNE, Dir::North | Dir::East)
                | (Self::PipeSE, Dir::South | Dir::East)
                | (Self::PipeSW, Dir::South | Dir::West)
                | (Self::PipeNW, Dir::North | Dir::West)
        )
    }

    fn coming_from_going_to(&self, dir: &Dir) -> Option<Dir> {
        match (self, dir) {
            (Self::PipeNS, Dir::North) => Some(Dir::South),
            (Self::PipeNS, Dir::South) => Some(Dir::North),
            (Self::PipeEW, Dir::East) => Some(Dir::West),
            (Self::PipeEW, Dir::West) => Some(Dir::East),
            (Self::PipeNE, Dir::North) => Some(Dir::East),
            (Self::PipeNE, Dir::East) => Some(Dir::North),
            (Self::PipeSE, Dir::South) => Some(Dir::East),
            (Self::PipeSE, Dir::East) => Some(Dir::South),
            (Self::PipeSW, Dir::South) => Some(Dir::West),
            (Self::PipeSW, Dir::West) => Some(Dir::South),
            (Self::PipeNW, Dir::North) => Some(Dir::West),
            (Self::PipeNW, Dir::West) => Some(Dir::North),
            _ => None,
        }
    }

    fn neighbors(&self, pos: &Position, coming_from: &Dir) -> (Vec<Position>, Vec<Position>) {
        match (self, coming_from) {
            (Self::PipeNS, Dir::North) => (vec![pos.go(&Dir::East)], vec![pos.go(&Dir::West)]),
            (Self::PipeNS, Dir::South) => (vec![pos.go(&Dir::West)], vec![pos.go(&Dir::East)]),

            (Self::PipeEW, Dir::East) => (vec![pos.go(&Dir::South)], vec![pos.go(&Dir::North)]),
            (Self::PipeEW, Dir::West) => (vec![pos.go(&Dir::North)], vec![pos.go(&Dir::South)]),

            (Self::PipeNE, Dir::North) => (vec![], vec![pos.go(&Dir::South), pos.go(&Dir::West)]),
            (Self::PipeNE, Dir::East) => (vec![pos.go(&Dir::South), pos.go(&Dir::West)], vec![]),

            (Self::PipeSE, Dir::South) => (vec![pos.go(&Dir::North), pos.go(&Dir::West)], vec![]),
            (Self::PipeSE, Dir::East) => (vec![], vec![pos.go(&Dir::North), pos.go(&Dir::West)]),

            (Self::PipeSW, Dir::South) => (vec![], vec![pos.go(&Dir::North), pos.go(&Dir::East)]),
            (Self::PipeSW, Dir::West) => (vec![pos.go(&Dir::North), pos.go(&Dir::East)], vec![]),

            (Self::PipeNW, Dir::North) => (vec![pos.go(&Dir::South), pos.go(&Dir::East)], vec![]),
            (Self::PipeNW, Dir::West) => (vec![], vec![pos.go(&Dir::South), pos.go(&Dir::East)]),
            _ => (vec![], vec![]),
        }
    }
}
#[derive(Debug, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn inverse(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: isize,
    column: isize,
}

impl Position {
    fn go(&self, direction: &Dir) -> Position {
        match direction {
            Dir::North => Position {
                row: self.row - 1,
                column: self.column,
            },
            Dir::South => Position {
                row: self.row + 1,
                column: self.column,
            },
            Dir::East => Position {
                row: self.row,
                column: self.column + 1,
            },
            Dir::West => Position {
                row: self.row,
                column: self.column - 1,
            },
        }
    }

    fn neighbors(&self) -> Vec<Position> {
        [Dir::North, Dir::East, Dir::South, Dir::West]
            .map(|elem| self.go(&elem))
            .into_iter()
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Maze(Vec<Vec<Tile>>);

impl Maze {
    fn size(&self) -> usize {
        self.0.iter().map(|elem| elem.len()).sum()
    }

    fn find_start(&self) -> Position {
        self.0
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter().enumerate().find_map(|(column_idx, tile)| {
                    matches!(tile, Tile::Start).then_some(Position {
                        row: row_idx as isize,
                        column: column_idx as isize,
                    })
                })
            })
            .unwrap()
    }

    fn get(&self, pos: &Position) -> Option<Tile> {
        let row: usize = pos.row.try_into().ok()?;
        let column: usize = pos.column.try_into().ok()?;
        self.0.get(row).and_then(|row| row.get(column)).cloned()
    }

    fn loop_tiles(&self) -> Vec<Position> {
        let mut pos = self.find_start();
        let mut next = self.start_direction(&pos);

        let mut loop_positions = vec![];

        loop {
            loop_positions.push(pos.clone());

            pos = pos.go(&next);
            let tile = self.get(&pos).unwrap();

            if let Some(tile) = tile.coming_from_going_to(&next.inverse()) {
                next = tile;
            } else {
                break;
            }
        }
        loop_positions
    }

    fn start_direction(&self, pos: &Position) -> Dir {
        [Dir::North, Dir::East, Dir::South]
            .into_iter()
            .find(|elem| {
                self.get(&pos.go(elem))
                    .map_or(false, |tile| tile.connects(&elem.inverse()))
            })
            .unwrap()
    }

    fn contains(&self, pos: &Position) -> bool {
        if let (Ok(row), Ok(column)) = (pos.row.try_into(), pos.column.try_into()) {
            self.0.len() > row && self.0[row].len() > column
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> Maze {
    Maze(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|elem| match elem {
                        'S' => Tile::Start,
                        '|' => Tile::PipeNS,
                        '-' => Tile::PipeEW,
                        'L' => Tile::PipeNE,
                        'J' => Tile::PipeNW,
                        'F' => Tile::PipeSE,
                        '7' => Tile::PipeSW,
                        _ => Tile::Empty,
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn part1(input: &str) -> usize {
    let maze = parse_input(input);

    (maze.loop_tiles().len() + 1) / 2
}

pub fn part2(input: &str) -> usize {
    let maze = parse_input(input);

    let loop_tiles = maze.loop_tiles();

    let mut dir = maze.start_direction(&loop_tiles[0]);

    let mut left = HashSet::new();
    let mut right = HashSet::new();

    for pos in &loop_tiles[1..] {
        let tile = maze.get(pos).unwrap();
        let coming_from = dir.inverse();
        dir = tile.coming_from_going_to(&coming_from).unwrap();
        let (mut ln, mut rn) = tile.neighbors(pos, &coming_from);
        ln.retain(|pos| maze.contains(pos) && !loop_tiles.contains(pos));
        rn.retain(|pos| maze.contains(pos) && !loop_tiles.contains(pos));
        left.extend(ln);
        right.extend(rn);
    }

    let maze_size = maze.size();

    let mut new_left = left.iter().cloned().collect::<VecDeque<_>>();

    while let Some(pos) = new_left.pop_front() {
        let mut neighbors = pos.neighbors();
        neighbors.retain(|pos: &Position| {
            maze.contains(pos)
                && !new_left.contains(pos)
                && !loop_tiles.contains(pos)
                && !left.contains(pos)
        });
        left.extend(neighbors.clone());
        new_left.extend(neighbors)
    }

    let mut new_right = right.iter().cloned().collect::<VecDeque<_>>();

    while let Some(pos) = new_right.pop_front() {
        let mut neighbors = pos.neighbors();
        neighbors.retain(|pos| {
            maze.contains(pos)
                && !new_right.contains(pos)
                && !loop_tiles.contains(pos)
                && !right.contains(pos)
        });
        right.extend(neighbors.clone());
        new_right.extend(neighbors)
    }

    // every tile should be either
    // - part of the loop
    // - left of the loop
    // - right of the loop

    assert_eq!(left.len() + right.len() + loop_tiles.len(), maze_size);

    if left
        .iter()
        .any(|pos| pos.neighbors().iter().any(|pos| !maze.contains(pos)))
    {
        right.len()
    } else if right
        .iter()
        .any(|pos| pos.neighbors().iter().any(|pos| !maze.contains(pos)))
    {
        left.len()
    } else {
        panic!("Ambiguous!")
    }
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day10.example1.txt"
    ));
    assert_eq!(part1(input), 4);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day10.example2.txt"
    ));
    assert_eq!(part1(input), 8);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day10.txt"
    ));
    assert_eq!(part1(input), 6979);
}

#[test]
fn part2_example3() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day10.example3.txt"
    ));
    assert_eq!(part2(input), 4);
}

#[test]
fn part2_example4() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day10.example4.txt"
    ));
    assert_eq!(part2(input), 4);
}

#[test]
fn part2_example5() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day10.example5.txt"
    ));
    assert_eq!(part2(input), 8);
}

#[test]
fn part2_example6() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day10.example6.txt"
    ));
    assert_eq!(part2(input), 10);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day10.txt"
    ));
    assert_eq!(part2(input), 443);
}
