use crate::note::Note;

use super::interval::Interval;

#[derive(Clone)]
pub struct Pitch {
    pub name: String,
    pub alter: f64,
    pub accidental: String,
    pub octave: Option<i32>,
    pub diatonic_note_num: i32,
    // pub frequency: f64,
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch
    }
}

impl Pitch {
    pub fn new(string: String) -> Pitch {
        let mut tokens = string.chars().peekable();

        let name = tokens.next().expect("no name");
        if !('A'..='G').contains(&name) {
            panic!("Invalid note name");
        }
        let alter;
        let accidental;
        match tokens.peek() {
            Some('b') => {
                tokens.next();
                if tokens.peek() == Some(&'b') {
                    tokens.next();
                    alter = -2.0;
                    accidental = "bb".to_string();
                } else {
                    alter = -1.0;
                    accidental = "b".to_string();
                }
            }
            Some('#') => {
                tokens.next();
                if tokens.peek() == Some(&'#') {
                    tokens.next();
                    alter = 2.0;
                    accidental = "##".to_string();
                } else {
                    alter = 1.0;
                    accidental = "#".to_string();
                }
            }
            _ => {
                alter = 0.0;
                accidental = "".to_string();
            }
        }

        let octave: Option<i32> = if tokens.peek().is_some() {
            let x = tokens.collect::<String>();
            Some(
                x.parse::<i32>()
                    .unwrap_or_else(|_| panic!("Invalid octave: {}", x)),
            )
        } else {
            None
        };

        Pitch {
            name: name.to_string(),
            alter,
            accidental,
            octave,
        }
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

pub(crate) fn simplify_multiple_enharmonics(pitches: Vec<Pitch>) -> Vec<Pitch> {
    let old_pitches: Vec<Pitch> = pitches.clone();
    let criterion = dissonance_score(&pitches, true, true, true);
    if old_pitches.len() < 5 {
        brute_force_enharmonics_search(old_pitches, criterion)
    } else {
        greedy_enharmonics_search(old_pitches, criterion)
    }
}

fn dissonance_score(
    pitches: &Vec<Pitch>,
    small_pythagorean_ratio: bool,
    accidental_penalty: bool,
    triad_award: bool,
) -> f64 {
    let mut score_accidentals = 0.0;
    let mut score_ratio = 0.0;
    let mut score_triad = 0.0;

    if pitches.is_empty() {
        return 0.0;
    }

    if accidental_penalty {
        let accidentals = pitches.iter().map(|p| p.alter.abs()).collect::<Vec<f64>>();
        score_accidentals = accidentals
            .iter()
            .map(|a| if *a > 1.0 { *a } else { 0.0 })
            .sum::<f64>()
            / pitches.len() as f64;
    }

    let mut intervals = vec![];

    if small_pythagorean_ratio | triad_award {
        for (index, p1) in pitches.iter().enumerate() {
            for p2 in pitches.iter().skip(index + 1) {
                match Interval::new(p1.clone(), p2.clone()) {
                    Some(interval) => intervals.push(interval),
                    None => return std::f64::INFINITY,
                }
            }
        }

        if small_pythagorean_ratio {
            intervals.iter().for_each(|interval| {
                let ratio = interval.interval_to_pythagorean_ratio();
                let penalty = ((ratio.numerator * ratio.denominator) as f64 / ratio.f64()).ln()
                    * 0.03792663444;
                score_ratio += penalty;
            });
            score_ratio /= pitches.len() as f64;
        }

        if triad_award {
            intervals.into_iter().for_each(|interval| {
                let simple_directed = interval.generic.simple_directed;
                let interval_semitones = interval.chromatic.semitones % 12;
                if simple_directed == 3 && (interval_semitones == 3 || interval_semitones == 4) {
                    score_triad -= 1.0;
                } else if simple_directed == 6
                    && (interval_semitones == 8 || interval_semitones == 9)
                {
                    score_triad -= 1.0;
                }
            });
            score_triad /= pitches.len() as f64;
        }
    }

    (score_accidentals + score_ratio + score_triad)
        / (small_pythagorean_ratio as i32 + accidental_penalty as i32 + triad_award as i32) as f64
}

fn greedy_enharmonics_search(old_pitches: Vec<Pitch>, criterion: f64) -> Vec<Pitch> {
    todo!()
}

fn brute_force_enharmonics_search(old_pitches: Vec<Pitch>, criterion: f64) -> Vec<Pitch> {
    todo!()
}

fn convert_harmonic_to_cents(mut value: f64) -> i32 {
    if value < 0.0 {
        value = 1.0 / value.abs();
    }
    (1200.0 * value.log2()).round() as i32
}
