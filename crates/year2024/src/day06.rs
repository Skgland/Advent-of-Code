use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    ops::ControlFlow,
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

struct Map {
    tiles: HashMap<(isize, isize), Tile>,
    pos: (isize, isize),
    dir: Direction,
}
impl Map {
    fn run(&mut self) -> ControlFlow<HashMap<(isize, isize), ((isize, isize), Direction)>> {
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
        loop {
            let next = self.next();
            match self.tiles.get(&next) {
                None => return false,
                Some(Tile::Empty | Tile::Guard) => {
                    self.pos = next;
                    return true;
                }
                Some(Tile::Obstacle) => {
                    self.dir.turn_right();
                    return true;
                }
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day06.txt"
    ));
    assert_eq!(part1(input), 4515);
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day06.txt"
    ));
    assert_eq!(part2(input), 1309);
}
