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

const INPUT_EXAMPLE2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/example/year2024/day17.example2.txt"
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
    run: || println!("{}", part2_specialized(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static CODE: Task = Task {
    path: &["2024", "17", "code"],
    run: || print_code(INPUT),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static CODE2: Task = Task {
    path: &["2024", "17", "code_example2"],
    run: || print_code(INPUT_EXAMPLE2),
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

    fn print(&self, arg: u8) {
        fn combo_arg(arg: u8) -> &'static str {
            match arg {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "a",
                5 => "b",
                6 => "c",
                _ => panic!(),
            }
        }
        match self {
            OpCode::Adv => {
                println!("    a /= 2u64.pow({} as u32);", combo_arg(arg));
            }
            OpCode::Bxl => {
                println!("    b ^= {arg};")
            }
            OpCode::Bst => println!("    b = {} % 8;", combo_arg(arg)),
            OpCode::Jnz => {
                println!("    if a == 0 {{ break; }};")
            }
            OpCode::Bxc => {
                println!("    b ^= c;");
            }
            OpCode::Out => {
                println!("    output.push(({} % 8) as u8);", combo_arg(arg))
            }
            OpCode::Bdv => {
                println!("    b = a / 2u64.pow({} as u32);", combo_arg(arg));
            }
            OpCode::Cdv => {
                println!("    c = a / 2u64.pow({} as u32);", combo_arg(arg));
            }
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

pub fn print_code(input: &str) {
    let input = parse_input(input);

    // the last instruction is a jump to the begining
    assert_eq!(input.program_data.chunks(2).last(), Some([3, 0].as_slice()));

    // all other instructions arn't jump instructions
    assert!(input
        .program_data
        .chunks(2)
        .rev()
        .skip(1)
        .all(|chunk| chunk[0] != 3));

    println!("let mut output = vec![];");
    println!("loop {{");
    for chunk in input.program_data.chunks(2) {
        OpCode::parse(chunk[0]).print(chunk[1])
    }
    println!("}}");
}

#[allow(unused_assignments)]
pub fn run(mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
    /* output from print_code */
    let mut output = vec![];
    loop {
        b = a % 8;
        b ^= 7;
        c = a / 2u64.pow(b as u32);
        b ^= 7;
        b ^= c;
        a /= 2u64.pow(3 as u32);
        output.push((b % 8) as u8);
        if a == 0 {
            break;
        };
    }
    output
}

pub fn run_simplified(mut a: u64) -> Vec<u8> {
    let mut output = vec![];
    loop {
        // run() with basically every instance of b or c  inlined
        output.push((((a % 8) ^ (a / 2u64.pow(((a % 8) ^ 7) as u32))) % 8) as u8);
        a /= 2u64.pow(3 as u32);
        if a == 0 {
            break;
        };
    }
    output
}

pub fn next_digit(a: u64) -> u8 {
    // just the argument expression to push from run_simplified
    (((a % 8) ^ (a / 2u64.pow(((a % 8) ^ 7) as u32))) % 8) as u8
}

pub fn part2_specialized(input: &str) -> u64 {
    // spcefic to my input due to the use of next_digit which is manually created based on my specific input
    let input = parse_input(input);

    let (last, rest) = input.program_data.split_last().unwrap();
    for a in 1..8 {
        let result = next_digit(a);
        println!("Next digi: {last}, i: {a}, temp_a: {a}, Result: {result}");
        if &result == last {
            if let Some(a) = find_a(rest, a) {
                assert_eq!(run(a, 0, 0), input.program_data);
                return a;
            }
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
fn part2_slow(input: &str) -> u64 {
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
        if (runtime).eq(input.program_data.iter().copied()) {
            return a;
        }
    }
    panic!("No solutuion found")
}

pub fn find_a(remaining: &[u8], a: u64) -> Option<u64> {
    match remaining {
        [] => Some(a),
        [rest @ .., last] => {
            for i in 0..8 {
                let a = (a << 3) | i;
                if &next_digit(a) == last {
                    if let Some(a) = find_a(rest, a) {
                        return Some(a);
                    }
                }
            }
            None
        }
    }
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
fn part2_example2() {
    assert_eq!(part2_slow(INPUT_EXAMPLE2), 117440);
}

#[test]
fn part2_simplify() {
    for a in 0..200_000 {
        assert_eq!(run_simplified(a), run(a, 0, 0))
    }
}

#[test]
fn part2_exec() {
    let input = parse_input(INPUT);
    for a in 0..200_000 {
        assert_eq!(
            ProgramIter {
                state: ProgramState {
                    register_a: a,
                    register_b: 0,
                    register_c: 0,
                    inst_pointer: 0
                },
                program: &input.program_data
            }
            .collect::<Vec<_>>(),
            run_simplified(a)
        )
    }
}

#[test]
fn part2_full() {
    assert_eq!(part2_specialized(INPUT), 0);
}
