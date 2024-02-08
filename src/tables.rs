use lazy_static::lazy_static;
use std::collections::HashMap;

//type Fraction = (i32, i32);
struct Fraction(i32, i32);

impl From<Fraction> for f64 {
    fn from(num: Fraction) -> f64 {
        return num.0 as f64 / num.1 as f64;
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(nums: (i32, i32)) -> Fraction {
        return Fraction(nums.0, nums.1);
    }
}

type FractionTable = HashMap<i32, Fraction>;

lazy_static! {
    static ref JUST_INTONATION: FractionTable = {
        let mut m = FractionTable::new();
        m.insert(0, Fraction(1, 1));
        m.insert(1, Fraction(17, 16));
        m.insert(2, Fraction(9, 8));
        m.insert(3, Fraction(19, 16));
        m.insert(4, Fraction(5, 4));
        m.insert(5, Fraction(4, 3));
        m.insert(6, Fraction(45, 32));
        m.insert(7, Fraction(3, 2));
        m.insert(8, Fraction(51, 32));
        m.insert(9, Fraction(27, 16));
        m.insert(10, Fraction(57, 32));
        m.insert(11, Fraction(15, 8));
        m
    };
    static ref JUST_INTONATION_24: FractionTable = {
        let mut m = FractionTable::new();
        m.insert(0, Fraction(1, 1));
        m.insert(1, Fraction(33, 32));
        m.insert(2, Fraction(17, 16));
        m.insert(3, Fraction(35, 32));
        m.insert(4, Fraction(9, 8));
        m.insert(5, Fraction(37, 32));
        m.insert(6, Fraction(19, 16));
        m.insert(7, Fraction(39, 32));
        m.insert(8, Fraction(5, 4));
        m.insert(9, Fraction(41, 32));
        m.insert(10, Fraction(2, 3));
        m.insert(11, Fraction(11, 8));
        m.insert(12, Fraction(45, 32));
        m.insert(13, Fraction(93, 64));
        m.insert(14, Fraction(3, 2));
        m.insert(15, Fraction(99, 64));
        m.insert(16, Fraction(51, 32));
        m.insert(17, Fraction(105, 64));
        m.insert(18, Fraction(27, 16));
        m.insert(19, Fraction(111, 64));
        m.insert(20, Fraction(57, 32));
        m.insert(21, Fraction(117, 64));
        m.insert(22, Fraction(15, 8));
        m.insert(23, Fraction(31, 16));
        m
    };
    static ref PYTHAGOREAN_TUNING: FractionTable = {
        let mut m = FractionTable::new();
        m.insert(0, Fraction(1,1));
        m
    };
    /*static ref FIVE_LIMIT: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };
    static ref X: FractionTable = {
        let mut m = FractionTable::new();
    };*/
}
