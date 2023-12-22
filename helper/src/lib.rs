use std::ops::{Rem, Div, Mul};

pub trait OptionExtension {
    type Some;
    fn is_none_or<F: FnOnce(&Self::Some) -> bool>(&self, fun: F) -> bool ;
}

impl<T> OptionExtension for Option<T> {
    type Some = T;

    fn is_none_or<F: FnOnce(&<Self as OptionExtension>::Some) -> bool>(&self, fun: F) -> bool  {
        {
            match self {
                None => true,
                Some(val) => fun(val)
            }
        }
    }
}

pub trait Zero {
    const ZERO : Self;
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

impl_zero!(u8, u16, u32, u64, u128, usize);

pub fn lcm<T>(a: T, b: T) -> T  where T: Copy + Ord + Rem<Output = T> + Div<Output = T> + Zero + Mul<Output = T>{
    a / gcd(a, b) * b
}

pub fn gcd<T>(width: T, height: T) -> T where T: Copy + Ord + Rem<Output = T> + Zero {
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
