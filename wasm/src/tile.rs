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

    pub fn can_be_entered_by(&self, tile: Tile) -> bool {
        if tile == Tile::King {
            if *self == Tile::Castle {
                return true;
            };
        }
        *self == Tile::Empty
    }

    pub fn entering_tile(&self , end_tile: Tile) -> Tile {
        match end_tile {
            Tile::Castle => Tile::CastleWithKing,
            _ => *self,
        }
    }

    pub fn leaving_tile(&self) -> Tile {
        Tile::Empty
    }
}
