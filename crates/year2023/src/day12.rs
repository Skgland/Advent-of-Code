use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum SpringCondition {
    Good,
    Bad,
    Unknown,
}

struct Row {
    list: Vec<SpringCondition>,
    contiguous_bad: Vec<u8>,
}

impl Row {
    fn unfold(&mut self) {
        self.list.insert(0, SpringCondition::Unknown);
        self.list = std::iter::repeat(std::mem::take(&mut self.list))
            .take(5)
            .flatten()
            .collect();
        self.list.remove(0);

        self.contiguous_bad = std::iter::repeat(std::mem::take(&mut self.contiguous_bad))
            .take(5)
            .flatten()
            .collect()
    }

    fn valid_permutations(&self) -> usize {
        let mut suffix_cache = HashMap::new();

        fn solve_suffix(
            mut list: &[SpringCondition],
            cont: &[u8],
            cache: &mut HashMap<(usize, usize), usize>,
        ) -> usize {
            match cont {
                [] => {
                    if list.iter().all(|elem| {
                        matches!(elem, SpringCondition::Good | SpringCondition::Unknown)
                    }) {
                        1
                    } else {
                        // no streak left but still bad springs left
                        0
                    }
                }
                &[next_streak, ref remaining_streaks @ ..] => {
                    // skip leading good springs
                    while let [SpringCondition::Good, rem @ ..] = list {
                        list = rem
                    }
                    match list {
                        [SpringCondition::Bad, ..] => {
                            solve_bad_suffix(next_streak as usize, remaining_streaks, list, cache)
                        }
                        [SpringCondition::Unknown, remaining_list @ ..] => {
                            let key = (list.len(), cont.len());
                            if let Some(&value) = cache.get(&key) {
                                value
                            } else {
                                let good = solve_suffix(remaining_list, cont, cache);
                                let bad = solve_bad_suffix(
                                    next_streak as usize,
                                    remaining_streaks,
                                    list,
                                    cache,
                                );
                                let value = good + bad;
                                cache.insert(key, value);
                                value
                            }
                        }
                        _ => {
                            // end of list but still more streaks left
                            0
                        }
                    }
                }
            }
        }

        // assume the next bad streak is starting now
        fn solve_bad_suffix(
            streak: usize,
            remaining_streaks: &[u8],
            list: &[SpringCondition],
            cache: &mut HashMap<(usize, usize), usize>,
        ) -> usize {
            if list.len() < streak {
                // too few springs remaining
                return 0;
            }

            if list[..streak]
                .iter()
                .any(|elem| matches!(elem, SpringCondition::Good))
            {
                // interrupted streak
                return 0;
            }

            match list.get(streak..) {
                Some([SpringCondition::Good | SpringCondition::Unknown, remaining_list @ ..]) => {
                    solve_suffix(remaining_list, remaining_streaks, cache)
                }
                Some([SpringCondition::Bad, ..]) => {
                    // streak too long
                    0
                }
                Some([]) | None => {
                    if remaining_streaks.is_empty() {
                        1
                    } else {
                        0
                    }
                }
            }
        }

        solve_suffix(&self.list, &self.contiguous_bad, &mut suffix_cache)
    }
}

#[test]
fn check_valid_permutations() {
    assert_eq!(
        Row::from_str("???.### 1,1,3").unwrap().valid_permutations(),
        1
    );
    assert_eq!(
        Row::from_str(".??..??...?##. 1,1,3")
            .unwrap()
            .valid_permutations(),
        4
    );
    assert_eq!(
        Row::from_str("?#?#?#?#?#?#?#? 1,3,1,6")
            .unwrap()
            .valid_permutations(),
        1
    );
    assert_eq!(
        Row::from_str("????.#...#... 4,1,1")
            .unwrap()
            .valid_permutations(),
        1
    );
    assert_eq!(
        Row::from_str("????.######..#####. 1,6,5")
            .unwrap()
            .valid_permutations(),
        4
    );
    assert_eq!(
        Row::from_str("?###???????? 3,2,1")
            .unwrap()
            .valid_permutations(),
        10
    );
}

impl FromStr for Row {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (list, cont) = line.split_once(' ').ok_or(())?;
        #[allow(clippy::wildcard_in_or_patterns)]
        let list = list
            .chars()
            .map(|c| match c {
                '#' => SpringCondition::Bad,
                '.' => SpringCondition::Good,
                '?' | _ => SpringCondition::Unknown,
            })
            .collect();

        let contiguous_bad = cont
            .split(',')
            .map(|elem| elem.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ())?;

        Ok(Row {
            list,
            contiguous_bad,
        })
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Row> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn part1(input: &str) -> usize {
    parse_input(input).map(|row| row.valid_permutations()).sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|mut row| {
            row.unfold();
            row.valid_permutations()
        })
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day12.example.txt"
    ));
    assert_eq!(part1(input), 1 + 4 + 1 + 1 + 4 + 10);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day12.txt"
    ));
    assert_eq!(part1(input), 6827);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day12.example.txt"
    ));
    assert_eq!(part2(input), 1 + 16384 + 1 + 16 + 2500 + 506250);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day12.txt"
    ));
    assert_eq!(part2(input), 1537505634471);
}
