enum RPS {
    RockLose,
    PaperDraw,
    SissorWin,
}
impl RPS {
    fn shape_reward(&self) -> u32 {
        match self {
            RPS::RockLose => 1,
            RPS::PaperDraw => 2,
            RPS::SissorWin => 3,
        }
    }

    fn win_reward(&self, other: &RPS) -> u32 {
        use RPS::*;
        match (self, other) {
            (RockLose, PaperDraw) | (PaperDraw, SissorWin) | (SissorWin, RockLose) => 0,
            (RockLose, RockLose) | (PaperDraw, PaperDraw) | (SissorWin, SissorWin) => 3,
            (SissorWin, PaperDraw) | (PaperDraw, RockLose) | (RockLose, SissorWin) => 6,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            RPS::RockLose => RPS::PaperDraw,
            RPS::PaperDraw => RPS::SissorWin,
            RPS::SissorWin => RPS::RockLose,
        }
    }

    fn wins_agains(&self) -> Self {
        match self {
            RPS::RockLose => RPS::SissorWin,
            RPS::PaperDraw => RPS::RockLose,
            RPS::SissorWin => RPS::PaperDraw,
        }
    }

    fn result_reward(&self, other: &RPS) -> u32 {
        match self {
            RPS::RockLose => other.wins_agains().shape_reward(),
            RPS::PaperDraw => other.shape_reward() + 3,
            RPS::SissorWin => other.loses_to().shape_reward() + 6,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = (RPS, RPS)> + '_ {
    input.lines().map(|line| {
        (
            match line.as_bytes() {
                [b'A', _, _] => RPS::RockLose,
                [b'B', _, _] => RPS::PaperDraw,
                [b'C', _, _] => RPS::SissorWin,
                _ => panic!("invalid input"),
            },
            match line.as_bytes() {
                [_, _, b'Y'] => RPS::PaperDraw,
                [_, _, b'X'] => RPS::RockLose,
                [_, _, b'Z'] => RPS::SissorWin,
                _ => panic!("invalid input"),
            },
        )
    })
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|(other, me)| me.shape_reward() + me.win_reward(&other))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|(other, me)| me.result_reward(&other))
        .sum()
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day02.example.txt");
    assert_eq!(part1(input), 15);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day02.txt"));
    assert_eq!(part1(input), 10941);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day02.example.txt");
    assert_eq!(part2(input), 12);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day02.txt"));
    assert_eq!(part2(input), 13071);
}
