use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, HashSet};
use strum::IntoEnumIterator;

pub mod pitch;
use pitch::Pitch;

pub mod string_number;
use string_number::StringNumber;

#[derive(Debug, PartialEq)]
pub struct Guitar {
    pub tuning: BTreeMap<StringNumber, Pitch>,
    pub num_frets: u8,
    pub range: HashSet<Pitch>,
    pub string_ranges: BTreeMap<StringNumber, Vec<Pitch>>,
}
impl Guitar {
    pub fn new(tuning: BTreeMap<StringNumber, Pitch>, num_frets: u8) -> Result<Self> {
        Guitar::check_fret_number(num_frets)?;

        let mut string_ranges: BTreeMap<StringNumber, Vec<Pitch>> = BTreeMap::new();
        for (string_number, string_open_pitch) in tuning.iter() {
            string_ranges.insert(
                string_number.clone().to_owned(),
                Guitar::create_string_range(string_open_pitch, num_frets)?,
            );
        }

        let range = string_ranges.clone().into_iter().fold(
            HashSet::new(),
            |mut all_pitches, string_pitches| {
                all_pitches.extend(string_pitches.1);
                all_pitches
            },
        );

        Ok(Guitar {
            tuning,
            num_frets,
            range,
            string_ranges,
        })
    }

    /// Check if the number of frets is within a maximum limit and returns an error if it exceeds the limit.
    fn check_fret_number(num_frets: u8) -> Result<()> {
        const MAX_NUM_FRETS: u8 = 30;
        if num_frets > MAX_NUM_FRETS {
            return Err(anyhow!(
                "Too many frets ({num_frets}). The maximum is {MAX_NUM_FRETS}."
            ));
        }

        Ok(())
    }

    /// Generates a vector of pitches representing the range of the string.
    ///
    /// Arguments:
    ///
    /// * `open_string_pitch`: The `open_string_pitch` parameter represents the pitch of the open
    /// string.
    /// * `num_frets`: The `num_frets` parameter represents the number of
    ///   subsequent number of half steps to include in the range.
    fn create_string_range(open_string_pitch: &Pitch, num_frets: u8) -> Result<Vec<Pitch>> {
        let lowest_pitch_index = Pitch::iter().position(|x| &x == open_string_pitch).unwrap();

        let all_pitches_vec: Vec<Pitch> = Pitch::iter().collect();
        let string_range_result =
            all_pitches_vec.get(lowest_pitch_index..=lowest_pitch_index + num_frets as usize);

        match string_range_result {
            Some(string_range_slice) => Ok(string_range_slice.to_vec()),
            None => {
                let highest_pitch = all_pitches_vec
                    .last()
                    .expect("The Pitch enum should not be empty.");
                let highest_pitch_fret = highest_pitch.index() - open_string_pitch.index();
                let err_msg = format!("Too many frets ({num_frets}) for string starting at pitch {open_string_pitch}. \
                The highest pitch is {highest_pitch}, which would only exist at fret number {highest_pitch_fret}.");

                Err(anyhow!(err_msg))
            }
        }
    }

    /// Takes a pitch as input and returns a fingering for that pitch on the guitar given its tuning.
    // TODO benchmark memoization
    fn generate_pitch_fingering(
        string_ranges: &BTreeMap<StringNumber, Vec<Pitch>>,
        pitch: &Pitch,
    ) -> Fingering {
        let mut fingering: BTreeMap<StringNumber, u8> = BTreeMap::new();
        for (string_number, string_range) in string_ranges.iter() {
            match string_range.iter().position(|x| x == pitch) {
                None => (),
                Some(fret_number) => {
                    fingering.insert(string_number.clone().to_owned(), fret_number as u8);
                }
            }
        }

        Fingering {
            pitch: *pitch,
            fingering,
        }
    }
}

#[cfg(test)]
mod test_guitar_new {
    use super::*;

    fn create_default_tuning() -> BTreeMap<StringNumber, Pitch> {
        BTreeMap::from([
            (StringNumber::new(1).unwrap(), Pitch::E4),
            (StringNumber::new(2).unwrap(), Pitch::B3),
            (StringNumber::new(3).unwrap(), Pitch::G3),
            (StringNumber::new(4).unwrap(), Pitch::D3),
            (StringNumber::new(5).unwrap(), Pitch::A2),
            (StringNumber::new(6).unwrap(), Pitch::E2),
        ])
    }

