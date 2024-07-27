use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
enum SnafuDigit {
    Two,
    One,
    Zero,
    MinusOne,
    MinusTwo,
}

impl Display for SnafuDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SnafuDigit::Two => '2',
            SnafuDigit::One => '1',
            SnafuDigit::Zero => '0',
            SnafuDigit::MinusOne => '-',
            SnafuDigit::MinusTwo => '=',
        };
        f.write_char(c)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SnafuNumber {
    digits: Vec<SnafuDigit>,
}

impl From<SnafuNumber> for i64 {
    fn from(value: SnafuNumber) -> Self {
        let mut current = 0;
        for digit in value.digits {
            current *= 5;
            current += match digit {
                SnafuDigit::Two => 2,
                SnafuDigit::One => 1,
                SnafuDigit::Zero => 0,
                SnafuDigit::MinusOne => -1,
                SnafuDigit::MinusTwo => -2,
            }
        }
        current
    }
}

impl From<i64> for SnafuNumber {
    fn from(mut value: i64) -> Self {
        let mut digits = vec![];
        while value != 0 {
            let carry = match value % 5 {
                0 => {
                    digits.push(SnafuDigit::Zero);
                    0
                }
                1 => {
                    digits.push(SnafuDigit::One);
                    0
                }
                2 => {
                    digits.push(SnafuDigit::Two);
                    0
                }
                3 => {
                    digits.push(SnafuDigit::MinusTwo);
                    1
                }
                4 => {
                    digits.push(SnafuDigit::MinusOne);
                    1
                }
                _ => panic!("Modulo is broken, or maybe negative value isn't treated correctly!"),
            };
            value = value / 5 + carry;
        }
        digits.reverse();
        SnafuNumber { digits }
    }
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.digits {
            digit.fmt(f)?;
        }
        Ok(())
    }
}

impl FromStr for SnafuNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];
        for c in s.chars() {
            let d = match c {
                '2' => SnafuDigit::Two,
                '1' => SnafuDigit::One,
                '0' => SnafuDigit::Zero,
                '-' => SnafuDigit::MinusOne,
                '=' => SnafuDigit::MinusTwo,
                _ => return Err(()),
            };
            digits.push(d);
        }
        Ok(Self { digits })
    }
}

pub fn part1(input: &str) -> SnafuNumber {
    let numbers: Vec<SnafuNumber> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    numbers.into_iter().map(i64::from).sum::<i64>().into()
}

pub fn part2(input: &str) -> u32 {
    panic!("There is no part two for day25")
}

#[test]
fn snafu_to_decimal_conversion() {
    let examples = [
        ("1=-0-2", 1747),
        ("12111", 906),
        ("2=0=", 198),
        ("21", 11),
        ("2=01", 201),
        ("111", 31),
        ("20012", 1257),
        ("112", 32),
        ("1=-1=", 353),
        ("1-12", 107),
        ("12", 7),
        ("1=", 3),
        ("122", 37),
    ];
    for (snafu, decimal) in examples {
        let snafu: i64 = snafu.parse::<SnafuNumber>().unwrap().into();
        assert_eq!(snafu, decimal);
    }
}

#[test]
fn decimal_to_snafu_conversion() {
    let examples = [
        (1747, "1=-0-2"),
        (906, "12111"),
        (198, "2=0="),
        (11, "21"),
        (201, "2=01"),
        (31, "111"),
        (1257, "20012"),
        (32, "112"),
        (353, "1=-1="),
        (107, "1-12"),
        (7, "12"),
        (3, "1="),
        (37, "122"),
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];
    for (decimal, snafu) in examples {
        let snafu: SnafuNumber = snafu.parse().unwrap();
        let decimal: SnafuNumber = decimal.into();
        assert_eq!(snafu, decimal);
    }
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2022/day25.example.txt"
    ));
    assert_eq!(part1(input).to_string().as_str(), "2=-1=0");
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2022/day25.txt"
    ));
    assert_eq!(part1(input).to_string().as_str(), "20-=0=02=-21=00-02=2");
}
