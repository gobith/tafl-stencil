use super::tile::Tile;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Attacker,
    Defender,
}


impl Side {
    pub fn tile_is_same_side(&self, tile: Tile) -> bool {
        match tile {
            Tile::Attacker => *self == Side::Attacker,
            Tile::Defender => *self == Side::Defender,
            Tile::King => *self == Side::Defender,
            _ => false,
        }
    }

    pub fn tile_is_opposite_side(&self, tile: Tile) -> bool {
        !self.tile_is_same_side(tile)
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let str = match *self {
            Side::Attacker => "Attacker",
            Side::Defender => "Defender",
        };
        write!(f, "{}", str)
    }
}