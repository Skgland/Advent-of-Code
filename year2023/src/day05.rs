use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Debug)]
struct Mapping {
    dest_start: usize,
    source_range: RangeInclusive<usize>,
}

impl Mapping {

    fn map(&self, val: RangeInclusive<usize>) -> (Option<RangeInclusive<usize>>, [Option<RangeInclusive<usize>>; 2]) {

        match (self.source_range.contains(val.start()), self.source_range.contains(val.end())) {
            (true, true) =>  {
                (Some(val.start() - self.source_range.start() + self.dest_start..= val.end() - self.source_range.start() + self.dest_start), [None, None])
            },
            (true, false) => {
                (Some(val.start() - self.source_range.start() + self.dest_start ..= self.dest_start + self.source_range.clone().count()), [Some(self.source_range.end()+1..=*val.end()), None])
            },
            (false, true) => {
                (Some(self.dest_start ..= val.end() - self.source_range.start() + self.dest_start), [Some(*val.start()..=self.source_range.start()-1), None])
            },
            (false, false) => {
                if self.source_range.start() <= val.end() && self.source_range.end() >= val.start(){
                    // val completely covers mapping
                    (Some(self.dest_start..=self.source_range.clone().count()), [Some(*val.start()..=self.source_range.start()-1), Some(self.source_range.end()+1..=*val.end())])
                } else {
                    // no overlap
                    (None, [Some(val), None])
                }
            },
        }

    }
}

#[derive(Debug)]
struct Mappings {
    dest: String,
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn map(
        &self,
        range: RangeInclusive<usize>,
    ) -> impl Iterator<Item = RangeInclusive<usize>> + '_ {

        let mut todo_next = vec![range];
        let mut result = vec![];

            for mapping in &self.mappings {
                for range in std::mem::take(&mut todo_next) {
                    let (mapped, unmapped) =  mapping.map(range);

                    todo_next.extend(unmapped.into_iter().flatten().filter(|elem|!elem.is_empty()));
                    result.extend(mapped.into_iter());
                }
            }

        result.append(&mut todo_next);

        result.into_iter()
    }
}


#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps_by_src: HashMap<String, Mappings>,
}

impl Almanac {
    fn lookup(&self, target: &str, ranges: bool) -> impl Iterator<Item = RangeInclusive<usize>> {
        let mut current_name = "seed";

        let mut current_values: Vec<RangeInclusive<usize>> = if !ranges {
            self.seeds.iter().map(|&elem| elem..=elem).collect()
        } else {
            self.seeds
                .chunks(2)
                .map(|elem| elem[0]..=elem[0] + elem[1] - 1)
                .collect()
        };

        while current_name != target {
            let mapping = &self.maps_by_src[current_name];
            current_name = &mapping.dest;
            current_values = current_values
                .into_iter()
                .flat_map(|val| mapping.map(val))
                .collect();
        }

        current_values.into_iter()
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();
    let seed_line = lines.next().unwrap();
    assert!(lines.next().unwrap().trim().is_empty());

    let seeds = seed_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|elem| elem.parse().unwrap())
        .collect();

    let mappings = lines
        .map(Some)
        .chain(std::iter::once(None))
        .scan(None, |acc: &mut Option<(_, Mappings)>, cur| {
            let Some(cur) = cur else {
                return Some(acc.take());
            };

            let cur = cur.trim();

            if let Some((_, state)) = acc {
                if cur.is_empty() {
                    Some(acc.take())
                } else {
                    let parts = cur.split(' ').collect::<Vec<_>>();
                    let [dest_start, src_start, len] = parts.as_slice() else {
                        panic!()
                    };
                    let from = src_start.parse().unwrap();
                    state.mappings.push(Mapping {
                        dest_start: dest_start.parse().unwrap(),
                        source_range: from..=(from + len.parse::<usize>().unwrap() - 1),
                    });
                    Some(None)
                }
            } else {
                let cur = cur.trim().strip_suffix(" map:").unwrap();
                let parts = cur.split('-').collect::<Vec<_>>();
                let [src, _to, dest] = parts.as_slice() else {
                    panic!()
                };
                *acc = Some((
                    src.to_string(),
                    Mappings {
                        dest: dest.to_string(),
                        mappings: vec![],
                    },
                ));
                Some(None)
            }
        })
        .flatten()
        .collect();
    Almanac {
        seeds,
        maps_by_src: mappings,
    }
}

pub fn part1(input: &str) -> usize {
    parse_input(input).lookup("location", false).map(|range|*range.start()).min().unwrap()
}

pub fn part2(input: &str) -> usize {
    parse_input(input).lookup("location", true).map(|range|*range.start()).min().unwrap()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day05.example.txt"));
    assert_eq!(part1(input), 35);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day05.txt"));
    assert_eq!(part1(input), 403695602);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day05.example.txt");
    assert_eq!(part2(input), 46);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day05.txt"));
    assert_eq!(part2(input), 219529182);
}
