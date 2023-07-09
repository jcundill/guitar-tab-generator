use crate::{guitar::Fingering, Guitar, Pitch};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct InvalidInput {
    value: String,
    line_number: u16,
}

pub type PitchOptionsVec<T> = Vec<T>;
type BeatVec<T> = Vec<T>;

#[derive(Debug)]
pub struct Arrangement {}

impl Arrangement {
    pub fn new(guitar: Guitar, input_pitches: Vec<BeatVec<Pitch>>) -> Result<Self> {
        // TODO! add type alias for BeatVec, PitchVec, Candidates, ...
        // https://doc.rust-lang.org/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases

        let pitch_fingering_options: Vec<BeatVec<PitchOptionsVec<Fingering>>> =
            Arrangement::validate_fingerings(&guitar, &input_pitches)?;
        dbg!(&pitch_fingering_options);

        Ok(Arrangement {})
    }

    /// Generates fingerings for each pitch, and returns a result containing the fingerings or
    /// an error message if any impossible pitches (with no fingerings) are found.
    ///
    /// Arguments:
    ///
    /// * `guitar`: A reference to a `Guitar` object, which contains information about the guitar's
    /// string ranges.
    /// * `input_pitches`: A slice of vectors, where each vector represents a beat and contains a
    /// vector of pitches.
    ///
    /// Returns:
    ///
    /// The function `validate_fingerings` returns a `Result` containing either a
    /// `Vec<Vec<Vec<Fingering>>>` if the input pitches are valid, or an `Err` containing an error
    /// message if there are invalid pitches.
    fn validate_fingerings(
        guitar: &Guitar,
        input_pitches: &[BeatVec<Pitch>],
    ) -> Result<Vec<BeatVec<PitchOptionsVec<Fingering>>>> {
        let mut impossible_pitches: Vec<InvalidInput> = vec![];
        let fingerings: Vec<BeatVec<PitchOptionsVec<Fingering>>> = input_pitches[0..]
            .iter()
            .enumerate()
            .map(|(beat_index, beat_pitches)| {
                beat_pitches
                    .iter()
                    .map(|beat_pitch| {
                        let pitch_fingerings: PitchOptionsVec<Fingering> =
                            Guitar::generate_pitch_fingerings(&guitar.string_ranges, beat_pitch);
                        if pitch_fingerings.is_empty() {
                            impossible_pitches.push(InvalidInput {
                                value: format!("{:?}", beat_pitch),
                                line_number: (beat_index as u16) + 1,
                            })
                        }
                        pitch_fingerings
                    })
                    .collect()
            })
            .collect();

        if !impossible_pitches.is_empty() {
            let error_string = impossible_pitches
                .iter()
                .map(|invalid_input| {
                    format!(
                        "Pitch {} on line {} cannot be played on any strings of the configured guitar.",
                        invalid_input.value, invalid_input.line_number
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            return Err(anyhow!(error_string));
        }

        Ok(fingerings)
    }
}

#[cfg(test)]
mod test_validate_fingerings {
    use super::*;
    use crate::StringNumber;
    use std::collections::{BTreeMap, HashSet};

    fn generate_standard_guitar() -> Guitar {
        Guitar {
            tuning: BTreeMap::from([
                (StringNumber::new(1).unwrap(), Pitch::E4),
                (StringNumber::new(2).unwrap(), Pitch::B3),
                (StringNumber::new(3).unwrap(), Pitch::G3),
                (StringNumber::new(4).unwrap(), Pitch::D3),
                (StringNumber::new(5).unwrap(), Pitch::A2),
                (StringNumber::new(6).unwrap(), Pitch::E2),
            ]),
            num_frets: 12,
            range: HashSet::from([
                Pitch::E2,
                Pitch::F2,
                Pitch::FSharp2,
                Pitch::G2,
                Pitch::A2,
                Pitch::ASharp2,
                Pitch::B2,
                Pitch::C3,
                Pitch::D3,
                Pitch::DSharp3,
                Pitch::E3,
                Pitch::F3,
                Pitch::G3,
                Pitch::GSharp3,
                Pitch::A3,
                Pitch::ASharp3,
                Pitch::B3,
                Pitch::C4,
                Pitch::CSharp4,
                Pitch::D4,
                Pitch::E4,
                Pitch::F4,
                Pitch::FSharp4,
                Pitch::G4,
            ]),
            string_ranges: BTreeMap::from([
                (
                    StringNumber::new(1).unwrap(),
                    vec![Pitch::E4, Pitch::F4, Pitch::FSharp4, Pitch::G4],
                ),
                (
                    StringNumber::new(2).unwrap(),
                    vec![Pitch::B3, Pitch::C4, Pitch::CSharp4, Pitch::D4],
                ),
                (
                    StringNumber::new(3).unwrap(),
                    vec![Pitch::G3, Pitch::GSharp3, Pitch::A3, Pitch::ASharp3],
                ),
                (
                    StringNumber::new(4).unwrap(),
                    vec![Pitch::D3, Pitch::DSharp3, Pitch::E3, Pitch::F3],
                ),
                (
                    StringNumber::new(5).unwrap(),
                    vec![Pitch::A2, Pitch::ASharp2, Pitch::B2, Pitch::C3],
                ),
                (
                    StringNumber::new(6).unwrap(),
                    vec![Pitch::E2, Pitch::F2, Pitch::FSharp2, Pitch::G2],
                ),
            ]),
        }
    }

    #[test]
    fn valid_simple() {
        let guitar = generate_standard_guitar();
        let input_pitches = vec![vec![Pitch::G3]];
        let expected_fingerings = vec![vec![Guitar::generate_pitch_fingerings(
            &guitar.string_ranges,
            &Pitch::G3,
        )]];

        assert_eq!(
            Arrangement::validate_fingerings(&guitar, &input_pitches).unwrap(),
            expected_fingerings
        );
    }
    #[test]
    fn valid_complex() {
        let guitar = generate_standard_guitar();
        let input_pitches = vec![vec![Pitch::G3], vec![Pitch::B3], vec![Pitch::D4, Pitch::G4]];
        let expected_fingerings = vec![
            vec![Guitar::generate_pitch_fingerings(
                &guitar.string_ranges,
                &Pitch::G3,
            )],
            vec![Guitar::generate_pitch_fingerings(
                &guitar.string_ranges,
                &Pitch::B3,
            )],
            vec![
                Guitar::generate_pitch_fingerings(&guitar.string_ranges, &Pitch::D4),
                Guitar::generate_pitch_fingerings(&guitar.string_ranges, &Pitch::G4),
            ],
        ];

        assert_eq!(
            Arrangement::validate_fingerings(&guitar, &input_pitches).unwrap(),
            expected_fingerings
        );
    }
    #[test]
    fn invalid_simple() {
        let guitar = generate_standard_guitar();
        let input_pitches = vec![vec![Pitch::B9]];

        let error = Arrangement::validate_fingerings(&guitar, &input_pitches).unwrap_err();
        let error_string = format!("{error}");
        let expected_error_string =
            "Pitch B9 on line 1 cannot be played on any strings of the configured guitar.";
        assert_eq!(error_string, expected_error_string);
    }
    #[test]
    fn invalid_complex() {
        let guitar = generate_standard_guitar();
        let input_pitches = vec![
            vec![Pitch::A1],
            vec![Pitch::G3],
            vec![Pitch::B3],
            vec![Pitch::A1, Pitch::B1],
            vec![Pitch::G3, Pitch::D2],
            vec![Pitch::D4, Pitch::G4],
        ];

        let error = Arrangement::validate_fingerings(&guitar, &input_pitches).unwrap_err();
        let error_string = format!("{error}");
        let expected_error_string =
            "Pitch A1 on line 1 cannot be played on any strings of the configured guitar.\n\
            Pitch A1 on line 4 cannot be played on any strings of the configured guitar.\n\
            Pitch B1 on line 4 cannot be played on any strings of the configured guitar.\n\
            Pitch D2 on line 5 cannot be played on any strings of the configured guitar.";
        assert_eq!(error_string, expected_error_string);
    }
}
