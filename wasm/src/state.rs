use super::side::Side;
use super::tile::Tile;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct State<const N: usize> {
    pub side: Side,
    pub row_size: usize,
    pub board: [Tile; N],
}

impl<const N: usize> State<N> {

    pub fn switch_side(&mut self) -> () {
        self.side = match self.side {
            Side::Attacker => Side::Defender,
            Side::Defender => Side::Attacker,
        };
    }

    pub fn winner(&self) -> Option<Side> {
        if self.board.contains(&Tile::CastleWithKing) {
            return Some(Side::Defender);
        };
        if !self.board.contains(&Tile::King) {
            return Some(Side::Attacker);
        };

        None
    }

    pub fn move_piece(&self, start_idx: usize, end_idx: usize) -> Result<State<N>, String> {
        let start_tile = self.board[start_idx];

        match self.side {
            Side::Attacker => match start_tile {
                Tile::Empty => Err("Nothing on start position".to_string()),
                Tile::Attacker => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => Ok(self.next_state(start_idx, end_idx)),
                    Err(str) => Err("Illegal Move: ".to_string() + &str),
                },
                Tile::Defender => Err("Attacker cannot move Defender".to_string()),
                Tile::King => Err("Attacker cannot move King".to_string()),
                Tile::Castle => Err("Cannot move from Castle".to_string()),
                Tile::CastleWithKing => Err("Cannot move from Castle".to_string()),
                Tile::CenterCastle => Err("Cannot move from Castle".to_string()),
                Tile::CenterCastleWithKing => Err("Attacker cannot move King".to_string())
            },
            Side::Defender => match start_tile {
                Tile::Empty => Err("Nothing on start position".to_string()),
                Tile::Attacker => Err("Defender cannot move Attacker".to_string()),
                Tile::Defender | Tile::King | Tile::CenterCastleWithKing => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => Ok(self.next_state(start_idx, end_idx)),
                    Err(_) => Err("Illegal Move".to_string()),
                },
                Tile::Castle => Err("Cannot move from Castle".to_string()),
                Tile::CastleWithKing => Err("Cannot move from Castle".to_string()),
                Tile::CenterCastle => Err("Cannot move from Castle".to_string())
            },
        }
    }

    fn validate_move(&self, start_idx: usize, end_idx: usize) -> Result<(), String> {

        let start_tile = self.board[start_idx];

        if !self.board[end_idx].can_be_entered_by(start_tile){
            return Err("End Tile is occupied".to_string());
        };

    

        let start_row = start_idx / self.row_size;
        let end_row = end_idx / self.row_size;
        let start_column = start_idx % self.row_size;
        let end_column = end_idx % self.row_size;

        println!("Start_idx {}", start_idx);
        println!("End_idx {}", end_idx);
        println!("Start_row {}", start_row);
        println!("End_row {}", end_row);
        println!("Start_column {}", start_column);
        println!("End_column {}", end_column);

        match start_row.cmp(&end_row) {
            Ordering::Less => {
                // End row greater than Start row
                if start_column != end_column {
                    return Err("Start end End Should be on the same Row or Column".to_string());
                };
                for n in 1..end_row - start_row {
                    if self.board[start_idx + (n * self.row_size)] != Tile::Empty {
                        return Err("Cannot move piece through another piece".to_string());
                    };
                }
            }
            Ordering::Greater => {
                // Start row greater than End row
                if start_column != end_column {
                    return Err("Start end End Should be on the same Row or Column".to_string());
                };

                for n in 1..start_row - end_row {
                    if self.board[start_idx - (n * self.row_size)] != Tile::Empty {
                        return Err("Cannot move piece through another piece".to_string());
                    };
                }
            }
            Ordering::Equal => {
                if start_column < end_column {
                    for n in start_idx + 1..end_idx {
                        println!("Index from Start is {}", n);
                        if self.board[n] != Tile::Empty {
                            return Err("Cannot move piece through another piece".to_string());
                        };
                    }
                } else {
                    for n in end_idx + 1..start_idx {
                        println!("Index from End is {}", n);
                        if self.board[n] != Tile::Empty {
                            return Err("Cannot move piece throught another piece".to_string());
                        };
                    }
                }
            }
        }

        Ok(())
    }

    fn next_state(&self, start_idx: usize, end_idx: usize) -> State<N> {
        let start_tile = self.board[start_idx];
        let end_tile = self.board[end_idx];
        let mut clone = self.clone();

    
        clone.board[end_idx] = start_tile.entering_tile(end_tile);
        clone.board[start_idx] = start_tile.leaving_tile();

        let end_row = end_idx / self.row_size;

        //left
        let index = end_idx - 2;
        println!("left index = {}", index);
        if index > 0 && index / self.row_size == end_row {
            if self.side.tile_is_same_side(clone.board[index])
                && self.side.tile_is_opposite_side(clone.board[index + 1])
            {
                clone.board[index + 1] = Tile::Empty;
            };
        };

        //right
        let index = end_idx + 2;
        println!("right index = {}", index);

        if index < self.row_size * self.row_size && index / self.row_size == end_row {
            if self.side.tile_is_same_side(clone.board[index])
                && self.side.tile_is_opposite_side(clone.board[index - 1])
            {
                clone.board[index - 1] = Tile::Empty;
            }
        };

        //up
        let index = end_idx as isize - (self.row_size * 2) as isize;

        println!("up index = {}", index);
        if index > 0 {
            let index = end_idx - (self.row_size * 2);
            if self.side.tile_is_same_side(clone.board[index])
                && self
                    .side
                    .tile_is_opposite_side(clone.board[index + self.row_size])
            {
                clone.board[index + self.row_size] = Tile::Empty;
            };
        };

        //down

        let index = end_idx + (self.row_size * 2);
        println!("down index = {}", index);
        if index < self.row_size * self.row_size {
            if self.side.tile_is_same_side(clone.board[index])
                && self
                    .side
                    .tile_is_opposite_side(clone.board[index - self.row_size])
            {
                clone.board[index - self.row_size] = Tile::Empty;
            };
        };

        clone
    }
}
