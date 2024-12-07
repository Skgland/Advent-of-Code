use std::ops::{Div, Mul, Rem};

pub mod iter;

#[macro_export]
macro_rules! run_inner {
    ($year:ident, $day:ident, $part:ident) => {
        let result = $day::$part(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../inputs/personal/",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        )));
        println!("{}", result)
    };
}

#[macro_export]
macro_rules! run {
    ( for $year:ident do match ($day:ident, $part:ident) => {  $(pat $pat:pat => $expr:block),* $(,)? | $($id:ident)|* => default  }) => {
        match ($day.as_deref(), $part.as_deref()) {
            $($pat => $expr)*
            $((Some(stringify!($id)), Some("1")) => {
                $crate::run_inner!($year, $id, part1);
            }
            (Some(stringify!($id)), Some("2")) => {
                $crate::run_inner!($year, $id, part2);
            })*
            (Some(day), Some(part)) => {
                eprintln!("Unknown Day Part combination: Day {} Part {}", day, part);
            },
            _ => {
                eprintln!("Expected two arguments: dayXX and part# e.g. day22 2");
            }
        }
    };
}

pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_zero {
    ($($t:ty),+) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0;
            }
        )*
    };
}

impl_zero!(u8, u16, u32, u64, u128, usize);

pub trait IntegerExtension {
    fn next_power_of_ten(&self) -> Self;
    fn length_base10(&self) -> u32;
}

impl IntegerExtension for u64 {
    fn next_power_of_ten(&self) -> Self {
        match self {
            1..10 => 10,
            10..100 => 100,
            100..1000 => 1000,
            1000..10_000 => 10_000,
            _ => 10u64.pow(self.length_base10()),
        }
    }

    fn length_base10(&self) -> u32 {
        match self {
            1..10 => 1,
            10..100 => 2,
            100..1000 => 3,
            1000..10_000 => 4,
            _ => self.ilog10() + 1,
        }
    }
}

impl IntegerExtension for usize {
    fn next_power_of_ten(&self) -> Self {
        match self {
            1..10 => 10,
            10..100 => 100,
            100..1000 => 1000,
            1000..10_000 => 10_000,
            _ => 10usize.pow(self.ilog10() + 1),
        }
    }

    fn length_base10(&self) -> u32 {
        match self {
            1..10 => 1,
            10..100 => 2,
            100..1000 => 3,
            1000..10_000 => 4,
            _ => self.ilog10() + 1,
        }
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Ord + Rem<Output = T> + Div<Output = T> + Zero + Mul<Output = T>,
{
    a / gcd(a, b) * b
}

pub fn gcd<T>(width: T, height: T) -> T
where
    T: Copy + Ord + Rem<Output = T> + Zero,
{
    let mut a = width;
    let mut b = height;
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b != Zero::ZERO {
        (a, b) = (b, a % b);
    }
    a
}
