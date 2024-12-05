use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Blizzard(Direction),
}
impl Tile {
    // given the current position and the interior width and height (i.e. mot including the walls)
    // compute the new position
    fn move_by(&self, pos: &Position, offset: usize, width: usize, height: usize) -> Position {
        let mut current_pos = *pos;
        for _ in 0..offset {
            let new_pos = match self {
                Tile::Wall => current_pos, // walls don't move
                Tile::Blizzard(Direction::Left) => Position {
                    y: current_pos.y,
                    x: if current_pos.x == 1 {
                        width
                    } else {
                        current_pos.x - 1
                    },
                },
                Tile::Blizzard(Direction::Right) => Position {
                    y: current_pos.y,
                    x: if current_pos.x == width {
                        1
                    } else {
                        current_pos.x + 1
                    },
                },
                Tile::Blizzard(Direction::Up) => Position {
                    x: current_pos.x,
                    y: if current_pos.y == 1 {
                        height
                    } else {
                        current_pos.y - 1
                    },
                },
                Tile::Blizzard(Direction::Down) => Position {
                    x: current_pos.x,
                    y: if current_pos.y == height {
                        1
                    } else {
                        current_pos.y + 1
                    },
                },
            };
            current_pos = new_pos;
        }
        current_pos
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
struct Board {
    tiles: HashMap<Position, Tile>,
    width: usize,
    height: usize,
}

impl Board {
    fn expand(&self) -> Vec<Board> {
        // subtract two to get the inner dimensions
        let (inner_width, inner_height) = (self.width - 2, self.height - 2);
        let cycle_length = lcm(inner_width, inner_height);

        let mut result: Vec<_> = (0..=cycle_length)
            .map(|offset| {
                let tiles = self
                    .tiles
                    .iter()
                    .map(|(pos, tile)| {
                        let new_pos = tile.move_by(pos, offset, inner_width, inner_height);
                        (new_pos, *tile)
                    })
                    .collect();
                Board {
                    tiles,
                    width: self.width,
                    height: self.height,
                }
            })
            .collect();

        let last = result.pop().unwrap();
        if result[0] != last {
            println!("Got:\n{last}");
            println!("Expected:\n{}", result[0]);
            panic!("Boards didn't repeat after {cycle_length} cycles!");
        }
        result
    }

    fn find_start(&self) -> Position {
        for x in 1.. {
            let pos = Position { x, y: 0 };
            if !self.tiles.contains_key(&pos) {
                return pos;
            }
        }
        panic!("Couldn't find the start")
    }

    fn find_end(&self) -> Position {
        let y = self.tiles.keys().map(|key| key.y).max().unwrap();
        for x in 1.. {
            let pos = Position { x, y };
            if !self.tiles.contains_key(&pos) {
                return pos;
            }
        }
        panic!("Couldn't find the end")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_row, max_column) = self.tiles.keys().fold((0, 0), |(row, column), pos| {
            (row.max(pos.y), column.max(pos.x))
        });
        for row in 0..=max_row {
            for column in 0..=max_column {
                match self.tiles.get(&Position { x: column, y: row }) {
                    Some(Tile::Wall) => f.write_char('#')?,
                    Some(Tile::Blizzard(_)) => f.write_char('x')?,
                    None => f.write_char('.')?,
                };
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn lcm(width: usize, height: usize) -> usize {
    let result = width / gcd(width, height) * height;
    println!("lcm of {width} and {height} is {result}");
    result
}

fn gcd(width: usize, height: usize) -> usize {
    let mut a = width;
    let mut b = height;
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn parse(input: &str) -> Board {
    let tiles: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes().enumerate().flat_map(move |(column, b)| {
                let pos = Position { x: column, y: row };
                match b {
                    b'#' => Some((pos, Tile::Wall)),
                    b'<' => Some((pos, Tile::Blizzard(Direction::Left))),
                    b'>' => Some((pos, Tile::Blizzard(Direction::Right))),
                    b'^' => Some((pos, Tile::Blizzard(Direction::Up))),
                    b'v' => Some((pos, Tile::Blizzard(Direction::Down))),
                    _ => None,
                }
            })
        })
        .collect();

    let (max_x, max_y) = tiles.keys().fold((0, 0), |(max_x, max_y), next| {
        (max_x.max(next.x), max_y.max(next.y))
    });

    Board {
        tiles,
        width: max_x + 1,
        height: max_y + 1,
    }
}

enum DijkstraDirection {
    Forward,
    Backward,
}

fn dijkstra(input: &[Board], direction: DijkstraDirection) -> usize {
    let (start, end) = {
        let start = input[0].find_start();
        let end = input[0].find_end();

        match direction {
            DijkstraDirection::Forward => (start, end),
            DijkstraDirection::Backward => (end, start),
        }
    };

    println!("Finding Path from {start:?} to {end:?}");

    let mut visited = HashMap::new();

    let mut todo = HashMap::new();
    todo.insert((start, 0), 0);

    while let Some((from, distance)) = todo
        .iter()
        .min_by_key(|(_, distance)| **distance)
        .map(|(key, value)| (*key, *value))
    {
        todo.remove(&from);
        visited.insert(from, distance);

        if from.0 == end {
            return distance;
        }

        let next_board_idx = (from.1 + 1) % input.len();
        let next_board = &input[next_board_idx];
        let new_distance = distance + 1;

        for offset in [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)] {
            let next_pos = Position {
                x: from.0.x.saturating_add_signed(offset.0),
                y: from.0.y.saturating_add_signed(offset.1),
            };
            let next_key = (next_pos, next_board_idx);
            let visited = visited.contains_key(&next_key);
            let blocked = next_board.tiles.contains_key(&next_pos);
            let inbounds = next_pos.x < next_board.width && next_pos.y < next_board.height;
            if !visited && !blocked && inbounds {
                todo.entry(next_key)
                    .and_modify(|old_value| *old_value = new_distance.min(*old_value))
                    .or_insert(new_distance);
            } else if new_distance == 600 {
                println!("Skipping {next_key:?} as is blocked {blocked} or visited {visited}");
            }
        }
    }
    panic!("We never reached the end: {visited:?}");
}

pub fn part1(input: &str) -> usize {
    let initial_board = parse(input);
    let boards = initial_board.expand();

    dijkstra(&boards, DijkstraDirection::Forward)
}

pub fn part2(input: &str) -> usize {
    let initial_board = parse(input);
    let mut boards = initial_board.expand();

    let board_count = boards.len();

    let p1 = dijkstra(&boards, DijkstraDirection::Forward);
    boards.rotate_left(p1 % board_count);

    let p2 = dijkstra(&boards, DijkstraDirection::Backward);
    boards.rotate_left(p2 % board_count);

    let p3 = dijkstra(&boards, DijkstraDirection::Forward);

    p1 + p2 + p3
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day24.example.txt"
    ));
    assert_eq!(part1(input), 10);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day24.example2.txt"
    ));
    assert_eq!(part1(input), 18);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day24.txt"
    ));
    assert_eq!(part1(input), 305);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day24.example2.txt"
    ));
    assert_eq!(part2(input), 18 + 23 + 13);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day24.txt"
    ));
    assert_eq!(part2(input), 305 + 284 + 316);
}
