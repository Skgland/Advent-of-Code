use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

impl std::ops::Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

struct Module<'m> {
    dest: Vec<&'m str>,
    kind: ModuleKind<'m>,
}
impl<'m> Module<'m> {
    fn handle<'s, 'd, 'q>(
        &mut self,
        src: &'s str,
        this: &'d str,
        pulse: Pulse,
    ) -> Vec<QueuedPulse<'q>>
    where
        'm: 'q,
        'd: 'q,
    {
        self.kind
            .handle(pulse, src)
            .into_iter()
            .flat_map(|send| {
                self.dest.iter().map(move |&dest| QueuedPulse {
                    src: this,
                    dest,
                    pulse: send,
                })
            })
            .collect()
    }
}
enum ModuleKind<'m> {
    FlipFlop { state: Pulse },
    Conjunction { inputs: HashMap<&'m str, Pulse> },
    Broadcaster,
    Target { received_low: bool },
}

impl ModuleKind<'_> {
    fn handle(&mut self, pulse: Pulse, src: &str) -> Option<Pulse> {
        let send = match self {
            ModuleKind::FlipFlop { state } => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *state = !*state;
                    Some(*state)
                }
            },
            ModuleKind::Conjunction { inputs } => {
                *inputs.get_mut(src).expect("Missing Input Connection") = pulse;
                if inputs.values().all(|elem| matches!(elem, Pulse::High)) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleKind::Broadcaster => Some(pulse),
            ModuleKind::Target { received_low } => {
                if let Pulse::Low = pulse {
                    *received_low = true;
                }
                None
            }
        };
        send
    }
}

struct QueuedPulse<'m> {
    src: &'m str,
    dest: &'m str,
    pulse: Pulse,
}

struct State<'m> {
    modules: HashMap<&'m str, Module<'m>>,
}

impl State<'_> {
    fn push_button(&mut self) -> (u32, u32) {
        let mut low_count = 0;
        let mut high_count = 0;

        let mut queue = VecDeque::from([QueuedPulse {
            src: "button",
            dest: "broadcaster",
            pulse: Pulse::Low,
        }]);

        while let Some(QueuedPulse { src, dest, pulse }) = queue.pop_front() {
            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }
            match self.modules.get_mut(dest) {
                None => {}
                Some(module) => queue.extend(module.handle(src, dest, pulse)),
            }
        }

        (low_count, high_count)
    }
}

fn parse_input(input: &str) -> State {
    let mut modules: HashMap<_, _> = input
        .lines()
        .filter_map(|line| {
            let (module, dest) = line.split_once(" -> ")?;

            let (kind, src) = if let Some(module) = module.strip_prefix('&') {
                (
                    ModuleKind::Conjunction {
                        inputs: HashMap::default(),
                    },
                    module,
                )
            } else if let Some(module) = module.strip_prefix('%') {
                (ModuleKind::FlipFlop { state: Pulse::Low }, module)
            } else if module == "broadcaster" {
                (ModuleKind::Broadcaster, module)
            } else {
                return None;
            };
            Some((
                src,
                Module {
                    dest: dest.split(", ").collect(),
                    kind,
                },
            ))
        })
        .collect();

    let deps = modules.iter().fold(
        HashMap::new(),
        |mut state: HashMap<&str, HashMap<&str, Pulse>>, (&src, module)| {
            for &dest in &module.dest {
                state.entry(dest).or_default().insert(src, Pulse::Low);
            }
            state
        },
    );

    for (dest, srcs) in deps {
        if let Some(Module {
            dest: _,
            kind: ModuleKind::Conjunction { inputs },
        }) = modules.get_mut(dest)
        {
            *inputs = srcs
        }
    }

    State { modules }
}

pub fn part1(input: &str) -> u32 {
    let mut state = parse_input(input);
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = state.push_button();
        low += l;
        high += h;
    }
    low * high
}

/// Note: this assumes that the graph follows some properties
/// that are not stated in the problem statement, but which appear to hold for all inputs
pub fn part2(input: &str) -> u64 {
    let mut state = parse_input(input);

    let prev = state
        .modules
        .iter()
        .find_map(|(&elem, module)| module.dest.contains(&"rx").then_some(elem))
        .unwrap();
    let counter_outputs: Vec<_> = state
        .modules
        .iter()
        .filter_map(|(&elem, module)| module.dest.contains(&prev).then_some(elem))
        .collect();

    for counter_output in &counter_outputs {
        state.modules.insert(
            counter_output,
            Module {
                dest: vec![],
                kind: ModuleKind::Target {
                    received_low: false,
                },
            },
        );
    }

    let mut count = 0;

    let mut counter: Vec<_> = counter_outputs.into_iter().map(|elem| (elem, 0)).collect();

    loop {
        count += 1;
        state.push_button();

        for (name, reset_at) in &mut counter {
            if let Some(Module {
                dest: _,
                kind: ModuleKind::Target { received_low: true },
            }) = state.modules.get(name)
            {
                if *reset_at == 0 {
                    *reset_at = count;
                }
            }
        }

        if counter.iter().all(|(_, val)| *val != 0) {
            break;
        }
    }

    dbg!(counter)
        .into_iter()
        .fold(1, |acc, (_, cur)| helper::lcm(acc, cur))
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day20.example1.txt"
    ));
    assert_eq!(part1(input), 8000 * 4000);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day20.example2.txt"
    ));
    assert_eq!(part1(input), 4250 * 2750);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day20.txt"
    ));
    assert_eq!(part1(input), 869395600);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day20.txt"
    ));
    assert_eq!(part2(input), 232605773145467);
}

pub fn print_graph() {
    use std::fmt::Write;

    let input = parse_input(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day20.txt"
    )));
    let mut nodes_out = String::new();
    let mut edges_out = String::new();

    let mut done = HashSet::from(["broadcaster"]);

    let mut todo = VecDeque::from(["broadcaster"]);

    while let Some(elem) = todo.pop_front() {
        match input.modules.get(elem) {
            Some(Module { dest, kind }) => {
                match kind {
                    ModuleKind::FlipFlop { .. } => {
                        _ = writeln!(nodes_out, "node {elem} {{ label \"%{elem}\" }}")
                    }
                    ModuleKind::Conjunction { .. } => {
                        _ = writeln!(nodes_out, "node {elem} {{ label \"&{elem}\" }}")
                    }
                    ModuleKind::Broadcaster => {
                        _ = writeln!(nodes_out, "node {elem} {{ label \"Start\" }}")
                    }
                    ModuleKind::Target { .. } => {
                        _ = writeln!(nodes_out, "node {elem} {{ label \"Target\" }}")
                    }
                }

                for &dest in dest {
                    _ = writeln!(edges_out, "edge {elem} -> {dest}")
                }
                todo.extend(dest.iter().filter(|elem| done.insert(elem)).copied());
            }
            None => {
                if elem == "rx" {
                    _ = writeln!(nodes_out, "node {elem} {{ label \"Target\" }}");
                } else {
                    _ = writeln!(nodes_out, "node {elem}");
                }
            }
        }
    }

    if let Ok(mut file) = std::fs::File::create("day20.elkt") {
        use std::io::Write;
        _ = writeln!(file, "algorithm: org.eclipse.elk.stress");
        _ = writeln!(file, "org.eclipse.elk.stress.desiredEdgeLength: 150");
        _ = writeln!(file, "{nodes_out}");
        _ = writeln!(file, "{edges_out}");
    }
}
