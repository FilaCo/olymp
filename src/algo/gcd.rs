use std::ops::Rem;

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Default + PartialEq + Rem<Output = T> + Copy,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}
