use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2021/day04.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2021", "4", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2021", "4", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};
struct BoardCollector<I>(I);

impl<'a, I> Iterator for BoardCollector<I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = [[u32; 5]; 5];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()?;
        Some([(); 5].map(|_| {
            let mut line = self
                .0
                .next()
                .unwrap()
                .split(' ')
                .filter(|elem| !elem.is_empty())
                .map(|elem| elem.parse().unwrap());
            [(); 5].map(|_| line.next().unwrap())
        }))
    }
}

fn parse_input(input: &str) -> (Vec<u32>, impl Iterator<Item = [[u32; 5]; 5]> + '_) {
    let mut lines = input.lines();
    let balls = lines
        .next()
        .unwrap()
        .split(',')
        .map(|elem| elem.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    (balls, BoardCollector(lines))
}

pub fn process_board(numbers: &[u32], board: [[u32; 5]; 5]) -> (usize, u32) {
    let mut min = usize::MAX;

    for x in 0..5 {
        let mut max_row = usize::MIN;
        let mut max_column = usize::MIN;

        for y in 0..5 {
            max_column = max_column.max(
                numbers
                    .iter()
                    .position(|&elem| elem == board[y][x])
                    .unwrap(),
            );
            max_row = max_row.max(
                numbers
                    .iter()
                    .position(|&elem| elem == board[x][y])
                    .unwrap(),
            )
        }

        min = min.min(max_row.min(max_column));
    }

    let balls = &numbers[..=min];

    let open_numbers = board
        .iter()
        .flat_map(|elem| elem.iter())
        .filter(|&&elem| !balls.contains(&elem))
        .sum();
    (min, open_numbers)
}

pub enum DesiredResult {
    Win,
    Loose,
}

pub fn both(input: &str, want: DesiredResult) -> u32 {
    let (balls, mut boards) = parse_input(input);

    let (mut rounds, mut remaining_score) = process_board(&balls, boards.next().unwrap());

    for board in boards {
        let (new_rounds, new_remaining_score) = process_board(&balls, board);
        if match want {
            DesiredResult::Win => new_rounds < rounds,
            DesiredResult::Loose => new_rounds > rounds,
        } {
            rounds = new_rounds;
            remaining_score = new_remaining_score;
        }
    }

    balls[rounds] * remaining_score
}

pub fn part1(input: &str) -> u32 {
    both(input, DesiredResult::Win)
}

pub fn part2(input: &str) -> u32 {
    both(input, DesiredResult::Loose)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day04.example.txt"
    ));
    assert_eq!(part1(input), 188 * 24);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 6592);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2021/day04.example.txt"
    ));
    assert_eq!(part2(input), 148 * 13);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 31755);
}
