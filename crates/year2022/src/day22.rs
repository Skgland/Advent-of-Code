use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{
    cell::RefCell,
    collections::HashMap,
    ops::RangeInclusive,
    rc::{Rc, Weak},
};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day22.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "22", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "22", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
enum TileKind {
    Tile,
    Rock,
}

#[derive(Debug)]
struct CubeSide {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    left: RefCell<Option<(Weak<Self>, Direction)>>,
    right: RefCell<Option<(Weak<Self>, Direction)>>,
    up: RefCell<Option<(Weak<Self>, Direction)>>,
    down: RefCell<Option<(Weak<Self>, Direction)>>,
}

impl CubeSide {
    fn link(self: &Rc<Self>, our_dir: Direction, them: &Rc<Self>, their_dir: Direction) {
        self.set_dir(our_dir, them, their_dir);
        them.set_dir(their_dir, self, our_dir)
    }

    fn set_dir(&self, our_dir: Direction, them: &Rc<Self>, their_dir: Direction) {
        let val = Some((Rc::downgrade(them), their_dir));
        match our_dir {
            Direction::Up => *self.up.borrow_mut() = val,
            Direction::Down => *self.down.borrow_mut() = val,
            Direction::Left => *self.left.borrow_mut() = val,
            Direction::Right => *self.right.borrow_mut() = val,
        }
    }

    fn get_dir(&self, dir: Direction) -> Option<(Weak<Self>, Direction)> {
        match dir {
            Direction::Up => self.up.borrow().clone(),
            Direction::Down => self.down.borrow().clone(),
            Direction::Left => self.left.borrow().clone(),
            Direction::Right => self.right.borrow().clone(),
        }
    }
}

struct CubeMap {
    sides: [Rc<CubeSide>; 6],
}

impl CubeMap {
    fn determine_cube(tiles: &HashMap<Position, TileKind>) -> Self {
        let max_row = tiles
            .keys()
            .max_by_key(|Position { row, column: _ }| row)
            .unwrap()
            .row;
        let max_column = tiles
            .keys()
            .max_by_key(|Position { row: _, column }| column)
            .unwrap()
            .column;

        let side_length = (max_row + max_column) / 7;

        let mut side_map = HashMap::new();

        for x in 1..5 {
            for y in 1..5 {
                let pos = Position {
                    row: y * side_length,
                    column: x * side_length,
                };
                if tiles.contains_key(&pos) {
                    let x_range = (x - 1) * side_length + 1..=pos.column;
                    let y_range = (y - 1) * side_length + 1..=pos.row;

                    let tile = Rc::new(CubeSide {
                        x: x_range,
                        y: y_range,
                        left: RefCell::new(None),
                        right: RefCell::new(None),
                        up: RefCell::new(None),
                        down: RefCell::new(None),
                    });

                    side_map.insert(Position { row: y, column: x }, tile.clone());

                    for dir in [Direction::Left, Direction::Up] {
                        if let Some(neighbor) =
                            side_map.get(&dir.offset(Position { row: y, column: x }))
                        {
                            tile.link(dir, neighbor, dir.inverse());
                        }
                    }
                }
            }
        }

        let sides = side_map
            .into_values()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        // fold cube
        loop {
            let mut missing = false;
            let mut progress = false;

            for side in &sides {
                let side: &Rc<_> = side;
                for dir in Direction::DIRS {
                    if CubeSide::get_dir(side, dir).is_none() {
                        missing = true;
                        if let Some((neighbor_side, neighbor_dir)) =
                            CubeSide::get_dir(side, dir.turn_left())
                        {
                            if let Some((neighbor_side, neighbor_dir)) = CubeSide::get_dir(
                                &neighbor_side.upgrade().unwrap(),
                                neighbor_dir.turn_left(),
                            ) {
                                side.link(
                                    dir,
                                    &neighbor_side.upgrade().unwrap(),
                                    neighbor_dir.turn_left(),
                                );
                                progress = true;
                            }
                        }
                        if let Some((neighbor_side, neighbor_dir)) =
                            CubeSide::get_dir(side, dir.turn_right())
                        {
                            if let Some((neighbor_side, neighbor_dir)) = CubeSide::get_dir(
                                &neighbor_side.upgrade().unwrap(),
                                neighbor_dir.turn_right(),
                            ) {
                                side.link(
                                    dir,
                                    &neighbor_side.upgrade().unwrap(),
                                    neighbor_dir.turn_right(),
                                );
                                progress = true;
                            }
                        }
                    }
                }
            }

            if !missing {
                break CubeMap { sides };
            } else if !progress {
                panic!("Failed to fold cube!")
            }
        }
    }

