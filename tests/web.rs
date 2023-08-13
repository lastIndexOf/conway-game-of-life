//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use conway_game_of_life::Universe;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

fn input_spaceship() -> Universe {
    use conway_game_of_life::Universe;

    let mut universe = Universe::new(0, 0);
    universe.set_height(6);
    universe.set_width(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);

    universe
}

fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(0, 0);

    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    universe
}

#[wasm_bindgen_test]
fn test_tick() {
    let mut input_universe = input_spaceship();
    let mut expected_universe = expected_spaceship();

    input_universe.next_tick();

    assert_eq!(input_universe.get_cells(), expected_universe.get_cells());
}
