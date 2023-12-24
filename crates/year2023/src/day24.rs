use std::collections::HashMap;

use z3::{
    ast::{Int, Real},
    Symbol,
};

#[derive(Debug)]
struct Hail {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn parse_input(input: &str) -> impl Iterator<Item = Hail> + '_ {
    input.lines().map(|line| {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let [x, y, z] = pos
            .split(", ")
            .map(|elem| elem.trim().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let [dx, dy, dz] = vel
            .split(", ")
            .map(|elem| elem.trim().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Hail {
            pos: [x, y, z],
            vel: [dx, dy, dz],
        }
    })
}

pub fn part1(input: &str) -> u32 {
    let hail: Vec<_> = parse_input(input).collect();
    xy_collisions(&hail, 200000000000000.0..=400000000000000.0)
}

pub fn part2(input: &str) -> i64 {
    use z3::ast::Ast;

    let hails = parse_input(input).collect::<Vec<_>>();

    let mut z3_conf = z3::Config::new();
    z3_conf.set_bool_param_value("parallel.enable", true);
    let ctx = z3::Context::new(&z3_conf);
    let solver = z3::Solver::new(&ctx);

    let x_sym = Symbol::String(String::from("x"));
    let y_sym = Symbol::String(String::from("y"));
    let z_sym = Symbol::String(String::from("z"));
    let x = Int::new_const(&ctx, x_sym);
    let y = Int::new_const(&ctx, y_sym);
    let z = Int::new_const(&ctx, z_sym);

    let dx_sym = Symbol::String(String::from("dx"));
    let dy_sym = Symbol::String(String::from("dy"));
    let dz_sym = Symbol::String(String::from("dz"));
    let dx = Int::new_const(&ctx, dx_sym);
    let dy = Int::new_const(&ctx, dy_sym);
    let dz = Int::new_const(&ctx, dz_sym);

    let zero = Real::from_int(&Int::from_u64(&ctx, 0));

    let vars: Vec<_> = hails
        .iter()
        .enumerate()
        .map(|(idx, hail)| {
            let t = z3::ast::Real::new_const(&ctx, Symbol::Int(idx as u32));

            let hx = Int::from_i64(&ctx, hail.pos[0]);
            let hy = Int::from_i64(&ctx, hail.pos[1]);
            let hz = Int::from_i64(&ctx, hail.pos[2]);

            let hdx = Int::from_i64(&ctx, hail.vel[0]);
            let hdy = Int::from_i64(&ctx, hail.vel[1]);
            let hdz = Int::from_i64(&ctx, hail.vel[2]);

            (t, [(hx, hdx), (hy, hdy), (hz, hdz)])
        })
        .collect();

    for (t, dims) in &vars {
        solver.assert(&t.gt(&zero));

        for ((h, dh), (s, ds)) in dims.iter().zip([(&x, &dx), (&y, &dy), (&z, &dz)]) {
            let assume = Real::from_int(&Int::sub(&ctx, &[&h, &s]))._eq(&Real::mul(
                &ctx,
                &[&Real::from_int(&Int::sub(&ctx, &[&ds, &dh])), &t],
            ));
            solver.assert(&assume);
        }
    }

    let x_vels = hails.iter().enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<i64, Vec<_>>, (idx, hail)| {
            map.entry(hail.vel[0]).or_default().push((idx, hail));
            map
        },
    );

    for (vel, hails) in x_vels {
        if hails.len() > 1 {
            let mut rem = hails.as_slice();
            while let [(idxa, a), tail @ ..] = rem {
                rem = tail;
                for (idxb, b) in tail {
                    let dt = Real::sub(&ctx, &[&vars[*idxa].0, &vars[*idxb].0]);
                    let rx = Real::from_int(&Int::from_i64(&ctx, a.pos[0] - b.pos[0]));
                    let ddx = Int::sub(&ctx, &[&dx, &Int::from_i64(&ctx, vel)]);
                    let assert = Real::mul(&ctx, &[&Real::from_int(&ddx), &dt])._eq(&rx);
                    solver.assert(&assert);
                }
            }
        }
    }

    let y_vels = hails.iter().enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<i64, Vec<_>>, (idx, hail)| {
            map.entry(hail.vel[1]).or_default().push((idx, hail));
            map
        },
    );

