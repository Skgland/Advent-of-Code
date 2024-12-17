use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day17.txt"
));

#[cfg(test)]
const INPUT_EXAMPLE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day17.example1.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "17", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "17", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
enum OpCode {
    Adv, // a = floor(a / 2**combo op)
    Bxl, // b = b^lit op
    Bst, // b = combo op % 8
    Jnz, // if a != 0 then ip =  lit op, don't inc ip after
    Bxc, // b = b xor c, ignore op
    Out, // Output comb_op % 8, seperated by ,
    Bdv, // b = floor(a / 2**combo op)
    Cdv, // c = floor(a / 2**combo op)
}

impl OpCode {
    fn parse(op: u8) -> Self {
        match op {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid Instruction"),
        }
    }
}

#[derive(Debug)]
struct ProgramState {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    inst_pointer: usize,
}

impl ProgramState {
    fn combo_arg(&self, arg: u8) -> u64 {
        match arg {
            0..=3 => arg as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid Combo Arg"),
        }
    }

    fn apply(&mut self, op: u8, arg: u8) -> Option<u8> {
        let op = OpCode::parse(op);
        let mut result = None;
        match op {
            OpCode::Adv => self.register_a /= 2u64.pow(self.combo_arg(arg).try_into().unwrap()),
            OpCode::Bxl => self.register_b ^= arg as u64,
            OpCode::Bst => self.register_b = self.combo_arg(arg) % 8,
            OpCode::Jnz => {
                if self.register_a != 0 {
                    self.inst_pointer = arg as usize;
                    return None;
                }
            }
            OpCode::Bxc => self.register_b ^= self.register_c,
            OpCode::Out => result = Some((self.combo_arg(arg) % 8) as u8),
            OpCode::Bdv => {
                self.register_b =
                    self.register_a / 2u64.pow(self.combo_arg(arg).try_into().unwrap())
            }
            OpCode::Cdv => {
                self.register_c =
                    self.register_a / 2u64.pow(self.combo_arg(arg).try_into().unwrap())
            }
        }
        self.inst_pointer += 2;
        result
    }
}

struct Input {
    program_data: Vec<u8>,
    init_state: ProgramState,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let state = ProgramState {
        register_a: lines
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap(),
        register_b: lines
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap(),
        register_c: lines
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap(),
        inst_pointer: 0,
    };
    let program_data = lines
        .skip(1)
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|elem| elem.parse::<u8>().unwrap())
        .collect();
    Input {
        program_data,
        init_state: state,
    }
}

pub fn part1(input: &str) -> String {
    let input = parse_input(input);
    ProgramIter {
        state: input.init_state,
        program: &input.program_data,
    }
    .map(|i| i.to_string())
    .collect::<Vec<_>>()
    .join(",")
}

struct ProgramIter<'a> {
    state: ProgramState,
    program: &'a [u8],
}

impl Iterator for ProgramIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&[op, arg]) = self
            .program
            .get(self.state.inst_pointer..=self.state.inst_pointer + 1)
        {
            if let Some(out) = self.state.apply(op, arg) {
                return Some(out);
            }
        }
        None
    }
}

pub fn part2(input: &str) -> u64 {
    let input = parse_input(input);
    for a in 0.. {
        let runtime = ProgramIter {
            state: ProgramState {
                register_a: a,
                register_b: input.init_state.register_b,
                register_c: input.init_state.register_c,
                inst_pointer: 0,
            },
            program: &input.program_data,
        };
        if runtime.eq(input.program_data.iter().copied()) {
            return a;
        }
    }
    panic!("No solution found")
}

#[test]
fn part1_example1() {
    assert_eq!(part1(INPUT_EXAMPLE1), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), "1,4,6,1,6,4,3,0,3");
}

#[test]
fn part2_example1() {
    assert_eq!(part2(INPUT_EXAMPLE1), 117440);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 0);
}
