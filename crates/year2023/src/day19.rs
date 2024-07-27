use std::{collections::HashMap, ops::RangeInclusive};

enum Action<'s> {
    Accept,
    Reject,
    Goto(&'s str),
}

impl<'s> Action<'s> {
    fn parse(action: &'s str) -> Self {
        match action {
            "A" => Self::Accept,
            "R" => Self::Reject,
            other => Self::Goto(other),
        }
    }
}

struct Workflow<'s> {
    cases: Vec<Case<'s>>,
    otherwise: Action<'s>,
}
impl<'w> Workflow<'w> {
    fn process<'s>(&'s self, item: &&Item<u32>) -> &'s Action<'w> {
        for case in &self.cases {
            let item_value = match case.property {
                Property::XtremelyCool => item.xtremely_cool,
                Property::Musical => item.musical,
                Property::Aerodynamic => item.aerodynamic,
                Property::Shiny => item.shiny,
            };

            if match case.comparison {
                Condition::Lt => item_value < case.value,
                Condition::Gt => item_value > case.value,
            } {
                return &case.action;
            }
        }
        &self.otherwise
    }

    #[allow(clippy::type_complexity)]
    fn bulk_process(
        &self,
        items: Item<std::ops::RangeInclusive<u32>>,
    ) -> (u64, Vec<(&'w str, Item<RangeInclusive<u32>>)>) {
        let mut todo = Vec::new();
        let mut accepted = 0;

        let mut remaining = items;

        for case in &self.cases {
            let (current, rest) = remaining.split(case.property, case.comparison, case.value);
            remaining = rest;
            match &case.action {
                Action::Accept => {
                    accepted += current.count();
                }
                Action::Reject => {}
                Action::Goto(next) => {
                    if current.count() != 0 {
                        todo.push((*next, current));
                    }
                }
            }
        }

        match &self.otherwise {
            Action::Accept => {
                accepted += remaining.count();
            }
            Action::Reject => {}
            Action::Goto(next) => {
                if remaining.count() != 0 {
                    todo.push((*next, remaining));
                }
            }
        }

        (accepted, todo)
    }
}

struct Case<'s> {
    property: Property,
    comparison: Condition,
    value: u32,
    action: Action<'s>,
}

#[derive(Debug, Clone, Copy)]
enum Property {
    XtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    Lt,
    Gt,
}

#[derive(Debug, Clone)]
struct Item<Val> {
    xtremely_cool: Val,
    musical: Val,
    aerodynamic: Val,
    shiny: Val,
}
impl Item<RangeInclusive<u32>> {
    fn count(&self) -> u64 {
        self.xtremely_cool.clone().count() as u64
            * self.musical.clone().count() as u64
            * self.aerodynamic.clone().count() as u64
            * self.shiny.clone().count() as u64
    }

    fn split(&self, property: Property, comparison: Condition, value: u32) -> (Self, Self) {
        let old_range = match property {
            Property::XtremelyCool => self.xtremely_cool.clone(),
            Property::Musical => self.musical.clone(),
            Property::Aerodynamic => self.aerodynamic.clone(),
            Property::Shiny => self.shiny.clone(),
        };

        let (current, remaining) = match comparison {
            Condition::Lt => (
                (*old_range.start())..=(*old_range.end()).min(value - 1),
                (*old_range.start()).max(value)..=(*old_range.end()),
            ),
            Condition::Gt => (
                (*old_range.start()).max(value + 1)..=(*old_range.end()),
                (*old_range.start())..=(*old_range.end()).min(value),
            ),
        };

        match property {
            Property::XtremelyCool => (
                Self {
                    xtremely_cool: current,
                    ..self.clone()
                },
                Self {
                    xtremely_cool: remaining,
                    ..self.clone()
                },
            ),
            Property::Musical => (
                Self {
                    musical: current,
                    ..self.clone()
                },
                Self {
                    musical: remaining,
                    ..self.clone()
                },
            ),
            Property::Aerodynamic => (
                Self {
                    aerodynamic: current,
                    ..self.clone()
                },
                Self {
                    aerodynamic: remaining,
                    ..self.clone()
                },
            ),
            Property::Shiny => (
                Self {
                    shiny: current,
                    ..self.clone()
                },
                Self {
                    shiny: remaining,
                    ..self.clone()
                },
            ),
        }
    }
}

