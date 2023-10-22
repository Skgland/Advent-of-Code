fn parse_input(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines()
}

fn p1_is_nice(word: &str) -> bool {
    let crit1 = word.bytes().filter(|char| b"aeiou".contains(char)).count() >= 3;
    let crit2 = word
        .as_bytes()
        .windows(2)
        .any(|window| matches!(window, [a, b] if a == b));
    let crit3 = !["ab", "cd", "pq", "xy"].iter().any(|bad| word.contains(bad));
    crit1 && crit2 && crit3
}

pub fn part1(input: &str) -> usize {
    parse_input(input).filter(|word| p1_is_nice(word)).count()
}


fn p2_is_nice(word: &str) -> bool {
    let mut crit1 = false;

    for start in 0..word.len()-2 {
         let (pair, rest) = &word[start..].split_at(2);
         if rest.contains(pair) {
            crit1 = true;
            break;
         }
    }

    let crit2 = word
        .as_bytes()
        .windows(3)
        .any(|window| matches!(window, [a, _, b] if a == b));
    crit1 && crit2
}

pub fn part2(input: &str) -> usize {
    parse_input(input).filter(|word| p2_is_nice(word)).count()
}

#[test]
fn part1_example1() {
    let input = "ugknbfddgicrmopn";
    assert!(p1_is_nice(input));
}

#[test]
fn part1_example2() {
    let input = "aaa";
    assert!(p1_is_nice(input));
}

#[test]
fn part1_example3() {
    let input = "jchzalrnumimnmhp";
    assert!(!p1_is_nice(input));
}

#[test]
fn part1_example4() {
    let input = "haegwjzuvuyypxyu";
    assert!(!p1_is_nice(input));
}

#[test]
fn part1_example5() {
    let input = "dvszwmarrgswjxmb";
    assert!(!p1_is_nice(input));
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day05.txt"));
    assert_eq!(part1(input), 258);
}

#[test]
fn part2_example1() {
    let input = "qjhvhtzxzqqjkmpb";
    assert!(p2_is_nice(input));
}

#[test]
fn part2_example2() {
    let input = "xxyxx";
    assert!(p2_is_nice(input));
}

#[test]
fn part2_example3() {
    let input = "uurcxstgmygtbstg";
    assert!(!p2_is_nice(input));
}

#[test]
fn part2_example4() {
    let input = "ieodomkazucvgmuy";
    assert!(!p2_is_nice(input));
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day05.txt"));
    assert_eq!(part2(input), 53);
}
