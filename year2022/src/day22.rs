use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum TileKind {
    Tile,
    Rock,
}

#[derive(Debug)]
struct Tile {
    row: usize,
    column: usize,
    left: RefCell<Option<(Weak<Tile>, Direction)>>,
    right: RefCell<Option<(Weak<Tile>, Direction)>>,
    up: RefCell<Option<(Weak<Tile>, Direction)>>,
    down: RefCell<Option<(Weak<Tile>, Direction)>>,
}

impl Tile {
    fn link(self: &Rc<Self>, our_dir: Direction, them: &Rc<Self>, their_dir: Direction) {
        self.set_dir(our_dir, them, their_dir);
        them.set_dir(their_dir, self, our_dir)
    }

    fn set_dir(&self, our_dir: Direction, them: &Rc<Self>, their_dir: Direction) {
        let val = Some((Rc::downgrade(them), their_dir.inverse()));
        match our_dir {
            Direction::Up => *self.up.borrow_mut() = val,
            Direction::Down => *self.down.borrow_mut() = val,
            Direction::Left => *self.left.borrow_mut() = val,
            Direction::Right => *self.right.borrow_mut() = val,
        }
    }

    fn get_dir(&self, dir: Direction) -> Option<(Weak<Tile>, Direction)> {
        match dir {
            Direction::Up => self.up.borrow().clone(),
            Direction::Down => self.down.borrow().clone(),
            Direction::Left => self.left.borrow().clone(),
            Direction::Right => self.right.borrow().clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&mut self) {
        let next = match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
        *self = next;
    }

    fn turn_right(&mut self) {
        let next = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        *self = next;
    }

    fn inverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

#[derive(Debug)]

enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

enum WrapMode {
    DonutLike,
    Cube,
}

type Position = (usize, usize);

fn parse(input: &str) -> (HashMap<Position, TileKind>, Vec<Instruction>) {
    let mut lines = input.lines();

    let tiles = (&mut lines)
        .take_while(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .flat_map(move |(column, elem)| match elem {
                    b'.' => Some(((row + 1, column + 1), TileKind::Tile)),
                    b'#' => Some(((row + 1, column + 1), TileKind::Rock)),
                    _ => None,
                })
        })
        .collect();

    let instructions = lines
        .next()
        .unwrap()
        .split_inclusive(&['R', 'L'])
        .flat_map(|elem| match elem {
            "L" => {
                vec![Instruction::TurnLeft]
            }
            "R" => {
                vec![Instruction::TurnRight]
            }
            go => {
                if let Ok(val) = go.parse() {
                    vec![Instruction::Move(val)]
                } else {
                    let (go, lr) = go.split_at(go.len() - 1);
                    vec![
                        Instruction::Move(go.parse().unwrap()),
                        if lr == "L" {
                            Instruction::TurnLeft
                        } else if lr == "R" {
                            Instruction::TurnRight
                        } else {
                            panic!()
                        },
                    ]
                }
            }
        })
        .collect();

    (tiles, instructions)
}

struct NeighborMapping {
    their_pos: Position,
    our_dir: Direction,
    their_dir: Direction,
}

fn assemble_tiles(
    map: &HashMap<Position, TileKind>,
    mode: WrapMode,
) -> HashMap<Position, Rc<Tile>> {
    let mut tile_map = HashMap::new();
    let max_row = map.keys().max_by_key(|(row, _)| row).unwrap().0;
    let max_column = map.keys().max_by_key(|(_, column)| column).unwrap().1;

    let side_length = (max_row + max_column) / 7;

    for (&our_pos, tile_kind) in map {
        if let TileKind::Tile = tile_kind {
            let tile = Rc::new(Tile {
                row: our_pos.0,
                column: our_pos.1,
                left: RefCell::new(None),
                right: RefCell::new(None),
                up: RefCell::new(None),
                down: RefCell::new(None),
            });
            tile_map.insert(our_pos, tile.clone());

            let dirs = match mode {
                WrapMode::DonutLike => find_donut_neighbors(our_pos, map),
                WrapMode::Cube => find_cube_neighbors(our_pos, map, side_length),
            };

            for NeighborMapping {
                their_pos,
                our_dir,
                their_dir,
            } in dirs
            {
                // dbg!(our_pos, their_pos);
                if let TileKind::Tile = map.get(&their_pos).unwrap() {
                    if let Some(them) = tile_map.get(&their_pos) {
                        tile.link(our_dir, them, their_dir)
                    }
                }
            }
        }
    }

    tile_map
}

fn find_cube_neighbors(
    pos: (usize, usize),
    map: &HashMap<(usize, usize), TileKind>,
    size: usize,
) -> [NeighborMapping; 4] {
    let up = if map.contains_key(&(pos.0 - 1, pos.1)) {
        NeighborMapping {
            their_pos: (pos.0 - 1, pos.1),
            our_dir: Direction::Up,
            their_dir: Direction::Down,
        }
    } else {
        lookup_cube_mapping(size, pos, map, Direction::Up)
    };

    let down = if map.contains_key(&(pos.0 + 1, pos.1)) {
        NeighborMapping {
            their_pos: (pos.0 + 1, pos.1),
            our_dir: Direction::Down,
            their_dir: Direction::Up,
        }
    } else {
        lookup_cube_mapping(size, pos, map, Direction::Down)
    };

    let left = if map.contains_key(&(pos.0, pos.1 - 1)) {
        NeighborMapping {
            their_pos: (pos.0, pos.1 - 1),
            our_dir: Direction::Left,
            their_dir: Direction::Right,
        }
    } else {
        lookup_cube_mapping(size, pos, map, Direction::Left)
    };

    let right = if map.contains_key(&(pos.0, pos.1 + 1)) {
        NeighborMapping {
            their_pos: (pos.0, pos.1 + 1),
            our_dir: Direction::Right,
            their_dir: Direction::Left,
        }
    } else {
        lookup_cube_mapping(size, pos, map, Direction::Right)
    };

    [left, right, up, down]
}

fn lookup_cube_mapping(
    size: usize,
    pos: Position,
    _map: &HashMap<Position, TileKind>,
    our_dir: Direction,
) -> NeighborMapping {
    let (their_pos, their_dir) = match size {
        4 => {
            match (pos, our_dir) {
                // (5, 1..4) <=> (1, 9..12)
                ((5, c @ 1..=4), Direction::Up) => ((1, 13 - c), Direction::Up),
                ((1, c @ 9..=12), Direction::Up) => ((5, 13 - c), Direction::Up),

                // (5, 5..8) <=> (1..4, 9)
                ((5, c @ 5..=8), Direction::Up) => ((c - 4, 9), Direction::Left),
                ((r @ 1..=4, 9), Direction::Left) => ((5, r + 4), Direction::Up),

                // (8, 5..8) <=> (9..12, 9)
                ((8, c @ 5..=8), Direction::Down) => ((17 - c, 9), Direction::Left),
                ((r @ 9..=12, 9), Direction::Left) => ((8, 17 - r), Direction::Down),

                // (8, 1..4) <=> (12, 9..12)
                ((8, c @ 1..=4), Direction::Down) => ((12, 13 - c), Direction::Down),
                ((12, c @ 9..=12), Direction::Down) => ((8, 13 - c), Direction::Down),

                // (5..8, 1) <=> (12, 13..16)
                ((r @ 5..=8, 1), Direction::Left) => ((12, r + 8), Direction::Down),
                ((12, c @ 13..=16), Direction::Down) => ((c - 8, 1), Direction::Left),

                // (9, 13..16) <=> (5..8, 16)
                ((9, c @ 13..=16), Direction::Up) => ((21 - c, 12), Direction::Right),
                ((r @ 5..=8, 12), Direction::Right) => ((9, 21 - r), Direction::Up),

                // (1..4, 12) <=> (9..12, 16)
                ((r @ 1..=4, 12), Direction::Right) => ((13 - r, 16), Direction::Right),
                ((r @ 9..=12, 16), Direction::Right) => ((13 - r, 12), Direction::Right),
                _ => panic!(),
            }
        }
        50 => {
            match (pos, our_dir) {
                // (101, 1..50) <=> (51..100, 51)
                ((101, c @ 1..=50), Direction::Up) => ((c + 50, 51), Direction::Left),
                ((r @ 51..=100, 51), Direction::Left) => ((101, r - 50), Direction::Up),

                // (101..150, 1) <=> (1..50, 51)
                ((r @ 101..=150, 1), Direction::Left) => ((151 - r, 51), Direction::Left),
                ((r @ 1..=50, 51), Direction::Left) => ((151 - r, 1), Direction::Left),

                // (151..200, 1) <=> (1, 51..100)
                ((r @ 151..=200, 1), Direction::Left) => ((1, r - 100), Direction::Up),
                ((1, c @ 51..=100), Direction::Up) => ((c + 100, 1), Direction::Left),

                // (50, 101..150) <=>  (51..100, 100)
                ((50, c @ 101..=150), Direction::Down) => ((c - 50, 100), Direction::Right),
                ((r @ 51..=100, 100), Direction::Right) => ((50, r + 50), Direction::Down),

                // (150, 51..100) <=> (151..200, 50)
                ((150, c @ 51..=100), Direction::Down) => ((c + 100, 50), Direction::Right),
                ((r @ 151..=200, 50), Direction::Right) => ((150, r - 100), Direction::Down),

                // (1..50, 150) <=> (101..150, 100)
                ((r @ 1..=50, 150), Direction::Right) => ((151 - r, 100), Direction::Right),
                ((r @ 101..=150, 100), Direction::Right) => ((151 - r, 150), Direction::Right),

                // (1, 101..150) <=> (200, 1..50)
                ((1, c @ 101..=150), Direction::Up) => ((200, c - 100), Direction::Down),
                ((200, c @ 1..=50), Direction::Down) => ((1, c + 100), Direction::Up),

                _ => panic!(),
            }
        }
        _ => todo!("use map to determine cube net mapping {size}"),
    };
    NeighborMapping {
        their_pos,
        our_dir,
        their_dir,
    }
}

fn find_donut_neighbors(
    pos: (usize, usize),
    map: &HashMap<Position, TileKind>,
) -> [NeighborMapping; 4] {
    let up = if map.contains_key(&(pos.0 - 1, pos.1)) {
        (pos.0 - 1, pos.1)
    } else {
        *map.keys()
            .filter(|elem| elem.1 == pos.1)
            .max_by_key(|elem| elem.0)
            .unwrap()
    };

    let down = if map.contains_key(&(pos.0 + 1, pos.1)) {
        (pos.0 + 1, pos.1)
    } else {
        *map.keys()
            .filter(|elem| elem.1 == pos.1)
            .min_by_key(|elem| elem.0)
            .unwrap()
    };

    let left = if map.contains_key(&(pos.0, pos.1 - 1)) {
        (pos.0, pos.1 - 1)
    } else {
        *map.keys()
            .filter(|elem| elem.0 == pos.0)
            .max_by_key(|elem| elem.1)
            .unwrap()
    };

    let right = if map.contains_key(&(pos.0, pos.1 + 1)) {
        (pos.0, pos.1 + 1)
    } else {
        *map.keys()
            .filter(|elem| elem.0 == pos.0)
            .min_by_key(|elem| elem.1)
            .unwrap()
    };

    [
        NeighborMapping {
            their_pos: left,
            our_dir: Direction::Left,
            their_dir: Direction::Right,
        },
        NeighborMapping {
            their_pos: right,
            our_dir: Direction::Right,
            their_dir: Direction::Left,
        },
        NeighborMapping {
            their_pos: up,
            our_dir: Direction::Up,
            their_dir: Direction::Down,
        },
        NeighborMapping {
            their_pos: down,
            our_dir: Direction::Down,
            their_dir: Direction::Up,
        },
    ]
}

fn find_start(tiles: &HashMap<(usize, usize), Rc<Tile>>) -> Rc<Tile> {
    tiles
        .iter()
        .filter(|((row, _), _)| *row == 1)
        .min_by_key(|((_, column), _)| column)
        .unwrap()
        .1
        .clone()
}

fn walk(input: &str, mode: WrapMode) -> usize {
    let (tile_kind_map, instructions) = parse(input);
    let tiles = assemble_tiles(&tile_kind_map, mode);

    let mut current_pos = find_start(&tiles);
    let mut current_orientation = Direction::Right;

    for instruction in instructions {
        match instruction {
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    if let Some((next_pos, next_orientation)) =
                        current_pos.get_dir(current_orientation)
                    {
                        current_pos = next_pos.upgrade().unwrap();
                        current_orientation = next_orientation;
                    } else {
                        // we stopped so we can skip remaining movement steps
                        break;
                    }
                }
            }
            Instruction::TurnLeft => current_orientation.turn_left(),
            Instruction::TurnRight => current_orientation.turn_right(),
        }
    }
    let dir_val: usize = current_orientation.into();

    1000 * current_pos.row + 4 * current_pos.column + dir_val
}