struct Input<'s> {
    workflows: HashMap<&'s str, Workflow<'s>>,
    items: Vec<Item<u32>>,
}
impl Input<'_> {
    fn accepts(&self, item: &&Item<u32>) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        loop {
            match workflow.process(item) {
                Action::Accept => break true,
                Action::Reject => break false,
                Action::Goto(next) => workflow = self.workflows.get(next).unwrap(),
            }
        }
    }

    fn acceptable(&self) -> u64 {
        let mut todo = vec![(
            "in",
            Item {
                xtremely_cool: 1..=4000,
                musical: 1..=4000,
                aerodynamic: 1..=4000,
                shiny: 1..=4000,
            },
        )];
        let mut accepted = 0;

        while let Some((workflow_name, items)) = todo.pop() {
            let workflow = self.workflows.get(workflow_name).unwrap();
            let (local_accepted, mut process_further) = workflow.bulk_process(items);
            accepted += local_accepted;
            todo.append(&mut process_further)
        }

        accepted
    }
}

fn parse_input(input: &str) -> Input<'_> {
    let mut lines = input.lines();

    let workflows = (&mut lines)
        .take_while(|elem| !elem.is_empty())
        .map(|line| {
            let (name, parts) = line.split_once('{').unwrap();
            let mut parts = parts.trim_end_matches('}').split(',');

            let otherwise = Action::parse(parts.next_back().unwrap());

            let cases = parts
                .map(|case| {
                    let (cond, action) = case.split_once(':').unwrap();

                    Case {
                        property: match &cond[0..1] {
                            "x" => Property::XtremelyCool,
                            "m" => Property::Musical,
                            "a" => Property::Aerodynamic,
                            "s" => Property::Shiny,
                            other => panic!("Unexpected Property {other}"),
                        },
                        comparison: match &cond[1..2] {
                            "<" => Condition::Lt,
                            ">" => Condition::Gt,
                            other => panic!("Unexpected Condition {other}"),
                        },
                        value: cond[2..].parse().unwrap(),
                        action: Action::parse(action),
                    }
                })
                .collect();

            (name, Workflow { cases, otherwise })
        })
        .collect();

    let items = lines
        .map(|line| {
            let mut parts = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',');
            Item {
                xtremely_cool: parts
                    .next()
                    .unwrap()
                    .trim_start_matches("x=")
                    .parse()
                    .unwrap(),
                musical: parts
                    .next()
                    .unwrap()
                    .trim_start_matches("m=")
                    .parse()
                    .unwrap(),
                aerodynamic: parts
                    .next()
                    .unwrap()
                    .trim_start_matches("a=")
                    .parse()
                    .unwrap(),
                shiny: parts
                    .next()
                    .unwrap()
                    .trim_start_matches("s=")
                    .parse()
                    .unwrap(),
            }
        })
        .collect();

    Input { workflows, items }
}

pub fn part1(input: &str) -> u32 {
    let input = parse_input(input);

    input
        .items
        .iter()
        .filter(|item| input.accepts(item))
        .map(|item| item.xtremely_cool + item.musical + item.aerodynamic + item.shiny)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let input = parse_input(input);
    input.acceptable()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day19.example.txt"
    ));
    assert_eq!(part1(input), 19114);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day19.txt"
    ));
    assert_eq!(part1(input), 480738);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day19.example.txt"
    ));
    assert_eq!(part2(input), 167409079868000);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day19.txt"
    ));
    assert_eq!(part2(input), 131550418841958);
}
