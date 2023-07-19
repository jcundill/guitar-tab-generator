use crate::{
    arrangement::{BeatVec, Line},
    pitch::Pitch,
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::RegexBuilder;
use std::result::Result::Ok;
use std::{collections::HashSet, str::FromStr};

pub fn parse_pitches(input: String) -> Result<Vec<Line<BeatVec<Pitch>>>> {
    let line_parse_results: Vec<Result<Line<BeatVec<Pitch>>, anyhow::Error>> = input
        .lines()
        .enumerate()
        .map(|(input_index, input_line)| parse_line(input_index, input_line))
        .collect_vec();

    let unparsable_lines_error_msg = line_parse_results
        .iter()
        .filter_map(|line| match line {
            Err(err) => Some(format!("{}", err)),
            Ok(_) => None,
        })
        .collect::<Vec<String>>()
        .join("\n");
    if !unparsable_lines_error_msg.is_empty() {
        return Err(anyhow!(unparsable_lines_error_msg));
    }

    let parsed_lines: Vec<Line<BeatVec<Pitch>>> = line_parse_results
        .into_iter()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    Ok(parsed_lines)
}
#[cfg(test)]
mod test_parse_pitches {
    use super::*;

    #[test]
    fn valid() {
        let input = "A3\nE2// Comment\n\nG4BB2G4\n-\nE4".to_owned();
        let expected = vec![
            Line::Playable(vec![Pitch::A3]),
            Line::Playable(vec![Pitch::E2]),
            Line::Rest,
            Line::Playable(vec![Pitch::G4, Pitch::ASharpBFlat2, Pitch::G4]),
            Line::MeasureBreak,
            Line::Playable(vec![Pitch::E4]),
        ];
        assert_eq!(parse_pitches(input).unwrap(), expected);
    }
    #[test]
    fn invalid() {
        let input = "A3xyz\nE2\n\nG4BB.2\n-\nE4".to_owned();

        let error = parse_pitches(input).unwrap_err();
        let error_msg = format!("{error}");

        assert_eq!(
            error_msg,
            "Input 'xyz' on line 1 could not be parsed into a pitch.\nInput 'BB.2' on line 4 could not be parsed into a pitch."
        );
    }
}

fn parse_line(input_index: usize, mut input_line: &str) -> Result<Line<Vec<Pitch>>> {
    input_line = remove_comments(input_line);
    let line_content: String = remove_whitespace(input_line);

    if let Some(rest) = parse_rest(&line_content) {
        return Ok(rest);
    }
    if let Some(measure_break) = parse_measure_break(&line_content) {
        return Ok(measure_break);
    }
    parse_pitch(input_index, &line_content)
}
#[cfg(test)]
mod test_parse_line {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(parse_line(0, "").unwrap(), Line::Rest);
    }
    #[test]
    fn only_comment() {
        assert_eq!(parse_line(0, "  // Long comment.... ").unwrap(), Line::Rest);
    }
    #[test]
    fn measure_break() {
        assert_eq!(parse_line(0, "    --    ").unwrap(), Line::MeasureBreak);
        assert_eq!(parse_line(0, "- //comment").unwrap(), Line::MeasureBreak);
    }
    #[test]
    fn valid_pitch() {
        let expected = Line::Playable(vec![Pitch::GSharpAFlat2, Pitch::A4, Pitch::E3, Pitch::G2]);
        assert_eq!(parse_line(123, "    G#2A4  E3 G2 ").unwrap(), expected);
        assert_eq!(parse_line(123, "G#2A4E3 G2// Comment").unwrap(), expected);
    }
    #[test]
    fn test_parse_line_invalid_input() {
        let error = parse_line(4, "  Invalid Text  ").unwrap_err();
        let error_msg = format!("{error}");

        assert_eq!(
            error_msg,
            "Input 'InvalidText' on line 5 could not be parsed into a pitch."
        );
    }
}

fn remove_comments(input_line: &str) -> &str {
    input_line.split("//").next().unwrap_or(input_line)
}
#[cfg(test)]
mod test_remove_comments {
    use super::*;

