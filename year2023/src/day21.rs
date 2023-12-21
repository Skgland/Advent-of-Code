use std::collections::HashSet;

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
        let mut other_parity = HashSet::new();
        let mut new = HashSet::from([self.start]);

        let report = if steps > 1000 {steps/100} else {steps +1};

        for idx in 1..=steps {
            if idx % report == 0 {
                println!("Step {idx}: {}", new.len());
            }
            let reached_from_new: HashSet<_> = new
                .into_iter()
                .flat_map(|elem| elem.neighbors())
                .filter(|&pos| self.contains(pos))
                .collect();
            new = reached_from_new
                .difference(&other_parity)
                .copied()
                .collect();
            other_parity.extend(new.iter().copied());
            std::mem::swap(&mut other_parity, &mut current_parity);

            if new.is_empty() && idx % 2 == steps % 2 {
                break;
            }
        }

        current_parity.len()
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
    let input = parse_input(input);
    input.reachable_in_exactly(26501365)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day21.example.txt"));
    let input = parse_input(input);
    assert_eq!(input.reachable_in_exactly(6), 16);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day21.txt"));
    assert_eq!(part1(input), 3687);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!("../input/day21.example.txt"));
    let input = parse_input(input);

    let examples = [
        (6, 16),
        (10, 50),
        (50, 1594),
        (100, 6536),
        (500, 167004),
        (1000, 668697),
        (5000, 16733044)
    ];

    for (steps, reachable) in examples {
        println!("Steps: {steps}");
        assert_eq!(input.reachable_in_exactly(steps), reachable);
    }
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day21.txt"));
    assert_eq!(part2(input), 1262);
}
