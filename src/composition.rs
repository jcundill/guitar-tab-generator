use crate::guitar::PitchFingering;
use average::Mean;
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct InvalidInput {
    pub value: String,
    pub line_number: u16,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Line<T> {
    MeasureBreak,
    Rest,
    Playable(T),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum Node {
    Start,
    Rest {
        line_index: u16,
    },
    Note {
        line_index: u16,
        beat_fingering_combo: BeatFingeringCombo,
    },
}

pub type PitchVec<T> = Vec<T>;
pub type BeatVec<T> = Vec<T>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[allow(dead_code)]
pub struct BeatFingeringCombo {
    pub fingering_combo: BeatVec<PitchFingering>,
    pub avg_non_zero_fret: Option<OrderedFloat<f32>>,
    pub non_zero_fret_span: u8,
}
impl BeatFingeringCombo {
    pub fn new(beat_fingering_candidate: BeatVec<&PitchFingering>) -> Self {
        BeatFingeringCombo {
            fingering_combo: beat_fingering_candidate
                .clone()
                .into_iter()
                .cloned()
                .collect(),
            avg_non_zero_fret: calc_avg_non_zero_fret(&beat_fingering_candidate),
            non_zero_fret_span: calc_fret_span(beat_fingering_candidate).unwrap_or(0),
        }
    }
}

#[cfg(test)]
mod test_create_beat_fingering_combo {
    use super::*;
    use crate::{pitch::Pitch, string_number::StringNumber};

    #[test]
    fn simple() {
        let pitch_fingering_1 = PitchFingering {
            pitch: Pitch::A0,
            string_number: StringNumber::new(1).unwrap(),
            fret: 2,
        };

        let BeatFingeringCombo {
            fingering_combo,
            avg_non_zero_fret,
            non_zero_fret_span,
        } = BeatFingeringCombo::new(vec![&pitch_fingering_1]);

        assert_eq!(fingering_combo, vec![pitch_fingering_1]);
        assert_eq!(avg_non_zero_fret, Some(OrderedFloat(2.0)));
        assert_eq!(non_zero_fret_span, 0);
    }
    #[test]
    fn complex() {
        let pitch_fingering_1 = PitchFingering {
            pitch: Pitch::A0,
            string_number: StringNumber::new(1).unwrap(),
            fret: 2,
        };
        let pitch_fingering_2 = PitchFingering {
            pitch: Pitch::B1,
            string_number: StringNumber::new(2).unwrap(),
            fret: 5,
        };
        let pitch_fingering_3 = PitchFingering {
            pitch: Pitch::C2,
            string_number: StringNumber::new(3).unwrap(),
            fret: 0,
        };
        let pitch_fingering_4 = PitchFingering {
            pitch: Pitch::D3,
            string_number: StringNumber::new(4).unwrap(),
            fret: 1,
        };

        let BeatFingeringCombo {
            fingering_combo,
            avg_non_zero_fret,
            non_zero_fret_span,
        } = BeatFingeringCombo::new(vec![
            &pitch_fingering_1,
            &pitch_fingering_2,
            &pitch_fingering_3,
            &pitch_fingering_4,
        ]);

        assert_eq!(
            fingering_combo,
            vec![
                pitch_fingering_1,
                pitch_fingering_2,
                pitch_fingering_3,
                pitch_fingering_4
            ]
        );
        assert_eq!(avg_non_zero_fret, Some(OrderedFloat(8.0 / 3.0)));
        assert_eq!(non_zero_fret_span, 4);
    }
}

fn calc_avg_non_zero_fret(
    beat_fingering_candidate: &[&PitchFingering],
) -> Option<OrderedFloat<f32>> {
    let non_zero_fingerings = beat_fingering_candidate
        .iter()
        .filter(|fingering| fingering.fret != 0)
        .map(|fingering| fingering.fret as f64)
        .collect::<Mean>();

    match non_zero_fingerings.is_empty() {
        true => None,
        false => Some(OrderedFloat(non_zero_fingerings.mean() as f32)),
    }
}
#[cfg(test)]
mod test_calc_avg_non_zero_fret {
    use super::*;
    use crate::{pitch::Pitch, string_number::StringNumber};

    #[test]
    fn single_non_zero_fret() {
        let pitch_fingering_1 = PitchFingering {
            pitch: Pitch::A0,
            string_number: StringNumber::new(1).unwrap(),
            fret: 2,
        };

        assert_eq!(
            calc_avg_non_zero_fret(&[&pitch_fingering_1]),
            Some(OrderedFloat(2.0))
        );
    }
    #[test]
    fn single_zero_fret() {
        let pitch_fingering_1 = PitchFingering {
            pitch: Pitch::A0,
            string_number: StringNumber::new(1).unwrap(),
            fret: 0,
        };

        assert_eq!(calc_avg_non_zero_fret(&[&pitch_fingering_1]), None);
    }
    #[test]
    fn multiple_zero_frets() {
        let pitch_fingering_1 = PitchFingering {
            pitch: Pitch::A0,
            string_number: StringNumber::new(1).unwrap(),
            fret: 0,
        };
        let pitch_fingering_2 = PitchFingering {
            pitch: Pitch::B2,
            string_number: StringNumber::new(2).unwrap(),
            fret: 0,
        };

        assert_eq!(
            calc_avg_non_zero_fret(&[&pitch_fingering_1, &pitch_fingering_2]),
            None
        );
    }
    #[test]
    fn multiple_mixed_frets() {
        let pitch_fingering_1 = PitchFingering {
            pitch: Pitch::A0,
            string_number: StringNumber::new(1).unwrap(),
            fret: 2,
        };
        let pitch_fingering_2 = PitchFingering {
            pitch: Pitch::B1,
            string_number: StringNumber::new(2).unwrap(),
            fret: 5,
        };
        let pitch_fingering_3 = PitchFingering {
            pitch: Pitch::C2,
            string_number: StringNumber::new(3).unwrap(),
            fret: 0,
        };
        let pitch_fingering_4 = PitchFingering {
            pitch: Pitch::D3,
            string_number: StringNumber::new(4).unwrap(),
            fret: 1,
        };

        assert_eq!(
            calc_avg_non_zero_fret(&[
                &pitch_fingering_1,
                &pitch_fingering_2,
                &pitch_fingering_3,
                &pitch_fingering_4,
            ]),
            Some(OrderedFloat(8.0 / 3.0))
        );
    }
}

/// Calculates the difference between the maximum and minimum non-zero
/// fret numbers in a given vector of fingerings.
fn calc_fret_span(beat_fingering_candidate: Vec<&PitchFingering>) -> Option<u8> {
    let beat_fingering_option_fret_numbers = beat_fingering_candidate
        .iter()
        .filter(|fingering| fingering.fret != 0)
        .map(|fingering| fingering.fret);

    let min_non_zero_fret = match beat_fingering_option_fret_numbers.clone().min() {
        None => return None,
        Some(fret_num) => fret_num,
    };
    let max_non_zero_fret = match beat_fingering_option_fret_numbers.clone().max() {
        None => unreachable!("A maximum should exist if a minimum exists."),
        Some(fret_num) => fret_num,
    };

    Some(max_non_zero_fret - min_non_zero_fret)
}
#[cfg(test)]
mod test_calc_fret_span {
    use super::*;
    use crate::{pitch::Pitch, string_number::StringNumber};

    #[test]
    fn simple() {
        let fingering_1 = PitchFingering {
            pitch: Pitch::B6,
            string_number: StringNumber::new(2).unwrap(),
            fret: 3,
        };

        assert_eq!(calc_fret_span(vec![&fingering_1]).unwrap(), 0);
    }
    #[test]
    fn complex() {
        let fingering_1 = PitchFingering {
            pitch: Pitch::CSharpDFlat2,
            string_number: StringNumber::new(1).unwrap(),
            fret: 1,
        };
        let fingering_2 = PitchFingering {
            pitch: Pitch::F4,
            string_number: StringNumber::new(2).unwrap(),
            fret: 3,
        };
        let fingering_3 = PitchFingering {
            pitch: Pitch::A5,
            string_number: StringNumber::new(4).unwrap(),
            fret: 4,
        };
        let fingering_4 = PitchFingering {
            pitch: Pitch::DSharpEFlat6,
            string_number: StringNumber::new(11).unwrap(),
            fret: 0,
        };
        let beat_fingering_option: Vec<&PitchFingering> =
            vec![&fingering_1, &fingering_2, &fingering_3, &fingering_4];

        assert_eq!(calc_fret_span(beat_fingering_option).unwrap(), 3);
    }
    #[test]
    fn empty_input() {
        assert!(calc_fret_span(vec![]).is_none());
    }
}
