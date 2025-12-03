use std::collections::{BTreeMap, BTreeSet};

use helper::{TASKS, Task};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day20.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day20.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "20", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "20", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

type Pos = [isize; 2];

struct Input {
    map: BTreeMap<Pos, Tile>,
    start: Pos,
    end: Pos,
}

fn parse_input(input: &str) -> Input {
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
                            '.' => Tile::Empty,
                            '#' => Tile::Wall,
                            'S' => Tile::Start,
                            'E' => Tile::End,
                            _ => panic!("Unexpected char {char} in input"),
                        };
                        (pos, tile)
                    }
                })
            }
        })
        .collect::<BTreeMap<_, _>>();

    Input {
        start: *map
            .iter()
            .find(|(_, tile)| matches!(tile, Tile::Start))
            .unwrap()
            .0,
        end: *map
            .iter()
            .find(|(_, tile)| matches!(tile, Tile::End))
            .unwrap()
            .0,
        map,
    }
}

fn dijkstra(start: Pos, end: Pos, map: &BTreeMap<Pos, Tile>) -> Option<Vec<Pos>> {
    let mut visisted = BTreeSet::new();
    let mut todo = BTreeMap::<_, _>::from([(0, vec![start])]);
    let mut predecessor = BTreeMap::new();

    loop {
        let (current_score, todos) = todo.pop_first()?;

        let mut reached_end = false;
        for current in todos {
            if !visisted.insert(current) {
                continue;
            }

            if current == end {
                reached_end = true;
            }

            for next in neighbors(current) {
                if map
                    .get(&next)
                    .is_some_and(|tile| !matches!(tile, Tile::Wall))
                    && !visisted.contains(&next)
                {
                    predecessor.insert(next, current);
                    todo.entry(current_score + 1).or_default().push(next);
                }
            }
        }

        if reached_end {
            break;
        }
    }

    let mut cur = end;
    let mut path = vec![end];
    while let Some(prev) = predecessor.get(&cur).copied() {
        cur = prev;
        path.push(prev);
    }
    Some(path)
}

fn neighbors(current: Pos) -> [Pos; 4] {
    [
        [current[0] - 1, current[1]],
        [current[0] + 1, current[1]],
        [current[0], current[1] - 1],
        [current[0], current[1] + 1],
    ]
}

fn manhattan(a: Pos, b: Pos) -> usize {
    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn find_cheats(input: &Input, cheat_length: usize) -> BTreeMap<(Pos, Pos), isize> {
    let path = dijkstra(input.start, input.end, &input.map).unwrap();

    let mut cheats = BTreeMap::new();
    for start_idx in 0..path.len() - 1 {
        let (&[.., start], ends) = path.split_at(start_idx + 1) else {
            unreachable!()
        };

        for (end_idx, &end) in ends.iter().enumerate() {
            let distance = manhattan(start, end);
            if distance <= cheat_length {
                let saving = end_idx as isize - distance as isize + 1;
                cheats.insert((start, end), saving);
            }
        }
    }

    cheats
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    find_cheats(&input, 2)
        .iter()
        .filter(|(_cheat, saving)| **saving >= 100)
        .count()
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    find_cheats(&input, 20)
        .iter()
        .filter(|(_cheat, saving)| **saving >= 100)
        .count()
}

#[test]
fn part1_example1() {
    let input = parse_input(INPUT_EXAMPLE1);
    let statistic = find_cheats(&input, 2).iter().fold(
        BTreeMap::<_, usize>::new(),
        |mut acc, (_cheat, saving)| {
            if *saving > 0 {
                *acc.entry(*saving).or_default() += 1;
            }
            acc
        },
    );
    assert_eq!(
        statistic,
        BTreeMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ])
    );
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 1490);
}

#[test]
fn part2_example1() {
    let input = parse_input(INPUT_EXAMPLE1);
    let statistic = find_cheats(&input, 20).iter().fold(
        BTreeMap::<_, usize>::new(),
        |mut acc, (_cheat, saving)| {
            if *saving >= 50 {
                *acc.entry(*saving).or_default() += 1;
            }
            acc
        },
    );
    assert_eq!(
        statistic,
        BTreeMap::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ])
    );
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 1011325);
}
