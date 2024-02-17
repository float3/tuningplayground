use tuning_systems::Fraction;
use tuning_systems::Tone;

fn main() {
    let tone = Tone::new("C", Fraction::new(1, 1), 1, 0);
    println!("tone: {:?}", tone);
    let tone = Tone::new("C", Fraction::new(1, 1), 2, 0);
    println!("tone: {:?}", tone);
    let tone = Tone::new("C", Fraction::new(1, 1), 3, 0);
    println!("tone: {:?}", tone);
}
