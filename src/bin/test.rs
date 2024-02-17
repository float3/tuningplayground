use tuning_systems::equal_temperament_12;
use tuning_systems::Fraction;
use tuning_systems::Tone;

fn main() {
    construct_ET_tone(0);
    construct_ET_tone(1);
    construct_ET_tone(2);
    construct_ET_tone(3);
    construct_ET_tone(4);
    construct_ET_tone(5);
    construct_ET_tone(6);
    construct_ET_tone(7);
    construct_ET_tone(8);
    construct_ET_tone(9);
    construct_ET_tone(10);
    construct_ET_tone(11);
    construct_ET_tone(12);
    construct_ET_tone(13);
    construct_ET_tone(24);
    construct_ET_tone(36);
    construct_ET_tone(48);
    construct_ET_tone(57);
    construct_ET_tone(60);
    construct_ET_tone(72);
    // println!("{}", CN1);
}

fn construct_ET_tone(index: u32) {
    let frac: Fraction = equal_temperament_12(index);
    //let frac: Fraction = JUST_INTONATION[idx as usize];
    //println!("frac: {:?}", frac);
    let tone = Tone::new(frac, index);
    println!("tone: {:?}", tone);
    println!("freq: {:?}", tone.frequency());
}
