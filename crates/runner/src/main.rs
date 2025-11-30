use aoc2015 as _;
use aoc2021 as _;
use aoc2022 as _;
use aoc2023 as _;
use aoc2024 as _;
use aoc2025 as _;
use helper::{Task, TASKS};
use linkme::distributed_slice;
use std::io::Write;

fn main() {
    env_logger::init();

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if let Some((last, prefix)) = args.split_last() {
        match last.as_str() {
            "list" => {
                helper::list_with_prefix(prefix);
            }
            "all" => {
                helper::run_all_prefix(prefix);
            }
            "generate" => {
                if let [year, day] = prefix {
                    let Ok(year) = year.parse() else {
                        println!("{year} is not a valid year number");
                        return;
                    };
                    let Ok(day) = day.parse() else {
                        println!("{day} is not a valid day number");
                        return;
                    };
                    generate(day, year);
                } else {
                    println!("Generate takes exactly two argumenrs!")
                };
            }
            _ => {
                let mut found = false;
                for task in TASKS {
                    if task.path == args {
                        (task.run)();
                        found = true;
                    }
                }
                if !found {
                    help();
                }
            }
        }
    } else {
        help();
    }
}

#[distributed_slice(TASKS)]
static HELP: Task = Task {
    path: &["help"],
    run: help,
    include_in_all: false,
};

fn help() {
    println!(
        "\
        Available commands are:\n\
        \t- help                    - to show this help\n\
        \t- [prefix ...] list       - to list all task with that prefix\n\
        \t- [prefix ...] all        - to run all task with that prefix (unless marked as not included)\n\
        \t- <year> <day> generate   - to instantiate a template module for the specified year and day\n\
        "
    );
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
            .replace("DD", &format!("{day:02}"))
            .replace("YYYY", &format!("{year}"))
    )
    .unwrap();
}
