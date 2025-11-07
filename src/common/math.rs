use num_traits::PrimInt;

pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    if b.is_zero() { a } else { gcd(b, a % b) }
}

pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

// pub fn gcd_signed<T: PrimInt + Signed>(a: T, b: T) -> T {
//     gcd(a.abs(), b.abs())
// }

// pub fn lcm_signed<T: PrimInt + Signed>(a: T, b: T) -> T {
//     let gcd_val = gcd_signed(a, b);
//     a.abs() / gcd_val * b.abs()
// }
