use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2022/day10.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2022", "10", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2022", "10", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

enum Instruction {
    Noop,
    Add(isize),
}
impl Instruction {
    fn len(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }

    fn apply(&self, register: &mut isize) {
        match self {
            Instruction::Noop => {}
            Instruction::Add(summand) => *register += summand,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|elem| match elem.as_bytes() {
        b"noop" => Instruction::Noop,
        [b'a', b'd', b'd', b'x', b' ', _number @ ..] => {
            Instruction::Add(elem.split_once(' ').unwrap().1.parse().unwrap())
        }
        _ => panic!("{}", elem),
    })
}

pub fn run(input: &str, mut fun: impl FnMut(isize, isize)) {
    let mut cycle = 1;
    let mut register = 1;
    for instruction in parse(input) {
        for _ in 0..instruction.len() {
            fun(cycle, register);
            cycle += 1;
        }
        instruction.apply(&mut register);
    }
}

pub fn part1(input: &str) -> isize {
    let mut result = 0;
    run(input, |cycle, register| {
        if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
            result += cycle * register;
        }
    });
    result
}

pub fn part2(input: &str) -> String {
    let mut result = String::new();
    run(input, |cycle, register| {
        let c = if ((cycle - 1) % 40).abs_diff(register) <= 1 {
            "#"
        } else {
            "."
        };
        result += c;
        if cycle % 40 == 0 {
            result += "\n";
        }
        if cycle % 240 == 0 {
            result += "\n";
        }
    });
    result
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day10.example.txt"
    ));
    assert_eq!(part1(input), 420 + 1140 + 1800 + 2940 + 2880 + 3960);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 15360);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day10.example.txt"
    ));
    assert_eq!(
        part2(input),
        "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....

"
    );
}

#[test]
fn part2_full() {
    assert_eq!(
        part2(INPUT),
        "\
###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.\n\n"
    );
}
