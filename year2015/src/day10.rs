fn parse_input(input: &str) -> impl Iterator<Item = u8> + '_ {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|l| l as u8)
}

fn look_and_say(input: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    // https://doc.rust-lang.org/std/primitive.slice.html#method.group_by would be really useful here if it was stable

    input
        .map(Some)
        .chain(std::iter::once(None))
        .scan(None, |state, cur| {
            // dbg!(&state, cur);
            let res = match (state.as_mut(), cur) {
                (Some((counting, count)), Some(next)) if *counting == next => {
                    *count += 1;
                    Some(None)
                }
                (Some((counting, count)), next) => {
                    let emit = Some([*count, *counting]);
                    if let Some(next) = next {
                        *state = Some((next, 1));
                    } else {
                        *state = None;
                    }
                    Some(emit)
                }
                (None, Some(next)) => {
                    *state = Some((next, 1));
                    Some(None)
                }
                (None, None) => None,
            };
            // dbg!(res)
            res
        })
        .flatten()
        .flatten()
}

fn look_and_say_n(mut items: Vec<u8>, repititions: u8) -> Vec<u8> {
    for _ in 0..repititions {
        items = look_and_say(items.into_iter()).collect();
    }
    items
}

pub fn part1(input: &str) -> usize {
    look_and_say_n(parse_input(input).collect(), 40).len()
}

pub fn part2(input: &str) -> usize {
    look_and_say_n(parse_input(input).collect(), 50).len()
}

#[test]
fn part1_example1() {
    let res = look_and_say(parse_input("1")).collect::<Vec<_>>();
    assert_eq!(res, vec![1, 1]);
}

#[test]
fn part1_example2() {
    let res = look_and_say(parse_input("11")).collect::<Vec<_>>();
    assert_eq!(res, vec![2, 1]);
}

#[test]
fn part1_example3() {
    let res = look_and_say(parse_input("21")).collect::<Vec<_>>();
    assert_eq!(res, vec![1, 2, 1, 1]);
}

#[test]
fn part1_example4() {
    let res = look_and_say(parse_input("1211")).collect::<Vec<_>>();
    assert_eq!(res, vec![1, 1, 1, 2, 2, 1]);
}

#[test]
fn part1_example5() {
    let res = look_and_say(parse_input("111221")).collect::<Vec<_>>();
    assert_eq!(res, vec![3, 1, 2, 2, 1, 1]);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day10.txt"));
    assert_eq!(part1(input), 360154);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day10.txt"));
    assert_eq!(part2(input), 5103798);
}