#[allow(dead_code)]
fn print_steps(
    step: &HashMap<(usize, usize), Direction>,
    tile_kind_map: &HashMap<(usize, usize), TileKind>,
) {
    let max_x = *tile_kind_map
        .keys()
        .map(|(_, column)| column)
        .max()
        .unwrap();
    let max_y = *tile_kind_map.keys().map(|(row, _)| row).max().unwrap();

    for row in 1..max_y {
        for column in 1..=max_x {
            let symbol = if let Some(dir) = step.get(&(row, column)) {
                match dir {
                    Direction::Up => "^",
                    Direction::Down => "v",
                    Direction::Left => "<",
                    Direction::Right => ">",
                }
            } else {
                match tile_kind_map.get(&(row, column)) {
                    Some(TileKind::Tile) => ".",
                    Some(TileKind::Rock) => "#",
                    None => " ",
                }
            };
            print!("{symbol}");
        }
        println!()
    }
}

pub fn part1(input: &str) -> usize {
    walk(input, WrapMode::DonutLike)
}

pub fn part2(input: &str) -> usize {
    walk(input, WrapMode::Cube)
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day22.example.txt");
    assert_eq!(part1(input), 6032);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day22.txt"));
    assert_eq!(part1(input), 1428);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day22.example.txt");
    assert_eq!(part2(input), 5031);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day22.txt"));
    assert_eq!(part2(input), 142380);
}
