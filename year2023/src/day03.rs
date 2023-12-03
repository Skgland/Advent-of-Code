use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn neighbors(&self, number: usize) -> impl Iterator<Item = Position> {
        let x = self.x;
        let y = self.y;
        let length = number.ilog10() + 1;
        (-1..=length as isize)
            .flat_map(move |offset| {
                [
                    Position {
                        x: x + offset,
                        y: y - 1,
                    },
                    Position {
                        x: x + offset,
                        y: y + 1,
                    },
                ]
            })
            .chain([
                Position { x: x - 1, y },
                Position {
                    x: x + length as isize,
                    y,
                },
            ])
    }
}

#[derive(Debug, Default)]
struct Input {
    numbers: HashMap<Position, usize>,
    symbols: HashMap<Position, char>,
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .fold(Input::default(), |mut acc, (y, line)| {
            let (state, next) = line
                .chars()
                .enumerate()
                .map(Some)
                .chain(std::iter::once(None))
                .fold((Input::default(), None), |(mut state, num), cur| {
                    match (num, cur) {
                        (None, None) => (state, None),
                        (None, Some((x, char))) => match char {
                            '0'..='9' => (state, Some((x as isize, (char as u8 - b'0') as usize))),
                            '.' => (state, None),
                            symbol => {
                                state.symbols.insert(
                                    Position {
                                        x: x as isize,
                                        y: y as isize,
                                    },
                                    symbol,
                                );
                                (state, None)
                            }
                        },
                        (Some((x, number)), None) => {
                            state.numbers.insert(Position { x, y: y as isize }, number);
                            (state, None)
                        }
                        (Some((x, number)), Some((cur_x, char))) => match char {
                            '0'..='9' => {
                                (state, Some((x, number * 10 + (char as u8 - b'0') as usize)))
                            }
                            '.' => {
                                state.numbers.insert(Position { x, y: y as isize }, number);
                                (state, None)
                            }
                            symbol => {
                                state.numbers.insert(Position { x, y: y as isize }, number);
                                state.symbols.insert(
                                    Position {
                                        x: cur_x as isize,
                                        y: y as isize,
                                    },
                                    symbol,
                                );
                                (state, None)
                            }
                        },
                    }
                });
            assert!(next.is_none());
            acc.numbers.extend(state.numbers);
            acc.symbols.extend(state.symbols);
            acc
        })
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input
        .numbers
        .iter()
        .filter_map(|(number_position, &number)| {
            number_position
                .neighbors(number)
                .any(|neighbor| {
                    input
                        .symbols
                        .keys()
                        .any(|symbol_position| symbol_position == &neighbor)
                })
                .then_some(number)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    input
        .symbols
        .iter()
        .filter(|&(_, &symbol)| symbol == '*')
        .filter_map(|(symbol_position, _)| {
            let neighbors = input
                .numbers
                .iter()
                .filter(|&(number_position, &number)| {
                    number_position
                        .neighbors(number)
                        .any(|neighbor| &neighbor == symbol_position)
                })
                .map(|(_, number)| number)
                .collect::<Vec<_>>();
            if neighbors.len() == 2 {
                Some(neighbors[0] * neighbors[1])
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day03.example.txt"));
    assert_eq!(part1(input), 4361);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day03.txt"));
    assert_eq!(part1(input), 525181);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day03.example.txt");
    assert_eq!(part2(input), 467835);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day03.txt"));
    assert_eq!(part2(input), 84289137);
}
