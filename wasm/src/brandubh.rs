use wasm_bindgen::prelude::*;
use super::player::Player;
use super::tafl::Tafl;
use super::tafl::brandubh;
use std::fmt;

#[wasm_bindgen]
pub struct Brandubh {
    tafl: Tafl<49>,
}

#[wasm_bindgen]
impl Brandubh {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Brandubh {
        Self { tafl: brandubh() }
    }

    #[wasm_bindgen]
    pub fn board(&self) -> Vec<u8> {
        self.tafl.state.board.map(|tile| tile.number()).to_vec()
    }

    #[wasm_bindgen]
    pub fn move_piece(&mut self, start_idx: usize, end_idx: usize) -> usize {
        self.tafl.move_piece(start_idx, end_idx)
    }
    #[wasm_bindgen]
    pub fn get_string(&self) -> String {
        "Test".into()
    }

    pub fn start_game(
        &mut self,
        defender_index: usize,
        defender_level: usize,
        attacker_index: usize,
        attacker_level: usize,
    ) -> String {
        let defender = match defender_index {
            1 => Player::Human,
            2 => Player::Computer(defender_level),
            _ => panic!("wrong player index"),
        };

        let attacker = match attacker_index {
            1 => Player::Human,
            2 => Player::Computer(attacker_level),
            _ => panic!("wrong player index"),
        };

        self.tafl.start_game(defender, attacker);
        "Test".into()
    }

    // #[wasm_bindgen]
    // pub fn get_strings(&self) -> Vec<String> {
    //    let str = "Test".into();
    //     let vec: Vec<String> = vec!([str]);
    //     vec
    // }
}

impl fmt::Display for Brandubh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tafl)
    }
}
