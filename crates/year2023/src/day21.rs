use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: isize,
    column: isize,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .map(|elem| Position {
                row: self.row + elem.0,
                column: self.column + elem.1,
            })
            .into_iter()
            .collect()
    }
}

struct Input {
    start: Position,
    width: isize,
    height: isize,
    garden_plots: HashSet<Position>,
}

fn parse_input(input: &str) -> Input {
    let mut start = None;
    let paths = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let start = &mut start;
            line.chars()
                .enumerate()
                .filter_map(move |(column, symbol)| match symbol {
                    '#' => None,
                    '.' => Some(Position {
                        row: row as isize,
                        column: column as isize,
                    }),
                    'S' => {
                        let pos = Position {
                            row: row as isize,
                            column: column as isize,
                        };
                        *start = Some(pos);
                        Some(pos)
                    }
                    _ => None,
                })
                .collect::<HashSet<_>>()
        })
        .collect();
    Input {
        start: start.unwrap(),
        height: input.lines().count() as isize,
        width: input
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as isize,
        garden_plots: paths,
    }
}

impl Input {
    fn reachable_in_exactly(&self, steps: usize) -> usize {
        let mut current_parity = HashSet::new();
        let mut other_parity = HashSet::from([self.start]);

        let mut current_parity_count = 1;
        let mut other_parity_count = 0;

        for idx in 1..=steps {
            std::mem::swap(&mut current_parity_count, &mut other_parity_count);

            let reached_from_new: HashSet<_> = other_parity
                .iter()
                .flat_map(|elem| elem.neighbors())
                .filter(|&pos| self.contains(pos))
                .collect();
            current_parity = reached_from_new
                .difference(&current_parity)
                .copied()
                .collect();

            current_parity_count += current_parity.len();

            std::mem::swap(&mut other_parity, &mut current_parity);

            if other_parity.is_empty() && idx % 2 == steps % 2 {
                break;
            }
        }

        current_parity_count
    }

    fn distances(&self, steps: usize) -> HashMap<Position, usize> {
        let mut result = HashMap::from([(self.start, 0)]);

        let mut current_parity = HashSet::new();
        let mut other_parity = HashSet::from([self.start]);

        for idx in 1..=steps {
            let reached_from_new: HashSet<_> = other_parity
                .iter()
                .flat_map(|elem| elem.neighbors())
                .filter(|&pos| self.contains(pos))
                .collect();

            current_parity = reached_from_new
                .difference(&current_parity)
                .copied()
                .collect();

            for pos in current_parity.iter() {
                result.insert(*pos, idx);
            }

            std::mem::swap(&mut other_parity, &mut current_parity);
        }

        result
    }

    fn contains(&self, mut pos: Position) -> bool {
        pos.row = pos.row.rem_euclid(self.height);
        pos.column = pos.column.rem_euclid(self.width);
        self.garden_plots.contains(&pos)
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.reachable_in_exactly(64)
}

pub fn part2(input: &str) -> usize {
    // based on <https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21>

    let input = parse_input(input);
    const STEPS: isize = 26501365;

    assert_eq!(input.width, input.height);
    assert!(input.width % 2 == 1);
    assert_eq!(input.start.row, input.start.column);
    assert_eq!(input.width / 2, input.start.column);
    assert_eq!((STEPS - input.start.column) % input.width, 0);

    let radius = ((STEPS - input.start.column) / input.width) as usize;

    assert!(radius % 2 == 0);

    let distances = Input {
        start: input.start,
        // expand size because we don't want to wrap around
        width: input.width * 3,
        height: input.height * 3,
        garden_plots: input.garden_plots.clone(),
    }
    .distances(input.height as usize * 2 + 1);

    let full_odd = distances.values().filter(|&&dist| dist % 2 == 1).count();
    let full_even = distances.values().filter(|&&dist| dist % 2 == 0).count();

    let odd_corners = distances
        .values()
        .filter(|&&dist| dist % 2 == 1 && dist > 65)
        .count();
    let even_corners = distances
        .values()
        .filter(|&&dist| dist % 2 == 0 && dist > 65)
        .count();

    (radius + 1).pow(2) * full_odd + radius.pow(2) * full_even - (radius + 1) * odd_corners
        + radius * even_corners
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day21.example.txt"
    ));
    let input = parse_input(input);
    assert_eq!(input.reachable_in_exactly(6), 16);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day21.txt"
    ));
    assert_eq!(part1(input), 3687);
}

#[test]
#[ignore = "slow"]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day21.example.txt"
    ));
    let input = parse_input(input);

    let examples = [
        (6, 16),
        (10, 50),
        (50, 1594),
        (100, 6536),
        (500, 167004),
        (1000, 668697),
        (5000, 16733044),
    ];

    for (steps, reachable) in examples {
        println!("Steps: {steps}");
        assert_eq!(input.reachable_in_exactly(steps), reachable);
    }
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day21.txt"
    ));
    assert_eq!(part2(input), 610321885082978);
}
