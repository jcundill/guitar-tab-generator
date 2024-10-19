use std::{borrow::Borrow, fmt::Debug};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::{composition::Line, guitar::{generate_pitch_fingerings_for_pitch, Guitar, PitchFingering}, parser::parse_lines, pitch::Pitch, renderer::{render_tab, transpose}, string_number::StringNumber};

type Grip = Vec<BoxFingering>;
type PossibleFingerings = Vec<BoxFingering>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Finger {
    IShift,
    I,
    M,
    A,
    P,
    PShift,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BoxFingering {
    line_idx: u8,
    position: u8,
    finger: u8,
    string: u8,
}

#[cfg(test)]
mod test_it_out {
    use super::create_arrangements;

    #[test]
    fn test_major_scale() {
        let input = "C3
        D3
        E3
        F3
        G3
        A3
        B3
        C4
        D4
        E4
        F4
        G4
        A4
        B4
        C5"
        .to_string();
    
        create_arrangements(input);            
    }

    #[test]
    fn test_diatonic_arps() {
        let input = "C3
        E3
        G3
        B3
        -
        D3
        F3
        A3
        C4
        -
        E3
        G3
        B3
        D4
        -
        F3
        A3
        C4
        E4
        -
        G3
        B3
        D4
        F4
        -
        A3
        C4
        E4
        G4
        -
        B3
        D4
        F4
        A4".to_string();

        create_arrangements(input);            
    }

}

pub fn create_arrangements(input: String) {
    let lines: Vec<Line<Vec<Pitch>>> = parse_lines(input).ok().unwrap();
    let last_line_idx = (lines.len() - 1) as u8;

    let guitar = Guitar::default();

    let possible_box_fingerings = convert_lines(&guitar, &lines);

    let trimmed_lines = possible_box_fingerings
        .clone()
        .into_iter()
        .skip_while(|line| line.is_empty())
        .collect_vec();

    let first_playable_line = trimmed_lines[0].clone();
    //    let start_fingerings: Vec<&Grip> = get_playable_fingerings_for_line(&first_playable_line);

    let transposed = transpose(first_playable_line.clone());
    //for start in transposed {

    // dijkstra over all the possible first playable grips
    let mut results = vec![];
    for playable_fingering in transposed {
        let next_start_grip: &Grip = &playable_fingering;
        let result = dijkstra(
            next_start_grip,
            |p: &Grip| successors(p, possible_box_fingerings.borrow()),
            |p: &Grip| at_end(p, last_line_idx),
        );
        if let Some(solution) = result {
            results.push(solution)
        }
    }

    let ordered_results = results.iter().sorted_by(|a, b| a.1.cmp(&b.1)).collect_vec();

    for solution in ordered_results { 
        print_tab_for_solution(solution, lines.clone(), guitar.clone());
    }
}

fn print_tab_for_solution(solution: &(Vec<Vec<BoxFingering>>, i32), lines: Vec<Line<Vec<Pitch>>>, guitar: Guitar) {
    println!("score: {}", solution.1);
    let pitch_fingerings = convert_to_pitch_fingering(solution.0.clone(), lines.clone());

    let width = 60;
    let padding = 2;
    let playback = None;
    let tab = render_tab(&pitch_fingerings, &guitar, width, padding, playback);

    println!("{}", tab);
}

fn convert_to_pitch_fingering(
    box_fingerings: Vec<Vec<BoxFingering>>,
    lines: Vec<Line<Vec<Pitch>>>,
) -> Vec<Line<Vec<PitchFingering>>> {
    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| match line {
            Line::MeasureBreak => Line::<Vec<PitchFingering>>::MeasureBreak,
            Line::Rest => Line::<Vec<PitchFingering>>::Rest,
            Line::Playable(pitches) => {
                convert_playable_to_pitch_fingerings(box_fingerings.clone(), pitches, idx)
            }
        })
        .collect_vec()
}

fn convert_playable_to_pitch_fingerings(
    box_fingerings: Vec<Vec<BoxFingering>>,
    _pitches: &[Pitch],
    idx: usize,
) -> Line<Vec<PitchFingering>> {
    let pitch_fingerings_for_line = box_fingerings
        .iter()
        .filter(|f| !f.is_empty() && f[0].line_idx == idx as u8)
        .map(|bf| {
            let chosen = &bf[0];
            PitchFingering {
                string_number: StringNumber::new(chosen.string).unwrap(),
                fret: (chosen.position + chosen.finger) - 1,
                pitch: Pitch::A0, // doen't matter for render
            }
        })
        .collect_vec();
    Line::<Vec<PitchFingering>>::Playable(pitch_fingerings_for_line)
}

