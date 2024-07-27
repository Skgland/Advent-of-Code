use std::collections::HashMap;

enum RefOrConst<'a> {
    Ref(&'a str),
    Const(u16),
}
impl<'a> RefOrConst<'a> {
    fn eval(&self, evaluated: &HashMap<&str, u16>) -> Option<u16> {
        match self {
            RefOrConst::Ref(arg) => evaluated.get(arg).copied(),
            RefOrConst::Const(val) => Some(*val),
        }
    }

    fn from_str(s: &'a str) -> Self {
        if let Ok(val) = s.parse() {
            Self::Const(val)
        } else {
            Self::Ref(s)
        }
    }
}

enum Expression<'a> {
    Simple(RefOrConst<'a>),
    Not(RefOrConst<'a>),
    Lsh(RefOrConst<'a>, RefOrConst<'a>),
    Rsh(RefOrConst<'a>, RefOrConst<'a>),
    And(RefOrConst<'a>, RefOrConst<'a>),
    Or(RefOrConst<'a>, RefOrConst<'a>),
}
impl<'a> Expression<'a> {
    fn eval(&self, evaluated: &HashMap<&str, u16>) -> Option<u16> {
        Some(match self {
            Expression::Simple(arg) => arg.eval(evaluated)?,
            Expression::Not(arg) => !arg.eval(evaluated)?,
            Expression::Lsh(l, r) => l.eval(evaluated)? << r.eval(evaluated)?,
            Expression::Rsh(l, r) => l.eval(evaluated)? >> r.eval(evaluated)?,
            Expression::And(l, r) => l.eval(evaluated)? & r.eval(evaluated)?,
            Expression::Or(l, r) => l.eval(evaluated)? | r.eval(evaluated)?,
        })
    }

    fn references(&self) -> Vec<&'a str> {
        let single;
        let double;
        let vars = match self {
            Expression::Simple(a) | Expression::Not(a) => {
                single = [a];
                single.as_slice()
            }
            Expression::Lsh(a, b)
            | Expression::Rsh(a, b)
            | Expression::And(a, b)
            | Expression::Or(a, b) => {
                double = [a, b];
                double.as_slice()
            }
        };

        vars.iter()
            .filter_map(|elem| match elem {
                RefOrConst::Ref(name) => Some(*name),
                RefOrConst::Const(_) => None,
            })
            .collect()
    }

    fn from_str(s: &'a str) -> Option<Self> {
        Some(match s.split(' ').collect::<Vec<_>>().as_slice() {
            [var1, "AND", var2] => {
                Expression::And(RefOrConst::from_str(var1), RefOrConst::from_str(var2))
            }
            [var1, "OR", var2] => {
                Expression::Or(RefOrConst::from_str(var1), RefOrConst::from_str(var2))
            }
            [var1, "LSHIFT", var2] => {
                Expression::Lsh(RefOrConst::from_str(var1), RefOrConst::from_str(var2))
            }
            [var1, "RSHIFT", var2] => {
                Expression::Rsh(RefOrConst::from_str(var1), RefOrConst::from_str(var2))
            }
            ["NOT", var] => Expression::Not(RefOrConst::from_str(var)),
            [val] => {
                let s = &val;
                Expression::Simple(RefOrConst::from_str(s))
            }
            _ => return None,
        })
    }
}

struct State<'a> {
    evaluated: HashMap<&'a str, u16>,
    assignments: HashMap<&'a str, Expression<'a>>,
}

impl<'a> State<'a> {
    fn eval(&mut self, target: &'a str) -> u16 {
        let mut stack = vec![(target, self.assignments.get(target).unwrap())];

        loop {
            let (name, exp) = stack.last().unwrap();

            if let Some(result) = exp.eval(&self.evaluated) {
                self.evaluated.insert(name, result);
                stack.pop();
                if stack.is_empty() {
                    return result;
                }
            } else {
                for entry in exp.references() {
                    stack.push((entry, self.assignments.get(entry).unwrap()))
                }
            }
        }
    }
}

fn parse_input(input: &str) -> State<'_> {
    let assignments = input
        .lines()
        .flat_map(|line| {
            let (exp, target) = line.split_once(" -> ")?;
            Some((target, Expression::from_str(exp)?))
        })
        .collect();

    State {
        evaluated: HashMap::new(),
        assignments,
    }
}

pub fn part1(input: &str) -> u16 {
    let mut state = parse_input(input);
    state.eval("a")
}

pub fn part2(input: &str) -> u16 {
    let mut state = parse_input(input);

    let b = state.eval("a");
    state
        .assignments
        .insert("b", Expression::Simple(RefOrConst::Const(b)));
    state.evaluated.clear();

    state.eval("a")
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2015/day07.example.txt"
    ));
    let mut state = parse_input(input);
    assert_eq!(state.eval("d"), 72);
    assert_eq!(state.eval("e"), 507);
    assert_eq!(state.eval("f"), 492);
    assert_eq!(state.eval("g"), 114);
    assert_eq!(state.eval("h"), 65412);
    assert_eq!(state.eval("i"), 65079);
    assert_eq!(state.eval("x"), 123);
    assert_eq!(state.eval("y"), 456);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day07.txt"
    ));
    assert_eq!(part1(input), 46065);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day07.txt"
    ));
    assert_eq!(part2(input), 14134);
}
