use super::interval::Interval;

#[derive(Clone)]
pub struct Pitch {
    pub name: String,
    pub alter: f64,
    accidental: String,
    pub octave: i32,
    // pub octave: i32,
    // pub accidental: String,
    // pub frequency: f64,
}

impl Pitch {
    pub(crate) fn new(string: String) -> Pitch {
        let mut tokens = string.chars().peekable();

        let name = tokens.next().expect("no name");
        if name > 'G' || name < 'A' {
            panic!("Invalid note name");
        }
        let alter;
        let accidental;
        match tokens.next() {
            Some('b') => {
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
        Pitch {
            name: name.to_string(),
            alter,
            accidental,
        }
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

pub(crate) fn simplify_multiple_enharmonics(pitches: Vec<Pitch>) -> _ {
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
        //     try:
        //     intervals = [interval.Interval(noteStart=p1, noteEnd=p2)
        //                 for p1, p2 in itertools.combinations(pitches, 2)]
        //     except interval.IntervalException:
        //          return math.inf
        // in rust
        for (index, p1) in pitches.iter().enumerate() {
            for p2 in pitches.iter().skip(index + 1) {
                intervals.push(Interval::new(p1.clone(), p2.clone()));
            }
        }

        if small_pythagorean_ratio {
            // # score_ratio = Pythagorean ratio complexity per pitch
            // for this_interval in intervals:
            //     # does not accept weird intervals, e.g. with semitones
            //     ratio = interval.intervalToPythagoreanRatio(this_interval)
            //     # d2 is 1.0
            //     penalty = math.log(ratio.numerator * ratio.denominator / ratio) * 0.03792663444
            //     score_ratio += penalty

            // score_ratio /= len(pitches)
            for interval in intervals {
                let ratio = interval.interval_to_pythagorean_ratio();
                let penalty = (ratio.numerator * ratio.denominator / ratio).log() * 0.03792663444;
                score_ratio += penalty;
            }
        }

        // for (p1, p2) in pitches.iter().enumerate() {
        //     intervals.push(Interval::new(p1.clone(), p2.clone()));
        // }
    }

    return 0.0;
}

fn greedy_enharmonics_search(old_pitches: Vec<Pitch>, criterion: f64) -> Vec<Pitch> {
    todo!()
}

fn brute_force_enharmonics_search(old_pitches: Vec<Pitch>, criterion: f64) -> Vec<Pitch> {
    todo!()
}
