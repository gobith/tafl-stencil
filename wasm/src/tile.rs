#[derive(Clone, Copy, Debug, PartialEq)]

pub enum Tile {
    Attacker,
    Defender,
    King,
    Empty,
    Castle,
    CastleWithKing,
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        *self == Tile::Empty
    }

    pub fn number(&self) -> u8 {
        match *self {
            Tile::Attacker => 1,
            Tile::Defender => 2,
            Tile::Empty => 0,
            Tile::King => 3,
            Tile::Castle => 4,
            Tile::CastleWithKing => 5,
        }
    }
}