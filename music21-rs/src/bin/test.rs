use music21_rs::chord::Chord;
fn main() {
    let chord = Chord::new("C E G").unwrap();
    println!("{:?}", chord.pitched_common_name);
}
