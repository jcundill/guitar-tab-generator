use guitar_tab_generator::*;

fn main() {
    let pitches = "E4
    Eb4
    E4
    Eb4
    E4
    B3
    D4
    C4
    -
    A2A3
    E3
    A3
    C3
    E3
    A3
    -
    E3B3
    E3
    Ab3
    E3
    Ab3
    B3
    -
    A2C4
    E3
    A3
    E3
    -
    E4
    Eb4
    E4
    Eb4
    E4
    B3
    D4
    C4
    -
    A2A3
    E3
    A3
    C3
    E3
    A3
    -
    E3B3
    E3
    Ab3
    E3
    C4
    B3
    A3
    -
    C4
    C4
    C4
    C4
    F4
    E4
    E4
    D4
    -
    Bb4
    A4
    A4
    G4
    F4
    E4
    D4
    C4
    -
    Bb3
    Bb3
    A3
    G3
    A3
    Bb3
    C4
    -
    D4
    Eb4
    Eb4
    E4
    F4
    A3
    C4
    -
    D4
    B3
    C4
    "
    .to_owned();

    let comp: CompositionInput = CompositionInput {
        pitches: pitches,
        guitar_capo: 0,
        guitar_num_frets: 18,
        tuning_name: "standard".to_owned(),
        num_arrangements: 1,
        width: 100,
        padding: 2,
        playback_index: Some(1),
        open_string_cost: 1000,
    };

    let comp = wrapper_create_arrangements(comp).unwrap();

    println!("{}", comp[0].tab);
    //println!("{}", comp[1].tab);
    //println!("{}", comp[2].tab);
}
