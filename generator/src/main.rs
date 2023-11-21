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
        .open("src/lib.rs")
        .unwrap();

    let mut mod_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("src/day{:02}.rs", day))
        .unwrap();

    let bin_file_path = format!("src/bin/run{year}.rs");

    let bin_old = std::fs::read_to_string(&bin_file_path).unwrap();

    let mut run_bin = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open(bin_file_path)
        .unwrap();

    let _example_input = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("input/day{:02}.example.txt", day))
        .unwrap();

    let _input = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("input/day{:02}.txt", day))
        .unwrap();

    writeln!(lib_file, "pub mod day{:02};", day).unwrap();
    write!(
        mod_file,
        "{}",
        include_str!("../template/lib-mod.rs")
            .replace("dayX", &format!("day{:02}", day))
            .replace("partX", "part1")
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
