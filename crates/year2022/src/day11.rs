use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{cell::RefCell, ops::DerefMut};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day11.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "11", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "11", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    success: usize,
    fail: usize,
}

enum Operation {
    Mul(usize),
    Add(usize),
    Sq,
}
impl Operation {
    fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Mul(op) => item * op,
            Operation::Add(op) => item + op,
            Operation::Sq => item * item,
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let lines: Vec<_> = input.lines().collect();
    lines
        .chunks(7)
        .map(|lines| match lines {
            &[_monkey, items, operation, test, success, fail, ..] => {
                let items = items
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|elem| elem.parse().unwrap())
                    .collect();
                let (op, num) = operation
                    .split_once("= old ")
                    .unwrap()
                    .1
                    .split_once(' ')
                    .unwrap();
                let op = match op {
                    "*" => num.parse().map_or(Operation::Sq, Operation::Mul),
                    "+" => Operation::Add(num.parse().unwrap()),
                    _ => panic!(),
                };
                let test = test
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let success = success
                    .strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let fail = fail
                    .strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();

                Monkey {
                    items,
                    op,
                    test,
                    success,
                    fail,
                }
            }
            _ => panic!(),
        })
        .collect()
}

fn both(monkeys: Vec<Monkey>, iteration: usize, reduction: impl Fn(usize) -> usize) -> u64 {
    let mut monkeys = monkeys
        .into_iter()
        .map(|monkey| RefCell::new((monkey, 0)))
        .collect::<Vec<_>>();

    for round in 0..iteration {
        for entry in monkeys.iter() {
            let mut pair = entry.borrow_mut();
            let (monkey, inspections) = pair.deref_mut();

            let success_monkey = &mut monkeys.get(monkey.success).unwrap().borrow_mut().0;
            let fail_monkey = &mut monkeys.get(monkey.fail).unwrap().borrow_mut().0;

            for item in std::mem::take(&mut monkey.items) {
                *inspections += 1;
                let new = monkey.op.apply(item);
                let new = reduction(new);
                if new.is_multiple_of(monkey.test) {
                    success_monkey.items.push(new);
                } else {
                    fail_monkey.items.push(new);
                }
            }
        }

        print_state(round + 1, &monkeys);
    }

    monkeys.sort_by_key(|elem| elem.borrow().1);

    if log::log_enabled!(log::Level::Debug) {
        let counts = monkeys
            .iter()
            .map(|elem| elem.borrow().1)
            .collect::<Vec<_>>();

        log::debug!("{:?}", counts);
    }

    monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|cell| cell.borrow().1)
        .product()
}

fn print_state(round: usize, monkeys: &[RefCell<(Monkey, u64)>]) {
    if log::log_enabled!(log::Level::Debug) {
        log::debug!("Round {}", round);
        for (idx, monkey) in monkeys.iter().enumerate() {
            log::debug!("Monkey {}: {:?}", idx, monkey.borrow().0.items)
        }
        log::debug!("")
    }
}

pub fn part1(input: &str) -> u64 {
    let monkeys = parse(input);
    both(monkeys, 20, |x| x / 3)
}

pub fn part2(input: &str) -> u64 {
    let monkeys = parse(input);
    let lcm: usize = monkeys.iter().map(|monkey| monkey.test).product();
    both(monkeys, 10_000, |x| x % lcm)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day11.example.txt"
    ));
    assert_eq!(part1(input), 101 * 105);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 98280);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day11.example.txt"
    ));
    assert_eq!(part2(input), 52166 * 52013);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 17673687232);
}
