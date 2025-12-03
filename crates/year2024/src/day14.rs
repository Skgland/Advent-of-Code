use helper::lcm;
use helper::{TASKS, Task};
use linkme::distributed_slice;
use std::{collections::BTreeSet, io::BufWriter};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2024/day14.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2024", "14", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2024", "14", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

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
        #[allow(
            clippy::needless_range_loop,
            reason = "we index into multiple slices simultaneously"
        )]
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

pub fn part2(input: &str) -> u32 {
    part2_impl(input, [101, 103])
}

fn part2_impl(input: &str, dim: [isize; 2]) -> u32 {
    use std::io::Write as _;
    let mut stream = if log::log_enabled!(log::Level::Debug) {
        Some(BufWriter::new(std::fs::File::create("trees.txt").unwrap()))
    } else {
        None
    };

    let mut robots: Vec<_> = parse_input(input).collect();
    let mut min_avg_dist_step = 0;
    let mut min_avg_dist = avg_distance(&robots);

    for steps in 0..lcm(dim[0], dim[1]) {
        robots = move_robots(robots.into_iter(), dim, 1).collect();

        let avg_dist = avg_distance(&robots);
        if avg_dist < min_avg_dist {
            min_avg_dist_step = steps + 1;
            min_avg_dist = avg_dist;
        }

        if let Some(stream) = &mut stream {
            let _ = writeln!(stream, "Steps: {steps}");
            print_robots(&robots, dim, stream);
        }
    }

    min_avg_dist_step as u32
}

fn avg_distance(robots: &[Robot]) -> usize {
    let mut dist_sum = 0.0;

    for i in 1..robots.len() - 1 {
        let ([.., a], bs) = robots.split_at(i) else {
            unreachable!()
        };

        for b in bs {
            dist_sum += ((a.pos[0].abs_diff(b.pos[0]) * a.pos[1].abs_diff(b.pos[1])) as f64).sqrt();
        }
    }

    (2.0 * dist_sum / (robots.len() * robots.len() - 1) as f64) as usize
}

fn print_robots<W: std::io::Write>(robots: &[Robot], dim: [isize; 2], stream: &mut W) {
    let robot_pos = robots
        .iter()
        .map(|robot| robot.pos)
        .collect::<BTreeSet<_>>();
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
    assert_eq!(part1(INPUT), 224438715);
}

#[test]
fn part2_full() {
    assert_eq!(part2(INPUT), 7603);
}
