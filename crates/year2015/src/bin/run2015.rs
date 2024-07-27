pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next();
    let part = args.next();

    use aoc2015::*;

    helper::run! {
        for year2015 do
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
            => default
        }
    }
}