    #[test]
    fn valid_simple() -> Result<()> {
        let tuning = create_default_tuning();

        const NUM_FRETS: u8 = 3;

        let expected_guitar = Guitar {
            tuning: tuning.clone(),
            num_frets: NUM_FRETS,
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
        };

        assert_eq!(Guitar::new(tuning, NUM_FRETS)?, expected_guitar);

        Ok(())
    }
    #[test]
    fn valid_normal() -> Result<()> {
        let tuning = create_default_tuning();

        const NUM_FRETS: u8 = 18;

        let expected_guitar = Guitar {
            tuning: tuning.clone(),
            num_frets: NUM_FRETS,
            range: HashSet::from([
                Pitch::E2,
                Pitch::F2,
                Pitch::FSharp2,
                Pitch::G2,
                Pitch::GSharp2,
                Pitch::A2,
                Pitch::ASharp2,
                Pitch::B2,
                Pitch::C3,
                Pitch::CSharp3,
                Pitch::D3,
                Pitch::DSharp3,
                Pitch::E3,
                Pitch::F3,
                Pitch::FSharp3,
                Pitch::G3,
                Pitch::GSharp3,
                Pitch::A3,
                Pitch::ASharp3,
                Pitch::B3,
                Pitch::C4,
                Pitch::CSharp4,
                Pitch::D4,
                Pitch::DSharp4,
                Pitch::E4,
                Pitch::F4,
                Pitch::FSharp4,
                Pitch::G4,
                Pitch::GSharp4,
                Pitch::A4,
                Pitch::ASharp4,
                Pitch::B4,
                Pitch::C5,
                Pitch::CSharp5,
                Pitch::D5,
                Pitch::DSharp5,
                Pitch::E5,
                Pitch::F5,
                Pitch::FSharp5,
                Pitch::G5,
                Pitch::GSharp5,
                Pitch::A5,
                Pitch::ASharp5,
            ]),
            string_ranges: BTreeMap::from([
                (
                    StringNumber::new(1).unwrap(),
                    vec![
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::FSharp4,
                        Pitch::G4,
                        Pitch::GSharp4,
                        Pitch::A4,
                        Pitch::ASharp4,
                        Pitch::B4,
                        Pitch::C5,
                        Pitch::CSharp5,
                        Pitch::D5,
                        Pitch::DSharp5,
                        Pitch::E5,
                        Pitch::F5,
                        Pitch::FSharp5,
                        Pitch::G5,
                        Pitch::GSharp5,
                        Pitch::A5,
                        Pitch::ASharp5,
                    ],
                ),
                (
                    StringNumber::new(2).unwrap(),
                    vec![
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::CSharp4,
                        Pitch::D4,
                        Pitch::DSharp4,
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::FSharp4,
                        Pitch::G4,
                        Pitch::GSharp4,
                        Pitch::A4,
                        Pitch::ASharp4,
                        Pitch::B4,
                        Pitch::C5,
                        Pitch::CSharp5,
                        Pitch::D5,
                        Pitch::DSharp5,
                        Pitch::E5,
                        Pitch::F5,
                    ],
                ),
                (
                    StringNumber::new(3).unwrap(),
                    vec![
                        Pitch::G3,
                        Pitch::GSharp3,
                        Pitch::A3,
                        Pitch::ASharp3,
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::CSharp4,
                        Pitch::D4,
                        Pitch::DSharp4,
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::FSharp4,
                        Pitch::G4,
                        Pitch::GSharp4,
                        Pitch::A4,
                        Pitch::ASharp4,
                        Pitch::B4,
                        Pitch::C5,
                        Pitch::CSharp5,
                    ],
                ),
                (
                    StringNumber::new(4).unwrap(),
                    vec![
                        Pitch::D3,
                        Pitch::DSharp3,
                        Pitch::E3,
                        Pitch::F3,
                        Pitch::FSharp3,
                        Pitch::G3,
                        Pitch::GSharp3,
                        Pitch::A3,
                        Pitch::ASharp3,
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::CSharp4,
                        Pitch::D4,
                        Pitch::DSharp4,
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::FSharp4,
                        Pitch::G4,
                        Pitch::GSharp4,
                    ],
                ),
                (
                    StringNumber::new(5).unwrap(),
                    vec![
                        Pitch::A2,
                        Pitch::ASharp2,
                        Pitch::B2,
                        Pitch::C3,
                        Pitch::CSharp3,
                        Pitch::D3,
                        Pitch::DSharp3,
                        Pitch::E3,
                        Pitch::F3,
                        Pitch::FSharp3,
                        Pitch::G3,
                        Pitch::GSharp3,
                        Pitch::A3,
                        Pitch::ASharp3,
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::CSharp4,
                        Pitch::D4,
                        Pitch::DSharp4,
                    ],
                ),
                (
                    StringNumber::new(6).unwrap(),
                    vec![
                        Pitch::E2,
                        Pitch::F2,
                        Pitch::FSharp2,
                        Pitch::G2,
                        Pitch::GSharp2,
                        Pitch::A2,
                        Pitch::ASharp2,
                        Pitch::B2,
                        Pitch::C3,
                        Pitch::CSharp3,
                        Pitch::D3,
                        Pitch::DSharp3,
                        Pitch::E3,
                        Pitch::F3,
                        Pitch::FSharp3,
                        Pitch::G3,
                        Pitch::GSharp3,
                        Pitch::A3,
                        Pitch::ASharp3,
                    ],
                ),
            ]),
        };

        assert_eq!(Guitar::new(tuning, NUM_FRETS)?, expected_guitar);

        Ok(())
    }
    #[test]
    fn invalid_num_frets() {
        assert!(Guitar::new(create_default_tuning(), 35).is_err());
    }
}
#[cfg(test)]
mod test_check_fret_number {
    use super::Guitar;
    #[test]
    fn valid() {
        assert!(Guitar::check_fret_number(0).is_ok());
        assert!(Guitar::check_fret_number(2).is_ok());
        assert!(Guitar::check_fret_number(7).is_ok());
        assert!(Guitar::check_fret_number(20).is_ok());
    }
    #[test]
    fn invalid() {
        assert!(Guitar::check_fret_number(0).is_ok());
        assert!(Guitar::check_fret_number(12).is_ok());
        assert!(Guitar::check_fret_number(18).is_ok());
        assert!(Guitar::check_fret_number(27).is_ok());
        assert!(Guitar::check_fret_number(31).is_err());
        assert!(Guitar::check_fret_number(100).is_err());
    }
}
#[cfg(test)]
mod test_create_string_range {
    use super::*;
    #[test]
    fn valid() -> Result<()> {
        assert_eq!(Guitar::create_string_range(&Pitch::E2, 0)?, vec![Pitch::E2]);
        assert_eq!(
            Guitar::create_string_range(&Pitch::E2, 3)?,
            vec![Pitch::E2, Pitch::F2, Pitch::FSharp2, Pitch::G2]
        );
        assert_eq!(
            Guitar::create_string_range(&Pitch::E2, 12)?,
            vec![
                Pitch::E2,
                Pitch::F2,
                Pitch::FSharp2,
                Pitch::G2,
                Pitch::GSharp2,
                Pitch::A2,
                Pitch::ASharp2,
                Pitch::B2,
                Pitch::C3,
                Pitch::CSharp3,
                Pitch::D3,
                Pitch::DSharp3,
                Pitch::E3
            ]
        );
        Ok(())
    }
    #[test]
    fn invalid() {
        let error = Guitar::create_string_range(&Pitch::G9, 5).unwrap_err();
        let error_string = format!("{error}");
        let expected_error_string = "Too many frets (5) for string starting at pitch G9. The highest pitch is B9, which would only exist at fret number 4.";
        assert_eq!(error_string, expected_error_string);

        let error = Guitar::create_string_range(&Pitch::E2, 100).unwrap_err();
        let error_string = format!("{error}");
        let expected_error_string = "Too many frets (100) for string starting at pitch E2. The highest pitch is B9, which would only exist at fret number 91.";
        assert_eq!(error_string, expected_error_string);
    }
}
#[cfg(test)]
mod test_generate_pitch_fingering {
    use super::*;
    #[test]
    fn valid_normal() -> Result<()> {
        const NUM_FRETS: u8 = 12;
        let string_ranges = BTreeMap::from([
            (
                StringNumber::new(1).unwrap(),
                Guitar::create_string_range(&Pitch::E4, NUM_FRETS)?,
            ),
            (
                StringNumber::new(2).unwrap(),
                Guitar::create_string_range(&Pitch::B3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(3).unwrap(),
                Guitar::create_string_range(&Pitch::G3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(4).unwrap(),
                Guitar::create_string_range(&Pitch::D3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(5).unwrap(),
                Guitar::create_string_range(&Pitch::A2, NUM_FRETS)?,
            ),
            (
                StringNumber::new(6).unwrap(),
                Guitar::create_string_range(&Pitch::E2, NUM_FRETS)?,
            ),
        ]);

        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::E2),
            Fingering {
                pitch: Pitch::E2,
                fingering: BTreeMap::from([(StringNumber::new(6).unwrap(), 0)])
            }
        );
        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::D3),
            Fingering {
                pitch: Pitch::D3,
                fingering: BTreeMap::from([
                    (StringNumber::new(4).unwrap(), 0),
                    (StringNumber::new(5).unwrap(), 5),
                    (StringNumber::new(6).unwrap(), 10)
                ])
            }
        );
        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::CSharp4),
            Fingering {
                pitch: Pitch::CSharp4,
                fingering: BTreeMap::from([
                    (StringNumber::new(2).unwrap(), 2),
                    (StringNumber::new(3).unwrap(), 6),
                    (StringNumber::new(4).unwrap(), 11)
                ])
            }
        );
        Ok(())
    }

    #[test]
    fn valid_simple() -> Result<()> {
        const NUM_FRETS: u8 = 12;
        let string_ranges = BTreeMap::from([
            (
                StringNumber::new(1).unwrap(),
                Guitar::create_string_range(&Pitch::G4, NUM_FRETS)?,
            ),
            (
                StringNumber::new(2).unwrap(),
                Guitar::create_string_range(&Pitch::DSharp4, NUM_FRETS)?,
            ),
        ]);

        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::DSharp4),
            Fingering {
                pitch: Pitch::DSharp4,
                fingering: BTreeMap::from([(StringNumber::new(2).unwrap(), 0)])
            }
        );
        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::ASharp4),
            Fingering {
                pitch: Pitch::ASharp4,
                fingering: BTreeMap::from([
                    (StringNumber::new(1).unwrap(), 3),
                    (StringNumber::new(2).unwrap(), 7)
                ])
            }
        );
        Ok(())
    }

    #[test]
    fn valid_few_frets() -> Result<()> {
        const NUM_FRETS: u8 = 2;
        let string_ranges = BTreeMap::from([
            (
                StringNumber::new(1).unwrap(),
                Guitar::create_string_range(&Pitch::E4, NUM_FRETS)?,
            ),
            (
                StringNumber::new(2).unwrap(),
                Guitar::create_string_range(&Pitch::B3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(3).unwrap(),
                Guitar::create_string_range(&Pitch::G3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(4).unwrap(),
                Guitar::create_string_range(&Pitch::D3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(5).unwrap(),
                Guitar::create_string_range(&Pitch::A2, NUM_FRETS)?,
            ),
            (
                StringNumber::new(6).unwrap(),
                Guitar::create_string_range(&Pitch::E2, NUM_FRETS)?,
            ),
        ]);

        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::E3),
            Fingering {
                pitch: Pitch::E3,
                fingering: BTreeMap::from([(StringNumber::new(4).unwrap(), 2)])
            }
        );
        Ok(())
    }

    #[test]
    fn valid_impossible_pitch() -> Result<()> {
        const NUM_FRETS: u8 = 12;
        let string_ranges = BTreeMap::from([
            (
                StringNumber::new(1).unwrap(),
                Guitar::create_string_range(&Pitch::E4, NUM_FRETS)?,
            ),
            (
                StringNumber::new(2).unwrap(),
                Guitar::create_string_range(&Pitch::B3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(3).unwrap(),
                Guitar::create_string_range(&Pitch::G3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(4).unwrap(),
                Guitar::create_string_range(&Pitch::D3, NUM_FRETS)?,
            ),
            (
                StringNumber::new(5).unwrap(),
                Guitar::create_string_range(&Pitch::A2, NUM_FRETS)?,
            ),
            (
                StringNumber::new(6).unwrap(),
                Guitar::create_string_range(&Pitch::E2, NUM_FRETS)?,
            ),
        ]);

        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::D2),
            Fingering {
                pitch: Pitch::D2,
                fingering: BTreeMap::from([])
            }
        );
        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::F5),
            Fingering {
                pitch: Pitch::F5,
                fingering: BTreeMap::from([])
            }
        );
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fingering {
    pitch: Pitch,
    fingering: BTreeMap<StringNumber, u8>,
}

