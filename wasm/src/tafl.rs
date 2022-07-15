use super::game_status::GameStatus;
use super::player::Player;
use super::side::Side;
use super::state::State;
use super::tile::Tile;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Tafl<const N: usize> {
    row_size: usize,
    history: Vec<State<N>>,
    pub state: State<N>,
    game_status: GameStatus,
    defender: Player,
    attacker: Player,
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
            Attacker, Attacker, Empty, Defender, Defender, CenterCastleWithKing, Defender, Defender, Empty,
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
            Attacker, Defender, CenterCastleWithKing, Defender, Attacker, Attacker, Empty, Empty, Empty, Defender,
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
                    Tile::CenterCastle => '⬛' ,
                    Tile::CenterCastleWithKing => '⬤'
                };
                write!(f, "{} {}", char, '\t')?;
            }
            write!(f, "{} {} {}", '\n', '\n', '\n')?;
        }

        write!(
            f,
            "Status: {} , Defender: {} , Attacker: {} , Turn: {}",
            self.game_status, self.defender, self.attacker, self.state.side
        )?;
        write!(f, "{}", '\n')?;

        Ok(())
    }
}

impl<const N: usize> Tafl<N> {
    pub fn start_game(&mut self, defender: Player, attacker: Player) -> () {
        self.defender = defender;
        self.attacker = attacker;
        self.game_status = GameStatus::Playing;
    }

    pub fn move_piece(&mut self, start_idx: usize, end_idx: usize) -> Result<(), String> {
        match self.state.move_piece(start_idx, end_idx) {
            Ok(mut new_state) => {
                new_state.switch_side();
                self.history.push(self.state);
                self.state = new_state;
                self.validate_game_state();
                Ok(())
            }
            Err(error_string) => {
                println!("Error {}", error_string);
                Err(error_string)
            }
        }
    }

    fn validate_game_state(&mut self) -> () {
        match self.state.winner(){
            Some(side) => self.game_status = GameStatus::Over(side),
            None => {}
        }
    }

    pub fn status_string(&self) -> String {
        format!(
            "Status: {} , Defender: {} , Attacker: {} , Turn: {}",
            self.game_status, self.defender, self.attacker, self.state.side
        )
    }
}
