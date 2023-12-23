use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn neighbors(&self, map: &Map<Tile>, slippery: bool) -> Vec<Position> {
        let possible_neighbors = match map.path.get(self) {
            Some(Tile::DownSlope) if slippery => {
                vec![(1, 0)]
            }
            Some(Tile::RightSlope) if slippery => {
                vec![(0, 1)]
            }
            Some(_) => {
                vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
            }
            None => vec![],
        };

        // TODO check for walking up a slope
        possible_neighbors
            .into_iter()
            .filter_map(|(rd, cd)| {
                let new = Position {
                    row: self.row.checked_add_signed(rd)?,
                    column: self.column.checked_add_signed(cd)?,
                };
                match map.path.get(&new)? {
                    Tile::DownSlope if slippery => {
                        if rd == -1 {
                            None
                        } else {
                            Some(new)
                        }
                    }
                    Tile::RightSlope if slippery => {
                        if cd == -1 {
                            None
                        } else {
                            Some(new)
                        }
                    }
                    _ => Some(new),
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    DownSlope,
    RightSlope,
    FlatPath,
}

#[derive(Debug)]
struct Map<T> {
    path: HashMap<Position, T>,
    width: usize,
    height: usize,
}

impl Map<Tile> {
    fn collapse(&self, slippery: bool) -> Map<HashMap<Position, usize>> {
        let mut map = HashMap::new();

        // each edge begins either at the start or at a slope
        let starts: VecDeque<_> = [Position { row: 0, column: 1 }]
            .into_iter()
            .chain(
                self.path
                    .iter()
                    .filter(|(pos, tile)| !matches!(tile, Tile::FlatPath)|| pos.neighbors(self, slippery).len() > 2)
                    .map(|(pos, _)| pos)
                    .cloned(),
            )
            .collect();

        for start in starts {
            // find the longest path to each end for this start
            let destinations: &mut HashMap<_, usize> = map.entry(start.clone()).or_default();

            let mut todo = VecDeque::from([(start.clone(), HashSet::from([start.clone()]))]);

            while let Some((pos, visited)) = todo.pop_front() {
                let neighbors = pos.neighbors(self, slippery);
                for next in &neighbors {

                    // valid tile and not yet visited
                    if self.path.contains_key(next) && !visited.contains(next) {

                        // slope or goal or crossing
                        if next.row + 1 == self.height
                            || self
                                .path
                                .get(next)
                                .is_some_and(|tile| !matches!(tile, Tile::FlatPath))
                            || next.neighbors(self, slippery).len() > 2
                        {
                            destinations
                                .entry(next.clone())
                                .and_modify(|old| *old = (*old).max(visited.len()))
                                .or_insert(visited.len());
                        } else {
                            let mut new_visited = visited.clone();
                            new_visited.insert(next.clone());
                            todo.push_back((next.clone(), new_visited))
                        }
                    }
                }
            }
        }

        Map {
            path: map,
            width: self.width,
            height: self.height,
        }
    }
}

impl Map<HashMap<Position, usize>> {
    fn print_graph<W: std::io::Write>(&self, writer: &mut W) {
        #![allow(unused_must_use)]

        writeln!(writer, "algorithm: org.eclipse.elk.stress");
        writeln!(writer, "org.eclipse.elk.stress.desiredEdgeLength: 150");
        let mut emitted_node = HashSet::new();

        for (start, ends) in &self.path {
            let start_node = format!("P{}_{}", start.row, start.column);
            if !emitted_node.contains(start) {
                emitted_node.insert(start.clone());
                writeln!(writer, "node {start_node}");
            }

            for (end, distance) in ends {
                let end_node = format!("P{}_{}", end.row, end.column);
                if !emitted_node.contains(end) {
                    emitted_node.insert(end.clone());
                    writeln!(writer, "node {end_node}");
                }
                writeln!(
                    writer,
                    "edge {start_node} -> {end_node} {{ label \"{distance}\"}}"
                );
            }
        }
    }

    fn longest_distance(&self) -> usize {
        let mut reachable_in = HashMap::from([(
            Position { row: 0, column: 1 },
            HashMap::from([(BTreeSet::new(), 0)]),
        )]);

        let mut todo: BTreeSet<_> = reachable_in.keys().cloned().collect();

        while let Some(cur) = todo.pop_first() {
            let distances: HashMap<BTreeSet<_>, usize> = reachable_in
                .get(&cur)
                .expect("Should have already been reached!")
                .clone();

            for (next, delta) in self.path.get(&cur).into_iter().flatten() {
                for (cur_via, distance) in &distances {
                    if cur_via.contains(next) {
                        continue;
                    }
                    let mut next_via = cur_via.clone();
                    next_via.insert(cur.clone());

                    let new_distance = distance + delta;
                    reachable_in
                        .entry(next.clone())
                        .or_default()
                        .entry(next_via)
                        .and_modify(|old| {
                            if *old < new_distance {
                                *old = new_distance;
                                todo.insert(next.clone());
                            }
                        })
                        .or_insert_with(|| {
                            todo.insert(next.clone());
                            new_distance
                        });
                }
            }
        }

        let distances_to_goal = reachable_in
            .get(&Position {
                row: self.height - 1,
                column: self.width - 2,
            })
            .unwrap();
        let (longest_path, length) = distances_to_goal
            .iter()
            .max_by_key(|(_, value)| **value)
            .unwrap();
        dbg!(longest_path);
        *length
    }
}

fn parse_input(input: &str) -> Map<Tile> {
    let path = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column, symbol)| {
                    let pos = Position { row, column };
                    let tile = match symbol {
                        '.' => Tile::FlatPath,
                        '>' => Tile::RightSlope,
                        'v' => Tile::DownSlope,
                        '#' => return None,
                        other => {
                            eprintln!("Unexpected Tile: {other}");
                            return None;
                        }
                    };
                    Some((pos, tile))
                })
        })
        .collect();

    Map {
        path,
        height: input.lines().count(),
        width: input.lines().map(|line| line.len()).max().unwrap_or(0),
    }
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input).collapse(true);
    map.print_graph(&mut std::fs::File::create("./day23-p1.elkt").unwrap());
    map.longest_distance()
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input).collapse(false);
    map.print_graph(&mut std::fs::File::create("./day23-p2.elkt").unwrap());
    map.longest_distance()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day23.example.txt"));
    assert_eq!(part1(input), 94);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day23.txt"));
    assert_eq!(part1(input), 2034);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day23.example.txt");
    assert_eq!(part2(input), 154);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day23.txt"));
    assert_eq!(part2(input), 1262);
}