    #[test]
    fn no_comment() {
        assert_eq!(remove_comments("Hello, World!"), "Hello, World!");
        assert_eq!(remove_comments("B3C1"), "B3C1");
    }
    #[test]
    fn single_comment() {
        assert_eq!(
            remove_comments("Hello, World! // This is a comment"),
            "Hello, World! "
        );
    }
    #[test]
    fn multiple_comments() {
        assert_eq!(
            remove_comments("Hello, // Comment 1\nWorld! // Comment 2"),
            "Hello, "
        );
    }
    #[test]
    fn comment_at_start() {
        assert_eq!(remove_comments("// Comment at the start"), "");
    }
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_rest(input_line: &str) -> Option<Line<Vec<Pitch>>> {
    if input_line.is_empty() {
        return Some(Line::Rest);
    }
    None
}
#[cfg(test)]
mod test_parse_rest {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(parse_rest(""), Some(Line::Rest));
    }
    #[test]
    fn pitch_input() {
        assert_eq!(parse_rest("G7"), None);
    }
}

fn parse_measure_break(input_line: &str) -> Option<Line<Vec<Pitch>>> {
    let unique_chars: HashSet<char> = input_line.chars().collect();
    if unique_chars == HashSet::<char>::from(['-'])
        || unique_chars == HashSet::<char>::from(['–'])
        || unique_chars == HashSet::<char>::from(['—'])
    {
        return Some(Line::MeasureBreak);
    }
    None
}
#[cfg(test)]
mod test_parse_measure_break {
    use super::*;

    #[test]
    fn measure_break_dash() {
        assert_eq!(parse_measure_break("-"), Some(Line::MeasureBreak));
    }
    #[test]
    fn measure_break_en_dash() {
        assert_eq!(parse_measure_break("–"), Some(Line::MeasureBreak));
    }
    #[test]
    fn measure_break_em_dash() {
        assert_eq!(parse_measure_break("—"), Some(Line::MeasureBreak));
    }
    #[test]
    fn empty_input() {
        assert_eq!(parse_measure_break(""), None);
    }
    #[test]
    fn no_measure_break() {
        assert_eq!(parse_measure_break("ABCDEF"), None);
    }
    #[test]
    fn whitespace_input() {
        assert_eq!(parse_measure_break(" "), None);
    }
    #[test]
    fn multiple_dashes() {
        assert_eq!(parse_measure_break("---"), Some(Line::MeasureBreak));
    }
    #[test]
    fn multiple_en_dashes() {
        assert_eq!(parse_measure_break("–––"), Some(Line::MeasureBreak));
    }
    #[test]
    fn mixed_dashes() {
        assert_eq!(parse_measure_break("-–—"), None);
    }
}

/// Parses input line to extract valid musical pitches, returning an error if any part of the
/// input line cannot be parsed into a pitch.
fn parse_pitch(input_index: usize, input_line: &str) -> Result<Line<Vec<Pitch>>> {
    let pattern = r"(?P<three_char_pitch>[A-G][#|♯|b|♭][0-9])|(?P<two_char_pitch>[A-G][0-9])";
    // let re = Regex::new(pattern);
    let re = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .expect("Regex pattern should be valid");
    let (matched_index_ranges, matched_pitches): (Vec<Vec<usize>>, Vec<Pitch>) = re
        .find_iter(input_line)
        .filter_map(|regex_match| match Pitch::from_str(regex_match.as_str()) {
            Ok(pitch) => Some(((regex_match.start()..regex_match.end()).collect(), pitch)),
            _ => None,
        })
        .unzip();

    let matched_indices: HashSet<usize> = matched_index_ranges.into_iter().flatten().collect();
    let input_indices: HashSet<usize> = (0..input_line.len()).collect();

    let unmatched_indices: Vec<usize> = input_indices
        .difference(&matched_indices)
        .sorted()
        .cloned()
        .collect();

    if !unmatched_indices.is_empty() {
        let line_number = input_index + 1;
        let consecutive_indices = consecutive_slices(&unmatched_indices);
        let error_msg = consecutive_indices
            .into_iter()
            .sorted()
            .filter_map(|unmatched_input_indices| {
                let first_idx = *unmatched_input_indices.first().unwrap();
                let last_idx = *unmatched_input_indices.last().unwrap();
                let unmatched_input = &input_line[first_idx..=last_idx];
                Some(format!(
                    "Input '{}' on line {} could not be parsed into a pitch.",
                    unmatched_input, line_number
                ))
            })
            .collect::<Vec<_>>()
            .join("\n");

        return Err(anyhow!(error_msg));
    }

    Ok(Line::Playable(matched_pitches))
}
#[cfg(test)]
mod test_parse_pitch {
    use super::*;

