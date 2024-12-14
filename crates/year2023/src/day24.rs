use helper::{Task, TASKS};
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../inputs/personal/year2023/day01.txt"
));

#[distributed_slice(TASKS)]
static PART1: Task = Task {
    path: &["2023", "24", "part1"],
    run: || println!("{}", part1(INPUT)),
    include_in_all: true,
};

#[distributed_slice(TASKS)]
static PART2: Task = Task {
    path: &["2023", "24", "part2"],
    run: || println!("{}", part2(INPUT)),
    include_in_all: true,
};

#[derive(Debug)]
struct Hail {
    pos: [i64; 3],
    vel: [i64; 3],
}
impl Hail {
    fn adjust(&self, dv: [i64; 3]) -> Hail {
        Self {
            pos: self.pos,
            vel: [
                self.vel[0] - dv[0],
                self.vel[1] - dv[1],
                self.vel[2] - dv[2],
            ],
        }
    }
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
    let range = 200000000000000..=400000000000000;

    xy_collisions(&hail, range)
}

fn xy_collisions(mut rem: &[Hail], range: std::ops::RangeInclusive<i64>) -> u32 {
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

pub fn part2(input: &str) -> i64 {
    let hail: Vec<_> = parse_input(input).collect();

    for velocity in 0i64.. {
        println!("Checking total velociy of {velocity}");
        for dx in -velocity..=velocity {
            for dy in (-velocity + dx.abs())..=(velocity - dx.abs()) {
                for dz in [
                    -velocity + dx.abs() + dy.abs(),
                    velocity - dx.abs() - dy.abs(),
                ] {
                    if let Some(value) = check_velocity([dx, dy, dz], &hail) {
                        return value;
                    }
                }
            }
        }
    }
    panic!("Failed to find a solution");
}

fn check_velocity(delta: [i64; 3], hail: &[Hail]) -> Option<i64> {
    if let Some(pos) = xyz_collision(&hail[0].adjust(delta), &hail[1].adjust(delta)) {
        println!("Verifying {pos:?} @ {delta:?}");
        for hail in &hail[2..] {
            if !passes_trough(pos, &hail.adjust(delta)) {
                return None;
            }
        }

        return Some(pos.into_iter().sum());
    }
    None
}

fn passes_trough(pos: [i64; 3], hail: &Hail) -> bool {
    if pos == hail.pos {
        return true;
    }

    #[allow(
        clippy::needless_range_loop,
        reason = "we index into multiple slices simultaneously"
    )]
    for i in 0..=2 {
        if hail.vel[i] == 0 && hail.pos[i] != pos[i] {
            return false;
        }
    }

    if hail.vel[0] == 0 {
        if hail.vel[1] == 0 {
            // tz = sz + dz * t | - sz
            // tz - sz = dz * t | / dz
            // (tz - sz) / dz = t
            let y = hail.pos[1] + (pos[2] - hail.pos[2]) * hail.vel[1] / hail.vel[2];
            let z = hail.pos[2] + (pos[2] - hail.pos[2]) * hail.vel[2] / hail.vel[2];
            return y == pos[1] && z == pos[2];
        }
        // ty = sy + dy * t | - sy
        // ty - sy = dy * t | / dy
        // (ty - sy) / dy = t
        let y = hail.pos[1] + (pos[1] - hail.pos[1]) * hail.vel[1] / hail.vel[1];
        let z = hail.pos[2] + (pos[1] - hail.pos[1]) * hail.vel[2] / hail.vel[1];
        return y == pos[1] && z == pos[2];
    }

    // tx = sx + dx * t |-sx
    // tx - sx = dx * t | / dx
    // (tx - sx) / dx = t
    let y = hail.pos[1] + (pos[0] - hail.pos[0]) * hail.vel[1] / hail.vel[0];
    let z = hail.pos[2] + (pos[0] - hail.pos[0]) * hail.vel[2] / hail.vel[0];
    y == pos[1] && z == pos[2]
}

