use std::fmt;
#[derive(Clone, Copy, Debug)]
pub enum Player {
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