    #[test]
    fn single_natural_pitch() -> Result<()> {
        assert_eq!(parse_pitch(0, "A0")?, Line::Playable(vec![Pitch::A0]));
        assert_eq!(parse_pitch(0, "E6")?, Line::Playable(vec![Pitch::E6]));
        Ok(())
    }
    #[test]
    fn single_sharp_pitch() {
        assert_eq!(
            parse_pitch(0, "D#2").unwrap(),
            Line::Playable(vec![Pitch::DSharpEFlat2])
        );
    }
    #[test]
    fn single_flat_pitch() {
        assert_eq!(
            parse_pitch(0, "Db2").unwrap(),
            Line::Playable(vec![Pitch::CSharpDFlat2])
        );
        assert_eq!(
            parse_pitch(0, "Bb2").unwrap(),
            Line::Playable(vec![Pitch::ASharpBFlat2])
        );
    }
    #[test]
    fn case_insensitivity() {
        assert_eq!(
            parse_pitch(0, "A3").unwrap(),
            Line::Playable(vec![Pitch::A3])
        );
        assert_eq!(
            parse_pitch(0, "a3").unwrap(),
            Line::Playable(vec![Pitch::A3])
        );
        assert_eq!(
            parse_pitch(0, "Bb2").unwrap(),
            Line::Playable(vec![Pitch::ASharpBFlat2])
        );
        assert_eq!(
            parse_pitch(0, "bB2").unwrap(),
            Line::Playable(vec![Pitch::ASharpBFlat2])
        );
        assert_eq!(
            parse_pitch(0, "bb2").unwrap(),
            Line::Playable(vec![Pitch::ASharpBFlat2])
        );
    }
    #[test]
    fn multiple_pitches() {
        assert_eq!(
            parse_pitch(0, "C3G2A#1F8").unwrap(),
            Line::Playable(vec![Pitch::C3, Pitch::G2, Pitch::ASharpBFlat1, Pitch::F8])
        );
    }
    #[test]
    fn invalid_typo() {
        let error_msg = format!("{}", parse_pitch(12, "ZA2G#444B3").unwrap_err());
        let expected_error_msg = "Input 'Z' on line 13 could not be parsed into a pitch.\nInput '44' on line 13 could not be parsed into a pitch.";
        assert_eq!(error_msg, expected_error_msg);
    }
    #[test]
    fn invalid_pitch() {
        let error_msg = format!("{}", parse_pitch(28, "Fb3").unwrap_err());
        let expected_error_msg = "Input 'Fb3' on line 29 could not be parsed into a pitch.";
        assert_eq!(error_msg, expected_error_msg);
    }
    #[test]
    fn invalid_random() {
        let error_msg = format!("{}", parse_pitch(0, "baS3Q-hNr").unwrap_err());
        let expected_error_msg = "Input 'baS3Q-hNr' on line 1 could not be parsed into a pitch.";
        assert_eq!(error_msg, expected_error_msg);
    }
}

/// Returns a vector of consecutive slices of the input numbers.
///
/// This function does not sort the input vector and the consecutive slices are grouped together based
/// on the order of the input numbers as received.
/// Each returned slice is a reference to a subarray of `usize` elements from the original data array.
fn consecutive_slices(numbers: &[usize]) -> Vec<&[usize]> {
    let mut slice_start = 0;
    let mut result = Vec::new();
    for i in 1..numbers.len() {
        if numbers[i - 1] + 1 != numbers[i] {
            result.push(&numbers[slice_start..i]);
            slice_start = i;
        }
    }
    if !numbers.is_empty() {
        result.push(&numbers[slice_start..]);
    }
    result
}
#[cfg(test)]
mod test_consecutive_slices {
    use super::*;

    #[test]
    fn simple() {
        let flat_nums = vec![1, 2, 3, 4];
        let consecutive_nums = vec![vec![1, 2, 3, 4]];

        assert_eq!(consecutive_slices(&flat_nums), consecutive_nums);
    }
    #[test]
    fn complex() {
        let flat_nums = vec![1, 2, 3, 4, 113, 115, 116, 6, 7, 8];
        let consecutive_nums = vec![vec![1, 2, 3, 4], vec![113], vec![115, 116], vec![6, 7, 8]];

        assert_eq!(consecutive_slices(&flat_nums), consecutive_nums);
    }
    #[test]
    fn no_consecutive() {
        let flat_nums = vec![95, 65, 74, 96, 68, 29, 34, 32];
        let consecutive_nums = vec![
            vec![95],
            vec![65],
            vec![74],
            vec![96],
            vec![68],
            vec![29],
            vec![34],
            vec![32],
        ];

        assert_eq!(consecutive_slices(&flat_nums), consecutive_nums);
    }
}