    for (vel, hails) in y_vels {
        if hails.len() > 1 {
            let mut rem = hails.as_slice();
            while let [(idxa, a), tail @ ..] = rem {
                rem = tail;
                for (idxb, b) in tail {
                    let dt = Real::sub(&ctx, &[&vars[*idxa].0, &vars[*idxb].0]);
                    let ry = Real::from_int(&Int::from_i64(&ctx, a.pos[1] - b.pos[1]));
                    let ddy = Int::sub(&ctx, &[&dy, &Int::from_i64(&ctx, vel)]);
                    let assert = Real::mul(&ctx, &[&Real::from_int(&ddy), &dt])._eq(&ry);
                    solver.assert(&assert);
                }
            }
        }
    }

    let z_vels = hails.iter().enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<i64, Vec<_>>, (idx, hail)| {
            map.entry(hail.vel[2]).or_default().push((idx, hail));
            map
        },
    );

    for (vel, hails) in z_vels {
        if hails.len() > 1 {
            let mut rem = hails.as_slice();
            while let [(idxa, a), tail @ ..] = rem {
                rem = tail;
                for (idxb, b) in tail {
                    let dt = Real::sub(&ctx, &[&vars[*idxa].0, &vars[*idxb].0]);
                    let rz = Real::from_int(&Int::from_i64(&ctx, a.pos[2] - b.pos[2]));
                    let ddz = Int::sub(&ctx, &[&dz, &Int::from_i64(&ctx, vel)]);
                    let assert = Real::mul(&ctx, &[&Real::from_int(&ddz), &dt])._eq(&rz);
                    solver.assert(&assert);
                }
            }
        }
    }

    let ax1 = dx.le(&Int::from_i64(&ctx, 1000));
    let ay1 = dy.le(&Int::from_i64(&ctx, 1000));
    let az1 = dz.le(&Int::from_i64(&ctx, 1000));
    let ax2 = dx.ge(&Int::from_i64(&ctx, -1000));
    let ay2 = dy.ge(&Int::from_i64(&ctx, -1000));
    let az2 = dz.ge(&Int::from_i64(&ctx, -1000));

    println!("Build Ast, now solving");
    solver.check_assumptions(&[ax1, ax2, ay1, ay2, az1, az2]);
    let model = solver.get_model().unwrap();
    let res_x = model.eval(&x, true);
    let res_y = model.eval(&y, true);
    let res_z = model.eval(&z, true);
    let x = res_x.unwrap().as_i64();
    let y = res_y.unwrap().as_i64();
    let z = res_z.unwrap().as_i64();
    x.unwrap() + z.unwrap() + y.unwrap()
}

