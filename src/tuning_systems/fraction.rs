#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
    pub base: u32,
}

impl Fraction {
    pub const fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction::new_with_base(numerator, denominator, 0)
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
    fn from(frac: Fraction) -> f64 {
        if frac.base == 0 {
            frac.numerator as f64 / frac.denominator as f64
        } else {
            (frac.base as f64).powf(frac.numerator as f64 / frac.denominator as f64)
        }
    }
}

impl From<(u32, u32)> for Fraction {
    fn from(frac: (u32, u32)) -> Fraction {
        Fraction::new(frac.0, frac.1)
    }
}

impl From<(u32, u32, u32)> for Fraction {
    fn from(frac: (u32, u32, u32)) -> Fraction {
        Fraction::new_with_base(frac.0, frac.1, frac.2)
    }
}
