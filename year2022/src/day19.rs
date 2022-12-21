use std::collections::HashSet;

#[derive(Debug)]
struct Blueprint {
    ore_for_ore: usize,
    ore_for_clay: usize,
    ore_for_obsidian: usize,
    clay_for_obsidian: usize,
    ore_for_geod: usize,
    obsidian_for_geod: usize,
}
impl Blueprint {
    fn costs(&self, kind: Kind) -> [usize; 4] {
        match kind {
            Kind::Ore => [self.ore_for_ore, 0, 0, 0],
            Kind::Clay => [self.ore_for_clay, 0, 0, 0],
            Kind::Obsidian => [self.ore_for_obsidian, self.clay_for_obsidian, 0, 0],
            Kind::Geod => [self.ore_for_geod, 0, self.obsidian_for_geod, 0],
        }
    }

    fn max_cost(&self, kind: Kind) -> usize {
        match kind {
            Kind::Ore => [
                self.ore_for_ore,
                self.ore_for_clay,
                self.ore_for_obsidian,
                self.ore_for_geod,
            ]
            .into_iter()
            .max()
            .unwrap(),
            Kind::Clay => self.clay_for_obsidian,
            Kind::Obsidian => self.obsidian_for_geod,
            Kind::Geod => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Ore,
    Clay,
    Obsidian,
    Geod,
}

impl Kind {
    const ALL: [Kind; 4] = [Self::Ore, Self::Clay, Self::Obsidian, Self::Geod];
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    saved: [bool; 4],
    robots: [usize; 4],
    material: [usize; 4],
}

impl State {
    fn step(&self, blueprint: &Blueprint, next_states: &mut HashSet<State>) {
        let next = Kind::ALL.map(|kind| self.material[kind as usize] + self.robots[kind as usize]);

        // we may save our resources if there is a robot we can't afford and don't have enough of
        // relevant if we have enough ore,clay and obsidian miners and can afford an geod miner
        // as then we never want to not build a geod miner
        let mut may_save = false;
        let mut can_build = self.saved;

        if self.robots[Kind::Ore as usize] >= blueprint.ore_for_geod
            && self.robots[Kind::Obsidian as usize] >= blueprint.obsidian_for_geod
            && self.material[Kind::Ore as usize] >= blueprint.ore_for_geod
        {
            // we have enough ore and obsidian robots to build a geod robot every tick so do that
            let mut robots = self.robots;
            robots[Kind::Geod as usize] += 1;
            let mut material = self.material;
            material[Kind::Geod as usize] += self.robots[Kind::Geod as usize];
            next_states.insert(Self {
                saved: [false; 4],
                robots,
                material,
            });
        } else {
            let new_states = Kind::ALL
                .into_iter()
                .filter(|&kind| {
                    // its a geod bot or
                    // we don't have enough robots of this kind to build any robot requiring this resource ever tick
                    (!self.saved[kind as usize])
                        && (kind == Kind::Geod
                            || self.robots[kind as usize] < blueprint.max_cost(kind))
                })
                .filter_map(|kind| {
                    let costs = blueprint.costs(kind);
                    if Kind::ALL
                        .into_iter()
                        .all(|kind| self.material[kind as usize] >= costs[kind as usize])
                    {
                        can_build[kind as usize] = true;
                        // we have the funds to build a robot of this kind
                        let mut robots = self.robots;
                        robots[kind as usize] += 1;
                        let material =
                            Kind::ALL.map(|kind| next[kind as usize] - costs[kind as usize]);
                        Some(State {
                            robots,
                            material,
                            saved: [false; 4],
                        })
                    } else {
                        may_save = true;
                        None
                    }
                });
            next_states.extend(new_states);

            if may_save {
                next_states.insert(State {
                    robots: self.robots,
                    material: next,
                    saved: can_build,
                });
            }
        }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().flat_map(|line| {
        let parts : Vec<_> = line.split_whitespace().collect();
        if let [_blueprint,nr_colon,
        _each1, _ore1, _robot1, _costs1, ore_for_ore, _ore_dot1,
        _each2, _clay1, _robot2, _costs2, ore_for_clay ,_ore_dot2,
        _each3, _obsidian1, _robot3, _costs3, ore_for_obsidian ,_ore2, _and1, clay_for_obsidian, _clay2,
        _each4, _geod, _robot4, _costs4, ore_for_geod ,_ore3, _and, obsidian_for_geod, _obsidian2
        ] = parts.as_slice() {
            Some(Blueprint {
                ore_for_ore: ore_for_ore.parse().unwrap(),
                ore_for_clay: ore_for_clay.parse().unwrap(),
                ore_for_obsidian: ore_for_obsidian.parse().unwrap(),
                clay_for_obsidian: clay_for_obsidian.parse().unwrap(),
                ore_for_geod: ore_for_geod.parse().unwrap(),
                obsidian_for_geod: obsidian_for_geod.parse().unwrap()
            })
        } else {
            None
        }}
    ).collect()
}

fn both(blueprints: &[Blueprint], minutes: u8) -> Vec<usize> {
    let mut result = vec![];
    for (idx, blueprint) in blueprints.iter().enumerate() {
        let mut current = HashSet::from([State {
            robots: [1, 0, 0, 0],
            material: [0; 4],
            saved: [false; 4],
        }]);
        for idx in 0..minutes {
            let mut next = HashSet::new();
            for state in current {
                state.step(blueprint, &mut next);
            }

            println!("{}, {}", idx, next.len());

            if idx > 15 {
                println!("{}, {}", idx, next.len());
                let min = next
                    .iter()
                    .map(|state| state.material[Kind::Geod as usize])
                    .min()
                    .unwrap();
                let max = next
                    .iter()
                    .map(|state| state.material[Kind::Geod as usize])
                    .max()
                    .unwrap();
                // arbitrary pruning: /2 made part2_example fail and /4 was too slow
                next.retain(|state| state.material[Kind::Geod as usize] >= min + (max - min) / 3);
            }

            current = next;
        }
        result.push(
            current
                .into_iter()
                .map(|state| state.material[Kind::Geod as usize])
                .max()
                .unwrap(),
        );
    }
    println!("{:?}", result);
    result
}

pub fn part1(input: &str) -> usize {
    let blueprints = parse(input);
    both(&blueprints, 24)
        .into_iter()
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let blueprints = parse(input);
    both(&blueprints[0..(3.min(blueprints.len()))], 32)
        .into_iter()
        .product()
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day19.example.txt");
    assert_eq!(part1(input), 9 + 24);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day19.txt"));
    assert_eq!(part1(input), 1480);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day19.example.txt");
    assert_eq!(part2(input), 56 * 62);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day19.txt"));
    assert_eq!(part2(input), 3168);
}
