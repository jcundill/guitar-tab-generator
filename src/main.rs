use std::collections::BTreeMap;

use guitar_tab_generator::*;
fn main() {
    // let tuning = StringCollection {
    //     e: Pitch::E4,
    //     B: Pitch::B3,
    //     G: Pitch::G3,
    //     D: Pitch::D3,
    //     A: Pitch::A2,
    //     E: Pitch::E2,
    // };
    let tuning = BTreeMap::from([
        (StringNumber::new(1).unwrap(), Pitch::E4),
        (StringNumber::new(2).unwrap(), Pitch::B3),
        (StringNumber::new(3).unwrap(), Pitch::G3),
        (StringNumber::new(4).unwrap(), Pitch::D3),
        (StringNumber::new(5).unwrap(), Pitch::A2),
        (StringNumber::new(6).unwrap(), Pitch::E2),
        (StringNumber::new(7).unwrap(), Pitch::E2),
        (StringNumber::new(8).unwrap(), Pitch::E2),
    ]);
    dbg!(&tuning);

    let _g = Guitar::new(tuning, 16).unwrap();
    // dbg!(_g);

    // let input_pitches = vec![vec![Pitch::G3], vec![Pitch::B3], vec![Pitch::D4, Pitch::G4]];

    // let _arr = Arrangement::new(_g, input_pitches);
    // dbg!(_arr);
}
