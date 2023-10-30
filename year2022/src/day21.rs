use std::{collections::HashMap, rc::Rc};

type Variable = str;
pub enum Expression<'a> {
    Constant(i64),
    BinOp(&'a Variable, &'a Variable, Op),
}

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn eval(&self, l: i64, r: i64) -> i64 {
        match self {
            Op::Add => l + r,
            Op::Sub => l - r,
            Op::Mul => l * r,
            Op::Div => l / r,
        }
    }
}

#[derive(Clone, Debug)]
pub enum State {
    Constant(i64),
    BinOp(Rc<State>, Rc<State>, Op),
    Eq(Rc<State>, Rc<State>),
    Input,
}
impl State {
    fn simplify(&mut self) {
        loop {
            let new_self = match self {
                State::Constant(_) => break,
                State::BinOp(l, r, op) => match (l.as_ref(), r.as_ref()) {
                    (State::Constant(l), State::Constant(r)) => State::Constant(op.eval(*l, *r)),
                    (State::Eq(_, _), _) | (_, State::Eq(_, _)) => break, // a valid input should never reach this state
                    (State::Input, State::Constant(val)) => match (val, op) {
                        (0, Op::Add | Op::Sub) => State::Input,
                        (1, Op::Mul | Op::Div) => State::Input,
                        _ => break,
                    },
                    (State::Constant(val), State::Input) => match (val, op) {
                        (0, Op::Add) => State::Input,
                        (1, Op::Mul) => State::Input,
                        _ => break,
                    },
                    (State::Input, State::Input) => match op {
                        Op::Add => {
                            State::BinOp(Rc::new(Self::Input), Rc::new(Self::Constant(2)), Op::Mul)
                        }
                        Op::Sub => State::Constant(0),
                        Op::Mul => break,
                        Op::Div => State::Constant(1),
                    },
                    _ => break,
                },
                State::Eq(l, r) => {
                    println!("({:?}) == ({:?})", l, r);
                    match (l.as_ref(), r.as_ref()) {
                        (State::Constant(val), State::BinOp(l, r, op))
                        | (State::BinOp(l, r, op), State::Constant(val)) => {
                            match (l.as_ref(), r.as_ref()) {
                                (State::Constant(val2), _) => match op {
                                    Op::Add => {
                                        State::Eq(Rc::new(State::Constant(val - val2)), r.clone())
                                    }
                                    Op::Sub => {
                                        State::Eq(Rc::new(State::Constant(val2 - val)), r.clone())
                                    }
                                    Op::Mul => {
                                        State::Eq(Rc::new(State::Constant(val / val2)), r.clone())
                                    }
                                    Op::Div => {
                                        State::Eq(Rc::new(State::Constant(val2 / val)), r.clone())
                                    }
                                },
                                (_, State::Constant(val2)) => match op {
                                    Op::Add => {
                                        State::Eq(Rc::new(State::Constant(val - val2)), l.clone())
                                    }
                                    Op::Sub => {
                                        State::Eq(Rc::new(State::Constant(val + val2)), l.clone())
                                    }
                                    Op::Mul => {
                                        State::Eq(Rc::new(State::Constant(val / val2)), l.clone())
                                    }
                                    Op::Div => {
                                        State::Eq(Rc::new(State::Constant(val * val2)), l.clone())
                                    }
                                },
                                _ => panic!("Simplifications are insufficient"),
                            }
                        }
                        (State::Constant(_), State::Input) | (State::Input, State::Constant(_)) => {
                            break
                        }
                        _ => panic!("Simplifications are insufficient"),
                    }
                }
                State::Input => break,
            };
            *self = new_self;
        }
    }
}

