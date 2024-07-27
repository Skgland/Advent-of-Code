pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next();
    let part = args.next();

    pub use aoc2023::*;

    helper::run! {
        for year2023 do
        match (day, part) => {
            pat (Some("day20"), Some("graph")) =>{
                aoc2023::day20::print_graph()
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
