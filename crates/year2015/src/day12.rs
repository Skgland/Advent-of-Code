use serde_json::Value;

fn parse_input(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn sum_ints(value: Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(num) => num.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vals) => vals.into_iter().map(sum_ints).sum(),
        Value::Object(obj) => obj.into_iter().map(|(_, value)| sum_ints(value)).sum(),
    }
}

fn sum_ints_no_red(value: Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(num) => num.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vals) => vals.into_iter().map(sum_ints_no_red).sum(),
        Value::Object(obj) => {
            if obj.values().any(|value| value == "red") {
                0
            } else {
                obj.into_iter()
                    .map(|(_, value)| sum_ints_no_red(value))
                    .sum()
            }
        }
    }
}

pub fn part1(input: &str) -> i64 {
    sum_ints(parse_input(input))
}

pub fn part2(input: &str) -> i64 {
    sum_ints_no_red(parse_input(input))
}

#[test]
fn part1_example1() {
    let input = "[1,2,3]";
    assert_eq!(part1(input), 6);
}

#[test]
fn part1_example2() {
    let input = r#"{"a":2,"b":4}"#;
    assert_eq!(part1(input), 6);
}

#[test]
fn part1_example3() {
    let input = r#"[[[3]]]"#;
    assert_eq!(part1(input), 3);
}

#[test]
fn part1_example4() {
    let input = r#"{"a":{"b":4},"c":-1}"#;
    assert_eq!(part1(input), 3);
}

#[test]
fn part1_example5() {
    let input = r#"{"a":[-1,1]}"#;
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_example6() {
    let input = r#"[-1,{"a":1}]"#;
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_example7() {
    let input = r#"[]"#;
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_example8() {
    let input = r#"{}"#;
    assert_eq!(part1(input), 0);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day12.txt"
    ));
    assert_eq!(part1(input), 191164);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day12.txt"
    ));
    assert_eq!(part2(input), 87842);
}