fn parse(input: &'_ str) -> HashMap<&'_ Variable, Expression<'_>> {
    input
        .lines()
        .map(
            |line| match line.split(' ').collect::<Vec<_>>().as_slice() {
                [name_colon, constant] => (
                    name_colon.strip_suffix(':').unwrap(),
                    Expression::Constant(constant.parse().unwrap()),
                ),
                [name_colon, param1, "+", param2] => (
                    name_colon.strip_suffix(':').unwrap(),
                    Expression::BinOp(param1, param2, Op::Add),
                ),
                [name_colon, param1, "-", param2] => (
                    name_colon.strip_suffix(':').unwrap(),
                    Expression::BinOp(param1, param2, Op::Sub),
                ),
                [name_colon, param1, "*", param2] => (
                    name_colon.strip_suffix(':').unwrap(),
                    Expression::BinOp(param1, param2, Op::Mul),
                ),
                [name_colon, param1, "/", param2] => (
                    name_colon.strip_suffix(':').unwrap(),
                    Expression::BinOp(param1, param2, Op::Div),
                ),
                _ => panic!("Invalid Input: {line}"),
            },
        )
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let monkeys = parse(input);

    let mut store = HashMap::new();
    let mut stack = Vec::new();

    stack.push(("root", monkeys.get("root").unwrap()));

    while let Some((name, expression)) = stack.pop() {
        match expression {
            Expression::Constant(val) => {
                store.insert(name, *val);
            }
            Expression::BinOp(l, r, op) => match (store.get(l), store.get(r)) {
                (None, None) => {
                    stack.push((name, expression));
                    stack.push((l, monkeys.get(l).unwrap()));
                    stack.push((r, monkeys.get(r).unwrap()));
                }
                (None, Some(_)) => {
                    stack.push((name, expression));
                    stack.push((l, monkeys.get(l).unwrap()));
                }
                (Some(_), None) => {
                    stack.push((name, expression));
                    stack.push((r, monkeys.get(r).unwrap()));
                }
                (Some(l), Some(r)) => {
                    store.insert(name, op.eval(*l, *r));
                }
            },
        }
    }

    *store.get("root").unwrap()
}

pub fn part2(input: &str) -> i64 {
    let monkeys = parse(input);

    let mut stack = Vec::new();
    // put the expressions in evaluation order into stack
    // from root to leave expressions
    {
        let mut todo = Vec::new();
        todo.push("root");

        while let Some(name) = todo.pop() {
            stack.push(name);
            match monkeys.get(name).unwrap() {
                Expression::Constant(val) => {}
                Expression::BinOp(l, r, op) => {
                    todo.push(l);
                    todo.push(r);
                }
            }
        }
    }

    let mut state: HashMap<&str, Rc<State>> = HashMap::new();

    // build and simplify the expression tree
    while let Some(name) = stack.pop() {
        match monkeys.get(name).unwrap() {
            Expression::BinOp(l, r, op) => {
                if state.get(name).is_none() {
                    let l = state.get(l).unwrap().clone();
                    let r = state.get(r).unwrap().clone();
                    if name == "root" {
                        state.insert(name, Rc::new(State::Eq(l, r)));
                    } else {
                        let mut expr = State::BinOp(l, r, op.clone());
                        expr.simplify();
                        state.insert(name, Rc::new(expr));
                    }
                }
            }
            Expression::Constant(val) => {
                if name == "humn" {
                    state.insert(name, Rc::new(State::Input));
                } else {
                    state.insert(name, Rc::new(State::Constant(*val)));
                }
            }
        }
    }

    let mut root = state.get("root").unwrap().as_ref().clone();
    root.simplify();

    if let State::Eq(l, r) = root {
        if let (State::Constant(res), State::Input) | (State::Input, State::Constant(res)) =
            (l.as_ref(), r.as_ref())
        {
            *res
        } else {
            panic!("Simplification did not reach a final result")
        }
    } else {
        panic!("We lost the root state")
    }
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day21.example.txt");
    assert_eq!(part1(input), 152);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day21.txt"));
    assert_eq!(part1(input), 54703080378102);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day21.example.txt");
    assert_eq!(part2(input), 301);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day21.txt"));
    assert_eq!(part2(input), 3952673930912);
}
