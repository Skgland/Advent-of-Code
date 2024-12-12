use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    year: u16,
    day: u8,
}

fn main() {
    let args = CliArgs::parse();

    generate(args.day, args.year);
}

fn generate(day: u8, year: u16) {
    let mut lib_file = std::fs::OpenOptions::new()
        .append(true)
        .open(format!("crates/year{year}/src/lib.rs"))
        .unwrap();

    let mut mod_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("crates/year{year}/src/day{:02}.rs", day))
        .unwrap();

    let bin_file_path = format!("crates/year{year}/src/bin/run{year}.rs");

    let bin_old = std::fs::read_to_string(&bin_file_path).unwrap();

    let mut run_bin = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open(bin_file_path)
        .unwrap();

    let _example_input = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!(
            "inputs/example/year{year}/day{:02}.example1.txt",
            day
        ))
        .unwrap();

    let _input = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("inputs/personal/year{year}/day{:02}.txt", day))
        .unwrap();

    writeln!(lib_file, "pub mod day{:02};", day).unwrap();
    write!(
        mod_file,
        "{}",
        include_str!("../template/lib-mod.rs")
            .replace("dayX", &format!("day{day:02}"))
            .replace("partX", "part1")
            .replace("yearXXXX", &format!("year{year}"))
    )
    .unwrap();

    write!(
        run_bin,
        "{}",
        bin_old.replace(
            "=> default\n",
            &format!("| day{:02}\n            => default\n", day)
        )
    )
    .unwrap();
}
