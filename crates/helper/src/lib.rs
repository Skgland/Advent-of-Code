use linkme::distributed_slice;
use std::ops::{Div, Mul, Not, Rem, Sub};

pub mod iter;

pub struct Task {
    pub path: &'static [&'static str],
    pub run: fn(),
    pub include_in_all: bool,
}

pub struct Position<T, const DIM: usize> {
    pub coords: [T; DIM],
}

impl Position<isize, 2> {
    pub fn apply(dir: Direction) {
        match dir {
            Direction::North => todo!(),
            Direction::East => todo!(),
            Direction::South => todo!(),
            Direction::West => todo!(),
        }
    }
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[allow(non_upper_case_globals)]
impl Direction {
    pub const Up: Self = Self::North;
    pub const Right: Self = Self::East;
    pub const Down: Self = Self::South;
    pub const Left: Self = Self::West;
}

#[distributed_slice]
pub static TASKS: [Task];

pub fn list_with_prefix<S: AsRef<str>>(prefix: &[S]) {
    if prefix.is_empty() {
        println!("Available tasks: ");
    } else {
        println!(
            "All tasks begining with: {}",
            prefix
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
    for task in tasks_filtered_by_prefix(prefix) {
        println!("\t{}", task.path.join(" "));
    }
}

pub fn tasks_filtered_by_prefix<S: AsRef<str>>(
    prefix: &[S],
) -> impl Iterator<Item = &'static Task> + '_ {
    TASKS.iter().filter(move |task| {
        prefix.len() <= task.path.len()
            && task
                .path
                .iter()
                .zip(prefix)
                .all(|(task, prefix)| *task == prefix.as_ref())
    })
}

pub fn run_all_prefix<S: AsRef<str>>(prefix: &[S]) {
    for task in tasks_filtered_by_prefix(prefix) {
        if task.include_in_all {
            println!("Running {}", task.path.join(" "));
            (task.run)()
        } else {
            println!("Skipping {}", task.path.join(" "))
        }
    }
}

pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_zero {
    ($($t:ty),+) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0;
            }
        )*
    };
}

impl_zero!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

pub trait IntegerExtension {
    fn next_power_of_ten(&self) -> Self;
    fn length_base10(&self) -> u32;
}

impl IntegerExtension for u64 {
    fn next_power_of_ten(&self) -> Self {
        match self {
            1..10 => 10,
            10..100 => 100,
            100..1000 => 1000,
            1000..10_000 => 10_000,
            _ => 10u64.pow(self.length_base10()),
        }
    }

    fn length_base10(&self) -> u32 {
        match self {
            1..10 => 1,
            10..100 => 2,
            100..1000 => 3,
            1000..10_000 => 4,
            _ => self.ilog10() + 1,
        }
    }
}

impl IntegerExtension for usize {
    fn next_power_of_ten(&self) -> Self {
        match self {
            1..10 => 10,
            10..100 => 100,
            100..1000 => 1000,
            1000..10_000 => 10_000,
            _ => 10usize.pow(self.ilog10() + 1),
        }
    }

    fn length_base10(&self) -> u32 {
        match self {
            1..10 => 1,
            10..100 => 2,
            100..1000 => 3,
            1000..10_000 => 4,
            _ => self.ilog10() + 1,
        }
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Ord + Rem<Output = T> + Div<Output = T> + Zero + Mul<Output = T>,
{
    a / gcd(a, b) * b
}

pub fn gcd<T>(width: T, height: T) -> T
where
    T: Copy + Ord + Rem<Output = T> + Zero,
{
    let mut a = width;
    let mut b = height;
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b != Zero::ZERO {
        (a, b) = (b, a % b);
    }
    a
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InPoligon {
    In,
    Out,
    OnEdge,
}

impl InPoligon {
    #[allow(non_upper_case_globals)]
    const Cross: Self = InPoligon::In;
    #[allow(non_upper_case_globals)]
    const NoCross: Self = InPoligon::Out;
}

impl Mul for InPoligon {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (InPoligon::OnEdge, _) | (_, InPoligon::OnEdge) => InPoligon::OnEdge,
            (current, InPoligon::Cross) => !current,
            (current, InPoligon::NoCross) => current,
        }
    }
}

impl Not for InPoligon {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            InPoligon::In => InPoligon::Out,
            InPoligon::Out => InPoligon::In,
            InPoligon::OnEdge => InPoligon::OnEdge,
        }
    }
}

pub fn point_in_polygon<T>(point: [T; 2], polygon: &[[T; 2]]) -> InPoligon
where
    T: PartialOrd + Copy + Zero + Sub<Output = T> + Mul<Output = T>,
{
    // based on the pseudo code from https://de.wikipedia.org/wiki/Punkt-in-Polygon-Test_nach_Jordan 2025-12-09

    let mut state = InPoligon::Out;

    for i in 0..polygon.len() {
        state = state * cross_product_test(point, polygon[i], polygon[(i + 1) % polygon.len()]);
        if state == InPoligon::OnEdge {
            return InPoligon::OnEdge;
        }
    }

    state
}

fn cross_product_test<T>(
    [a_x, a_y]: [T; 2],
    [b_x, mut b_y]: [T; 2],
    [c_x, mut c_y]: [T; 2],
) -> InPoligon
where
    T: PartialOrd + Copy + Zero + Sub<Output = T> + Mul<Output = T>,
{
    if a_y == b_y && b_y == c_y {
        // horizontal line segment
        if (b_x..=c_x).contains(&a_x) || (c_x..=b_x).contains(&a_x) {
            // point on the line segment
            InPoligon::OnEdge
        } else {
            InPoligon::NoCross
        }
    } else if a_y == b_y && a_x == b_x {
        // a is b
        InPoligon::OnEdge
    } else {
        if b_y > c_y {
            std::mem::swap(&mut b_y, &mut c_y);
        }

        if a_y <= b_y || a_y > c_y {
            InPoligon::NoCross
        } else {
            let delta = (b_x - a_x) * (c_y - a_y) - (b_y - a_y) * (c_x - a_x);

            match delta.partial_cmp(&T::ZERO).unwrap() {
                std::cmp::Ordering::Greater => InPoligon::Cross,
                std::cmp::Ordering::Less => InPoligon::NoCross,
                std::cmp::Ordering::Equal => InPoligon::OnEdge,
            }
        }
    }
}
