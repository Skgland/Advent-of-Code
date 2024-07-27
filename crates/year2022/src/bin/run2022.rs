pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next();
    let part = args.next();

    pub use aoc2022::*;

    helper::run! {
        for year2022 do
        match (day, part) => {
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
