use super::side::Side;

#[derive(Clone, Copy, Debug, PartialEq)]

pub enum Tile {
    Attacker,
    Defender,
    King,
    Empty,
    Castle,
    CastleWithKing,
    CenterCastle,
    CenterCastleWithKing,
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
            Tile::CenterCastle => 6,
            Tile::CenterCastleWithKing => 7,
        }
    }

    pub fn can_be_entered_by(&self, tile: Tile) -> bool {
        if tile == Tile::King {
            if *self == Tile::Castle || *self == Tile::CenterCastle {
                return true;
            };
        }
        *self == Tile::Empty
    }

    pub fn entering_tile(&self, end_tile: Tile) -> Tile {
        match end_tile {
            Tile::Castle => Tile::CastleWithKing,
            Tile::CenterCastle => Tile::CenterCastleWithKing,
            _ => match *self {
                Tile::CenterCastleWithKing => Tile::King,
                _ => *self,
            },
        }
    }

    pub fn leaving_tile(&self) -> Tile {
        match *self {
            Tile::CenterCastleWithKing => Tile::CenterCastle,
            _ => Tile::Empty,
        }
    }

    pub fn is_king(&self) -> bool {
        match *self {
            Tile::King => true,
            Tile::CastleWithKing => true,
            Tile::CenterCastleWithKing => true,
            _ => false,
        }
    }

    pub fn is_side(&self, side: Side) -> bool {
        match *self {
            Tile::Empty => false,
            Tile::Attacker => side == Side::Attacker,
            Tile::Defender => side == Side::Defender,
            Tile::King => side == Side::Defender,
            _ => false,
        }
    }
}
