use std::cmp::Ordering;
use std::fmt;
use wasm_bindgen::prelude::*;

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
        // let mut t  = self.tafl.clone();
        // t.move_piece(start_idx, end_idx);
        // self.tafl = t;
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

#[derive(Debug, Clone)]
pub struct Tafl<const N: usize> {
    row_size: usize,
    history: Vec<State<N>>,
    state: State<N>,
    game_status: GameStatus,
    defender: Player,
    attacker: Player,
}

#[derive(Clone, Copy, Debug)]
struct State<const N: usize> {
    side: Side,
    row_size: usize,
    board: [Tile; N],
}

pub fn hnefatafl() -> Tafl<121> {
    use Tile::*;
    let state = State {
        side: Side::Attacker,
        row_size: 11,
        board: [
            Castle, Empty, Empty, Attacker, Attacker, Attacker, Attacker, Attacker, Empty, Empty,
            Castle, Empty, Empty, Empty, Empty, Empty, Attacker, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Attacker,
            Empty, Empty, Empty, Empty, Defender, Empty, Empty, Empty, Empty, Attacker, Attacker,
            Empty, Empty, Empty, Defender, Defender, Defender, Empty, Empty, Empty, Attacker,
            Attacker, Attacker, Empty, Defender, Defender, King, Defender, Defender, Empty,
            Attacker, Attacker, Attacker, Empty, Empty, Empty, Defender, Defender, Defender, Empty,
            Empty, Empty, Attacker, Attacker, Empty, Empty, Empty, Empty, Defender, Empty, Empty,
            Empty, Empty, Attacker, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Attacker, Empty, Empty, Empty, Empty,
            Empty, Castle, Empty, Empty, Attacker, Attacker, Attacker, Attacker, Attacker, Empty,
            Empty, Castle,
        ],
    };

    Tafl {
        row_size: 11,
        history: vec![state],
        state,
        game_status: GameStatus::Setup,
        defender: Player::Human,
        attacker: Player::Human,
    }
}

pub fn brandubh() -> Tafl<49> {
    use Tile::*;
    let state = State {
        side: Side::Attacker,
        row_size: 7,
        board: [
            Castle, Empty, Empty, Attacker, Empty, Empty, Castle, Empty, Empty, Empty, Attacker,
            Empty, Empty, Empty, Empty, Empty, Empty, Defender, Empty, Empty, Empty, Attacker,
            Attacker, Defender, King, Defender, Attacker, Attacker, Empty, Empty, Empty, Defender,
            Empty, Empty, Empty, Empty, Empty, Empty, Attacker, Empty, Empty, Empty, Castle, Empty,
            Empty, Attacker, Empty, Empty, Castle,
        ],
    };
    Tafl {
        row_size: 7,
        history: vec![state],
        state,
        game_status: GameStatus::Setup,
        defender: Player::Human,
        attacker: Player::Human,
    }
}

impl<const N: usize> fmt::Display for Tafl<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.row_size {
            for y in 0..self.row_size {
                let tile = self.state.board[x * self.row_size + y];

                let char = match tile {
                    Tile::Empty => '⬜',
                    Tile::Attacker => '🔴',
                    Tile::Defender => '⭕',
                    Tile::King => '⬤',
                    Tile::Castle => '⬛',
                    Tile::CastleWithKing => '⬤',
                };
                write!(f, "{} {}", char, '\t')?;
            }
            write!(f, "{} {} {}", '\n', '\n', '\n')?;
        }

        write!(
            f,
            "Defender: {} , Attacker: {} , Turn: {}",
            self.defender, self.attacker, self.state.side 
        )?;
        write!(f, "{}", '\n')?;

        Ok(())
    }
}

impl<const N: usize> Tafl<N> {
    fn start_game(&mut self, defender: Player, attacker: Player) -> () {
        self.defender = defender;
        self.attacker = attacker;
        self.game_status = GameStatus::Playing;
    }

    pub fn move_piece(&mut self, start_idx: usize, end_idx: usize) -> usize {
        match self.state.move_piece(start_idx, end_idx) {
            Ok(mut new_state) => {
                new_state.switch_side();
                self.history.push(self.state);
                self.state = new_state;
                1
            }
            Err(error_string) => {
                println!("Error {}", error_string);
                0
            }
        }
    }
}

impl<const N: usize> State<N> {
    // fn row_col(&self, index: usize) -> (usize, usize) {
    //     (index / size, index % self.row_size)
    // }

    fn switch_side(&mut self) -> () {
        self.side = match self.side {
            Side::Attacker => Side::Defender,
            Side::Defender => Side::Attacker,
        };
    }

    fn move_piece(&self, start_idx: usize, end_idx: usize) -> Result<State<N>, String> {
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
            },
            Side::Defender => match start_tile {
                Tile::Empty => Err("Nothing on start position".to_string()),
                Tile::Attacker => Err("Defender cannot move Attacker".to_string()),
                Tile::Defender | Tile::King => match self.validate_move(start_idx, end_idx) {
                    Ok(_) => Ok(self.next_state(start_idx, end_idx)),
                    Err(_) => Err("Illegal Move".to_string()),
                },
                Tile::Castle => Err("Cannot move from Castle".to_string()),
                Tile::CastleWithKing => Err("Cannot move from Castle".to_string()),
            },
        }
    }

    fn validate_move(&self, start_idx: usize, end_idx: usize) -> Result<(), String> {
        if self.board[end_idx] != Tile::Empty {
            return Err("End Tile is occupied".to_string());
        };

        // Range<usize >;

        // 1..4

        // let (start_row, start_col) = self.row_col(start_idx);

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
        let start = self.board[start_idx];
        let mut clone = self.clone();
        clone.board[end_idx] = start;
        clone.board[start_idx] = Tile::Empty;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Attacker,
    Defender,
}

impl Side {
    fn tile_is_same_side(&self, tile: Tile) -> bool {
        match tile {
            Tile::Attacker => *self == Side::Attacker,
            Tile::Defender => *self == Side::Defender,
            Tile::King => *self == Side::Defender,
            _ => false,
        }
    }

    fn tile_is_opposite_side(&self, tile: Tile) -> bool {
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

#[derive(Clone, Copy, Debug)]
enum GameStatus {
    Setup,
    Playing,
    Over(Side),
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let str = match *self {
            GameStatus::Setup => "Setup".to_owned(),
            GameStatus::Playing => "Playing".to_owned(),
            GameStatus::Over(winner) => format!("Game Over Winner: {}" , winner)
        };
        write!(f, "{}", str)
    }
}



#[derive(Clone, Copy, Debug)]
enum Player {
    Human,
    Computer(usize),
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Player::Human => "Human".to_owned(),
            Player::Computer(lvl) => format!("Computer lvl: {}", lvl),
        };
        write!(f, "{}", str)
    }
}

#[test]
fn tfl_move_pieve() {
    let mut tafl = Brandubh::new();
    tafl.move_piece(26, 5);
}

#[test]
fn tfl_start_game() {
    let mut tafl = Brandubh::new();
    tafl.start_game(1, 0, 2, 1);
    println!("{}", tafl)
}
