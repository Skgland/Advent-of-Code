pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn gcd(width: usize, height: usize) -> usize {
    let mut a = width;
    let mut b = height;
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
