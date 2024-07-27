fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn assemble_result(numbers: &[isize], tracked_idx: &[usize]) -> Vec<isize> {
    let mut result = vec![0; numbers.len()];
    for (idx, &entry) in tracked_idx.iter().enumerate() {
        result[entry] = numbers[idx];
    }
    result
}

fn mix(numbers: &[isize], iterations: usize) -> Vec<isize> {
    // track_idx[idx] contains the index to which the number at number[idx] has moved
    let mut track_idx: Vec<_> = (0..numbers.len()).collect();

    for mix in 0..iterations {
        println!("Mix {}", mix + 1);
        for (idx, &number) in numbers.iter().enumerate() {
            let old = track_idx[idx];
            let new = wrapping_add(old, number, numbers.len() - 1);
            let (update_range, update_direction): (_, isize) = if new < old {
                (new..old, 1)
            } else {
                ((old + 1)..(new + 1), -1)
            };
            track_idx.iter_mut().for_each(|elem| {
                if update_range.contains(elem) {
                    *elem = wrapping_add(*elem, update_direction, numbers.len())
                }
            });
            track_idx[idx] = new;
        }
    }
    assemble_result(numbers, &track_idx)
}

fn wrapping_add(elem: usize, update_direction: isize, len: usize) -> usize {
    (elem as isize + update_direction).rem_euclid(len as isize) as usize
}

fn extract_coordinates(result: &[isize]) -> (isize, isize, isize) {
    let (idx, _) = result
        .iter()
        .enumerate()
        .find(|(_idx, &elem)| elem == 0)
        .unwrap();
    let x = result[(idx + 1000) % result.len()];
    let y = result[(idx + 2000) % result.len()];
    let z = result[(idx + 3000) % result.len()];
    (x, y, z)
}

pub fn part1(input: &str) -> isize {
    let numbers = parse(input);
    let result = mix(&numbers, 1);
    let (x, y, z) = extract_coordinates(&result);
    dbg!(x, y, z);
    x + y + z
}

pub fn part2(input: &str) -> isize {
    let numbers = parse(input);
    let decrypted: Vec<_> = numbers.iter().map(|elem| elem * 811589153).collect();
    let result = mix(&decrypted, 10);
    let (x, y, z) = extract_coordinates(&result);
    dbg!(x, y, z);
    x + y + z
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day20.example.txt"
    ));
    assert_eq!(part1(input), 4 + -3 + 2);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day20.txt"
    ));
    assert_eq!(part1(input), 4578);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day20.example.txt"
    ));
    assert_eq!(part2(input), 811589153 + 2434767459 + -1623178306);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day20.txt"
    ));
    assert_eq!(part2(input), 2159638736133);
}
