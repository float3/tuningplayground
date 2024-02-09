#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction(pub i32, pub i32);

impl From<Fraction> for f64 {
    fn from(num: Fraction) -> f64 {
        num.0 as f64 / num.1 as f64
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(nums: (i32, i32)) -> Fraction {
        Fraction(nums.0, nums.1)
    }
}
