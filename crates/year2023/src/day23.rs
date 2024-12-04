use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

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
    path: BTreeMap<Position, T>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    fn start(&self) -> Position {
        Position { row: 0, column: 1 }
    }

    fn end(&self) -> Position {
        Position {
            row: self.height - 1,
            column: self.width - 2,
        }
    }
}

impl Map<Tile> {
    fn collapse(&self, slippery: bool) -> Map<BTreeMap<Position, usize>> {
        let mut map = BTreeMap::new();

        // each edge begins either at the start or at a slope
        let starts: VecDeque<_> = [self.start()]
            .into_iter()
            .chain(
                self.path
                    .iter()
                    .filter(|(pos, tile)| {
                        !matches!(tile, Tile::FlatPath) || pos.neighbors(self, slippery).len() > 2
                    })
                    .map(|(pos, _)| pos)
                    .cloned(),
            )
            .collect();

        for start in starts {
            // find the longest path to each end for this start
            let destinations: &mut BTreeMap<_, usize> = map.entry(start.clone()).or_default();

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

struct PathEnumerator<'m> {
    map: &'m Map<BTreeMap<Position, usize>>,
    path: Vec<(Position, Vec<Position>)>,
}

impl<'m> Iterator for PathEnumerator<'m> {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.path.is_empty() {
                break None;
            } else {
                let (segment, nexts) = self.path.last_mut().unwrap();

                if *segment == self.map.end() {
                    let res = self.path.iter().map(|elem| elem.0.clone()).collect();
                    self.path.pop();
                    break Some(res);
                } else {
                    let next = nexts.pop();
                    if let Some(next) = next {
                        if self.path.iter().all(|(elem, _)| elem != &next) {
                            let neighbors = self
                                .map
                                .path
                                .get(&next)
                                .into_iter()
                                .flat_map(|elem| elem.keys().cloned())
                                .collect();
                            self.path.push((next, neighbors));
                        }
                    } else {
                        self.path.pop();
                    }
                }
            }
        }
    }
}

impl Map<BTreeMap<Position, usize>> {
    fn enumerate_paths(&self) -> PathEnumerator<'_> {
        let neighbors = self
            .path
            .get(&self.start())
            .unwrap()
            .keys()
            .cloned()
            .collect();
        PathEnumerator {
            map: self,
            path: vec![(self.start(), neighbors)],
        }
    }

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
        let mut cache: BTreeMap<_, Option<usize>> = BTreeMap::new();

        let target_key = (self.start(), BTreeSet::from([self.start()]), self.end());

        let mut todo = Vec::from([target_key.clone()]);

        'outer: while let Some(ref key @ (ref start, ref not_visiting, ref end)) = todo.pop() {
            let mut max = None;
            for (neighbor, delta) in self.path.get(start).unwrap() {
                if not_visiting.contains(neighbor) {
                    continue;
                } else if neighbor == end {
                    max = Some(max.take().map_or(*delta, |elem: usize| elem.max(*delta)));
                    continue;
                }

                let mut new_not_visiting = not_visiting.clone();
                new_not_visiting.insert(neighbor.clone());

                if let Some(cache) =
                    cache.get(&(neighbor.clone(), new_not_visiting.clone(), end.clone()))
                {
                    if let Some(&cache) = cache.as_ref() {
                        max = Some(
                            max.take()
                                .map_or(cache + delta, |elem: usize| elem.max(cache + delta)),
                        );
                    }
                } else {
                    todo.push(key.clone());
                    todo.push((neighbor.clone(), new_not_visiting, end.clone()));
                    continue 'outer;
                }
            }

            cache
                .entry(key.clone())
                .and_modify(|old: &mut Option<usize>| {
                    *old = match (old.take(), max) {
                        (None, None) => None,
                        (None, Some(v)) | (Some(v), None) => Some(v),
                        (Some(a), Some(b)) => Some(a.max(b)),
                    }
                })
                .or_insert(max);
        }

        cache.get(&target_key).unwrap().unwrap()
    }

    fn simplify(&mut self) {
        let nodes: Vec<_> = self.path.keys().cloned().collect();
        for n1 in nodes {
            let neighbors: Vec<_> = self
                .path
                .get(&n1)
                .into_iter()
                .flatten()
                .map(|(key, val)| (key.clone(), *val))
                .collect();
            for (n2, d1) in neighbors {
                let neighbors_neighbors: Vec<_> = self
                    .path
                    .get(&n2)
                    .into_iter()
                    .flatten()
                    .map(|(key, val)| (key.clone(), *val))
                    .collect();
                if neighbors_neighbors.len() <= 2 {
                    for (n3, d2) in neighbors_neighbors {
                        if n1 == n3 {
                            continue;
                        }

                        self.path.remove(&n2);

                        let node1 = self.path.get_mut(&n1).unwrap();
                        node1.remove(&n2);
                        node1.insert(n3.clone(), d1 + d2);

                        if let Some(node3) = self.path.get_mut(&n3) {
                            node3.remove(&n2);
                            node3.insert(n1.clone(), d1 + d2);
                        }
                    }
                }
            }
        }
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
    let mut map = parse_input(input).collapse(false);
    map.simplify();
    map.print_graph(&mut std::fs::File::create("./day23-p2.elkt").unwrap());
    map.enumerate_paths()
        .map(|path| {
            path.windows(2)
                .map(|elems| map.path.get(&elems[0]).unwrap().get(&elems[1]).unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day23.example.txt"
    ));
    assert_eq!(part1(input), 94);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day23.txt"
    ));
    assert_eq!(part1(input), 2034);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day23.example.txt"
    ));
    assert_eq!(part2(input), 154);
}

#[test]
#[ignore = "slow"]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day23.txt"
    ));
    assert_eq!(part2(input), 6302);
}
