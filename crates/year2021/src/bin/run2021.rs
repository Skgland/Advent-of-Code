pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next();
    let part = args.next();

    pub use aoc2021::*;

    helper::run! {
        for year2021 do
        match (day, part) => {
            pat (Some("day24"), Some("code")) => {
                let code = aoc2021::day24::part1_instructions_to_code(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../inputs/personal/year2021/day24.txt")));
                println!("{}", code);
            }
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
            | day11
            | day12
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
            | day24
            | day25
            => default
        }
    }
}
