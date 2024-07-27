#[derive(PartialEq, Eq, Debug)]
struct PuzzleInput {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}
impl PuzzleInput {
    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }

    fn apply_moves_9000(&mut self) {
        for movement in &self.moves {
            for _ in 0..movement.count {
                let parcel = self.stacks[movement.from - 1].pop().unwrap();
                self.stacks[movement.to - 1].push(parcel);
            }
        }
    }

    fn apply_moves_9001(&mut self) {
        for movement in &self.moves {
            let src_stack = &mut self.stacks[movement.from - 1];
            let src_stack_len = src_stack.len();
            let mut parcel = src_stack.split_off(src_stack_len - movement.count);
            self.stacks[movement.to - 1].append(&mut parcel);
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> PuzzleInput {
    let (towers, moves) = input
        .split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))
        .unwrap();

    let mut stacks = vec![];
    for row in towers
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .rev()
        .skip(1)
    {
        stacks.resize(row.len(), Vec::default());
        for (idx, char) in row.into_iter().enumerate() {
            if char != ' ' {
                stacks[idx].push(char);
            }
        }
    }

    let moves = moves
        .lines()
        .map(|line| {
            let (count, src_target) = line
                .trim_start_matches("move ")
                .split_once(" from ")
                .unwrap();
            let (source, target) = src_target.split_once(" to ").unwrap();
            Move {
                count: count.parse().unwrap(),
                from: source.parse().unwrap(),
                to: target.parse().unwrap(),
            }
        })
        .collect();

    PuzzleInput { stacks, moves }
}

#[test]
fn test_parse() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day05.example.txt"
    ));
    let input = parse(input);
    assert_eq!(
        input,
        PuzzleInput {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            moves: vec![
                Move {
                    count: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    count: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    count: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    count: 1,
                    from: 1,
                    to: 2
                },
            ]
        }
    )
}

pub fn part1(input: &str) -> String {
    let mut input = parse(input);
    input.apply_moves_9000();
    input.tops()
}

pub fn part2(input: &str) -> String {
    let mut input = parse(input);
    input.apply_moves_9001();
    input.tops()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day05.example.txt"
    ));
    assert_eq!(part1(input).as_str(), "CMZ");
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day05.txt"
    ));
    assert_eq!(part1(input).as_str(), "NTWZZWHFV");
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day05.example.txt"
    ));
    assert_eq!(part2(input).as_str(), "MCD");
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day05.txt"
    ));
    assert_eq!(part2(input).as_str(), "BRZGFVBTJ");
}
