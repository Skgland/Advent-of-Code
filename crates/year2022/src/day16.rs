use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Valve<'input> {
    name: &'input str,
    flow_rate: usize,
    connections: Vec<&'input str>,
}

fn parse(input: &str) -> Vec<Valve<'_>> {
    input
        .lines()
        .map(|line| {
            let (valve, tunnel) = line.split_once("; ").unwrap();
            let name = &valve["Valve ".len()..][..2];
            let flow_rate = valve["Valve AA has flow rate=".len()..].parse().unwrap();
            let connections = tunnel
                .strip_prefix("tunnels lead to valves ")
                .or_else(|| tunnel.strip_prefix("tunnel leads to valve "))
                .unwrap()
                .split(", ")
                .collect();
            Valve {
                name,
                flow_rate,
                connections,
            }
        })
        .collect()
}

fn flow(valves_open: u32, interesting_valves: &[Valve]) -> usize {
    interesting_valves
        .iter()
        .enumerate()
        .filter(|(idx, _)| is_open(valves_open, *idx))
        .map(|(_, valve)| valve.flow_rate)
        .sum()
}

pub fn part1(input: &str) -> usize {
    let valves = parse(input);
    let valves_by_name = valves
        .iter()
        .map(|valve| (valve.name, valve))
        .collect::<HashMap<_, _>>();
    let interesting_valves = valves
        .iter()
        .filter(|valve| valve.flow_rate != 0)
        .cloned()
        .collect::<Vec<_>>();
    let interesting_valves_by_name = interesting_valves
        .iter()
        .enumerate()
        .map(|valve| (valve.1.name, valve))
        .collect::<HashMap<_, _>>();

    let interesting_count = interesting_valves.len();
    println!("Non-Brocken Valves: {}", interesting_count);

    assert!(interesting_count <= u32::BITS as _);

    let mut current = HashMap::from([((0, "AA"), 0)]);

    let time = 0;

    for step in 0..30 {
        let mut next = HashMap::new();
        for ((valves_open, pos), relieve) in current {
            let new_relieve = relieve + flow(valves_open, &interesting_valves);
            if let Some((idx, valve)) = interesting_valves_by_name.get(pos) {
                if !is_open(valves_open, *idx) {
                    next.entry((valves_open | (1 << idx), pos))
                        .and_modify(|old| *old = new_relieve.max(*old))
                        .or_insert(new_relieve);
                }
            }
            for neighbor in valves_by_name.get(pos).unwrap().connections.iter() {
                next.entry((valves_open, *neighbor))
                    .and_modify(|old| *old = new_relieve.max(*old))
                    .or_insert(new_relieve);
            }
        }

        println!("Step {:2} size: {:6}", step, next.len());
        current = next;
    }

    current
        .into_iter()
        .max_by_key(|(_key, value)| *value)
        .unwrap()
        .1
}

fn is_open(valves_open: u32, idx: usize) -> bool {
    valves_open & (1 << idx) != 0
}

pub fn part2(input: &str) -> usize {
    let valves = parse(input);
    let valves_by_name = valves
        .iter()
        .map(|valve| (valve.name, valve))
        .collect::<HashMap<_, _>>();
    let interesting_valves = valves
        .iter()
        .filter(|valve| valve.flow_rate != 0)
        .cloned()
        .collect::<Vec<_>>();
    let interesting_valves_by_name = interesting_valves
        .iter()
        .enumerate()
        .map(|valve| (valve.1.name, valve))
        .collect::<HashMap<_, _>>();

    let interesting_count = interesting_valves.len();
    println!("Non-Brocken Valves: {}", interesting_count);

    assert!(interesting_count <= u32::BITS as _);

    let mut current = HashMap::from([((0, "AA", "AA"), 0)]);

    let time = 0;

    for step in 0..26 {
        let mut next = HashMap::new();
        for ((valves_open, pos_me, pos_elephant), relieve) in current {
            let new_relieve = relieve + flow(valves_open, &interesting_valves);

            let my_valve = interesting_valves_by_name.get(pos_me);
            let elephant_valve = interesting_valves_by_name.get(pos_elephant);

            // see if both are in front of a different closed interesting valve
            if pos_me != pos_elephant {
                if let (Some(my_valve), Some(elephant_valve)) = (my_valve, elephant_valve) {
                    if !is_open(valves_open, my_valve.0) && !is_open(valves_open, elephant_valve.0)
                    {
                        // both have different valves and neither is already open, so each can open their valve
                        next.entry((
                            valves_open | (1 << my_valve.0) | (1 << elephant_valve.0),
                            pos_me,
                            pos_elephant,
                        ))
                        .and_modify(|old| *old = new_relieve.max(*old))
                        .or_insert(new_relieve);
                    }
                }
            }

            for neighbor_me in valves_by_name.get(pos_me).unwrap().connections.iter() {
                // have the elephant open their valve if they can
                if let Some(elephant_valve) = elephant_valve {
                    if valves_open & (1 << elephant_valve.0) == 0 {
                        // elephant opens valve and I move
                        next.entry((
                            valves_open | (1 << elephant_valve.0),
                            neighbor_me,
                            pos_elephant,
                        ))
                        .and_modify(|old| *old = new_relieve.max(*old))
                        .or_insert(new_relieve);
                    }
                }

                for neighbor_elephant in
                    valves_by_name.get(pos_elephant).unwrap().connections.iter()
                {
                    // neither open valve, both move
                    next.entry((valves_open, *neighbor_me, *neighbor_elephant))
                        .and_modify(|old| *old = new_relieve.max(*old))
                        .or_insert(new_relieve);

                    // I open my valve if I can
                    if let Some(my_valve) = my_valve {
                        if valves_open & (1 << my_valve.0) == 0 {
                            // elephant opens valve and I move
                            next.entry((
                                valves_open | (1 << my_valve.0),
                                pos_me,
                                neighbor_elephant,
                            ))
                            .and_modify(|old| *old = new_relieve.max(*old))
                            .or_insert(new_relieve);
                        }
                    }
                }
            }
        }
        println!("Step {:2} pre-pruning size: {:6}", step, next.len());
        if next.len() < 50_000 {
            current = next;
        } else {
            // start pruning, only keep those above the middle
            let max = *next.values().max().unwrap();
            let min = *next.values().min().unwrap();
            let threshold = min + (max - min) / 2;
            next.retain(|_, value| *value >= threshold);
            current = next;
        }
    }

    current
        .into_iter()
        .max_by_key(|(_key, value)| *value)
        .unwrap()
        .1
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day16.example.txt"
    ));
    assert_eq!(part1(input), 1651);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day16.txt"
    ));
    assert_eq!(part1(input), 1488);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day16.example.txt"
    ));
    assert_eq!(part2(input), 1707);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day16.txt"
    ));
    assert_eq!(part2(input), 2111);
}
