use std::collections::{BTreeMap, BTreeSet, VecDeque};

use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day15.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day15.example1.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day15.example2.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "15", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "15", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum SimleTiles {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(Debug, Clone, Copy)]
enum ComplexTiles {
    Wall,
    LeftBoxHalve,
    RightBoxHalve,
    Empty,
    Robot,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn apply_to(&self, robot: Pos) -> Pos {
        match self {
            Move::Up => [robot[0], robot[1] - 1],
            Move::Down => [robot[0], robot[1] + 1],
            Move::Left => [robot[0] - 1, robot[1]],
            Move::Right => [robot[0] + 1, robot[1]],
        }
    }
}

type Pos = [isize; 2];

struct Input<Tile> {
    map: BTreeMap<Pos, Tile>,
    robot: Pos,
    moves: Vec<Move>,
}

impl Input<SimleTiles> {
    fn apply_moves(&mut self) {
        for &m in &self.moves {
            let first = m.apply_to(self.robot);

            let mut current = self.robot;
            loop {
                let next: [isize; 2] = m.apply_to(current);
                let next_tile = self.map.get(&next).unwrap();
                match next_tile {
                    SimleTiles::Empty | SimleTiles::Robot => {
                        *self.map.get_mut(&self.robot).unwrap() = SimleTiles::Empty;
                        *self.map.get_mut(&next).unwrap() = SimleTiles::Box;
                        *self.map.get_mut(&first).unwrap() = SimleTiles::Robot;
                        self.robot = first;
                        break;
                    }
                    SimleTiles::Wall => {
                        break;
                    }
                    SimleTiles::Box => {
                        current = next;
                    }
                }
            }
        }
    }

    fn gps_costs(&self) -> isize {
        self.map
            .iter()
            .filter(|(_, tile)| matches!(tile, SimleTiles::Box))
            .map(|([column, row], _)| row * 100 + column)
            .sum()
    }

    fn print(&self) {
        for y in 0.. {
            for x in 0.. {
                let Some(tile) = self.map.get(&[x, y]) else {
                    if x == 0 {
                        return;
                    } else {
                        println!();
                        break;
                    }
                };
                match tile {
                    SimleTiles::Wall => print!("#"),
                    SimleTiles::Box => print!("O"),
                    SimleTiles::Empty => print!("."),
                    SimleTiles::Robot => print!("@"),
                }
            }
        }
    }

    fn into_complex_tiles(self) -> Input<ComplexTiles> {
        let map = self
            .map
            .into_iter()
            .flat_map(|(pos, tile)| {
                let left_pos = [pos[0] * 2, pos[1]];
                let right_pos = [pos[0] * 2 + 1, pos[1]];
                match tile {
                    SimleTiles::Wall => [
                        (left_pos, ComplexTiles::Wall),
                        (right_pos, ComplexTiles::Wall),
                    ],
                    SimleTiles::Box => [
                        (left_pos, ComplexTiles::LeftBoxHalve),
                        (right_pos, ComplexTiles::RightBoxHalve),
                    ],
                    SimleTiles::Empty => [
                        (left_pos, ComplexTiles::Empty),
                        (right_pos, ComplexTiles::Empty),
                    ],
                    SimleTiles::Robot => [
                        (left_pos, ComplexTiles::Robot),
                        (right_pos, ComplexTiles::Empty),
                    ],
                }
            })
            .collect::<BTreeMap<_, _>>();

        Input {
            robot: *map
                .iter()
                .find(|(_, tile)| matches!(tile, ComplexTiles::Robot))
                .unwrap()
                .0,
            map,
            moves: self.moves,
        }
    }
}

