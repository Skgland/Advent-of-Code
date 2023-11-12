fn parse_input(input: &str) -> impl Iterator<Item = &'_ str> + '_ {
    input.lines()
}

fn encode_length(input: &str) -> usize {
    input.len() + input.bytes().filter(|b| matches!(b, b'"' | b'\\')).count() + 2
}

fn code_length(input: &str) -> usize {
    input.len()
}

fn memory_length(input: &str) -> usize {
    let mut rem = input.as_bytes();
    let mut len = 0;

    let [b'"', new_rem @ .., b'"'] = rem else {
        panic!("Missing quotes around string!")
    };

    rem = new_rem;

    loop {
        match rem {
            [] => break len,
            [b'\\', b'"' | b'\\', new_rem @ ..]
            | [b'\\', b'x', b'a'..=b'f' | b'A'..=b'F' | b'0'..=b'9', b'a'..=b'f' | b'A'..=b'F' | b'0'..=b'9', new_rem @ ..]
            | [_, new_rem @ ..] => {
                rem = new_rem;
                len += 1;
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|elem| code_length(elem) - memory_length(elem))
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .map(|elem| encode_length(elem) - code_length(elem))
        .sum()
}

#[cfg(test)]
#[track_caller]
fn test(input: &str, code: usize, memory: usize) {
    assert_eq!(
        code_length(input),
        code,
        "Code Size of {input} should be {code}!"
    );
    assert_eq!(
        memory_length(input),
        memory,
        "Memory Size of {input} should be {memory}!"
    );
}

#[test]
fn part1_example1() {
    test(r#""""#, 2, 0);
}

#[test]
fn part1_example2() {
    test(r#""abc""#, 5, 3);
}

#[test]
fn part1_example3() {
    test(r#""aaa\"aaa""#, 10, 7);
}

#[test]
fn part1_example4() {
    test(r#""\x27""#, 6, 1);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day08.txt"));
    assert_eq!(part1(input), 1333);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day08.txt"));
    assert_eq!(part2(input), 2046);
}
