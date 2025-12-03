use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    ops::ControlFlow,
};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day06.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "6", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "6", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum Tile {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

type Pos = (isize, isize);
struct Map {
    tiles: HashMap<Pos, Tile>,
    pos: Pos,
    dir: Direction,
}

impl Map {
    fn run(&mut self) -> ControlFlow<HashMap<Pos, (Pos, Direction)>> {
        // which tiles did we visit while facing which direction
        let mut visited = HashSet::new();

        // from where did we visit each tile for the first time
        let mut first_visited = HashMap::new();

        visited.insert((self.pos, self.dir));

        let mut prev = self.pos;
        while self.step() {
            if !visited.insert((self.pos, self.dir)) {
                return ControlFlow::Continue(());
            }
            first_visited.entry(self.pos).or_insert((prev, self.dir));
            prev = self.pos;
        }

        ControlFlow::Break(first_visited)
    }

    fn step(&mut self) -> bool {
        let next = self.next();
        match self.tiles.get(&next) {
            None => false,
            Some(Tile::Empty | Tile::Guard) => {
                self.pos = next;
                true
            }
            Some(Tile::Obstacle) => {
                self.dir.turn_right();
                true
            }
        }
    }

    fn next(&self) -> (isize, isize) {
        match self.dir {
            Direction::Up => (self.pos.0 - 1, self.pos.1),
            Direction::Down => (self.pos.0 + 1, self.pos.1),
            Direction::Left => (self.pos.0, self.pos.1 - 1),
            Direction::Right => (self.pos.0, self.pos.1 + 1),
        }
    }
}

fn parse_input(input: &str) -> Map {
    let start = Cell::new((0, 0));

    let tiles = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let start = &start;
            line.chars().enumerate().map(move |(column, tile)| {
                let pos = (row as isize, column as isize);

                match tile {
                    '.' => (pos, Tile::Empty),
                    '#' => (pos, Tile::Obstacle),
                    '^' => {
                        start.set(pos);
                        (pos, Tile::Guard)
                    }
                    _ => panic!("unexpected input char: {tile:?}"),
                }
            })
        })
        .collect();

    Map {
        tiles,
        pos: start.get(),
        dir: Direction::Up,
    }
}

pub fn part1(input: &str) -> usize {
    parse_input(input).run().break_value().unwrap().len()
}

pub fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    map.run()
        .break_value()
        .unwrap()
        .into_iter()
        .filter(|&(next, (pos, dir))| {
            map.pos = pos;
            map.dir = dir;
            let mut result = false;
            if let Some(tile @ Tile::Empty) = map.tiles.get_mut(&next) {
                *tile = Tile::Obstacle;

                if let ControlFlow::Continue(_) = map.run() {
                    result = true
                }

                let tile = map.tiles.get_mut(&next).unwrap();
                *tile = Tile::Empty;
            }
            result
        })
        .count()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day06.example.txt"
    ));
    assert_eq!(part1(input), 41);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 4515);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day06.example.txt"
    ));
    assert_eq!(part2(input), 6);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1309);
}