    fn lookup_cube_mapping(&self, pos: Position, our_dir: Direction) -> NeighborMapping {
        let side = self
            .sides
            .iter()
            .find(|side| side.x.contains(&pos.column) && side.y.contains(&pos.row))
            .unwrap();
        let (neighbor_side, neighbor_dir) = CubeSide::get_dir(side, our_dir).unwrap();
        let neighbor_side = neighbor_side.upgrade().unwrap();

        let (x, y) = match (our_dir, neighbor_dir) {
            (Direction::Up, Direction::Up) => (
                side.x.start() + neighbor_side.x.end() - pos.column,
                *neighbor_side.y.start(),
            ),
            (Direction::Up, Direction::Down) => (
                pos.column + neighbor_side.x.start() - side.x.start(),
                *neighbor_side.y.end(),
            ),
            (Direction::Up, Direction::Left) => (
                *neighbor_side.x.start(),
                pos.column + neighbor_side.y.start() - side.x.start(),
            ),
            (Direction::Up, Direction::Right) => (
                *neighbor_side.x.end(),
                side.x.start() + neighbor_side.y.end() - pos.column,
            ),
            (Direction::Down, Direction::Up) => (
                pos.column + neighbor_side.x.start() - side.x.start(),
                *neighbor_side.y.start(),
            ),
            (Direction::Down, Direction::Down) => (
                side.x.start() + neighbor_side.x.end() - pos.column,
                *neighbor_side.y.end(),
            ),
            (Direction::Down, Direction::Left) => (
                *neighbor_side.x.start(),
                side.x.start() + neighbor_side.y.end() - pos.column,
            ),
            (Direction::Down, Direction::Right) => (
                *neighbor_side.x.end(),
                pos.column + neighbor_side.y.start() - side.x.start(),
            ),
            (Direction::Left, Direction::Up) => (
                pos.row + neighbor_side.x.start() - side.y.start(),
                *neighbor_side.y.start(),
            ),
            (Direction::Left, Direction::Down) => (
                side.y.start() + neighbor_side.x.end() - pos.row,
                *neighbor_side.y.end(),
            ),
            (Direction::Left, Direction::Left) => (
                *neighbor_side.x.start(),
                side.y.start() + neighbor_side.y.end() - pos.row,
            ),
            (Direction::Left, Direction::Right) => (
                *neighbor_side.x.end(),
                pos.row + neighbor_side.y.start() - side.y.start(),
            ),
            (Direction::Right, Direction::Up) => (
                side.y.start() + neighbor_side.x.end() - pos.row,
                *neighbor_side.y.start(),
            ),
            (Direction::Right, Direction::Down) => (
                pos.row + neighbor_side.x.start() - side.y.start(),
                *neighbor_side.y.end(),
            ),
            (Direction::Right, Direction::Left) => (
                *neighbor_side.x.start(),
                pos.row + neighbor_side.y.start() - side.y.start(),
            ),
            (Direction::Right, Direction::Right) => (
                *neighbor_side.x.end(),
                side.y.start() + neighbor_side.y.end() - pos.row,
            ),
        };

        let their_pos = Position { row: y, column: x };

        NeighborMapping {
            their_pos,
            our_dir,
            their_dir: neighbor_dir,
        }
    }
}

#[derive(Debug)]
struct Tile {
    row: usize,
    column: usize,
    left: RefCell<Option<(Weak<Self>, Direction)>>,
    right: RefCell<Option<(Weak<Self>, Direction)>>,
    up: RefCell<Option<(Weak<Self>, Direction)>>,
    down: RefCell<Option<(Weak<Self>, Direction)>>,
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