fn xy_collisions(hail: &[Hail], range: std::ops::RangeInclusive<f64>) -> u32 {
    let mut rem = hail;
    let mut count = 0;

    while let [head, tail @ ..] = rem {
        rem = tail;
        for other in tail {
            if let Some(pos) = xy_collision(head, other) {
                if range.contains(&pos[0]) && range.contains(&pos[1]) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn xy_collision(head: &Hail, other: &Hail) -> Option<[f64; 2]> {
    assert_ne!(other.vel[0], 0);
    assert_ne!(other.vel[1], 0);
    assert_ne!(head.vel[0], 0);
    assert_ne!(head.vel[1], 0);

    // x1 + dx1 * t1 == x2 + dx2 * t2
    // y1 + dy1 * t1 == y2 + dy2 * t2
    //
    // x1 - x2 + dx1 * t1 == dx2 * t2                                           dx1 * t1 == dx2 * t2 + x2 - x1
    // y1 - y2 + dy1 * t1 == dy2 * t2                                           dy1 * t1 == dy2 * t2 + y2 - y1
    //
    // (x1 - x2 + dx1 * t1) / dx2 == t2                                         t1 = (dx2 * t2 + x2 - x1) / dx1
    // (y1 - y2 + dy1 * t1) / dy2 == t2                                         t1 = (dy2 * t2 + y2 - y1) / dy1
    //
    // (x1 - x2 + dx1 * t1) / dx2 == (y1 - y2 + dy1 * t1) / dy2                 (dx2 * t2 + x2 - x1) / dx1 = (dy2 * t2 + y2 - y1) / dy1
    // (x1 - x2 + dx1 * t1) * dy2 == (y1 - y2 + dy1 * t1) * dx2                 (dx2 * t2 + x2 - x1) * dy1 = (dy2 * t2 + y2 - y1) * dx1
    // (x1 - x2) * dy2 +  dy2 * dx1 * t1 == (y1 - y2) * dx2 + dy1 * t1 * dx2    dx2 * dy1 * t2 + (x2 - x1) * dy1 = dy2 * dx1 * t2 + (y2 - y1) * dx1
    // (x1 - x2) * dy2 - (y1 - y2) * dx2  / (dx2 * dy1 - dy2 * dx1) == t1       (x2 - x1) * dy1 - (y2 - y1) * dx1 / (dy2 * dx1 - dx2 * dy1) = t2

    if head.vel[0] * other.vel[1] != head.vel[1] * other.vel[0] {
        // (x1 - y1 + y2 -x2) / (dy1 - dx1) = t1
        let t1_num = (head.pos[0] - other.pos[0]) * other.vel[1]
            - (head.pos[1] - other.pos[1]) * other.vel[0];
        let t1_den = other.vel[0] * head.vel[1] - other.vel[1] * head.vel[0];

        if t1_num.signum() != t1_den.signum() {
            return None;
        }

        let t2_num =
            (other.pos[0] - head.pos[0]) * head.vel[1] - (other.pos[1] - head.pos[1]) * head.vel[0];
        let t2_den = other.vel[1] * head.vel[0] - other.vel[0] * head.vel[1];

        if t2_num.signum() != t2_den.signum() {
            return None;
        }

        let x = head.pos[0] as f64 + (head.vel[0] as i128 * t1_num as i128) as f64 / t1_den as f64;
        let y = head.pos[1] as f64 + (head.vel[1] as i128 * t1_num as i128) as f64 / t1_den as f64;

        Some([x, y])
    } else {
        // dx1 * dy2 == dy1 * dx2 = a

        // (x1 - x2) * dy2 +  dy2 * dx1 * t1 == (y1 - y2) * dx2 + dy1 * dx2 * t1
        // (x1 - x2) * dy2 +  a * t1 == (y1 - y2) * dx2 + a * t1
        // (x1 - x2) * dy2 == (y1 - y2) * dx2

        if (head.pos[0] - other.pos[0]) * other.vel[1]
            == (head.pos[1] - other.pos[1]) * other.vel[0]
        {
            panic!()
        } else {
            None
        }
    }
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day24.example.txt");
    let hail: Vec<_> = parse_input(input).collect();
    let result = xy_collisions(&hail, 7.0..=27.0);
    assert_eq!(result, 2);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day24.txt"));
    let result = part1(input);
    assert_eq!(result, 15107);
}

#[test]
#[ignore = "too slow"]
fn part2_example() {
    let input = include_str!("../input/day24.example.txt");
    assert_eq!(part2(input), 24 + 13 + 10);
}

#[test]
// #[ignore = "too slow"]
fn part2_full() {
    let input = include_str!(concat!("../input/day24.txt"));
    assert_eq!(part2(input), 1262);
}
