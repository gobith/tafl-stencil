use wasm_bindgen::prelude::*;

#[derive(Clone , Copy)]
pub enum Tile {
    Empty,
    Attacker,
    Defender,
    King,
    Castle,
}

impl Tile {
    pub fn number(&self) -> u8 {
        match *self {
            Tile::Empty => 0,
            Tile::Attacker => 1,
            Tile::Defender => 2,
            Tile::King => 3,
            Tile::Castle => 4,
        }
    }
}

#[wasm_bindgen]
pub struct Size {
    width: u32,
    height: u32,
    contents: [Tile; 49],
}


#[wasm_bindgen]
impl Size {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Size {
        use Tile::*;
        Size {
            width,
            height,
            contents: [ Castle, Empty, Attacker, Attacker, Attacker, Empty, Castle, Empty, Empty, Empty,
            Attacker, Empty, Empty, Empty, Attacker, Empty, Empty, Defender, Empty, Empty,
            Attacker, Attacker, Attacker, Defender, King, Defender, Attacker, Attacker, Attacker,
            Empty, Empty, Defender, Empty, Empty, Attacker, Empty, Empty, Empty, Attacker, Empty,
            Empty, Empty, Castle, Empty, Attacker, Attacker, Attacker, Empty, Castle,],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn contents(&self) -> Vec<u8> {
        self.contents.map(|tile| tile.number()).to_vec()
    }
}
