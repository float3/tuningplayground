use wasm_bindgen::convert::{FromWasmAbi, WasmAbi};
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
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

//if wasm-bindgen is enabled impl WasmDescribe for Fraction
#[cfg(feature = "wasm-bindgen")]
impl WasmAbi for Fraction {
    type Prim1;

    type Prim2;

    type Prim3;

    type Prim4;

    fn split(self) -> (Self::Prim1, Self::Prim2, Self::Prim3, Self::Prim4) {
        todo!()
    }

    fn join(
        prim1: Self::Prim1,
        prim2: Self::Prim2,
        prim3: Self::Prim3,
        prim4: Self::Prim4,
    ) -> Self {
        todo!()
    }
}
#[cfg(feature = "wasm-bindgen")]
impl FromWasmAbi for Fraction {
    type Abi = (u32, u32, u32);

    unsafe fn from_abi(js: Self::Abi) -> Self {
        todo!()
    }
}
