use std::fmt;
use super::side::Side;
#[derive(Clone, Copy, Debug)]
pub enum GameStatus {
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
