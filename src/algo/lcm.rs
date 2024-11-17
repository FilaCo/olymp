use crate::algo::gcd;
use std::ops::{Div, Mul, Rem};

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Default + PartialEq + Rem<Output = T> + Copy + Div<Output = T> + Mul<Output = T>,
{
    a / gcd(a, b) * b
}
