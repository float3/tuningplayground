#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
    pub base: u32,
}

impl Fraction {
    pub const fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction {
            numerator,
            denominator,
            base: 0,
        }
    }

    pub const fn new_with_base(numerator: u32, denominator: u32, base: u32) -> Fraction {
        Fraction {
            numerator,
            denominator,
            base,
        }
    }

    pub fn numerator(&self) -> u32 {
        self.numerator
    }

    pub fn denominator(&self) -> u32 {
        self.denominator
    }
}

impl From<Fraction> for f64 {
    fn from(num: Fraction) -> f64 {
        if num.base == 0 {
            num.denominator as f64 / num.numerator as f64
        } else {
            (num.base as f64).powf(num.denominator as f64 / num.numerator as f64)
        }
    }
}

impl From<(u32, u32)> for Fraction {
    fn from(nums: (u32, u32)) -> Fraction {
        Fraction::new(nums.0, nums.1)
    }
}