fn successors(
    grip: &Grip,
    possible_box_fingerings: &[Vec<PossibleFingerings>],
) -> Vec<(Grip, i32)> {
    let playable_nexts = get_playable_positions_for_all_notes_on_next_line(
        possible_box_fingerings,
        grip[0].line_idx,
    );

    // each element in this slice is a potential next grip
    // that the current grip should be evaluated against

 playable_nexts
        .into_iter()
        .map(|playable_next: Grip| {
            let score = score_beat_transition(grip, &playable_next);
            let mut grip = vec![];
            for fingering in playable_next {
                grip.push(fingering.clone());
            }
            (grip.to_owned(), score)
        })
        .collect_vec()
}

fn get_playable_positions_for_all_notes_on_next_line(
    possible_box_fingerings: &[Vec<Vec<BoxFingering>>],
    curr_idx: u8,
) -> Vec<Vec<BoxFingering>> {
    let nexts = possible_box_fingerings
        .iter()
        .skip_while(|line| line.is_empty() || is_prior_idx(curr_idx, line))
        .collect_vec()[0];

    get_playable_fingerings_for_line(nexts)
 }

fn is_prior_idx(curr_idx: u8, nexts: &[PossibleFingerings]) -> bool {
    let next_idx = nexts[0][0].line_idx;
    next_idx <= curr_idx
}

