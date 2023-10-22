use std::ops::Range;

#[derive(Debug, Clone)]
struct Square {
    x: Range<u32>,
    y: Range<u32>,
}

impl Square {
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty()
    }

    fn size(&self) -> usize {
        self.x.len() * self.y.len()
    }

    fn overlap(&self, other: &Self) -> Option<Square> {
        Some(Square {
            x: self.x.start.max(other.x.start)..self.x.end.min(other.x.end),
            y: self.y.start.max(other.y.start)..self.y.end.min(other.y.end),
        })
        .filter(|square| !square.is_empty())
    }

    fn without(&self, other: &Self) -> Vec<Square> {
        // no overlap just return as is
        if let Some(overlap) = self.overlap(other) {
            let pre_y = Square {
                x: self.x.clone(),
                y: self.y.start..self.y.end.min(overlap.y.start),
            };

            let post_y = Square {
                x: self.x.clone(),
                y: self.y.start.max(overlap.y.end)..self.y.end,
            };

            let remaining_y = Square {
                x: self.x.clone(),
                y: overlap.y.clone(),
            };

            let pre_x = Square {
                x: remaining_y.x.start..remaining_y.x.end.min(overlap.x.start),
                y: remaining_y.y.clone(),
            };

            let post_x = Square {
                x: remaining_y.x.start.max(overlap.x.end)..remaining_y.x.end,
                y: remaining_y.y.clone(),
            };

            let mut squares = vec![pre_y, post_y, pre_x, post_x];
            squares.retain(|square| !square.is_empty());
            squares
        } else {
            vec![Square {
                x: self.x.clone(),
                y: self.y.clone(),
            }]
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Action, Square)> + '_ {
    input.lines().filter_map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let (action, start, end) = match parts.as_slice() {
            ["turn", "on", start, "through", end] => (Action::On, start, end),
            ["turn", "off", start, "through", end] => (Action::Off, start, end),
            ["toggle", start, "through", end] => (Action::Toggle, start, end),
            _ => return None,
        };
        let (start_x, start_y) = start.split_once(',')?;
        let (end_x, end_y) = end.split_once(',')?;
        let start_x = start_x.parse().ok()?;
        let start_y = start_y.parse().ok()?;
        let end_x: u32 = end_x.parse().ok()?;
        let end_y: u32 = end_y.parse().ok()?;
        Some((
            action,
            Square {
                x: start_x..(end_x + 1),
                y: start_y..(end_y + 1),
            },
        ))
    })
}

// TODO optimize this
pub fn part1(input: &str) -> usize {
    let iter = parse_input(input);
    let mut lights = vec![];

    for (action, square) in iter {
        match action {
            Action::On => {
                let mut new = lights.iter().fold(vec![square], |acc, current| {
                    acc.iter()
                        .flat_map(|square| square.without(current))
                        .collect()
                });
                lights.append(&mut new);
            }
            Action::Off => {
                lights = lights
                    .iter()
                    .flat_map(|current| current.without(&square))
                    .collect()
            }
            Action::Toggle => {
                let mut new = lights.iter().fold(vec![square.clone()], |acc, current| {
                    acc.iter()
                        .flat_map(|square| square.without(current))
                        .collect()
                });
                lights = lights
                    .iter()
                    .flat_map(|current| current.without(&square))
                    .collect();
                lights.append(&mut new);
            }
        }
    }

    lights.iter().map(|square| square.size()).sum()
}

pub fn part2(input: &str) -> usize {
    let iter = parse_input(input);
    let mut lights: Vec<(Square, usize)> = vec![];

    for (action, square) in iter {

        let unchanged = lights
            .iter()
            .flat_map(|&(ref current, intensity)| {
                current.without(&square).into_iter().map(move |square| (square, intensity))
            });

        let updated = lights.iter().flat_map(|&(ref current, intensity)| {
            current.overlap(&square).map(move |square| (square, match action {
                Action::On => intensity + 1,
                Action::Off => intensity.saturating_sub(1),
                Action::Toggle => intensity + 2,
            }))
        });

        let new = lights
            .iter()
            .fold(vec![square.clone()], |acc, (current, _)| {
                acc.iter()
                    .flat_map(|square| square.without(current))
                    .collect()
            }).into_iter().map(|square| (square, match action {
                Action::On =>  1,
                Action::Off => 0,
                Action::Toggle => 2,
            }));

        lights = unchanged.chain(updated.chain(new)).filter(|(square, intensity)| (!square.is_empty()) && (*intensity > 0)).collect();
    }

    lights.iter().map(|(square, intensity)| square.size()*intensity).sum()
}

#[test]
fn part1_example1() {
    let input = "turn on 0,0 through 999,999";
    assert_eq!(part1(input), 1_000_000);
}

#[test]
fn part1_example2() {
    let input = "toggle 0,0 through 999,0";
    assert_eq!(part1(input), 1_000);
}

#[test]
fn part1_example3() {
    let input = "turn off 499,499 through 500,500";
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day06.txt"));
    assert_eq!(part1(input), 400410);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day06.txt"));
    assert_eq!(part2(input), 15343601);
}
