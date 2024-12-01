pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next();
    let part = args.next();

    pub use aoc2024::*;

    helper::run! {
        for year2022 do
        match (day, part) => {
            | day01
            => default
        }
    }
}
