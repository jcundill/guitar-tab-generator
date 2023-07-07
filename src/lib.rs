use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
    fmt,
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, FromRepr};

#[derive(
    Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, EnumIter, FromRepr, EnumString,
)]
pub enum Pitch {
    A1,
    A1Sharp,
    B1,
    C2,
    C2Sharp,
    D2,
    D2Sharp,
    E2,
    F2,
    F2Sharp,
    G2,
    G2Sharp,
    A2,
    A2Sharp,
    B2,
    C3,
    C3Sharp,
    D3,
    D3Sharp,
    E3,
    F3,
    F3Sharp,
    G3,
    G3Sharp,
    A3,
    A3Sharp,
    B3,
    C4,
    C4Sharp,
    D4,
    D4Sharp,
    E4,
    F4,
    F4Sharp,
    G4,
    G4Sharp,
    A4,
    A4Sharp,
    B4,
    C5,
    C5Sharp,
    D5,
    D5Sharp,
    E5,
    F5,
    F5Sharp,
    G5,
    G5Sharp,
    A5,
    A5Sharp,
    B5,
    C6,
    C6Sharp,
    D6,
    D6Sharp,
    E6,
    F6,
    F6Sharp,
    G6,
    G6Sharp,
}

impl Pitch {
    fn index(&self) -> usize {
        *self as usize
    }
}
impl std::ops::Sub for Pitch {
    type Output = isize;
    fn sub(self, pitch_2: Self) -> Self::Output {
        let pitch_1_index = self.index() as isize;
        let pitch_2_index = pitch_2.index() as isize;
        pitch_1_index - pitch_2_index
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringNumber(usize);
impl StringNumber {
    pub fn new(string_number: usize) -> Result<Self, Box<dyn Error>> {
        const MAX_NUM_STRINGS: usize = 12;
        if string_number > MAX_NUM_STRINGS {
            return Err(format!(
                "The string number ({}) is too high. The maximum is {}.",
                string_number, MAX_NUM_STRINGS
            )
            .into());
        }
        Ok(StringNumber(string_number))
    }
}
impl fmt::Debug for StringNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.0)
        let string_number = self.0;
        let string_pitch_letter = match string_number {
            1 => "1_e".to_owned(),
            2 => "2_B".to_owned(),
            3 => "3_G".to_owned(),
            4 => "4_D".to_owned(),
            5 => "5_A".to_owned(),
            6 => "6_E".to_owned(),
            string_number => string_number.to_string(),
        };
        write!(f, "{}", string_pitch_letter)
    }
}

