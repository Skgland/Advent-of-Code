use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Packet {
    data: Vec<ListOrInt>,
}

#[derive(Debug, Clone)]
enum ListOrInt {
    List(Vec<ListOrInt>),
    Int(u64),
}

impl Eq for ListOrInt {}

impl PartialEq for ListOrInt {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for ListOrInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListOrInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ListOrInt::List(left), ListOrInt::List(right)) => {
                let mut left = left.as_slice();
                let mut right = right.as_slice();

                while let ([l, l_rem @ ..], [r, r_rem @ ..]) = (left, right) {
                    left = l_rem;
                    right = r_rem;
                    return match l.cmp(r) {
                        std::cmp::Ordering::Less => Ordering::Less,
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => Ordering::Greater,
                    };
                }
                left.len().cmp(&right.len())
            }
            (list @ ListOrInt::List(_), ListOrInt::Int(i)) => {
                list.cmp(&ListOrInt::List(vec![ListOrInt::Int(*i)]))
            }
            (ListOrInt::Int(i), list @ ListOrInt::List(_)) => {
                ListOrInt::List(vec![ListOrInt::Int(*i)]).cmp(list)
            }
            (ListOrInt::Int(l), ListOrInt::Int(r)) => l.cmp(r),
        }
    }
}

fn parse(input: &str) -> Vec<[Packet; 2]> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunck| match chunck {
            &[l, r, ..] => [l, r].map(parse_packet),
            _ => panic!(),
        })
        .collect()
}

fn parse_packet(packet: &str) -> Packet {
    let mut stack = vec![];
    let mut current_list = vec![];
    let mut current_int = None;

    let packet = packet.strip_prefix('[').unwrap();

    for c in packet.chars() {
        match c {
            '[' => {
                stack.push(std::mem::take(&mut current_list));
            }
            ']' => {
                if let Some(i) = current_int.take() {
                    current_list.push(ListOrInt::Int(i))
                }

                if let Some(mut parent) = stack.pop() {
                    parent.push(ListOrInt::List(std::mem::take(&mut current_list)));
                    current_list = parent;
                } else {
                    return Packet { data: current_list };
                }
            }
            ',' => {
                if let Some(i) = current_int.take() {
                    current_list.push(ListOrInt::Int(i))
                }
            }
            '0'..='9' => {
                let ci = current_int.get_or_insert(0);
                *ci *= 10;
                *ci += (c.to_digit(10)).unwrap() as u64;
            }
            _ => panic!("Unexpected char {}", c),
        }
    }

    panic!()
}

pub fn part1(input: &str) -> usize {
    let input = parse(input);

    input
        .iter()
        .enumerate()
        .map(
            |(idx, [left, right])| {
                if left <= right {
                    dbg!(idx + 1)
                } else {
                    0
                }
            },
        )
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut input = parse(input);
    let two = Packet {
        data: vec![ListOrInt::Int(2)],
    };
    let six = Packet {
        data: vec![ListOrInt::Int(6)],
    };
    input.push([two.clone(), six.clone()]);
    let mut input = input
        .into_iter()
        .flat_map(|elem| elem.into_iter())
        .collect::<Vec<_>>();
    input.sort();
    let two_idx = input.binary_search(&two).unwrap();
    let six_idx = input.binary_search(&six).unwrap();

    (two_idx + 1) * (six_idx + 1)
}

#[test]
fn packet_eq() {
    let examples = [
        ListOrInt::Int(1),
        ListOrInt::Int(2),
        ListOrInt::Int(3),
        ListOrInt::Int(4),
        ListOrInt::List(vec![
            ListOrInt::List(vec![ListOrInt::Int(1)]),
            ListOrInt::Int(4),
        ]),
    ];

    for example in examples {
        assert_eq!(example, example);
    }
}