fn get_playable_fingerings_for_line(
    possible_box_fingerings: &[PossibleFingerings],
) -> Vec<Grip> {
    match possible_box_fingerings.len() {
        1 => {
            // these are all the fingerings for a single note
            // all fingerings are separate grips
            transpose(possible_box_fingerings.to_vec())
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test_get_playable_fingerings {


    #[test]
    fn test_simple() {
        use super::*;

        let input = "C3".to_string();
        let lines: Vec<Line<Vec<Pitch>>> = parse_lines(input).ok().unwrap();

        let guitar = Guitar::default();

        // Vec of all the possible fingerings for each of the notes on each line
        let possible_box_fingerings = convert_lines(&guitar, &lines);

        // vec of all the possible fingerings for each of the notes in the first line
        // this is the fingers for a C#
        let res = get_playable_fingerings_for_line(&possible_box_fingerings[0]);

        assert_eq!(res.len(), 6);
    }
}

fn at_end(p: &Grip, last_line_idx: u8) -> bool {
    let x = &p[0];
    let curr_line_idx = x.line_idx;
    last_line_idx == curr_line_idx
}

#[cfg(test)]
mod test_convert {
    use super::*;
    use super::convert_lines;

    #[test]
    fn test_it() {
        let input = "C3
                    D3
                    E3
                    F3
                    G3
                    A3
                    B3
                    C4"
        .to_string();

        let (lines, fingerings) = fun_name(input);

        assert_eq!(fingerings.len(), lines.len());
        for fingering in fingerings {
            assert_eq!(fingering.len(), 1);
        }
    }

    #[test]
    fn test_it_again() {
        let input = "C3E3G3
                     C4"
        .to_string();

        let (lines, fingerings) = fun_name(input);

        assert_eq!(fingerings.len(), lines.len());
        assert_eq!(fingerings[0].len(), 3);
    }

    #[test]
    fn test_it_with_srests_and_breaks() {
        let input = "C3E3G3
                    -

                     C4G3"
            .to_string();

        let (_, fingerings) = fun_name(input);

        assert_eq!(fingerings.len(), 2);
        assert_eq!(fingerings[0].len(), 3);
        assert_eq!(fingerings[1].len(), 2);
        assert_eq!(fingerings[1][0][0].line_idx, 3);
        assert_eq!(fingerings[1][1][0].line_idx, 3);
    }

    fn fun_name(input: String) -> (Vec<Line<Vec<Pitch>>>, Vec<Vec<PossibleFingerings>>) {
        let lines: Vec<Line<Vec<Pitch>>> = parse_lines(input).ok().unwrap();

        let guitar = Guitar::default();

        let fingerings = convert_lines(&guitar, &lines);
        (lines, fingerings)
    }
}

fn convert_lines(guitar: &Guitar, lines: &[Line<Vec<Pitch>>]) -> Vec<Vec<PossibleFingerings>> {
    lines
        .iter()
        .enumerate()
        .map(|(line_idx, beat_input)| match beat_input {
            Line::MeasureBreak => vec![],
            Line::Rest => vec![],
            Line::Playable(beat_pitches) => {
                convert_beat_to_possible_fingerings(guitar, line_idx as u8, beat_pitches)
            }
        })
        .filter(|bfs| !bfs.is_empty())
        .collect_vec()
}

fn convert_beat_to_possible_fingerings(
    guitar: &Guitar,
    line_idx: u8,
    beat_pitches: &[Pitch],
) -> Vec<PossibleFingerings> {
    beat_pitches
        .iter()
        .map(|pitch| {
            let pfs = generate_pitch_fingerings_for_pitch(&guitar.string_ranges, pitch);
            convert_pitch_fingerings_to_box_fingerings(line_idx, &pfs)
        })
        .collect_vec()
}

#[cfg(test)]
mod test_convert_it {
    use crate::{box_fingering::{convert_pitch_fingering_to_box_fingering, BoxFingering}, guitar::PitchFingering, pitch::Pitch, string_number::StringNumber};


    #[test]
    fn test_simple() {
        let pf = PitchFingering {
            fret: 15,
            string_number: StringNumber::new(4).unwrap(),
            pitch: Pitch::F5,
        };
        let expected1 = BoxFingering {
            line_idx: 5,
            finger: 1,
            string: 4,
            position: 15,
        };

        assert!(convert_pitch_fingering_to_box_fingering(5, &pf).contains(&expected1))
    }

    #[test]
    fn test_high() {
        let pf = PitchFingering {
            fret: 1,
            string_number: StringNumber::new(1).unwrap(),
            pitch: Pitch::F4,
        };
        let expected1 = BoxFingering {
            line_idx: 4,
            finger: 1,
            string: 1,
            position: 1,
        };
        let expected2 = BoxFingering {
            line_idx: 4,
            finger: 0,
            string: 1,
            position: 2,
        };

        assert_eq!(
            convert_pitch_fingering_to_box_fingering(4, &pf),
            vec![expected2, expected1]
        )
    }
}

fn convert_pitch_fingering_to_box_fingering(
    line_idx: u8,
    pf: &PitchFingering,
) -> PossibleFingerings {
    let mut ret = vec![];
    let string = pf.string_number.get();

    for used_finger in 0..6 {
        let box_pos = (pf.fret as i8 - used_finger as i8) + 1; //i not i_shift is pos 0
        let on_fretboard = (pf.fret >= used_finger) && ((box_pos - 1 + used_finger as i8) < 18);
        if on_fretboard {
            let fb = BoxFingering {
                line_idx,
                position: box_pos as u8,
                string,
                finger: used_finger,
            };
            ret.push(fb);
        }
    }

    ret
}

fn convert_pitch_fingerings_to_box_fingerings(
    line_idx: u8,
    pfs: &[PitchFingering],
) -> PossibleFingerings {
    pfs.iter()
        .flat_map(|pf| convert_pitch_fingering_to_box_fingering(line_idx, pf))
        .collect_vec()
}

#[cfg(test)]
mod test_scoring {
    use super::*;

    #[test]
    fn test_simple_transition() {
        let curr = vec![BoxFingering {
            finger: 1,
            position: 1,
            string: 1,
            line_idx: 1,
        }];
        let next = vec![BoxFingering {
            finger: 3,
            position: 1,
            string: 1,
            line_idx: 1,
        }];

        let score = score_beat_transition(&curr, &next);
        assert_eq!(score, 0);
    }
}

fn score_beat_transition(curr: &[BoxFingering], next: &[BoxFingering]) -> i32 {
    match (curr.len() == 1, next.len() == 1) {
        (true, true) => score_single_note_transition(&curr[0], &next[0]),
        (false, false) => score_chord_to_chord_transition(curr, next),
        (true, false) => score_note_to_chord_transition(&curr[0], next),
        (false, true) => score_chord_to_note_transition(curr, &next[0]),
    }
}
const UNPLAYABLE: i32 = 10000;

fn score_chord_to_note_transition(_curr: &[BoxFingering], _next: &BoxFingering) -> i32 {
    todo!()
}

fn score_note_to_chord_transition(_curr: &BoxFingering, next: &[BoxFingering]) -> i32 {
    let chord_playability = score_chord_playability(next);
    if chord_playability != UNPLAYABLE {
        12
    } else {
        UNPLAYABLE
    }
}

fn score_chord_playability(next: &[BoxFingering]) -> i32 {
    if all_fingerings_in_same_box(next) {
        0
    } else {
        UNPLAYABLE
    }
}

fn all_fingerings_in_same_box(next: &[BoxFingering]) -> bool {
    next.iter()
        .map(|fingering| fingering.position)
        .unique()
        .collect_vec()
        .len()
        == 1
}

fn score_chord_to_chord_transition(_curr: &[BoxFingering], next: &[BoxFingering]) -> i32 {
    let chord_playability = score_chord_playability(next);
    if chord_playability != UNPLAYABLE {
        12
    } else {
        UNPLAYABLE
    }
}

fn score_single_note_transition(curr: &BoxFingering, next: &BoxFingering) -> i32 {
    let hand_movement = curr.position.abs_diff(next.position);
    let shifted_finger_next = match next.finger {
        0 | 5 => 1,
        _ => 0
    };
    let shifted_finger_curr = match curr.finger {
        0 | 5 => 1,
        _ => 0
    };
    let mut same_finger_skip = 0;
    if curr.string != next.string && curr.finger == next.finger{
             same_finger_skip = 1;
    }
    (hand_movement + shifted_finger_next + shifted_finger_curr + same_finger_skip).into()
}