#[derive(Debug)]
pub struct InvalidInput {
    value: String,
    line_number: u8,
}

#[derive(Debug)]
pub struct Arrangement {}

impl Arrangement {
    pub fn new(guitar: Guitar, input_pitches: Vec<Vec<Pitch>>) -> Result<Self> {
        let fingerings: Vec<Vec<Fingering>> = input_pitches[0..]
            .iter()
            .map(|beat_pitches| {
                beat_pitches
                    .iter()
                    .map(|beat_pitch| {
                        Guitar::generate_pitch_fingering(&guitar.string_ranges, beat_pitch)
                    })
                    .collect()
            })
            .collect();

        Arrangement::check_for_invalid_pitches(fingerings)?;

        Ok(Arrangement {})
    }

    fn check_for_invalid_pitches(fingerings: Vec<Vec<Fingering>>) -> Result<()> {
        let impossible_pitches: Vec<Vec<Pitch>> = fingerings
            .iter()
            .map(|beat_fingerings| {
                {
                    beat_fingerings
                        .iter()
                        .filter(|beat_fingering| beat_fingering.fingering.is_empty())
                        .map(|beat_fingering| beat_fingering.pitch)
                        .collect()
                }
            })
            .collect();
        let invalid_inputs: Vec<InvalidInput> = impossible_pitches
            .iter()
            .filter(|beat_impossible_pitches| !beat_impossible_pitches.is_empty())
            .flat_map(|beat_impossible_pitches| {
                let line_number = impossible_pitches
                    .iter()
                    .position(|x| x == beat_impossible_pitches)
                    .unwrap() as u8;

                beat_impossible_pitches
                    .iter()
                    .map(move |beat_impossible_pitch| InvalidInput {
                        value: format!("{:?}", beat_impossible_pitch),
                        line_number,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        if !invalid_inputs.is_empty() {
            let error_string = invalid_inputs
                .iter()
                .map(|invalid_input| {
                    format!(
                        "Invalid pitch {} on line {}.",
                        invalid_input.value, invalid_input.line_number
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            return Err(anyhow!(error_string));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_check_for_invalid_pitches {
    use super::*;
    #[test]
    fn valid_simple() {
        let fingerings = vec![vec![Fingering {
            pitch: Pitch::G3,
            fingering: BTreeMap::from([
                (StringNumber::new(3).unwrap(), 0),
                (StringNumber::new(4).unwrap(), 5),
                (StringNumber::new(5).unwrap(), 10),
            ]),
        }]];

        assert!(Arrangement::check_for_invalid_pitches(fingerings).is_ok());
    }
    #[test]
    fn valid_complex() {
        let fingerings = vec![
            vec![Fingering {
                pitch: Pitch::G3,
                fingering: BTreeMap::from([
                    (StringNumber::new(3).unwrap(), 0),
                    (StringNumber::new(4).unwrap(), 5),
                    (StringNumber::new(5).unwrap(), 10),
                    (StringNumber::new(6).unwrap(), 15),
                ]),
            }],
            vec![Fingering {
                pitch: Pitch::B3,
                fingering: BTreeMap::from([
                    (StringNumber::new(2).unwrap(), 0),
                    (StringNumber::new(3).unwrap(), 4),
                    (StringNumber::new(4).unwrap(), 9),
                    (StringNumber::new(5).unwrap(), 14),
                ]),
            }],
            vec![
                Fingering {
                    pitch: Pitch::D4,
                    fingering: BTreeMap::from([
                        (StringNumber::new(2).unwrap(), 3),
                        (StringNumber::new(3).unwrap(), 7),
                        (StringNumber::new(4).unwrap(), 12),
                        (StringNumber::new(5).unwrap(), 17),
                    ]),
                },
                Fingering {
                    pitch: Pitch::G4,
                    fingering: BTreeMap::from([
                        (StringNumber::new(1).unwrap(), 3),
                        (StringNumber::new(2).unwrap(), 8),
                        (StringNumber::new(3).unwrap(), 12),
                        (StringNumber::new(4).unwrap(), 17),
                    ]),
                },
            ],
        ];

        assert!(Arrangement::check_for_invalid_pitches(fingerings).is_ok());
    }
    #[test]
    fn invalid_simple() {
        let fingerings = vec![vec![
            Fingering {
                pitch: Pitch::G3,
                fingering: BTreeMap::from([
                    (StringNumber::new(3).unwrap(), 0),
                    (StringNumber::new(4).unwrap(), 5),
                    (StringNumber::new(5).unwrap(), 10),
                    (StringNumber::new(6).unwrap(), 15),
                ]),
            },
            Fingering {
                pitch: Pitch::CSharp6,
                fingering: BTreeMap::from([]),
            },
        ]];

        let expected_error_string = "Invalid pitch CSharp6 on line 0.";
        let error = Arrangement::check_for_invalid_pitches(fingerings).unwrap_err();
        let error_string = format!("{error}");

        assert_eq!(error_string, expected_error_string);
    }
    #[test]
    fn invalid_complex() {
        let fingerings = vec![
            vec![Fingering {
                pitch: Pitch::A1,
                fingering: BTreeMap::from([]),
            }],
            vec![Fingering {
                pitch: Pitch::G3,
                fingering: BTreeMap::from([
                    (StringNumber::new(3).unwrap(), 0),
                    (StringNumber::new(4).unwrap(), 5),
                    (StringNumber::new(5).unwrap(), 10),
                    (StringNumber::new(6).unwrap(), 15),
                ]),
            }],
            vec![Fingering {
                pitch: Pitch::B3,
                fingering: BTreeMap::from([
                    (StringNumber::new(2).unwrap(), 0),
                    (StringNumber::new(3).unwrap(), 4),
                    (StringNumber::new(4).unwrap(), 9),
                    (StringNumber::new(5).unwrap(), 14),
                ]),
            }],
            vec![
                Fingering {
                    pitch: Pitch::A1,
                    fingering: BTreeMap::from([]),
                },
                Fingering {
                    pitch: Pitch::B1,
                    fingering: BTreeMap::from([]),
                },
            ],
            vec![
                Fingering {
                    pitch: Pitch::G3,
                    fingering: BTreeMap::from([
                        (StringNumber::new(3).unwrap(), 0),
                        (StringNumber::new(4).unwrap(), 5),
                        (StringNumber::new(5).unwrap(), 10),
                        (StringNumber::new(6).unwrap(), 15),
                    ]),
                },
                Fingering {
                    pitch: Pitch::D2,
                    fingering: BTreeMap::from([]),
                },
            ],
            vec![
                Fingering {
                    pitch: Pitch::D4,
                    fingering: BTreeMap::from([
                        (StringNumber::new(2).unwrap(), 3),
                        (StringNumber::new(3).unwrap(), 7),
                        (StringNumber::new(4).unwrap(), 12),
                        (StringNumber::new(5).unwrap(), 17),
                    ]),
                },
                Fingering {
                    pitch: Pitch::G4,
                    fingering: BTreeMap::from([
                        (StringNumber::new(1).unwrap(), 3),
                        (StringNumber::new(2).unwrap(), 8),
                        (StringNumber::new(3).unwrap(), 12),
                        (StringNumber::new(4).unwrap(), 17),
                    ]),
                },
            ],
        ];

        let expected_error_string = "Invalid pitch A1 on line 0.\nInvalid pitch A1 on line 3.\nInvalid pitch B1 on line 3.\nInvalid pitch D2 on line 4.";
        let error = Arrangement::check_for_invalid_pitches(fingerings).unwrap_err();
        let error_string = format!("{error}");

        assert_eq!(error_string, expected_error_string);
    }
}
