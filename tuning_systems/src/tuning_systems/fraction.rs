use std::cmp::Ordering;
use std::ops::{Div, DivAssign, Mul, MulAssign};
// use wasm_bindgen::convert::{FromWasmAbi, WasmAbi};
// #[cfg(feature = "wasm-bindgen")]
// use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug)]
// #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
    pub base: u32,
}

impl Fraction {
    pub const fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction::new_with_base(numerator, denominator, 0)
    }

    pub(crate) const fn new_with_base(numerator: u32, denominator: u32, base: u32) -> Fraction {
        Fraction {
            numerator,
            denominator,
            base,
        }
    }

    pub fn f64(&self) -> f64 {
        f64::from(*self)
    }

    pub fn pow(&self, exp: i32) -> Fraction {
        if exp < 0 {
            panic!("Negative exponent is not supported for Fraction::pow");
        }

        Fraction {
            numerator: self.numerator.pow(exp as u32),
            denominator: self.denominator.pow(exp as u32),
            base: self.base,
        }
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

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.base != rhs.base {
            panic!("Fractions have different bases");
        }

        if self.base == 0 {
            Self {
                numerator: self.numerator * rhs.numerator,
                denominator: self.denominator * rhs.denominator,
                base: self.base,
            }
        } else {
            Self {
                numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
                base: self.base,
            }
        }
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        let result = self.mul(rhs);
        self.numerator = result.numerator;
        self.denominator = result.denominator;
        // self.base = result.base;
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.base != rhs.base {
            panic!("Fractions have different bases");
        }

        if self.base == 0 {
            Self {
                numerator: self.numerator * rhs.denominator,
                denominator: self.denominator * rhs.numerator,
                base: self.base,
            }
        } else {
            Self {
                numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
                base: self.base,
            }
        }
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        let result = self.div(rhs);
        self.numerator = result.numerator;
        self.denominator = result.denominator;
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.f64() == other.f64()
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = self.f64();
        let right = other.f64();

        if left == right {
            Some(Ordering::Equal)
        } else if left < right {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

//if wasm-bindgen is enabled impl WasmDescribe for Fraction
// #[cfg(feature = "wasm-bindgen")]
// impl WasmAbi for Fraction {
//     type Prim1;

//     type Prim2;

//     type Prim3;

//     type Prim4;

//     fn split(self) -> (Self::Prim1, Self::Prim2, Self::Prim3, Self::Prim4) {
//         todo!()
//     }

//     fn join(
//         prim1: Self::Prim1,
//         prim2: Self::Prim2,
//         prim3: Self::Prim3,
//         prim4: Self::Prim4,
//     ) -> Self {
//         todo!()
//     }
// }
// #[cfg(feature = "wasm-bindgen")]
// impl FromWasmAbi for Fraction {
//     type Abi = (u32, u32, u32);

//     unsafe fn from_abi(js: Self::Abi) -> Self {
//         todo!()
//     }
// }