fn xyz_collision(head: &Hail, other: &Hail) -> Option<[i64; 3]> {
    if head.vel.into_iter().all(|c| c == 0) {
        if passes_trough(head.pos, other) {
            return Some(head.pos);
        } else {
            return None;
        }
    }

    if other.vel.into_iter().all(|c| c == 0) {
        if passes_trough(other.pos, head) {
            return Some(other.pos);
        } else {
            return None;
        }
    }

    for i in 0..=2 {
        #[allow(clippy::collapsible_if)]
        if head.vel[i] == 0 && other.vel[i] != 0 {
            // hpi = opi + t * ovi | - opi
            // hpi - opi = t * ovi | / ovi
            // (hpi - opi) / ovi = t | / ovi
            let pos = [
                other.pos[0] + (head.pos[i] - other.pos[i]) * other.vel[0] / other.vel[i],
                other.pos[1] + (head.pos[i] - other.pos[i]) * other.vel[1] / other.vel[i],
                other.pos[2] + (head.pos[i] - other.pos[i]) * other.vel[2] / other.vel[i],
            ];
            if passes_trough(pos, head) {
                return Some(pos);
            } else {
                return None;
            }
        } else if head.vel[i] != 0 && other.vel[i] == 0 {
            // opi = hpi + t * hvi | - hpi
            // opi - hpi = t * hvi | / hvi
            // (opi - hpi) / hvi = t | / hvi

            let pos = [
                head.pos[0] + (other.pos[i] - head.pos[i]) * head.vel[0] / head.vel[i],
                head.pos[1] + (other.pos[i] - head.pos[i]) * head.vel[1] / head.vel[i],
                head.pos[2] + (other.pos[i] - head.pos[i]) * head.vel[2] / head.vel[i],
            ];
            if passes_trough(pos, other) {
                return Some(pos);
            } else {
                return None;
            }
        } else if head.vel[i] == 0 && other.vel[i] == 0 {
            if head.pos[i] != other.pos[i] {
                return None;
            }
        }
    }

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
        let z = head.pos[2] as f64 + (head.vel[2] as i128 * t1_num as i128) as f64 / t1_den as f64;
        let oz =
            other.pos[2] as f64 + (other.vel[2] as i128 * t2_num as i128) as f64 / t2_den as f64;

        if z != oz {
            return None;
        }

        Some([x as i64, y as i64, z as i64])
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

fn xy_collision(head: &Hail, other: &Hail) -> Option<[i64; 2]> {
    if head.vel[0] == 0 {
        if other.vel[0] == 0 {
            if head.pos[0] != other.pos[0] {
                return None;
            }

            if head.vel[1] == 0 {
                if head.vel[1] == 0 {
                    if head.pos[1] != other.pos[1] {
                        return None;
                    }
                    panic!("Same Line")
                }

                let t = (head.pos[1] - other.pos[1]) / other.vel[1];
                let pos = [
                    other.pos[0] + t * other.vel[0],
                    other.pos[1] + t * other.vel[1],
                ];
                if [head.pos[0], head.pos[1]] == pos {
                    return Some(pos);
                } else {
                    return None;
                }
            }
        }
        // hpx = opx + t * ovx     | - opx
        // hpx - opx=  t * ovx     | / ovx
        // (hpx - opx) / ovx =  t  |

        let t = (head.pos[0] - other.pos[0]) / other.vel[0];
        let pos = [
            other.pos[0] + t * other.vel[0],
            other.pos[1] + t * other.vel[1],
        ];
        if [head.pos[0], head.pos[1]] == pos {
            return Some(pos);
        } else {
            return None;
        }
    } else if other.vel[0] == 0 {
        // opx = hpx + t * hvx | - hpx
        // opx - hpx = t * hvx | / hvx
        // (opx - hpx) / hvx = t

        let t = (other.pos[0] - head.pos[0]) / head.vel[0];
        let pos = [
            other.pos[0] + t * other.vel[0],
            other.pos[1] + t * other.vel[1],
        ];
        if [head.pos[0], head.pos[1]] == pos {
            return Some(pos);
        } else {
            return None;
        }
    } else if head.vel[1] == 0 {
        if other.vel[1] == 0 {
            if head.pos[1] != other.pos[1] {
                return None;
            }
            panic!()
        }
        // hpx = opx + t * ovx     | - opx
        // hpx - opx=  t * ovx     | / ovx
        // (hpx - opx) / ovx =  t  |

        let t = (head.pos[1] - other.pos[1]) / other.vel[1];
        let pos = [
            other.pos[0] + t * other.vel[0],
            other.pos[1] + t * other.vel[1],
        ];
        if [head.pos[0], head.pos[1]] == pos {
            return Some(pos);
        } else {
            return None;
        }
    } else if other.vel[1] == 0 {
        // opy = hpy + t * hvy | - hpy
        // opy - hpy = t * hvy | / hvy
        // (opy - hpy) / hvy = t

        let t = (other.pos[0] - head.pos[0]) / head.vel[0];
        let pos = [
            other.pos[0] + t * other.vel[0],
            other.pos[1] + t * other.vel[1],
        ];
        if [head.pos[0], head.pos[1]] == pos {
            return Some(pos);
        } else {
            return None;
        }
    }

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

        Some([x as i64, y as i64])
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
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day24.example.txt"
    ));
    let hail: Vec<_> = parse_input(input).collect();
    let result = xy_collisions(&hail, 7..=27);
    assert_eq!(result, 2);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day24.txt"
    ));
    let result = part1(input);
    assert_eq!(result, 15107);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day24.example.txt"
    ));
    assert_eq!(part2(input), 24 + 13 + 10);
}

#[test]
fn part2_example2() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day24.example.txt"
    ));
    let hail = parse_input(&input).collect::<Vec<_>>();

    assert!(check_velocity([-3, 1, 2], &hail).is_some())
}

#[test]
#[ignore = "too slow"]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day24.txt"
    ));
    assert_eq!(part2(input), 856642398547748);
}
