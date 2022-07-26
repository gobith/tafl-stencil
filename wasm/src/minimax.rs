use super::side::Side;
use super::state::State;

enum MINMAX {
    MIN,
    MAX,
}

impl MINMAX {
    fn opposite(&self) -> MINMAX {
        match *self {
            MINMAX::MIN => MINMAX::MAX,
            MINMAX::MAX => MINMAX::MIN,
        }
    }
}

pub fn next_state<const N: usize>(state: State<N>, side: Side, depth: usize) -> State<N> {
    let min_max = MINMAX::MAX;

    for tile_idx in 0..state.board_size() {
        let tile = state.board[tile_idx];
        if tile.is_side(side) {
            do_moves(state, tile_idx, side, min_max.opposite(), depth - 1)
        }
    }

    state
}

fn do_moves<const N: usize>(
    state: State<N>,
    tile_idx: usize,
    side: Side,
    min_max: MINMAX,
    depth: usize,
) -> State<N> {

    // +1

    // +7 state.row_size

    // -1 

    // -7 state.row_size
}

fn do_move<const N: usize>(
    state: State<N>,
    piece_idx: usize,
    direction: usize,
) -> Result<State<N>, ()> {
    Ok(state)
}
