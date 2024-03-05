use crate::chord::Chord;
use music21_rs::chord::chord;
fn main() {
    let chord = Chord::new("C E G").unwrap();
    println!("{:?}", chord.pitched_common_name);
}