#[derive(Debug, PartialEq)]
pub struct Guitar {
    pub tuning: BTreeMap<StringNumber, Pitch>,
    pub num_frets: usize,
    pub range: HashSet<Pitch>,
    pub string_ranges: BTreeMap<StringNumber, Vec<Pitch>>,
}
impl Guitar {
    pub fn new(
        tuning: BTreeMap<StringNumber, Pitch>,
        num_frets: usize,
    ) -> Result<Self, Box<dyn Error>> {
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
    fn check_fret_number(num_frets: usize) -> Result<(), Box<dyn Error>> {
        const MAX_NUM_FRETS: usize = 30;
        if num_frets > MAX_NUM_FRETS {
            return Err(format!(
                "Too many frets ({}). The maximum is {}.",
                num_frets, MAX_NUM_FRETS
            )
            .into());
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
    fn create_string_range(
        open_string_pitch: &Pitch,
        num_frets: usize,
    ) -> Result<Vec<Pitch>, Box<dyn Error>> {
        let lowest_pitch_index = Pitch::iter().position(|x| &x == open_string_pitch).unwrap();

        let all_pitches_vec = Pitch::iter().collect::<Vec<_>>();
        let string_range_result =
            all_pitches_vec.get(lowest_pitch_index..=lowest_pitch_index + num_frets);

        match string_range_result {
            Some(string_range_slice) => Ok(string_range_slice.to_vec()),
            None => {
                let highest_pitch = all_pitches_vec
                    .last()
                    .expect("The Pitch enum should not be empty.");
                let highest_pitch_fret = *highest_pitch - *open_string_pitch;
                let err_msg = format!("Too many frets ({num_frets}) for string starting at pitch {open_string_pitch:?}. \
                The highest pitch is {highest_pitch:?}, which would only exist at fret number {highest_pitch_fret}.");

                Err(err_msg.into())
            }
        }
    }

    /// Takes a pitch as input and returns a fingering for that pitch on the guitar given its tuning.
    // TODO benchmark memoization
    fn generate_pitch_fingering(
        string_ranges: &BTreeMap<StringNumber, Vec<Pitch>>,
        pitch: &Pitch,
    ) -> Fingering {
        let mut fingering: BTreeMap<StringNumber, usize> = BTreeMap::new();
        for (string_number, string_range) in string_ranges.iter() {
            match string_range.iter().position(|x| x == pitch) {
                None => (),
                Some(fret_number) => {
                    fingering.insert(string_number.clone().to_owned(), fret_number);
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
    fn few_strings_and_few_frets() -> Result<(), Box<dyn Error>> {
        let tuning = create_default_tuning();

        const NUM_FRETS: usize = 3;

        // // Sort range hashset
        // let g = Guitar::new(tuning.clone(), NUM_FRETS).unwrap();
        // let mut total_range = g.range.iter().collect::<Vec<_>>();
        // total_range.sort();
        // dbg!(total_range);
        // panic!();

        let expected_guitar = Guitar {
            tuning: tuning.clone(),
            num_frets: NUM_FRETS,
            range: HashSet::from([
                Pitch::E2,
                Pitch::F2,
                Pitch::F2Sharp,
                Pitch::G2,
                Pitch::A2,
                Pitch::A2Sharp,
                Pitch::B2,
                Pitch::C3,
                Pitch::D3,
                Pitch::D3Sharp,
                Pitch::E3,
                Pitch::F3,
                Pitch::G3,
                Pitch::G3Sharp,
                Pitch::A3,
                Pitch::A3Sharp,
                Pitch::B3,
                Pitch::C4,
                Pitch::C4Sharp,
                Pitch::D4,
                Pitch::E4,
                Pitch::F4,
                Pitch::F4Sharp,
                Pitch::G4,
            ]),
            string_ranges: BTreeMap::from([
                (
                    StringNumber::new(1).unwrap(),
                    vec![Pitch::E4, Pitch::F4, Pitch::F4Sharp, Pitch::G4],
                ),
                (
                    StringNumber::new(2).unwrap(),
                    vec![Pitch::B3, Pitch::C4, Pitch::C4Sharp, Pitch::D4],
                ),
                (
                    StringNumber::new(3).unwrap(),
                    vec![Pitch::G3, Pitch::G3Sharp, Pitch::A3, Pitch::A3Sharp],
                ),
                (
                    StringNumber::new(4).unwrap(),
                    vec![Pitch::D3, Pitch::D3Sharp, Pitch::E3, Pitch::F3],
                ),
                (
                    StringNumber::new(5).unwrap(),
                    vec![Pitch::A2, Pitch::A2Sharp, Pitch::B2, Pitch::C3],
                ),
                (
                    StringNumber::new(6).unwrap(),
                    vec![Pitch::E2, Pitch::F2, Pitch::F2Sharp, Pitch::G2],
                ),
            ]),
        };

        assert_eq!(Guitar::new(tuning, NUM_FRETS)?, expected_guitar);

        Ok(())
    }
    #[test]
    fn normal() -> Result<(), Box<dyn Error>> {
        let tuning = create_default_tuning();

        const NUM_FRETS: usize = 18;

        let expected_guitar = Guitar {
            tuning: tuning.clone(),
            num_frets: NUM_FRETS,
            range: HashSet::from([
                Pitch::E2,
                Pitch::F2,
                Pitch::F2Sharp,
                Pitch::G2,
                Pitch::G2Sharp,
                Pitch::A2,
                Pitch::A2Sharp,
                Pitch::B2,
                Pitch::C3,
                Pitch::C3Sharp,
                Pitch::D3,
                Pitch::D3Sharp,
                Pitch::E3,
                Pitch::F3,
                Pitch::F3Sharp,
                Pitch::G3,
                Pitch::G3Sharp,
                Pitch::A3,
                Pitch::A3Sharp,
                Pitch::B3,
                Pitch::C4,
                Pitch::C4Sharp,
                Pitch::D4,
                Pitch::D4Sharp,
                Pitch::E4,
                Pitch::F4,
                Pitch::F4Sharp,
                Pitch::G4,
                Pitch::G4Sharp,
                Pitch::A4,
                Pitch::A4Sharp,
                Pitch::B4,
                Pitch::C5,
                Pitch::C5Sharp,
                Pitch::D5,
                Pitch::D5Sharp,
                Pitch::E5,
                Pitch::F5,
                Pitch::F5Sharp,
                Pitch::G5,
                Pitch::G5Sharp,
                Pitch::A5,
                Pitch::A5Sharp,
            ]),
            string_ranges: BTreeMap::from([
                (
                    StringNumber::new(1).unwrap(),
                    vec![
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::F4Sharp,
                        Pitch::G4,
                        Pitch::G4Sharp,
                        Pitch::A4,
                        Pitch::A4Sharp,
                        Pitch::B4,
                        Pitch::C5,
                        Pitch::C5Sharp,
                        Pitch::D5,
                        Pitch::D5Sharp,
                        Pitch::E5,
                        Pitch::F5,
                        Pitch::F5Sharp,
                        Pitch::G5,
                        Pitch::G5Sharp,
                        Pitch::A5,
                        Pitch::A5Sharp,
                    ],
                ),
                (
                    StringNumber::new(2).unwrap(),
                    vec![
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::C4Sharp,
                        Pitch::D4,
                        Pitch::D4Sharp,
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::F4Sharp,
                        Pitch::G4,
                        Pitch::G4Sharp,
                        Pitch::A4,
                        Pitch::A4Sharp,
                        Pitch::B4,
                        Pitch::C5,
                        Pitch::C5Sharp,
                        Pitch::D5,
                        Pitch::D5Sharp,
                        Pitch::E5,
                        Pitch::F5,
                    ],
                ),
                (
                    StringNumber::new(3).unwrap(),
                    vec![
                        Pitch::G3,
                        Pitch::G3Sharp,
                        Pitch::A3,
                        Pitch::A3Sharp,
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::C4Sharp,
                        Pitch::D4,
                        Pitch::D4Sharp,
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::F4Sharp,
                        Pitch::G4,
                        Pitch::G4Sharp,
                        Pitch::A4,
                        Pitch::A4Sharp,
                        Pitch::B4,
                        Pitch::C5,
                        Pitch::C5Sharp,
                    ],
                ),
                (
                    StringNumber::new(4).unwrap(),
                    vec![
                        Pitch::D3,
                        Pitch::D3Sharp,
                        Pitch::E3,
                        Pitch::F3,
                        Pitch::F3Sharp,
                        Pitch::G3,
                        Pitch::G3Sharp,
                        Pitch::A3,
                        Pitch::A3Sharp,
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::C4Sharp,
                        Pitch::D4,
                        Pitch::D4Sharp,
                        Pitch::E4,
                        Pitch::F4,
                        Pitch::F4Sharp,
                        Pitch::G4,
                        Pitch::G4Sharp,
                    ],
                ),
                (
                    StringNumber::new(5).unwrap(),
                    vec![
                        Pitch::A2,
                        Pitch::A2Sharp,
                        Pitch::B2,
                        Pitch::C3,
                        Pitch::C3Sharp,
                        Pitch::D3,
                        Pitch::D3Sharp,
                        Pitch::E3,
                        Pitch::F3,
                        Pitch::F3Sharp,
                        Pitch::G3,
                        Pitch::G3Sharp,
                        Pitch::A3,
                        Pitch::A3Sharp,
                        Pitch::B3,
                        Pitch::C4,
                        Pitch::C4Sharp,
                        Pitch::D4,
                        Pitch::D4Sharp,
                    ],
                ),
                (
                    StringNumber::new(6).unwrap(),
                    vec![
                        Pitch::E2,
                        Pitch::F2,
                        Pitch::F2Sharp,
                        Pitch::G2,
                        Pitch::G2Sharp,
                        Pitch::A2,
                        Pitch::A2Sharp,
                        Pitch::B2,
                        Pitch::C3,
                        Pitch::C3Sharp,
                        Pitch::D3,
                        Pitch::D3Sharp,
                        Pitch::E3,
                        Pitch::F3,
                        Pitch::F3Sharp,
                        Pitch::G3,
                        Pitch::G3Sharp,
                        Pitch::A3,
                        Pitch::A3Sharp,
                    ],
                ),
            ]),
        };

        assert_eq!(Guitar::new(tuning, NUM_FRETS)?, expected_guitar);

        Ok(())
    }
    #[test]
    fn invalid_num_frets() {
        assert!(Guitar::new(create_default_tuning(), 30).is_err());
    }
}
#[cfg(test)]
mod test_check_fret_number {
    use super::Guitar;
    #[test]
    fn valid_frets() {
        assert!(Guitar::check_fret_number(0).is_ok());
        assert!(Guitar::check_fret_number(2).is_ok());
        assert!(Guitar::check_fret_number(7).is_ok());
        assert!(Guitar::check_fret_number(20).is_ok());
    }
    #[test]
    fn invalid_frets() {
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
    fn correct_string_range() -> Result<(), Box<dyn Error>> {
        assert_eq!(Guitar::create_string_range(&Pitch::E2, 0)?, vec![Pitch::E2]);
        assert_eq!(
            Guitar::create_string_range(&Pitch::E2, 3)?,
            vec![Pitch::E2, Pitch::F2, Pitch::F2Sharp, Pitch::G2]
        );
        assert_eq!(
            Guitar::create_string_range(&Pitch::E2, 12)?,
            vec![
                Pitch::E2,
                Pitch::F2,
                Pitch::F2Sharp,
                Pitch::G2,
                Pitch::G2Sharp,
                Pitch::A2,
                Pitch::A2Sharp,
                Pitch::B2,
                Pitch::C3,
                Pitch::C3Sharp,
                Pitch::D3,
                Pitch::D3Sharp,
                Pitch::E3
            ]
        );
        Ok(())
    }
}
#[cfg(test)]
mod test_generate_pitch_fingering {
    use super::*;
    #[test]
    fn normal() -> Result<(), Box<dyn Error>> {
        const NUM_FRETS: usize = 12;
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
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::C4Sharp),
            Fingering {
                pitch: Pitch::C4Sharp,
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
    fn few_strings() -> Result<(), Box<dyn Error>> {
        const NUM_FRETS: usize = 12;
        let string_ranges = BTreeMap::from([
            (
                StringNumber::new(1).unwrap(),
                Guitar::create_string_range(&Pitch::G4, NUM_FRETS)?,
            ),
            (
                StringNumber::new(2).unwrap(),
                Guitar::create_string_range(&Pitch::D4Sharp, NUM_FRETS)?,
            ),
        ]);

        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::D4Sharp),
            Fingering {
                pitch: Pitch::D4Sharp,
                fingering: BTreeMap::from([(StringNumber::new(2).unwrap(), 0)])
            }
        );
        assert_eq!(
            Guitar::generate_pitch_fingering(&string_ranges, &Pitch::A4Sharp),
            Fingering {
                pitch: Pitch::A4Sharp,
                fingering: BTreeMap::from([
                    (StringNumber::new(1).unwrap(), 3),
                    (StringNumber::new(2).unwrap(), 7)
                ])
            }
        );
        Ok(())
    }

    #[test]
    fn few_frets() -> Result<(), Box<dyn Error>> {
        const NUM_FRETS: usize = 2;
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
    fn impossible_pitch() -> Result<(), Box<dyn Error>> {
        const NUM_FRETS: usize = 12;
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

#[derive(Debug, PartialEq)]
pub struct Fingering {
    pitch: Pitch,
    fingering: BTreeMap<StringNumber, usize>,
}

#[derive(Debug)]
pub struct InvalidInput {
    value: String,
    line_number: usize,
}
pub struct Arrangement {}

impl Arrangement {
    pub fn new(guitar: Guitar, input_pitches: Vec<Vec<Pitch>>) -> Result<Self, Box<dyn Error>> {
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

    fn check_for_invalid_pitches(fingerings: Vec<Vec<Fingering>>) -> Result<(), Box<dyn Error>> {
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
                    .unwrap();

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

            return Err(error_string.into());
        }
        Ok(())
    }
}