#[test]
fn parse_example() {
    let input = include_str!("../input/day13.example.txt");
    let input = parse(input);

    let expected = vec![
        [
            Packet {
                data: vec![
                    ListOrInt::Int(1),
                    ListOrInt::Int(1),
                    ListOrInt::Int(3),
                    ListOrInt::Int(1),
                    ListOrInt::Int(1),
                ],
            },
            Packet {
                data: vec![
                    ListOrInt::Int(1),
                    ListOrInt::Int(1),
                    ListOrInt::Int(5),
                    ListOrInt::Int(1),
                    ListOrInt::Int(1),
                ],
            },
        ],
        [
            Packet {
                data: vec![
                    ListOrInt::List(vec![ListOrInt::Int(1)]),
                    ListOrInt::List(vec![
                        ListOrInt::Int(2),
                        ListOrInt::Int(3),
                        ListOrInt::Int(4),
                    ]),
                ],
            },
            Packet {
                data: vec![ListOrInt::List(vec![ListOrInt::Int(1)]), ListOrInt::Int(4)],
            },
        ],
        [
            Packet {
                data: vec![ListOrInt::Int(9)],
            },
            Packet {
                data: vec![ListOrInt::List(vec![
                    ListOrInt::Int(8),
                    ListOrInt::Int(7),
                    ListOrInt::Int(6),
                ])],
            },
        ],
        [
            Packet {
                data: vec![
                    ListOrInt::List(vec![ListOrInt::Int(4), ListOrInt::Int(4)]),
                    ListOrInt::Int(4),
                    ListOrInt::Int(4),
                ],
            },
            Packet {
                data: vec![
                    ListOrInt::List(vec![ListOrInt::Int(4), ListOrInt::Int(4)]),
                    ListOrInt::Int(4),
                    ListOrInt::Int(4),
                    ListOrInt::Int(4),
                ],
            },
        ],
        [
            Packet {
                data: vec![
                    ListOrInt::Int(7),
                    ListOrInt::Int(7),
                    ListOrInt::Int(7),
                    ListOrInt::Int(7),
                ],
            },
            Packet {
                data: vec![ListOrInt::Int(7), ListOrInt::Int(7), ListOrInt::Int(7)],
            },
        ],
        [
            Packet { data: vec![] },
            Packet {
                data: vec![ListOrInt::Int(3)],
            },
        ],
        [
            Packet {
                data: vec![ListOrInt::List(vec![ListOrInt::List(vec![])])],
            },
            Packet {
                data: vec![ListOrInt::List(vec![])],
            },
        ],
        [
            Packet {
                data: vec![
                    ListOrInt::Int(1),
                    ListOrInt::List(vec![
                        ListOrInt::Int(2),
                        ListOrInt::List(vec![
                            ListOrInt::Int(3),
                            ListOrInt::List(vec![
                                ListOrInt::Int(4),
                                ListOrInt::List(vec![
                                    ListOrInt::Int(5),
                                    ListOrInt::Int(6),
                                    ListOrInt::Int(7),
                                ]),
                            ]),
                        ]),
                    ]),
                    ListOrInt::Int(8),
                    ListOrInt::Int(9),
                ],
            },
            Packet {
                data: vec![
                    ListOrInt::Int(1),
                    ListOrInt::List(vec![
                        ListOrInt::Int(2),
                        ListOrInt::List(vec![
                            ListOrInt::Int(3),
                            ListOrInt::List(vec![
                                ListOrInt::Int(4),
                                ListOrInt::List(vec![
                                    ListOrInt::Int(5),
                                    ListOrInt::Int(6),
                                    ListOrInt::Int(0),
                                ]),
                            ]),
                        ]),
                    ]),
                    ListOrInt::Int(8),
                    ListOrInt::Int(9),
                ],
            },
        ],
    ];
    assert_eq!(input.len(), expected.len());
    for i in 0..input.len() {
        assert_eq!(input[i], expected[i]);
    }
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day13.example.txt");
    assert_eq!(part1(input), 1 + 2 + 4 + 6);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day13.txt"));
    assert_eq!(part1(input), 6235);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day13.example.txt");
    assert_eq!(part2(input), 10 * 14);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day13.txt"));
    assert_eq!(part2(input), 22866);
}
