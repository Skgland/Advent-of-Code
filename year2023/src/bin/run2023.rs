#[allow(unused_macros)]
macro_rules! run {
    ($day:ident, $part:ident) => {
        let result = aoc2023::$day::$part(include_str!(concat!(
            "../../input/",
            stringify!($day),
            ".txt"
        )));
        println!("{}", result)
    };
}

macro_rules! run_arms {
    ( match ($day:ident, $part:ident) => {  $(pat $pat:pat => $expr:block),* $(,)? | $($id:ident)|* => default }) => {
        match ($day.as_deref(), $part.as_deref()) {
            $($pat => $expr)*
            $((Some(stringify!($id)), Some("1")) => {
                run!($id, part1);
            }
            (Some(stringify!($id)), Some("2")) => {
                run!($id, part2);
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

pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next();
    let part = args.next();

    run_arms! {
        match (day, part) => {
            pat (Some("day20"), Some("graph")) =>{
                aoc2023::day20::print_graph()
            },
            | day01
            | day02
            | day03
            | day04
            | day05
            | day06
            | day07
            | day08
            | day09
            | day10
            | day12
            | day11
            | day13
            | day14
            | day15
            | day16
            | day17
            | day18
            | day19
            | day20
            | day21
            | day22
            | day23
            => default
        }
    }
}
