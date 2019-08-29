# Rust-Sdl-Framework

An SDL2-based 2D-game framework based on unsigned 4-bit integers written in Rust. An example binary can be found under [src/bin.rs](src/bin.rs)

## Colors

15 of the 16 possible values of the u4 ints are set by a color space table given to the renderer. The 16th value is always full alpha.

## Docs

You can build the documentation by running running `cargo rustdoc`

## Testing 

It is advisable to run the tests in the following manner: `cargo test -- --test-threads 1` as the tests otherwise block the SDL environment for each other.
