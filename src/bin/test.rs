fn main() {
    use tuning_systems::Fraction;
    let tone = tuning_systems::Tone::new("C", Fraction::new(1, 1), 4, 0);
    println!("Tone: {}", tone.fraction().numerator);
    println!("Tone: {}", tone.frequency());
    println!("Tone: {}", tone.fraction().numerator);
}
