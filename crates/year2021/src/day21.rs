use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day21.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "21", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "21", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Player {
    name: u16,
    position: u16,
    points: u16,
}

impl Player {
    pub fn advance_by(&mut self, roll: u16) {
        self.position = (self.position + roll - 1) % 10 + 1;
        self.points += self.position;
    }
}

fn parse_input(input: &str) -> (Player, Player) {
    let mut lines = input.lines();

    let player1 = lines.next().unwrap().splitn(5, ' ').collect::<Vec<_>>();
    let player2 = lines.next().unwrap().splitn(5, ' ').collect::<Vec<_>>();
    let (p1_idx, s1_idx) = match player1.as_slice() {
        &[_, p_idx, _, _, s_idx] => (p_idx, s_idx),
        _ => panic!(),
    };
    let (p2_idx, s2_idx) = match player2.as_slice() {
        &[_, p_idx, _, _, s_idx] => (p_idx, s_idx),
        _ => panic!(),
    };

    (
        Player {
            name: p1_idx.parse().unwrap(),
            position: s1_idx.parse().unwrap(),
            points: 0,
        },
        Player {
            name: p2_idx.parse().unwrap(),
            position: s2_idx.parse().unwrap(),
            points: 0,
        },
    )
}

pub fn part1(input: &str) -> u32 {
    let mut dice = (1..=100).cycle();
    let (mut player_a, mut player_b) = parse_input(input);

    let mut rolls = 0;

    'game: loop {
        for player in [&mut player_a, &mut player_b] {
            let roll = (&mut dice).take(3).sum();
            rolls += 3;
            player.advance_by(roll);
            log::trace!(
                "Player {} rolled {} and now has {} points!",
                player.name,
                roll,
                player.points
            );
            if player.points >= 1000 {
                break 'game;
            }
        }
    }

    player_a.points.min(player_b.points) as u32 * rolls
}

pub fn split_the_timeline(
    a: &Player,
    b: &Player,
    cache: &mut HashMap<(Player, Player), (u64, u64)>,
) -> (u64, u64) {
    if let Some(cache_result) = cache.get(&(a.clone(), b.clone())) {
        *cache_result
    } else {
        let mut wins_a = 0;
        let mut wins_b = 0;
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                for d3 in 1..=3 {
                    let mut a = a.clone();
                    let b = b.clone();

                    a.advance_by(d1 + d2 + d3);
                    if a.points >= 21 {
                        wins_a += 1;
                    } else {
                        let (win_b, win_a) = split_the_timeline(&b, &a, cache);
                        wins_a += win_a;
                        wins_b += win_b;
                    }
                }
            }
        }
        cache.insert((a.clone(), b.clone()), (wins_a, wins_b));
        (wins_a, wins_b)
    }
}

pub fn part2(input: &str) -> u64 {
    let (a, b) = parse_input(input);
    let mut cache: HashMap<(Player, Player), (u64, u64)> = HashMap::new();
    let (wins_a, wins_b) = split_the_timeline(&a, &b, &mut cache);
    wins_a.max(wins_b)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day21.example.txt"
    ));
    assert_eq!(part1(input), 745 * 993);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 734820);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day21.example.txt"
    ));
    assert_eq!(part2(input), 444356092776315);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 193170338541590);
}
