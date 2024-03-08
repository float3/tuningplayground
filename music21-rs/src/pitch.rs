use std::vec;

use crate::note::Note;

use super::interval::Interval;

#[derive(Clone, PartialEq)]
pub struct Pitch {
    pub name: String,
    pub alter: f64,
    pub accidental: Option<Accidental>,
    pub octave: Option<i32>,
    pub diatonic_note_num: i32,
    pub ps: i32,
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
        }
    }

    pub(crate) fn transpose(&self, arg: &Interval) -> Pitch {
        todo!()
    }

    fn get_all_common_enharmonics(&self, alter_limit: i32) -> Vec<Pitch> {
        let mut post: Vec<Pitch> = vec![];
        let c = self.simplify_enharmonic(false);
        if c.name != self.name {
            post.push(c);
        }
        let c = self.clone();
        while let Some(pitch) = c.get_higher_enharmonic() {
            let accidental = pitch.accidental.clone().unwrap();
            //this is incorrect TODO rework this
            if !accidental.name.is_empty() && accidental.alter.abs() > (alter_limit as f64) {
                break;
            }
            if !post.contains(&pitch) {
                post.push(pitch);
            } else {
                break;
            }
        }
        let c = self.clone();
        while let Some(pitch) = c.get_lower_enharmonic() {
            let accidental = pitch.accidental.clone().unwrap();
            //this is incorrect TODO rework this
            if !accidental.name.is_empty() && accidental.alter.abs() > (alter_limit as f64) {
                break;
            }
            if !post.contains(&pitch) {
                post.push(pitch);
            } else {
                break;
            }
        }

        post
    }

    fn simplify_enharmonic(&self, most_common: bool) -> Pitch {
        /*        if returnObj.accidental is not None:
            if (abs(returnObj.accidental.alter) < 2.0
                    and returnObj.name not in ('E#', 'B#', 'C-', 'F-')):
                pass
            else:
                # by resetting the pitch space value, we will get a simpler
                # enharmonic spelling
                saveOctave = self.octave
                returnObj.ps = self.ps
                if saveOctave is None:
                    returnObj.octave = None

        if mostCommon:
            if returnObj.name == 'D#':
                returnObj.step = 'E'
                returnObj.accidental = Accidental('flat')
            elif returnObj.name == 'A#':
                returnObj.step = 'B'
                returnObj.accidental = Accidental('flat')
            elif returnObj.name == 'G-':
                returnObj.step = 'F'
                returnObj.accidental = Accidental('sharp')
            elif returnObj.name == 'D-':
                returnObj.step = 'C'
                returnObj.accidental = Accidental('sharp') */

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

    fn get_higher_enharmonic(&self) -> Option<Pitch> {
        todo!()
    }

    fn get_lower_enharmonic(&self) -> Option<Pitch> {
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
    /*def _greedyEnharmonicsSearch(oldPitches, scoreFunc=_dissonanceScore):
    newPitches = oldPitches[:1]
    for oldPitch in oldPitches[1:]:
        candidates = [oldPitch] + oldPitch.getAllCommonEnharmonics()
        newPitch = min(candidates, key=lambda x: scoreFunc(newPitches + [x]))
        newPitches.append(newPitch)
    return newPitches */
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
