use std::{collections::HashMap, vec};

use crate::{interval::Interval, note::Note};

#[derive(Clone, PartialEq)]
pub struct Pitch {
    pub name: String,
    pub alter: f64,
    pub accidental: Option<Accidental>,
    pub octave: Option<i32>,
    pub diatonic_note_num: i32,
    pub ps: i32,
    pub pitch_class: i32,
    // pub frequency: f64,
}

#[derive(Clone, PartialEq)]
struct Accidental {
    name: String,
    alter: f64,
}

impl Accidental {
    fn new(arg: &str) -> Accidental {
        todo!()
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch.clone()
    }
}

#[derive(Clone)]
enum TranspositionIntervalDirection {
    Up,
    Down,
}

impl TranspositionIntervalDirection {
    fn to_string(&self) -> String {
        match self {
            TranspositionIntervalDirection::Up => "d2".to_string(),
            TranspositionIntervalDirection::Down => "-d2".to_string(),
        }
    }
}

type PitchReturn = Option<Pitch>;

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

        let accidental = Some(Accidental {
            name: accidental,
            alter,
        });

        Pitch {
            name: name.to_string(),
            alter,
            accidental,
            octave,
            diatonic_note_num: todo!(),
            ps: todo!(),
            pitch_class: todo!(),
        }
    }

    pub(crate) fn transpose(&self, arg: &Interval) -> Pitch {
        todo!()
    }

    fn transpose_note(&self, note: &Note) -> Note {
        let new_pitch = self.transpose_pitch(&note.pitch);
        let mut new_note = note.clone();
        new_note.pitch = new_pitch;
        return new_note;
    }

    fn transpose_pitch(&self, arg: &Pitch) -> Pitch {
        todo!()
    }

    fn get_all_common_enharmonics(&self, alter_limit: i32) -> Vec<Pitch> {
        let mut post: Vec<Pitch> = vec![];
        let c = self.simplify_enharmonic(false);
        if c.name != self.name {
            post.push(c);
        }
        let c = self.clone();

        let mut get_enharmonics = |c: Pitch, direction: TranspositionIntervalDirection| {
            let mut c = c;
            while let Some(pitch) = c.get_enharmonic_helper(direction.clone()) {
                if let Some(ref accidental) = pitch.accidental {
                    if accidental.alter.abs() > (alter_limit as f64) {
                        break;
                    }
                }
                if !post.contains(&pitch.clone()) {
                    post.push(pitch.clone());
                } else {
                    break;
                }
                c = pitch;
            }
        };

        get_enharmonics(c.clone(), TranspositionIntervalDirection::Up);
        get_enharmonics(c, TranspositionIntervalDirection::Down);

        post
    }

    fn simplify_enharmonic(&self, most_common: bool) -> Pitch {
        let mut c = self.clone();

        if let Some(ref accidental) = c.accidental {
            if accidental.alter.abs() < 2.0 && !["E#", "B#", "C-", "F-"].contains(&c.name.as_str())
            {
                // pass
            } else {
                let save_octave = self.octave;
                c.ps = self.ps;
                if save_octave.is_none() {
                    c.octave = None;
                }
            }
        }

        if most_common {
            match c.name.as_str() {
                "D#" => {
                    c.name = "E".to_string();
                    c.accidental = Some(Accidental::new("flat"));
                }
                "A#" => {
                    c.name = "B".to_string();
                    c.accidental = Some(Accidental::new("flat"));
                }
                "G-" => {
                    c.name = "F".to_string();
                    c.accidental = Some(Accidental::new("sharp"));
                }
                "D-" => {
                    c.name = "C".to_string();
                    c.accidental = Some(Accidental::new("sharp"));
                }
                _ => {}
            }
        }
        c
    }

    fn get_enharmonic_helper(&self, direction: TranspositionIntervalDirection) -> PitchReturn {
        /*
                       intervalString: t.Literal['d2', '-d2'] = 'd2'
               if not up:
                   intervalString = '-d2'

               if intervalString not in self._transpositionIntervals:
                   self._transpositionIntervals[intervalString] = interval.Interval(intervalString)
               intervalObj = self._transpositionIntervals[intervalString]
               octaveStored = self.octave  # may be None
               p = intervalObj.transposePitch(self, maxAccidental=None)
               if not inPlace:
                   if octaveStored is None:
                       p.octave = None
                   return p
               else:
                   self.step = p.step
                   self.accidental = p.accidental
                   if p.microtone is not None:
                       self.microtone = p.microtone
                   if octaveStored is None:
                       self.octave = None
                   else:
                       self.octave = p.octave
                   return None
        */
        let interval_string = match direction {
            TranspositionIntervalDirection::Up => "d2",
            TranspositionIntervalDirection::Down => "-d2",
        };

        // TODO: cache the transposition intervals?
        // if !self.transpostion_intevals.contains(&interval_string) {}

        let octave_stored = self.octave;
        let p = self.transpose(&Interval::new_from_name(interval_string).unwrap());

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
    pitches: &[Pitch],
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
            for interval in intervals.iter() {
                match interval.interval_to_pythagorean_ratio() {
                    Some(ratio) => {
                        score_ratio += ((ratio.numerator * ratio.denominator) as f64 / ratio.f64())
                            .ln()
                            * 0.03792663444
                    }
                    None => return std::f64::INFINITY,
                };
            }
            score_ratio /= pitches.len() as f64;
        }

        if triad_award {
            intervals.into_iter().for_each(|interval| {
                let simple_directed = interval.generic.simple_directed;
                let interval_semitones = interval.chromatic.semitones % 12;
                if (simple_directed == 3 && (interval_semitones == 3 || interval_semitones == 4))
                    || (simple_directed == 6
                        && (interval_semitones == 8 || interval_semitones == 9))
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
    let mut new_pitches = vec![old_pitches[0].clone()];
    for old_pitch in old_pitches.iter().skip(1) {
        let mut candidates = vec![old_pitch.clone()];
        candidates.extend(old_pitch.get_all_common_enharmonics(2));
        let new_pitch = candidates
            .iter()
            .min_by(|x, y| {
                dissonance_score(&new_pitches, true, true, true)
                    .partial_cmp(&dissonance_score(&new_pitches, true, true, true))
                    .unwrap()
            })
            .unwrap();
        new_pitches.push(new_pitch.clone());
    }
    new_pitches
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