    fn get_dir(&self, dir: Direction) -> Option<(Weak<Self>, Direction)> {
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
    const DIRS: [Direction; 4] = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    fn offset(&self, Position { row, column }: Position) -> Position {
        match self {
            Direction::Up => Position {
                row: row - 1,
                column,
            },
            Direction::Down => Position {
                row: row + 1,
                column,
            },
            Direction::Left => Position {
                row,
                column: column - 1,
            },
            Direction::Right => Position {
                row,
                column: column + 1,
            },
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
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
    Cube(CubeMap),
}
impl WrapMode {
    fn find_neighbor(
        &self,
        pos: Position,
        map: &HashMap<Position, TileKind>,
    ) -> [NeighborMapping; 4] {
        Direction::DIRS.map(|dir| {
            let new_pos = dir.offset(pos);
            if map.contains_key(&new_pos) {
                NeighborMapping {
                    their_pos: new_pos,
                    our_dir: dir,
                    their_dir: dir.inverse(),
                }
            } else {
                self.lookup_mapping(pos, map, dir)
            }
        })
    }

    fn lookup_mapping(
        &self,
        pos: Position,
        map: &HashMap<Position, TileKind>,
        dir: Direction,
    ) -> NeighborMapping {
        match self {
            WrapMode::DonutLike => lookup_doughnut_mapping(pos, map, dir),
            WrapMode::Cube(cube) => cube.lookup_cube_mapping(pos, dir),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    row: usize,
    column: usize,
}

fn parse(input: &str) -> (HashMap<Position, TileKind>, Vec<Instruction>) {
    let mut lines = input.lines();

    let tiles = (&mut lines)
        .take_while(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .flat_map(move |(column, elem)| match elem {
                    b'.' => Some((
                        Position {
                            row: row + 1,
                            column: column + 1,
                        },
                        TileKind::Tile,
                    )),
                    b'#' => Some((
                        Position {
                            row: row + 1,
                            column: column + 1,
                        },
                        TileKind::Rock,
                    )),
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

#[derive(PartialEq, Eq, Debug)]
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

    for (&our_pos, tile_kind) in map {
        if let TileKind::Tile = tile_kind {
            let tile = Rc::new(Tile {
                row: our_pos.row,
                column: our_pos.column,
                left: RefCell::new(None),
                right: RefCell::new(None),
                up: RefCell::new(None),
                down: RefCell::new(None),
            });
            tile_map.insert(our_pos, tile.clone());

            let dirs = mode.find_neighbor(our_pos, map);

            for NeighborMapping {
                their_pos,
                our_dir,
                their_dir,
            } in dirs
            {
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

fn lookup_doughnut_mapping(
    pos: Position,
    map: &HashMap<Position, TileKind>,
    dir: Direction,
) -> NeighborMapping {
    let pos = match dir {
        Direction::Up => *map
            .keys()
            .filter(|elem| elem.column == pos.column)
            .max_by_key(|elem| elem.row)
            .unwrap(),
        Direction::Down => *map
            .keys()
            .filter(|elem| elem.column == pos.column)
            .min_by_key(|elem| elem.row)
            .unwrap(),
        Direction::Left => *map
            .keys()
            .filter(|elem| elem.row == pos.row)
            .max_by_key(|elem| elem.column)
            .unwrap(),
        Direction::Right => *map
            .keys()
            .filter(|elem| elem.row == pos.row)
            .min_by_key(|elem| elem.column)
            .unwrap(),
    };
    NeighborMapping {
        their_pos: pos,
        our_dir: dir,
        their_dir: dir.inverse(),
    }
}

fn find_start(tiles: &HashMap<Position, Rc<Tile>>) -> Rc<Tile> {
    tiles
        .iter()
        .filter(|(Position { row, column: _ }, _)| *row == 1)
        .min_by_key(|(Position { row: _, column }, _)| column)
        .unwrap()
        .1
        .clone()
}

fn walk(
    tile_kind_map: &HashMap<Position, TileKind>,
    instructions: &[Instruction],
    mode: WrapMode,
) -> usize {
    let tiles = assemble_tiles(tile_kind_map, mode);

    let mut current_pos = find_start(&tiles);
    let mut current_orientation = Direction::Right;

    for instruction in instructions {
        match instruction {
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    match current_pos.get_dir(current_orientation) {
                        Some((next_pos, next_orientation)) => {
                            current_pos = next_pos.upgrade().unwrap();
                            current_orientation = next_orientation;
                        }
                        _ => {
                            // we stopped so we can skip remaining movement steps
                            break;
                        }
                    }
                }
            }
            Instruction::TurnLeft => current_orientation = current_orientation.turn_left(),
            Instruction::TurnRight => current_orientation = current_orientation.turn_right(),
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
    let (tile_kind_map, instructions) = parse(input);
    walk(&tile_kind_map, &instructions, WrapMode::DonutLike)
}

pub fn part2(input: &str) -> usize {
    let (tile_kind_map, instructions) = parse(input);
    let cube = CubeMap::determine_cube(&tile_kind_map);
    walk(&tile_kind_map, &instructions, WrapMode::Cube(cube))
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day22.example.txt"
    ));
    assert_eq!(part1(input), 6032);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1428);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day22.example.txt"
    ));
    assert_eq!(part2(input), 5031);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 142380);
}
