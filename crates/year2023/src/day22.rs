use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day22.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "22", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "22", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Clone, Debug)]
struct Quader {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}
impl Quader {
    fn xy_overlaps(&self, settled: &Quader) -> bool {
        ranges_overlap(&self.x, &settled.x) && ranges_overlap(&self.y, &settled.y)
    }
}

fn ranges_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.end() >= b.start() && b.end() >= a.start()
}

#[derive(Debug)]
struct Tower {
    bricks: Vec<Quader>,
    rests_on: HashMap<usize, Vec<usize>>,
    supports: HashMap<usize, Vec<usize>>,
}

impl Tower {
    fn safe_to_disintegrate(&self, idx: usize) -> bool {
        self.supports.get(&idx).is_none_or(|supported| {
            supported.iter().all(|supported| {
                self
                    .rests_on
                    .get(supported)
                    .expect("This brick is supported by a brick such it should rest on at least that brick")
                    .len()
                    > 1
            })
        })
    }

    fn settle(&mut self) {
        for settling in 0..self.bricks.len() {
            let brick = self.bricks[settling].clone();
            let mut dest_z = 1;
            let mut may_rest_on = Vec::new();
            for (idx, settled) in self.bricks[0..settling].iter().enumerate() {
                if brick.xy_overlaps(settled) {
                    may_rest_on.push(idx);
                    dest_z = dest_z.max(*settled.z.end() + 1)
                }
            }
            for (idx, settled) in may_rest_on
                .into_iter()
                .map(|elem| (elem, &self.bricks[elem]))
            {
                if settled.z.end() + 1 == dest_z {
                    self.rests_on.entry(settling).or_default().push(idx);
                    self.supports.entry(idx).or_default().push(settling);
                }
            }
            let brick = &mut self.bricks[settling];
            brick.z = dest_z..=(brick.z.end() - brick.z.start() + dest_z);
        }
    }

    fn count_fallen_if_disintegrated(&self, idx: usize) -> usize {
        let mut falling = HashSet::from([idx]);

        let mut check = vec![];
        let mut check_next = self.supports.get(&idx).cloned().unwrap_or_default();

        while !check_next.is_empty() {
            std::mem::swap(&mut check, &mut check_next);
            for check in check.drain(..) {
                if self.rests_on[&check]
                    .iter()
                    .all(|elem| falling.contains(elem))
                {
                    falling.insert(check);
                    check_next.extend(
                        self.supports
                            .get(&check)
                            .iter()
                            .flat_map(|elem| elem.iter())
                            .copied(),
                    );
                }
            }
        }

        // -1 as we include the disintegrated brick
        falling.len() - 1
    }
}

fn parse_input(input: &str) -> Tower {
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start_parts = start.splitn(3, ',').collect::<Vec<_>>();
            let end_parts = end.splitn(3, ',').collect::<Vec<_>>();
            let [x1, y1, z1] = start_parts.as_slice() else {
                panic!()
            };
            let [x2, y2, z2] = end_parts.as_slice() else {
                panic!()
            };
            let [x1, x2, y1, y2, z1, z2] =
                [x1, x2, y1, y2, z1, z2].map(|elem| elem.parse().unwrap());
            Quader {
                x: x1..=x2,
                y: y1..=y2,
                z: z1..=z2,
            }
        })
        .collect();
    bricks.sort_by_key(|brick| *brick.z.start());
    Tower {
        bricks,
        supports: HashMap::default(),
        rests_on: HashMap::default(),
    }
}

pub fn part1(input: &str) -> usize {
    let mut tower = parse_input(input);
    tower.settle();
    (0..tower.bricks.len())
        .filter(|&idx| tower.safe_to_disintegrate(idx))
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut tower = parse_input(input);
    tower.settle();
    (0..tower.bricks.len())
        .map(|idx| tower.count_fallen_if_disintegrated(idx))
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day22.example.txt"
    ));
    assert_eq!(part1(input), 5);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day22.txt"
    ));
    assert_eq!(part1(input), 471);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day22.example.txt"
    ));
    assert_eq!(part2(input), 7);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day22.txt"
    ));
    assert_eq!(part2(input), 68525);
}
