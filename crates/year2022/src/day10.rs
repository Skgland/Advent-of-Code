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
        [b'a', b'd', b'd', b'x', b' ', number @ ..] => {
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day10.txt"
    ));
    assert_eq!(part1(input), 15360);
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day10.txt"
    ));
    assert_eq!(
        part2(input),
        "\
###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.\n\n"
    );
}
