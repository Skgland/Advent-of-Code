use std::{cell::RefCell, ops::DerefMut};

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
                println!("'{}'", num);
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
                if new % monkey.test == 0 {
                    success_monkey.items.push(new);
                } else {
                    fail_monkey.items.push(new);
                }
            }
        }

        print_state(round + 1, &monkeys);
    }

    monkeys.sort_by_key(|elem| elem.borrow().1);

    let counts = monkeys
        .iter()
        .map(|elem| elem.borrow().1)
        .collect::<Vec<_>>();

    println!("{:?}", counts);

    monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|cell| cell.borrow().1)
        .product()
}

fn print_state(round: usize, monkeys: &[RefCell<(Monkey, u64)>]) {
    println!("Round {}", round);
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", idx, monkey.borrow().0.items)
    }
    println!()
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day11.txt"
    ));
    assert_eq!(part1(input), 98280);
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day11.txt"
    ));
    assert_eq!(part2(input), 17673687232);
}