impl Input<ComplexTiles> {
    fn apply_moves(&mut self) {
        'outer: for &m in &self.moves {
            let mut visited = BTreeSet::from([self.robot]);
            let mut todo: VecDeque<[isize; 2]> = VecDeque::from([m.apply_to(self.robot)]);
            let mut to_move = vec![self.robot];
            while let Some(current) = todo.pop_front() {
                let current_tile = self.map.get(&current).unwrap();
                match current_tile {
                    ComplexTiles::Robot => {
                        log::debug!("Move: {m:?}, Robot: {:?}", self.robot);
                        self.print();
                        unreachable!()
                    }
                    ComplexTiles::Empty => {
                        continue;
                    }
                    ComplexTiles::Wall => {
                        continue 'outer;
                    }
                    ComplexTiles::LeftBoxHalve => {
                        let value = m.apply_to(current);
                        if visited.insert(value) {
                            todo.push_back(value);
                        }

                        let right_halve = Move::Right.apply_to(current);
                        let value = m.apply_to(right_halve);
                        if visited.insert(value) {
                            todo.push_back(value);
                        }

                        if !to_move.contains(&current) {
                            to_move.push(current)
                        };
                        if !to_move.contains(&right_halve) {
                            to_move.push(right_halve)
                        };
                    }
                    ComplexTiles::RightBoxHalve => {
                        let value = m.apply_to(current);
                        if visited.insert(value) {
                            todo.push_back(value);
                        }

                        let left_halve = Move::Left.apply_to(current);
                        let value = m.apply_to(left_halve);
                        if visited.insert(value) {
                            todo.push_back(value);
                        }

                        if !to_move.contains(&current) {
                            to_move.push(current)
                        };
                        if !to_move.contains(&left_halve) {
                            to_move.push(left_halve)
                        };
                    }
                }
            }
            while let Some(pos) = to_move.pop() {
                let tile = self.map.get_mut(&pos).unwrap();
                let old_tile = *tile;
                *tile = ComplexTiles::Empty;
                *self.map.get_mut(&m.apply_to(pos)).unwrap() = old_tile;
            }
            self.robot = m.apply_to(self.robot);
        }
    }

    fn gps_costs(&self) -> isize {
        self.map
            .iter()
            .filter(|(_, tile)| matches!(tile, ComplexTiles::LeftBoxHalve))
            .map(|([column, row], _)| row * 100 + column)
            .sum()
    }

    fn print(&self) {
        for y in 0.. {
            for x in 0.. {
                let Some(tile) = self.map.get(&[x, y]) else {
                    if x == 0 {
                        return;
                    } else {
                        println!();
                        break;
                    }
                };
                match tile {
                    ComplexTiles::Wall => print!("#"),
                    ComplexTiles::LeftBoxHalve => print!("["),
                    ComplexTiles::RightBoxHalve => print!("]"),
                    ComplexTiles::Empty => print!("."),
                    ComplexTiles::Robot => print!("@"),
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Input<SimleTiles> {
    let mut lines = input.lines();

    let map = (&mut lines)
        .take_while(|line| !line.is_empty())
        .enumerate()
        .flat_map({
            |(row, line)| {
                line.chars().enumerate().map({
                    move |(column, char)| {
                        let pos = [column as isize, row as isize];
                        let tile = match char {
                            '.' => SimleTiles::Empty,
                            '#' => SimleTiles::Wall,
                            '@' => SimleTiles::Robot,
                            'O' => SimleTiles::Box,
                            _ => panic!("Unexpected char {char} in input"),
                        };
                        (pos, tile)
                    }
                })
            }
        })
        .collect::<BTreeMap<_, _>>();

    let moves = lines
        .flat_map(|line| line.chars())
        .map(|char| match char {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Unexpected char {char} in input"),
        })
        .collect();

    Input {
        robot: *map
            .iter()
            .find(|(_, tile)| matches!(tile, SimleTiles::Robot))
            .unwrap()
            .0,
        map,
        moves,
    }
}

pub fn part1(input: &str) -> isize {
    let mut input = parse_input(input);
    input.apply_moves();
    let gps_cost = input.gps_costs();
    if log::log_enabled!(log::Level::Debug) {
        input.print();
    }
    gps_cost
}

pub fn part2(input: &str) -> isize {
    let mut input = parse_input(input).into_complex_tiles();
    input.apply_moves();
    let gps_cost = input.gps_costs();
    if log::log_enabled!(log::Level::Debug) {
        input.print();
    }
    gps_cost
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), 10092);
}

#[test]
fn part1_example2() {
    assert_eq!(part1(INPUT_EXAMPLE2), 2028);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1563092);
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 9021);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1582688);
}
