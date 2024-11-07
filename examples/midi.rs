use core::str;

use guitar_tab_generator::{wrapper_create_arrangements, CompositionInput};
use midly::{num::u28, MidiMessage, Smf, TrackEventKind};

static NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

fn main() {
    let smf = Smf::parse(include_bytes!("./All_Of_Me.mid"))
        .map_err(|e| format!("Failed to parse MIDI file: {}", e))
        .expect("Failed to parse MIDI file");
    let mut notes: Vec<String> = vec![];
    let mut last_delta: u28 = 0.into();
    for event in smf.tracks[0].iter() {
        match event.kind {
            TrackEventKind::Midi { message, .. } => match message {
                MidiMessage::NoteOn { key, vel } => {
                    let delta = event.delta;
                    if vel > 0 {
                        let note = get_note_name(key.as_int().into());
                        notes.push(note);
                    }
                    last_delta = delta;
                }
                MidiMessage::NoteOff { .. } => {
                    last_delta = event.delta;
                }
                _ => {}
            },
            _ => {}
        }
    }

    create_tab(notes.join("\n"));
}

fn create_tab(notes: String) {
    let comp: CompositionInput = CompositionInput {
        pitches: notes,
        guitar_capo: 0,
        guitar_num_frets: 18,
        tuning_name: "standard".to_owned(),
        num_arrangements: 1,
        width: 100,
        padding: 1,
        playback_index: Some(1),
        open_string_cost: 1000,
    };

    let comp = wrapper_create_arrangements(comp).unwrap();

    println!("{}", comp[0].tab);
}

fn get_note_name(midi_note: usize) -> String {
    let name = NOTE_NAMES[midi_note % 12].to_owned();
    let val = (midi_note as u8 / 12) - 2;

    format!("{}{}", name, val)
}
