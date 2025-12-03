use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::collections::VecDeque;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day09.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "9", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "9", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
struct FileSystemChunk {
    start: usize,
    // as the file length is a single digit a u8 would suffice,
    // but working with usize is simpler as otherwise we would eb casting back and forth all the time
    length: usize,
    kind: FileSystemChunkKind,
}

#[derive(Debug, Clone)]
enum FileSystemChunkKind {
    File { id: usize },
    Empty,
}

fn parse_input(input: &str) -> VecDeque<FileSystemChunk> {
    let mut result = VecDeque::new();
    let mut start = 0;
    let mut id = 0;
    for chunk in input.as_bytes().chunks(2) {
        match chunk {
            [file, space] => {
                let file_length = (*file as char).to_digit(10).unwrap() as usize;
                result.push_back(FileSystemChunk {
                    start,
                    length: file_length,
                    kind: FileSystemChunkKind::File { id },
                });
                id += 1;
                start += file_length;

                let space_length = (*space as char).to_digit(10).unwrap() as usize;
                if space_length != 0 {
                    result.push_back(FileSystemChunk {
                        start,
                        length: space_length,
                        kind: FileSystemChunkKind::Empty,
                    });
                    start += space_length;
                }
            }
            [file] => {
                let file_length = (*file as char).to_digit(10).unwrap() as usize;
                result.push_back(FileSystemChunk {
                    start,
                    length: file_length,
                    kind: FileSystemChunkKind::File { id },
                });
                id += 1;
                start += file_length;
            }
            _ => unreachable!("chunk should be of length 1 or 2 "),
        }
    }
    result
}

pub fn part1(input: &str) -> usize {
    let mut todo = parse_input(input);
    let new = compact(&mut todo, usize::min);
    calc_checksum(&new)
}

fn calc_checksum(new: &[FileSystemChunk]) -> usize {
    let mut checksum = 0;
    for chunk in new {
        if let FileSystemChunkKind::File { id } = chunk.kind {
            checksum += (chunk.start..chunk.start + chunk.length).sum::<usize>() * id
        }
    }
    checksum
}

pub fn part2(input: &str) -> usize {
    let mut todo = parse_input(input);
    let new = compact(
        &mut todo,
        |space, file| if file <= space { file } else { 0 },
    );
    calc_checksum(&new)
}

fn compact(
    todo: &mut VecDeque<FileSystemChunk>,
    compaction_function: impl Fn(usize, usize) -> usize,
) -> Vec<FileSystemChunk> {
    let mut new = Vec::new();
    while let Some(mut chunk) = todo.pop_front() {
        if chunk.length == 0 {
            continue;
        }
        match chunk.kind {
            FileSystemChunkKind::File { id: _ } => {
                new.push(chunk);
            }
            FileSystemChunkKind::Empty => {
                for other in todo.iter_mut().rev() {
                    let FileSystemChunkKind::File { id: _ } = other.kind else {
                        continue;
                    };
                    if other.length == 0 {
                        continue;
                    }

                    let move_amount = compaction_function(chunk.length, other.length);
                    new.push(FileSystemChunk {
                        start: chunk.start,
                        length: move_amount,
                        kind: other.kind.clone(),
                    });
                    chunk.start += move_amount;
                    chunk.length -= move_amount;
                    other.length -= move_amount;
                }
                todo.retain(|entry| entry.length != 0);
            }
        }
    }
    new
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day09.example1.txt"
    ));
    assert_eq!(part1(input), 1928);
}

#[test]
fn part1_full() {
    assert_eq!(part1(INPUT), 6399153661894);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day09.example1.txt"
    ));
    assert_eq!(part2(input), 2858);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 6421724645083);
}
