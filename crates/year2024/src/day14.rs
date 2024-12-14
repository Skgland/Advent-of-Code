use std::{collections::BTreeSet, io::BufWriter};

use helper::lcm;

#[derive(Debug)]
struct Robot {
    pos: [isize; 2],
    vel: [isize; 2],
}

fn parse_input(input: &str) -> impl Iterator<Item = Robot> + '_ {
    input.lines().map(|line| {
        let (p, v) = line.split_once(' ').unwrap();
        let p = p.strip_prefix("p=").unwrap();
        let v = v.strip_prefix("v=").unwrap();
        let mut ps = p.split(',').map(|val| val.parse().unwrap());
        let mut vs = v.split(',').map(|val| val.parse().unwrap());
        Robot {
            pos: [ps.next().unwrap(), ps.next().unwrap()],
            vel: [vs.next().unwrap(), vs.next().unwrap()],
        }
    })
}

fn move_robots<'a, I: Iterator<Item = Robot> + 'a>(
    robots: I,
    dim: [isize; 2],
    steps: isize,
) -> impl Iterator<Item = Robot> + 'a {
    robots.map(move |mut robot| {
        for idx in 0..=1 {
            robot.pos[idx] += robot.vel[idx] * steps;
            robot.pos[idx] = robot.pos[idx].rem_euclid(dim[idx]);
        }
        robot
    })
}

fn quadrant_occupation(robots: impl Iterator<Item = Robot>, dim: [isize; 2]) -> [u32; 4] {
    robots.fold([0, 0, 0, 0], |mut acc, robot| {
        match robot.pos[0].cmp(&(dim[0] / 2)) {
            std::cmp::Ordering::Less => match robot.pos[1].cmp(&(dim[1] / 2)) {
                std::cmp::Ordering::Less => acc[0] += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => acc[1] += 1,
            },
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => match robot.pos[1].cmp(&(dim[1] / 2)) {
                std::cmp::Ordering::Less => acc[2] += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => acc[3] += 1,
            },
        }
        acc
    })
}

fn part1_impl(input: &str, dim: [isize; 2], steps: isize) -> u32 {
    quadrant_occupation(move_robots(parse_input(input), dim, steps), dim)
        .into_iter()
        .product()
}

pub fn part1(input: &str) -> u32 {
    part1_impl(input, [101, 103], 100)
}

pub fn part2(input: &str) -> &'static str {
    part2_impl(input, [101, 103]);
    "See trees.txt"
}

fn part2_impl(input: &str, dim: [isize; 2]) {
    use std::io::Write as _;
    let mut stream = BufWriter::new(std::fs::File::create("trees.txt").unwrap());
    for steps in 0..lcm(dim[0], dim[1]) {
        let _ = writeln!(stream, "Steps: {steps}");
        print_robots(
            move_robots(parse_input(input), dim, steps),
            dim,
            &mut stream,
        );
    }
}

fn print_robots<W: std::io::Write>(
    robots: impl Iterator<Item = Robot>,
    dim: [isize; 2],
    stream: &mut W,
) {
    let robot_pos = robots.map(|robot| robot.pos).collect::<BTreeSet<_>>();
    for y in 0..dim[1] {
        for x in 0..dim[0] {
            let _ = stream.write(if robot_pos.contains(&[x, y]) {
                b"#"
            } else {
                b"."
            });
        }
        let _ = stream.write(b"\n");
    }
    let _ = stream.write(b"\n");
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2024/day14.example1.txt"
    ));
    assert_eq!(part1_impl(input, [11, 7], 100), 1 * 3 * 4 * 1);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2024/day14.txt"
    ));
    assert_eq!(part1(input), 224438715);
}
