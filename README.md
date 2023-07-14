# guitar-tab-generator

[![Rust](https://github.com/noahbaculi/guitar-tab-generator/actions/workflows/rust.yml/badge.svg)](https://github.com/noahbaculi/guitar-tab-generator/actions/workflows/rust.yml)

Guitar tab generator from note names considering difficulty of different finger positions.

Old versions:

- [Java](https://github.com/noahbaculi/guitar-tab-generator_java) (2019 - 2022)
- [Typescript](https://github.com/noahbaculi/guitar-tab-generator_typescript) (2022)

Running To-Dos:

- [ ] re-examine namespace of functions (object functions vs standalone) (public vs private)
- [ ] handle measure breaks and commented lines and test
- [ ] `let non_zero_fret_avg = non_zero_frets.iter().sum::<usize>() as f32 / non_zero_frets.len() as f32;`
- [ ] filter unplayable fingering options from beat_fingering_candidates (based on the fret span and whether there are any candidates with smaller fret spans)
- [ ] [pathfinding](https://docs.rs/pathfinding/latest/pathfinding/)
- [ ] [property testing](https://altsysrq.github.io/proptest-book/)
- [ ] benchmarking via [Criterion](https://crates.io/crates/criterion)
- [ ] borrowed types vs box vs RC
- [ ] [Rayon](https://docs.rs/rayon/latest/rayon/#how-to-use-rayon) parallelism
- [ ] 